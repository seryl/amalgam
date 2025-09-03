//! Package walker adapter that bridges Package types to the walker infrastructure

use crate::walkers::{DependencyGraph, TypeRegistry, WalkerError};
use amalgam_core::{
    ir::{Import, Module, TypeDefinition, IR},
    types::Type,
    ImportPathCalculator,
};
use std::collections::{HashMap, HashSet};
use tracing::{debug, instrument};

/// Adapter to convert Package's internal type storage to walker-compatible format
pub struct PackageWalkerAdapter;

impl PackageWalkerAdapter {
    /// Convert Package types for a version into TypeRegistry
    pub fn build_registry(
        types: &HashMap<String, TypeDefinition>,
        group: &str,
        version: &str,
    ) -> Result<TypeRegistry, WalkerError> {
        let mut registry = TypeRegistry::new();

        for (kind, type_def) in types {
            // Keep original case for the kind/type name
            let fqn = format!("{}.{}.{}", group, version, kind);
            registry.add_type(&fqn, type_def.clone());
        }

        Ok(registry)
    }

    /// Build dependency graph from type registry
    pub fn build_dependencies(registry: &TypeRegistry) -> DependencyGraph {
        let mut graph = DependencyGraph::new();

        for (fqn, type_def) in &registry.types {
            let refs = Self::extract_references(&type_def.ty);

            for ref_info in refs {
                // Build the full qualified name of the dependency
                let dep_fqn = if let Some(module) = &ref_info.module {
                    // Check if module already contains the full FQN
                    if module
                        .to_lowercase()
                        .ends_with(&format!(".{}", ref_info.name.to_lowercase()))
                    {
                        // Module is already the full FQN
                        module.to_lowercase()
                    } else {
                        // Module needs type name appended
                        format!("{}.{}", module, ref_info.name.to_lowercase())
                    }
                } else {
                    // Try to find the type in the same module first
                    let self_module = fqn.rsplit_once('.').map(|(m, _)| m).unwrap_or("");
                    format!("{}.{}", self_module, ref_info.name.to_lowercase())
                };

                // NOTE: Legacy io.k8s.* format conversion is now handled by SpecialCasePipeline
                // The conversion happens during IR processing, not during dependency walking

                // Add dependency if it exists in registry, is a k8s type, or is in the same module
                let self_module = fqn.rsplit_once('.').map(|(m, _)| m).unwrap_or("");
                let dep_module = dep_fqn.rsplit_once('.').map(|(m, _)| m).unwrap_or("");

                if registry.types.contains_key(&dep_fqn)
                    || dep_fqn.starts_with("k8s.io.")
                    || dep_fqn.starts_with("io.k8s.") // Also handle legacy io.k8s format
                    || self_module == dep_module
                {
                    graph.add_dependency(fqn, &dep_fqn);
                }
            }
        }

        graph
    }

    /// Generate IR with imports from registry and dependencies
    #[instrument(skip(registry, deps), fields(group = %group, version = %version), level = "debug")]
    pub fn generate_ir(
        registry: TypeRegistry,
        deps: DependencyGraph,
        group: &str,
        version: &str,
    ) -> Result<IR, WalkerError> {
        debug!("Generating IR for {}.{}", group, version);
        let mut ir = IR::new();

        // Create separate modules for each type (one type per module)
        for (fqn, type_def) in registry.types {
            // Extract the type name from the FQN (last component)
            let type_name = fqn.rsplit('.').next().unwrap_or(&fqn);
            let module_name = format!("{}.{}.{}", group, version, type_name);

            let mut module = Module {
                name: module_name,
                imports: Vec::new(),
                types: vec![type_def], // Single type per module
                constants: Vec::new(),
                metadata: Default::default(),
            };

            // Get cross-module dependencies and add imports for this specific type
            let cross_deps = deps.get_cross_module_deps(&fqn);
            let mut imports_map: HashMap<String, HashSet<String>> = HashMap::new();

            for dep_fqn in cross_deps {
                let (import_path, import_type_name) =
                    Self::calculate_import(&fqn, &dep_fqn, group, version);

                imports_map
                    .entry(import_path)
                    .or_default()
                    .insert(import_type_name);
            }

            // Convert imports map to Import structs for this module
            for (import_path, import_types) in imports_map {
                let alias = Self::generate_alias(&import_path);

                module.imports.push(Import {
                    path: import_path,
                    alias: Some(alias),
                    items: import_types.into_iter().collect(),
                });
            }

            ir.add_module(module);
        }

        Ok(ir)
    }

    /// Extract type references from a Type
    fn extract_references(ty: &Type) -> Vec<ReferenceInfo> {
        let mut refs = Vec::new();
        Self::collect_references(ty, &mut refs);
        refs
    }

    fn collect_references(ty: &Type, refs: &mut Vec<ReferenceInfo>) {
        match ty {
            Type::Reference { name, module } => {
                refs.push(ReferenceInfo {
                    name: name.clone(),
                    module: module.clone(),
                });
            }
            Type::Array(inner) => Self::collect_references(inner, refs),
            Type::Optional(inner) => Self::collect_references(inner, refs),
            Type::Map { value, .. } => Self::collect_references(value, refs),
            Type::Record { fields, .. } => {
                for field in fields.values() {
                    Self::collect_references(&field.ty, refs);
                }
            }
            Type::Union { types, .. } => {
                for t in types {
                    Self::collect_references(t, refs);
                }
            }
            Type::TaggedUnion { variants, .. } => {
                for t in variants.values() {
                    Self::collect_references(t, refs);
                }
            }
            Type::Contract { base, .. } => Self::collect_references(base, refs),
            _ => {}
        }
    }

    /// Calculate import path and type name for a dependency
    fn calculate_import(
        _from_fqn: &str,
        to_fqn: &str,
        group: &str,
        version: &str,
    ) -> (String, String) {
        let calc = ImportPathCalculator::new_standalone();

        // Extract type name from dependency FQN
        let type_name = to_fqn.rsplit('.').next().unwrap_or(to_fqn).to_string();

        // Handle k8s core types specially
        if to_fqn.starts_with("io.k8s.") {
            // Determine target version from FQN
            let target_version = if to_fqn.contains(".v1.") || to_fqn.contains(".meta.v1.") {
                "v1"
            } else if to_fqn.contains(".v1alpha1.") {
                "v1alpha1"
            } else if to_fqn.contains(".v1alpha3.") {
                "v1alpha3"
            } else if to_fqn.contains(".v1beta1.") {
                "v1beta1"
            } else if to_fqn.contains(".v2.") {
                "v2"
            } else if to_fqn.contains(".runtime.") || to_fqn.contains(".pkg.") {
                // Unversioned runtime types go in v0
                "v0"
            } else {
                "v1"
            };

            // Use unified calculator for k8s imports
            let path = calc.calculate(group, version, "k8s.io", target_version, &type_name);
            (path, type_name)
        } else {
            // Internal cross-version reference
            let to_parts: Vec<&str> = to_fqn.split('.').collect();
            if to_parts.len() >= 2 {
                let to_version = to_parts[to_parts.len() - 2];
                // Use unified calculator for internal imports
                let path = calc.calculate(group, version, group, to_version, &type_name);
                (path, type_name)
            } else {
                // Default to same directory
                (format!("./{}.ncl", type_name), type_name)
            }
        }
    }

    /// Generate an alias for an import path
    fn generate_alias(import_path: &str) -> String {
        // Extract meaningful part from path
        import_path
            .trim_end_matches(".ncl")
            .rsplit('/')
            .next()
            .unwrap_or("import")
            .to_string()
    }
}

#[derive(Debug, Clone)]
struct ReferenceInfo {
    name: String,
    module: Option<String>,
}
