//! Validation report generation
//!
//! Generates human-readable and machine-readable reports.

use crate::binding_resolver::BindingReport;
use crate::nickel_typechecker::TypeCheckReport;
use crate::schema_validator::SchemaValidationReport;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub timestamp: String,
    pub summary: ValidationSummary,
    pub binding_resolution: Option<BindingReportSummary>,
    pub type_checking: Option<TypeCheckReportSummary>,
    pub schema_validation: Option<SchemaValidationReportSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub success: bool,
    pub total_files: usize,
    pub duration_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindingReportSummary {
    pub files_scanned: usize,
    pub imports_found: usize,
    pub dangling_references: usize,
    pub case_mismatches: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeCheckReportSummary {
    pub files_checked: usize,
    pub passed: usize,
    pub failed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaValidationReportSummary {
    pub files_validated: usize,
    pub valid: usize,
    pub invalid: usize,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            summary: ValidationSummary {
                success: true,
                total_files: 0,
                duration_ms: 0,
            },
            binding_resolution: None,
            type_checking: None,
            schema_validation: None,
        }
    }

    pub fn add_binding_report(&mut self, report: &BindingReport) {
        self.binding_resolution = Some(BindingReportSummary {
            files_scanned: report.files_scanned,
            imports_found: report.imports_found,
            dangling_references: report.dangling_references.len(),
            case_mismatches: report.case_mismatches.len(),
        });

        if !report.is_success() {
            self.summary.success = false;
        }

        self.summary.total_files += report.files_scanned;
    }

    pub fn add_typecheck_report(&mut self, report: &TypeCheckReport) {
        self.type_checking = Some(TypeCheckReportSummary {
            files_checked: report.files_checked,
            passed: report.passed,
            failed: report.failed.len(),
        });

        if !report.is_success() {
            self.summary.success = false;
        }
    }

    pub fn add_schema_validation_report(&mut self, report: &SchemaValidationReport) {
        self.schema_validation = Some(SchemaValidationReportSummary {
            files_validated: report.files_validated,
            valid: report.valid,
            invalid: report.invalid.len(),
        });

        if !report.is_success() {
            self.summary.success = false;
        }
    }

    pub fn set_duration(&mut self, duration_ms: u128) {
        self.summary.duration_ms = duration_ms;
    }

    /// Generate markdown report
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Amalgam Verification Report\n\n");
        md.push_str(&format!("**Generated:** {}\n\n", self.timestamp));

        // Summary
        md.push_str("## Summary\n\n");
        if self.summary.success {
            md.push_str("✅ **PASS** - All validation levels passed\n\n");
        } else {
            md.push_str("❌ **FAIL** - Some validation levels failed\n\n");
        }

        md.push_str(&format!("- **Total Files:** {}\n", self.summary.total_files));
        md.push_str(&format!(
            "- **Duration:** {:.2}s\n\n",
            self.summary.duration_ms as f64 / 1000.0
        ));

        // Binding Resolution
        if let Some(ref binding) = self.binding_resolution {
            md.push_str("## Import Resolution\n\n");
            md.push_str(&format!("- Files scanned: {}\n", binding.files_scanned));
            md.push_str(&format!("- Imports found: {}\n", binding.imports_found));

            if binding.dangling_references > 0 {
                md.push_str(&format!(
                    "- ❌ Dangling references: {}\n",
                    binding.dangling_references
                ));
            } else {
                md.push_str("- ✅ Dangling references: 0\n");
            }

            if binding.case_mismatches > 0 {
                md.push_str(&format!(
                    "- ❌ Case mismatches: {}\n",
                    binding.case_mismatches
                ));
            } else {
                md.push_str("- ✅ Case mismatches: 0\n");
            }

            md.push_str("\n");
        }

        // Type Checking
        if let Some(ref typecheck) = self.type_checking {
            md.push_str("## Type Checking\n\n");
            md.push_str(&format!("- Files checked: {}\n", typecheck.files_checked));

            if typecheck.failed > 0 {
                md.push_str(&format!("- ❌ Failed: {}\n", typecheck.failed));
                md.push_str(&format!("- Passed: {}\n", typecheck.passed));
            } else {
                md.push_str(&format!("- ✅ Passed: {}/{}\n", typecheck.passed, typecheck.files_checked));
            }

            md.push_str("\n");
        }

        // Schema Validation
        if let Some(ref schema) = self.schema_validation {
            md.push_str("## Schema Validation\n\n");
            md.push_str(&format!("- Files validated: {}\n", schema.files_validated));

            if schema.invalid > 0 {
                md.push_str(&format!("- ❌ Invalid: {}\n", schema.invalid));
                md.push_str(&format!("- Valid: {}\n", schema.valid));
            } else {
                md.push_str(&format!("- ✅ Valid: {}/{}\n", schema.valid, schema.files_validated));
            }

            md.push_str("\n");
        }

        md
    }

    /// Generate JSON report
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_markdown())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binding_resolver::BindingReport;

    #[test]
    fn test_report_generation() {
        let mut report = ValidationReport::new();

        let binding_report = BindingReport {
            files_scanned: 10,
            imports_found: 50,
            dangling_references: vec![],
            case_mismatches: vec![],
            ..Default::default()
        };

        report.add_binding_report(&binding_report);
        report.set_duration(1500);

        let markdown = report.to_markdown();
        assert!(markdown.contains("PASS"));
        assert!(markdown.contains("Files scanned: 10"));
    }

    #[test]
    fn test_report_with_failures() {
        let mut report = ValidationReport::new();

        let binding_report = BindingReport {
            files_scanned: 10,
            imports_found: 50,
            dangling_references: vec![(std::path::PathBuf::from("test.ncl"), "Foo".to_string())],
            case_mismatches: vec![],
            ..Default::default()
        };

        report.add_binding_report(&binding_report);

        assert!(!report.summary.success);
        let markdown = report.to_markdown();
        assert!(markdown.contains("FAIL"));
    }
}
