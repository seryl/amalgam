//! Main validation orchestrator
//!
//! Coordinates all validation steps and generates reports.

use crate::binding_resolver::BindingResolver;
use crate::error::Result;
use crate::nickel_typechecker::NickelTypeChecker;
use crate::reporter::ValidationReport;
use crate::schema_validator::SchemaValidator;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub struct Validator {
    base_path: PathBuf,
    enable_typecheck: bool,
    enable_schema_validation: bool,
}

impl Validator {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            enable_typecheck: true,
            enable_schema_validation: true,
        }
    }

    /// Disable Nickel type checking
    pub fn without_typecheck(mut self) -> Self {
        self.enable_typecheck = false;
        self
    }

    /// Disable schema validation
    pub fn without_schema_validation(mut self) -> Self {
        self.enable_schema_validation = false;
        self
    }

    /// Run all validation steps
    pub fn validate_all(&self) -> Result<ValidationReport> {
        let start = Instant::now();
        let mut report = ValidationReport::new();

        // Step 1: Import binding resolution
        println!("🔍 Step 1: Validating import bindings...");
        let binding_resolver = BindingResolver::new(&self.base_path);
        let binding_report = binding_resolver.validate_all()?;

        println!(
            "   Files scanned: {}, Imports: {}, Dangling refs: {}, Case mismatches: {}",
            binding_report.files_scanned,
            binding_report.imports_found,
            binding_report.dangling_references.len(),
            binding_report.case_mismatches.len()
        );

        if !binding_report.dangling_references.is_empty() {
            println!("   ❌ Found dangling references:");
            for (file, type_name) in &binding_report.dangling_references {
                println!("      - {} uses undefined type '{}'", file.display(), type_name);
            }
        }

        if !binding_report.case_mismatches.is_empty() {
            println!("   ❌ Found case mismatches:");
            for (file, binding, expected) in &binding_report.case_mismatches {
                println!(
                    "      - {}: binding '{}' should be '{}'",
                    file.display(),
                    binding,
                    expected
                );
            }
        }

        report.add_binding_report(&binding_report);

        // Step 2: Nickel type checking (optional)
        if self.enable_typecheck {
            println!("\n🔍 Step 2: Type-checking Nickel files...");

            match NickelTypeChecker::new() {
                Ok(typechecker) => {
                    let typecheck_report = typechecker.typecheck_all(&self.base_path)?;

                    println!(
                        "   Files checked: {}, Passed: {}, Failed: {}",
                        typecheck_report.files_checked,
                        typecheck_report.passed,
                        typecheck_report.failed.len()
                    );

                    if !typecheck_report.failed.is_empty() {
                        println!("   ❌ Type check failures:");
                        for result in &typecheck_report.failed {
                            println!("      - {}", result.file.display());
                            println!("        {}", result.stderr.trim());
                        }
                    }

                    report.add_typecheck_report(&typecheck_report);
                }
                Err(e) => {
                    println!("   ⚠️  Nickel not available, skipping type checking: {}", e);
                }
            }
        }

        // Step 3: Schema validation (optional)
        if self.enable_schema_validation {
            println!("\n🔍 Step 3: Schema validation...");

            let schema_validator = SchemaValidator::new();
            if schema_validator.is_available() {
                // TODO: Implement schema validation report collection
                println!("   ⚠️  Schema validation not yet fully implemented");
            } else {
                println!("   ⚠️  kubeconform/kubectl not available, skipping schema validation");
            }
        }

        let duration = start.elapsed();
        report.set_duration(duration.as_millis());

        println!("\n" + "=".repeat(60));
        if report.summary.success {
            println!("✅ VALIDATION PASSED");
        } else {
            println!("❌ VALIDATION FAILED");
        }
        println!("Duration: {:.2}s", duration.as_secs_f64());
        println!("=".repeat(60) + "\n");

        Ok(report)
    }

    /// Validate and generate markdown report to file
    pub fn validate_and_report<P: AsRef<Path>>(&self, output_path: P) -> Result<ValidationReport> {
        let report = self.validate_all()?;

        let markdown = report.to_markdown();
        std::fs::write(output_path, markdown)?;

        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_validator_empty_directory() {
        let temp_dir = TempDir::new().unwrap();

        let validator = Validator::new(temp_dir.path())
            .without_typecheck()
            .without_schema_validation();

        let report = validator.validate_all().unwrap();

        assert!(report.summary.success);
        assert_eq!(report.summary.total_files, 0);
    }

    #[test]
    fn test_validator_with_valid_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.ncl");

        fs::write(
            &file_path,
            r#"
let ObjectMeta = import "./ObjectMeta.ncl" in
{
  metadata | ObjectMeta | optional,
}
"#,
        )
        .unwrap();

        let validator = Validator::new(temp_dir.path())
            .without_typecheck()
            .without_schema_validation();

        let report = validator.validate_all().unwrap();

        // Will have dangling reference to ObjectMeta (file doesn't exist)
        // but no case mismatches
        assert_eq!(
            report
                .binding_resolution
                .as_ref()
                .unwrap()
                .case_mismatches,
            0
        );
    }

    #[test]
    fn test_validator_detects_case_mismatch() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.ncl");

        fs::write(
            &file_path,
            r#"
let objectMeta = import "./ObjectMeta.ncl" in
{
  metadata | ObjectMeta | optional,
}
"#,
        )
        .unwrap();

        let validator = Validator::new(temp_dir.path())
            .without_typecheck()
            .without_schema_validation();

        let report = validator.validate_all().unwrap();

        // Should detect case mismatch
        assert_eq!(
            report
                .binding_resolution
                .as_ref()
                .unwrap()
                .case_mismatches,
            1
        );
        assert!(!report.summary.success);
    }
}
