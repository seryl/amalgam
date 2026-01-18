//! Nickel contract validation integration.
//!
//! This module provides integration with the Nickel language runtime,
//! allowing Rust code to validate values against Nickel contracts.
//!
//! # Example
//!
//! ```rust,ignore
//! use amalgam_runtime::NickelValidator;
//!
//! // Load contracts from a package
//! let validator = NickelValidator::from_package("./pkgs")?;
//!
//! // Validate a Rust value against a Nickel contract
//! let deployment = serde_json::json!({
//!     "apiVersion": "apps/v1",
//!     "kind": "Deployment",
//!     "metadata": { "name": "my-app" }
//! });
//!
//! validator.validate_json(&deployment, "k8s.apps.v1.Deployment")?;
//! ```

use crate::errors::{ValidationError, ValidationErrors};
use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Validator that uses Nickel contracts for validation.
///
/// This wraps the Nickel runtime and provides methods for:
/// - Loading contract definitions from packages
/// - Validating JSON/YAML values against contracts
/// - Caching contracts for performance
#[derive(Debug)]
pub struct NickelValidator {
    /// Root path for package imports
    package_root: PathBuf,

    /// Cached contract sources by qualified name
    /// e.g., "k8s.apps.v1.Deployment" -> Nickel source code
    contracts: HashMap<String, ContractInfo>,

    /// Configuration options
    config: ValidatorConfig,
}

/// Information about a loaded contract.
#[derive(Debug, Clone)]
struct ContractInfo {
    /// Path to the contract file
    #[allow(dead_code)]
    path: PathBuf,
    /// Nickel source code (cached)
    #[allow(dead_code)]
    source: Option<String>,
}

/// Configuration for the Nickel validator.
#[derive(Debug, Clone)]
pub struct ValidatorConfig {
    /// Whether to cache contract sources in memory
    pub cache_sources: bool,
    /// Maximum cache size (number of contracts)
    pub max_cache_size: usize,
    /// Whether to include Nickel error details in validation errors
    pub detailed_errors: bool,
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self {
            cache_sources: true,
            max_cache_size: 1000,
            detailed_errors: true,
        }
    }
}

impl NickelValidator {
    /// Create a new validator for a package directory.
    ///
    /// # Arguments
    ///
    /// * `package_root` - Path to the root of the Nickel packages (e.g., "./pkgs")
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let validator = NickelValidator::from_package("./pkgs")?;
    /// ```
    pub fn from_package(package_root: impl AsRef<Path>) -> Result<Self, ValidationErrors> {
        let package_root = package_root.as_ref().to_path_buf();

        if !package_root.exists() {
            return Err(ValidationErrors::from_error(ValidationError::new(
                "",
                format!("Package root does not exist: {}", package_root.display()),
            )));
        }

        Ok(Self {
            package_root,
            contracts: HashMap::new(),
            config: ValidatorConfig::default(),
        })
    }

    /// Create a validator with custom configuration.
    pub fn with_config(mut self, config: ValidatorConfig) -> Self {
        self.config = config;
        self
    }

    /// Validate a serializable Rust value against a Nickel contract.
    ///
    /// # Arguments
    ///
    /// * `value` - Any serializable Rust value
    /// * `contract_name` - Qualified contract name (e.g., "k8s.apps.v1.Deployment")
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// #[derive(Serialize)]
    /// struct MyConfig {
    ///     name: String,
    ///     replicas: i32,
    /// }
    ///
    /// let config = MyConfig { name: "app".into(), replicas: 3 };
    /// validator.validate(&config, "MyConfig")?;
    /// ```
    pub fn validate<T: Serialize>(
        &self,
        value: &T,
        contract_name: &str,
    ) -> Result<(), ValidationErrors> {
        // Serialize to JSON
        let json_value = serde_json::to_value(value).map_err(|e| {
            ValidationErrors::from_error(ValidationError::new(
                "",
                format!("Failed to serialize value: {}", e),
            ))
        })?;

        self.validate_json(&json_value, contract_name)
    }

    /// Validate a JSON value against a Nickel contract.
    pub fn validate_json(
        &self,
        value: &serde_json::Value,
        contract_name: &str,
    ) -> Result<(), ValidationErrors> {
        // Convert contract name to file path
        // e.g., "k8s.apps.v1.Deployment" -> "k8s_io/api/apps/v1.ncl"
        let contract_path = self.resolve_contract_path(contract_name)?;

        // Build the Nickel expression to evaluate
        // We import the contract and apply it to the JSON value
        let nickel_expr = self.build_validation_expression(value, &contract_path, contract_name)?;

        // Evaluate using Nickel runtime
        self.evaluate_nickel(&nickel_expr, contract_name)
    }

    /// Resolve a contract name to its file path.
    fn resolve_contract_path(&self, contract_name: &str) -> Result<PathBuf, ValidationErrors> {
        // Parse the contract name
        // Format: "k8s.apps.v1.Deployment" or "apiextensions.crossplane.io.v1.Composition"
        let parts: Vec<&str> = contract_name.split('.').collect();

        if parts.len() < 2 {
            return Err(ValidationErrors::from_error(ValidationError::new(
                "",
                format!("Invalid contract name: {}", contract_name),
            )));
        }

        // Build the path based on the contract structure
        let path = if contract_name.starts_with("k8s.") {
            // K8s types: k8s.apps.v1.Deployment -> k8s_io/api/apps/v1.ncl
            self.resolve_k8s_path(&parts[1..])
        } else {
            // Other packages: use dots as path separators
            self.resolve_generic_path(&parts)
        };

        if path.exists() {
            Ok(path)
        } else {
            Err(ValidationErrors::from_error(ValidationError::new(
                "",
                format!(
                    "Contract file not found: {} (looked at: {})",
                    contract_name,
                    path.display()
                ),
            )))
        }
    }

    /// Resolve a K8s contract path.
    fn resolve_k8s_path(&self, parts: &[&str]) -> PathBuf {
        // parts: ["apps", "v1", "Deployment"] or ["core", "v1", "Pod"]
        if parts.len() < 3 {
            // Likely a direct v1 type: ["v1", "Pod"]
            if parts.len() == 2 && parts[0].starts_with('v') {
                return self
                    .package_root
                    .join("k8s_io")
                    .join("api")
                    .join("core")
                    .join(format!("{}.ncl", parts[0]));
            }
            return self.package_root.join("k8s_io").join("mod.ncl");
        }

        let api_group = parts[0];
        let version = parts[1];
        // Type name is ignored for path - we load the whole version module

        if api_group == "core" {
            self.package_root
                .join("k8s_io")
                .join("api")
                .join("core")
                .join(format!("{}.ncl", version))
        } else {
            self.package_root
                .join("k8s_io")
                .join("api")
                .join(api_group)
                .join(format!("{}.ncl", version))
        }
    }

    /// Resolve a generic (non-K8s) contract path.
    fn resolve_generic_path(&self, parts: &[&str]) -> PathBuf {
        // Convert dots to path separators and underscores
        // "apiextensions.crossplane.io.v1.Composition" ->
        // "apiextensions_crossplane_io/v1/Composition.ncl"

        let domain_parts: Vec<&str> = parts
            .iter()
            .take_while(|p| !p.starts_with('v') || p.len() > 3)
            .copied()
            .collect();

        let rest: Vec<&str> = parts.iter().skip(domain_parts.len()).copied().collect();

        let domain = domain_parts.join("_").replace('.', "_");

        if rest.len() >= 2 {
            let version = rest[0];
            let type_name = rest[1];
            self.package_root
                .join(&domain)
                .join(version)
                .join(format!("{}.ncl", type_name))
        } else if rest.len() == 1 {
            self.package_root
                .join(&domain)
                .join(format!("{}.ncl", rest[0]))
        } else {
            self.package_root.join(&domain).join("mod.ncl")
        }
    }

    /// Build the Nickel expression to validate a value against a contract.
    fn build_validation_expression(
        &self,
        value: &serde_json::Value,
        contract_path: &Path,
        contract_name: &str,
    ) -> Result<String, ValidationErrors> {
        // Serialize the value to JSON
        let json_str = serde_json::to_string(value).map_err(|e| {
            ValidationErrors::from_error(ValidationError::new(
                "",
                format!("Failed to serialize value: {}", e),
            ))
        })?;

        // Extract the type name from the contract name
        let type_name = contract_name.rsplit('.').next().unwrap_or(contract_name);

        // Build the Nickel expression
        // This imports the module and applies the contract to the deserialized JSON
        let expr = format!(
            r#"
let contract = import "{}" in
let value = {} in
value | contract.{}
"#,
            contract_path.display(),
            json_str,
            type_name
        );

        Ok(expr)
    }

    /// Evaluate a Nickel expression and return validation result.
    fn evaluate_nickel(
        &self,
        expr: &str,
        contract_name: &str,
    ) -> Result<(), ValidationErrors> {
        use nickel_lang_core::error::NullReporter;
        use nickel_lang_core::eval::cache::CacheImpl;
        use nickel_lang_core::program::Program;

        // Create a program from the expression
        let mut program = Program::<CacheImpl>::new_from_source(
            expr.as_bytes(),
            "<validation>",
            std::io::sink(),
            NullReporter {},
        )
        .map_err(|e| {
            ValidationErrors::from_error(ValidationError::contract(
                "",
                contract_name,
                format!("Failed to parse Nickel expression: {:?}", e),
            ))
        })?;

        // Set the import path to include our package root
        program.add_import_paths(std::iter::once(&self.package_root));

        // Evaluate the program
        match program.eval_full() {
            Ok(_) => Ok(()),
            Err(e) => {
                // Extract error details
                let message = if self.config.detailed_errors {
                    // Format the error with debug representation
                    format!("Contract violation: {:?}", e)
                } else {
                    format!("Contract violation: {}", contract_name)
                };

                Err(ValidationErrors::from_error(ValidationError::contract(
                    "",
                    contract_name,
                    message,
                )))
            }
        }
    }
}

/// Trait for types that can be validated with Nickel contracts.
///
/// This is automatically implemented for types that implement `Serialize`.
pub trait ValidateWithNickel: Serialize + Sized {
    /// Validate this value against a Nickel contract.
    fn validate_with_nickel(
        &self,
        validator: &NickelValidator,
        contract_name: &str,
    ) -> Result<(), ValidationErrors> {
        validator.validate(self, contract_name)
    }
}

// Blanket implementation for all serializable types
impl<T: Serialize + Sized> ValidateWithNickel for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_k8s_path() {
        let validator = NickelValidator {
            package_root: PathBuf::from("/pkgs"),
            contracts: HashMap::new(),
            config: ValidatorConfig::default(),
        };

        let path = validator.resolve_k8s_path(&["apps", "v1", "Deployment"]);
        assert!(path.to_string_lossy().contains("k8s_io/api/apps/v1.ncl"));

        let path = validator.resolve_k8s_path(&["core", "v1", "Pod"]);
        assert!(path.to_string_lossy().contains("k8s_io/api/core/v1.ncl"));

        let path = validator.resolve_k8s_path(&["v1", "Pod"]);
        assert!(path.to_string_lossy().contains("k8s_io/api/core/v1.ncl"));
    }

    #[test]
    fn test_resolve_generic_path() {
        let validator = NickelValidator {
            package_root: PathBuf::from("/pkgs"),
            contracts: HashMap::new(),
            config: ValidatorConfig::default(),
        };

        let path =
            validator.resolve_generic_path(&["apiextensions", "crossplane", "io", "v1", "Composition"]);
        assert!(path
            .to_string_lossy()
            .contains("apiextensions_crossplane_io/v1/Composition.ncl"));
    }

    #[test]
    fn test_build_validation_expression() {
        let validator = NickelValidator {
            package_root: PathBuf::from("/pkgs"),
            contracts: HashMap::new(),
            config: ValidatorConfig::default(),
        };

        let value = serde_json::json!({
            "name": "test",
            "replicas": 3
        });

        let expr = validator
            .build_validation_expression(
                &value,
                Path::new("/pkgs/k8s_io/api/apps/v1.ncl"),
                "k8s.apps.v1.Deployment",
            )
            .unwrap();

        assert!(expr.contains("import"));
        assert!(expr.contains("Deployment"));
        assert!(expr.contains("\"name\""));
        assert!(expr.contains("\"replicas\""));
    }
}
