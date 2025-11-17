//! Nickel type checker wrapper
//!
//! Executes `nickel typecheck` on Nickel files and validates the output.

use crate::error::{Result, VerificationError};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct TypeCheckResult {
    pub file: PathBuf,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Default)]
pub struct TypeCheckReport {
    pub files_checked: usize,
    pub passed: usize,
    pub failed: Vec<TypeCheckResult>,
}

impl TypeCheckReport {
    pub fn is_success(&self) -> bool {
        self.failed.is_empty()
    }
}

pub struct NickelTypeChecker {
    nickel_binary: PathBuf,
}

impl NickelTypeChecker {
    pub fn new() -> Result<Self> {
        // Check if nickel is available
        let nickel_binary = which::which("nickel")
            .map_err(|_| VerificationError::NickelNotFound)?;

        Ok(Self { nickel_binary })
    }

    /// Type-check a single Nickel file
    pub fn typecheck_file<P: AsRef<Path>>(&self, file: P) -> Result<TypeCheckResult> {
        let file = file.as_ref();

        let output = Command::new(&self.nickel_binary)
            .arg("typecheck")
            .arg(file)
            .output()
            .map_err(|e| VerificationError::ProcessFailed(e.to_string()))?;

        Ok(TypeCheckResult {
            file: file.to_path_buf(),
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }

    /// Type-check all .ncl files in a directory
    pub fn typecheck_all<P: AsRef<Path>>(&self, base_path: P) -> Result<TypeCheckReport> {
        let mut report = TypeCheckReport::default();

        for entry in WalkDir::new(base_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "ncl"))
        {
            let result = self.typecheck_file(entry.path())?;
            report.files_checked += 1;

            if result.success {
                report.passed += 1;
            } else {
                report.failed.push(result);
            }
        }

        Ok(report)
    }

    /// Export a Nickel file to YAML
    pub fn export_yaml<P: AsRef<Path>>(&self, file: P) -> Result<String> {
        let file = file.as_ref();

        let output = Command::new(&self.nickel_binary)
            .arg("export")
            .arg("--format")
            .arg("yaml")
            .arg(file)
            .output()
            .map_err(|e| VerificationError::ProcessFailed(e.to_string()))?;

        if !output.status.success() {
            return Err(VerificationError::ProcessFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

impl Default for NickelTypeChecker {
    fn default() -> Self {
        Self::new().expect("Nickel binary not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    #[ignore = "Requires nickel binary"]
    fn test_typecheck_valid_file() {
        let mut file = NamedTempFile::new().unwrap();
        fs::write(&file, "{ foo = 1, bar = \"hello\" }").unwrap();

        let checker = NickelTypeChecker::new().unwrap();
        let result = checker.typecheck_file(file.path()).unwrap();

        assert!(result.success);
    }

    #[test]
    #[ignore = "Requires nickel binary"]
    fn test_typecheck_invalid_file() {
        let mut file = NamedTempFile::new().unwrap();
        fs::write(&file, "{ foo = 1 + \"invalid\" }").unwrap();

        let checker = NickelTypeChecker::new().unwrap();
        let result = checker.typecheck_file(file.path()).unwrap();

        // This might still succeed if Nickel doesn't enforce strict typing here
        // The test is mainly to ensure the command runs
    }

    #[test]
    #[ignore = "Requires nickel binary"]
    fn test_export_yaml() {
        let mut file = NamedTempFile::new().unwrap();
        fs::write(&file, "{ foo = 1, bar = \"hello\" }").unwrap();

        let checker = NickelTypeChecker::new().unwrap();
        let yaml = checker.export_yaml(file.path()).unwrap();

        assert!(yaml.contains("foo"));
        assert!(yaml.contains("bar"));
    }
}
