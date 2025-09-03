/// Debug configuration and context for the compilation pipeline
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Debug configuration passed through the compilation pipeline
#[derive(Debug, Clone, Default)]
pub struct DebugConfig {
    /// Enable import debugging
    pub debug_imports: bool,
    /// Export path for debug information
    pub export_path: Option<PathBuf>,
    /// Tracing level (0=off, 1=info, 2=debug, 3=trace)
    pub trace_level: u8,
}

impl DebugConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_imports(mut self, enabled: bool) -> Self {
        self.debug_imports = enabled;
        self
    }

    pub fn with_export(mut self, path: Option<PathBuf>) -> Self {
        self.export_path = path;
        self
    }

    pub fn with_trace_level(mut self, level: u8) -> Self {
        self.trace_level = level;
        self
    }

    /// Check if import debugging is enabled
    pub fn should_debug_imports(&self) -> bool {
        self.debug_imports || self.trace_level >= 2
    }

    /// Check if we should export debug data
    pub fn should_export(&self) -> bool {
        self.export_path.is_some()
    }
}

/// Debug information collected during import resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDebugInfo {
    /// Module being processed
    pub module_name: String,
    /// Type being processed
    pub type_name: String,
    /// Imports found for this type
    pub imports: Vec<ImportDebugEntry>,
    /// Symbol table state
    pub symbol_table: HashMap<String, SymbolDebugInfo>,
    /// Import path calculations
    pub path_calculations: Vec<PathCalculationDebug>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDebugEntry {
    /// The dependency type name
    pub dependency: String,
    /// The generated import statement
    pub import_statement: String,
    /// The calculated import path
    pub import_path: String,
    /// Resolution strategy used
    pub resolution_strategy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolDebugInfo {
    pub name: String,
    pub module: String,
    pub group: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathCalculationDebug {
    pub from_module: String,
    pub to_module: String,
    pub from_group: String,
    pub from_version: String,
    pub to_group: String,
    pub to_version: String,
    pub calculated_path: String,
    pub type_name: String,
}

/// Aggregated debug information for the entire compilation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompilationDebugInfo {
    /// Debug info per module
    pub modules: HashMap<String, Vec<ImportDebugInfo>>,
    /// Module name transformations
    pub module_name_transforms: Vec<ModuleNameTransform>,
    /// Import extraction attempts
    pub import_extractions: Vec<ImportExtractionAttempt>,
    /// Errors encountered
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleNameTransform {
    pub original: String,
    pub normalized: String,
    pub group: String,
    pub version: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportExtractionAttempt {
    pub module: String,
    pub type_name: String,
    pub strategy: String,
    pub success: bool,
    pub imports_found: usize,
    pub error: Option<String>,
}

impl CompilationDebugInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_import_debug(&mut self, module: &str, info: ImportDebugInfo) {
        self.modules
            .entry(module.to_string())
            .or_default()
            .push(info);
    }

    pub fn add_module_transform(&mut self, transform: ModuleNameTransform) {
        self.module_name_transforms.push(transform);
    }

    pub fn add_extraction_attempt(&mut self, attempt: ImportExtractionAttempt) {
        self.import_extractions.push(attempt);
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    /// Export debug information to JSON file
    pub fn export_to_file(&self, path: &PathBuf) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}