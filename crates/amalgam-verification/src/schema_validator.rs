//! Schema validation using kubectl or kubeconform
//!
//! Validates generated YAML against Kubernetes CRD schemas.

use crate::error::{Result, VerificationError};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct SchemaValidationResult {
    pub file: PathBuf,
    pub valid: bool,
    pub message: String,
}

#[derive(Debug, Default)]
pub struct SchemaValidationReport {
    pub files_validated: usize,
    pub valid: usize,
    pub invalid: Vec<SchemaValidationResult>,
}

impl SchemaValidationReport {
    pub fn is_success(&self) -> bool {
        self.invalid.is_empty()
    }
}

pub struct SchemaValidator {
    use_kubectl: bool,
    kubeconform_binary: Option<PathBuf>,
    kubectl_binary: Option<PathBuf>,
}

impl SchemaValidator {
    /// Create a new schema validator
    /// Prefers kubeconform (doesn't need cluster) over kubectl
    pub fn new() -> Self {
        let kubeconform_binary = which::which("kubeconform").ok();
        let kubectl_binary = which::which("kubectl").ok();

        Self {
            use_kubectl: kubeconform_binary.is_none() && kubectl_binary.is_some(),
            kubeconform_binary,
            kubectl_binary,
        }
    }

    /// Check if validation is available
    pub fn is_available(&self) -> bool {
        self.kubeconform_binary.is_some() || self.kubectl_binary.is_some()
    }

    /// Validate a YAML file against schemas
    pub fn validate_file<P: AsRef<Path>>(&self, file: P) -> Result<SchemaValidationResult> {
        let file = file.as_ref();

        if let Some(ref kubeconform) = self.kubeconform_binary {
            self.validate_with_kubeconform(kubeconform, file)
        } else if let Some(ref kubectl) = self.kubectl_binary {
            self.validate_with_kubectl(kubectl, file)
        } else {
            Err(VerificationError::Other(
                "Neither kubeconform nor kubectl available".to_string(),
            ))
        }
    }

    /// Validate YAML content (not a file)
    pub fn validate_content(&self, yaml_content: &str) -> Result<SchemaValidationResult> {
        // Write to temp file and validate
        use std::io::Write;
        let mut temp_file = tempfile::NamedTempFile::new()?;
        temp_file.write_all(yaml_content.as_bytes())?;
        temp_file.flush()?;

        self.validate_file(temp_file.path())
    }

    fn validate_with_kubeconform(
        &self,
        kubeconform: &Path,
        file: &Path,
    ) -> Result<SchemaValidationResult> {
        let output = Command::new(kubeconform)
            .arg("-strict")
            .arg("-summary")
            .arg(file)
            .output()
            .map_err(|e| VerificationError::ProcessFailed(e.to_string()))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let message = format!("{}\n{}", stdout, stderr);

        Ok(SchemaValidationResult {
            file: file.to_path_buf(),
            valid: output.status.success(),
            message,
        })
    }

    fn validate_with_kubectl(&self, kubectl: &Path, file: &Path) -> Result<SchemaValidationResult> {
        let output = Command::new(kubectl)
            .arg("apply")
            .arg("--dry-run=client")
            .arg("--validate=true")
            .arg("-f")
            .arg(file)
            .output()
            .map_err(|e| VerificationError::ProcessFailed(e.to_string()))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let message = format!("{}\n{}", stdout, stderr);

        Ok(SchemaValidationResult {
            file: file.to_path_buf(),
            valid: output.status.success(),
            message,
        })
    }
}

impl Default for SchemaValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_validator_creation() {
        let validator = SchemaValidator::new();
        // Should not panic
        println!("Validator available: {}", validator.is_available());
    }

    #[test]
    #[ignore = "Requires kubeconform or kubectl"]
    fn test_validate_valid_k8s_yaml() {
        let yaml = r#"
apiVersion: v1
kind: ConfigMap
metadata:
  name: test-config
data:
  key: value
"#;

        let validator = SchemaValidator::new();
        if !validator.is_available() {
            return; // Skip if no validator available
        }

        let result = validator.validate_content(yaml).unwrap();
        assert!(result.valid);
    }

    #[test]
    #[ignore = "Requires kubeconform or kubectl"]
    fn test_validate_invalid_yaml() {
        let yaml = r#"
apiVersion: v1
kind: InvalidKind
metadata:
  name: test
"#;

        let validator = SchemaValidator::new();
        if !validator.is_available() {
            return;
        }

        let result = validator.validate_content(yaml).unwrap();
        // Should fail validation
        println!("Validation result: {:?}", result);
    }
}
