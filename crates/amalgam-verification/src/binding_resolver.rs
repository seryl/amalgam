//! Import binding resolution validator
//!
//! Validates that all import bindings match their usage in type contracts.
//! This is critical for catching the camelCase/PascalCase bug.

use crate::error::{Result, VerificationError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct ImportBinding {
    pub binding_name: String,
    pub type_name: String,
    pub import_path: String,
    pub line_number: usize,
}

#[derive(Debug, Clone)]
pub struct TypeUsage {
    pub type_name: String,
    pub line_number: usize,
    pub context: String,
}

#[derive(Debug, Default)]
pub struct BindingReport {
    pub files_scanned: usize,
    pub imports_found: usize,
    pub usages_found: usize,
    pub dangling_references: Vec<(PathBuf, String)>,
    pub case_mismatches: Vec<(PathBuf, String, String)>,
}

impl BindingReport {
    pub fn is_success(&self) -> bool {
        self.dangling_references.is_empty() && self.case_mismatches.is_empty()
    }
}

pub struct BindingResolver {
    base_path: PathBuf,
}

impl BindingResolver {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    /// Validate all .ncl files in the base path
    pub fn validate_all(&self) -> Result<BindingReport> {
        let mut report = BindingReport::default();

        for entry in WalkDir::new(&self.base_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "ncl"))
        {
            let path = entry.path();
            report.files_scanned += 1;

            let content = std::fs::read_to_string(path)?;

            // Extract imports and usages
            let imports = self.extract_imports(&content);
            let usages = self.extract_usages(&content);

            report.imports_found += imports.len();
            report.usages_found += usages.len();

            // Check for binding mismatches
            for import in &imports {
                if import.binding_name != import.type_name {
                    report.case_mismatches.push((
                        path.to_path_buf(),
                        import.binding_name.clone(),
                        import.type_name.clone(),
                    ));
                }
            }

            // Check for dangling references
            let imported_types: HashMap<_, _> = imports
                .iter()
                .map(|i| (i.type_name.clone(), i.clone()))
                .collect();

            for usage in &usages {
                if !imported_types.contains_key(&usage.type_name) {
                    // Check if it's defined in this file (not a dangling ref)
                    if !self.is_defined_locally(&content, &usage.type_name) {
                        report.dangling_references.push((
                            path.to_path_buf(),
                            usage.type_name.clone(),
                        ));
                    }
                }
            }
        }

        Ok(report)
    }

    /// Extract import bindings from Nickel code
    /// Matches: let TypeName = import "path/to/Type.ncl" in
    fn extract_imports(&self, content: &str) -> Vec<ImportBinding> {
        let mut imports = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // Match: let <binding> = import "<path>" in
            if trimmed.starts_with("let ") && trimmed.contains("= import") {
                if let Some(binding_part) = trimmed.strip_prefix("let ").and_then(|s| s.split('=').next()) {
                    let binding_name = binding_part.trim().to_string();

                    // Extract import path
                    if let Some(path_part) = trimmed.split('"').nth(1) {
                        let import_path = path_part.to_string();

                        // Extract type name from path (last component without .ncl)
                        let type_name = import_path
                            .trim_end_matches(".ncl")
                            .split('/')
                            .last()
                            .unwrap_or(&binding_name)
                            .to_string();

                        imports.push(ImportBinding {
                            binding_name,
                            type_name,
                            import_path,
                            line_number: line_num + 1,
                        });
                    }
                }
            }
        }

        imports
    }

    /// Extract type usages from Nickel code
    /// Matches: | TypeName | or | TypeName\n
    fn extract_usages(&self, content: &str) -> Vec<TypeUsage> {
        let mut usages = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // Match type contract usage: | TypeName |
            if trimmed.contains('|') {
                let parts: Vec<&str> = trimmed.split('|').collect();

                for part in parts {
                    let part_trimmed = part.trim();

                    // Skip empty parts and common keywords
                    if part_trimmed.is_empty()
                        || part_trimmed == "optional"
                        || part_trimmed == "default"
                        || part_trimmed.starts_with('{')
                        || part_trimmed.starts_with('[')
                    {
                        continue;
                    }

                    // Extract type name (handle Array TypeName, etc.)
                    let type_name = part_trimmed
                        .split_whitespace()
                        .last()
                        .unwrap_or(part_trimmed)
                        .to_string();

                    // Only track if it looks like a PascalCase type
                    if type_name.chars().next().map_or(false, |c| c.is_uppercase()) {
                        usages.push(TypeUsage {
                            type_name,
                            line_number: line_num + 1,
                            context: trimmed.to_string(),
                        });
                    }
                }
            }
        }

        usages
    }

    /// Check if a type is defined locally in the file
    fn is_defined_locally(&self, content: &str, type_name: &str) -> bool {
        // Check if this is the file defining the type
        // (typically the file exports a record with this type)
        content.contains(&format!("# {}", type_name))
            || content.contains(&format!("# Module: {}", type_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_imports() {
        let content = r#"
let ObjectMeta = import "../../k8s_io/v1/ObjectMeta.ncl" in
let PodSpec = import "./PodSpec.ncl" in
"#;

        let resolver = BindingResolver::new(".");
        let imports = resolver.extract_imports(content);

        assert_eq!(imports.len(), 2);
        assert_eq!(imports[0].binding_name, "ObjectMeta");
        assert_eq!(imports[0].type_name, "ObjectMeta");
        assert_eq!(imports[1].binding_name, "PodSpec");
        assert_eq!(imports[1].type_name, "PodSpec");
    }

    #[test]
    fn test_extract_usages() {
        let content = r#"
{
  metadata | ObjectMeta | optional,
  spec | PodSpec,
}
"#;

        let resolver = BindingResolver::new(".");
        let usages = resolver.extract_usages(content);

        assert!(usages.iter().any(|u| u.type_name == "ObjectMeta"));
        assert!(usages.iter().any(|u| u.type_name == "PodSpec"));
    }

    #[test]
    fn test_detects_case_mismatch() {
        let content = r#"
let objectMeta = import "../../k8s_io/v1/ObjectMeta.ncl" in
{
  metadata | ObjectMeta | optional,
}
"#;

        let resolver = BindingResolver::new(".");
        let imports = resolver.extract_imports(content);

        assert_eq!(imports[0].binding_name, "objectMeta");
        assert_eq!(imports[0].type_name, "ObjectMeta");
        assert_ne!(imports[0].binding_name, imports[0].type_name);
    }
}
