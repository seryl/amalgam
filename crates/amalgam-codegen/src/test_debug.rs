/// Test utilities for capturing and validating debug information
use amalgam_core::debug::{CompilationDebugInfo, DebugConfig};
use std::path::PathBuf;

/// Test helper for capturing debug information during tests
pub struct TestDebugCapture {
    config: DebugConfig,
    capture_path: Option<PathBuf>,
}

impl Default for TestDebugCapture {
    fn default() -> Self {
        Self::new()
    }
}

impl TestDebugCapture {
    /// Create a new test debug capture with import debugging enabled
    pub fn new() -> Self {
        Self {
            config: DebugConfig::new().with_imports(true).with_trace_level(2),
            capture_path: None,
        }
    }

    /// Enable export to a temporary file
    pub fn with_export(mut self) -> Self {
        let temp_dir = std::env::temp_dir();
        let capture_file = temp_dir.join(format!("amalgam_test_debug_{}.json", std::process::id()));
        self.capture_path = Some(capture_file.clone());
        self.config = self.config.clone().with_export(Some(capture_file));
        self
    }

    /// Get the debug configuration
    pub fn config(&self) -> &DebugConfig {
        &self.config
    }

    /// Load captured debug information
    pub fn load_captured(&self) -> Result<CompilationDebugInfo, std::io::Error> {
        if let Some(path) = &self.capture_path {
            let json = std::fs::read_to_string(path)?;
            Ok(serde_json::from_str(&json)?)
        } else {
            Ok(CompilationDebugInfo::new())
        }
    }

    /// Clean up temporary files
    pub fn cleanup(&self) {
        if let Some(path) = &self.capture_path {
            let _ = std::fs::remove_file(path);
        }
    }
}

impl Drop for TestDebugCapture {
    fn drop(&mut self) {
        self.cleanup();
    }
}

/// Assertions for debug information
pub struct DebugAssertions;

impl DebugAssertions {
    /// Assert that a module name was transformed correctly
    pub fn assert_module_transform(
        debug_info: &CompilationDebugInfo,
        original: &str,
        expected_normalized: &str,
    ) -> Result<(), String> {
        let transform = debug_info
            .module_name_transforms
            .iter()
            .find(|t| t.original == original)
            .ok_or_else(|| format!("No transform found for module '{}'", original))?;

        if transform.normalized != expected_normalized {
            return Err(format!(
                "Module transform mismatch for '{}': expected '{}', got '{}'",
                original, expected_normalized, transform.normalized
            ));
        }
        Ok(())
    }

    /// Assert that imports were generated for a type
    pub fn assert_has_imports(
        debug_info: &CompilationDebugInfo,
        module: &str,
        type_name: &str,
        expected_count: usize,
    ) -> Result<(), String> {
        let module_debug = debug_info
            .modules
            .get(module)
            .ok_or_else(|| format!("No debug info for module '{}'", module))?;

        let type_debug = module_debug
            .iter()
            .find(|d| d.type_name == type_name)
            .ok_or_else(|| format!("No debug info for type '{}'", type_name))?;

        if type_debug.imports.len() != expected_count {
            return Err(format!(
                "Import count mismatch for type '{}': expected {}, got {}",
                type_name,
                expected_count,
                type_debug.imports.len()
            ));
        }
        Ok(())
    }

    /// Assert that a specific import exists
    pub fn assert_has_import_path(
        debug_info: &CompilationDebugInfo,
        module: &str,
        type_name: &str,
        dependency: &str,
        expected_path: &str,
    ) -> Result<(), String> {
        let module_debug = debug_info
            .modules
            .get(module)
            .ok_or_else(|| format!("No debug info for module '{}'", module))?;

        let type_debug = module_debug
            .iter()
            .find(|d| d.type_name == type_name)
            .ok_or_else(|| format!("No debug info for type '{}'", type_name))?;

        let import = type_debug
            .imports
            .iter()
            .find(|i| i.dependency == dependency)
            .ok_or_else(|| format!("No import found for dependency '{}'", dependency))?;

        if import.import_path != expected_path {
            return Err(format!(
                "Import path mismatch for dependency '{}': expected '{}', got '{}'",
                dependency, expected_path, import.import_path
            ));
        }
        Ok(())
    }

    /// Assert that extraction was successful
    pub fn assert_extraction_success(
        debug_info: &CompilationDebugInfo,
        module: &str,
        type_name: &str,
    ) -> Result<(), String> {
        let extraction = debug_info
            .import_extractions
            .iter()
            .find(|e| e.module == module && e.type_name == type_name)
            .ok_or_else(|| format!("No extraction attempt for type '{}'", type_name))?;

        if !extraction.success {
            return Err(format!(
                "Extraction failed for type '{}' with strategy '{}': {:?}",
                type_name, extraction.strategy, extraction.error
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_capture_creation() {
        let capture = TestDebugCapture::new();
        assert!(capture.config().should_debug_imports());
    }

    #[test]
    fn test_debug_export() -> Result<(), Box<dyn std::error::Error>> {
        let capture = TestDebugCapture::new().with_export();
        assert!(capture.capture_path.is_some());

        // Create a dummy debug info and export it
        let mut debug_info = CompilationDebugInfo::new();
        debug_info.add_error("Test error".to_string());

        if let Some(path) = &capture.capture_path {
            debug_info.export_to_file(path)?;

            // Load it back
            let loaded = capture.load_captured()?;
            assert_eq!(loaded.errors.len(), 1);
            assert_eq!(loaded.errors[0], "Test error");
        }
        Ok(())
    }
}
