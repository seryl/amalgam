//! Library interface for amalgam CLI components

pub mod manifest;
pub mod source_detector;
pub mod validate;
mod vendor;

use amalgam_parser::k8s_types::K8sTypesFetcher;
use anyhow::Result;
use std::fs;
use std::path::Path;
use tracing::info;

pub async fn handle_k8s_core_import(
    version: &str,
    output_base: &Path,
    _nickel_package: bool, // Legacy parameter - we now always generate manifests
) -> Result<()> {
    info!(
        "Fetching Kubernetes {} core types using unified pipeline...",
        version
    );

    // Automatically create k8s_io subdirectory if the output path doesn't end with it
    // This matches the behavior of package managers like npm, cargo, etc.
    let output_dir = if output_base
        .file_name()
        .map(|name| name.to_string_lossy())
        .map(|name| name == "k8s_io")
        .unwrap_or(false)
    {
        // Output path already ends with k8s_io, use it directly
        output_base.to_path_buf()
    } else {
        // Create k8s_io subdirectory in the specified output directory
        output_base.join("k8s_io")
    };

    info!("Generating k8s types in: {:?}", output_dir);

    // Create fetcher
    let fetcher = K8sTypesFetcher::new();

    // Fetch the OpenAPI schema
    let openapi = fetcher.fetch_k8s_openapi(version).await?;

    // Extract core types
    let types_map = fetcher.extract_core_types(&openapi)?;

    let total_types = types_map.len();
    info!("Extracted {} core types", total_types);

    // Create a NamespacedPackage to use the unified pipeline
    let mut package = amalgam_parser::package::NamespacedPackage::new("k8s.io".to_string());

    // Add all types to the package, organizing by API group
    // Type references come in the form io.k8s.api.{group}.{version}.{Type}
    // We need to extract the API group and organize accordingly
    for (type_ref, type_def) in types_map {
        // Extract the API group from the type reference
        // For example: io.k8s.api.apps.v1 -> apps
        //             io.k8s.api.core.v1 -> core (which maps to root)
        //             io.k8s.apimachinery.pkg.api.resource -> apimachinery/pkg/api/resource
        let api_group = if type_ref.group.starts_with("io.k8s.api.") {
            // Extract the API group (e.g., "apps", "batch", "core")
            let group_part = type_ref
                .group
                .strip_prefix("io.k8s.api.")
                .unwrap_or(&type_ref.group);

            // Core API group is special - it goes at the root
            if group_part == "core" || group_part.is_empty() {
                "k8s.io".to_string()
            } else {
                format!("k8s.io.{}", group_part)
            }
        } else if type_ref.group.starts_with("io.k8s.apimachinery.") {
            // Apimachinery types go in their own namespace
            format!(
                "k8s.io.apimachinery.{}",
                type_ref
                    .group
                    .strip_prefix("io.k8s.apimachinery.")
                    .unwrap_or("")
            )
        } else {
            // Default to using the group as-is
            type_ref.group.clone()
        };

        package.add_type(
            api_group,
            type_ref.version.clone(),
            type_ref.kind.clone(),
            type_def,
        );
    }

    // Process all API groups (not just k8s.io)
    let all_groups: Vec<String> = package.types.keys().cloned().collect();
    info!("Processing {} API groups", all_groups.len());

    // Generate files for each API group and version using the unified pipeline
    for api_group in &all_groups {
        let versions = package.versions(api_group);
        info!(
            "Processing API group {} with {} versions",
            api_group,
            versions.len()
        );

        for version_name in versions {
            let files = package.generate_version_files(api_group, &version_name);

            // Determine the output directory based on the API group structure
            // k8s.io -> k8s_io/{version}/
            // k8s.io.apps -> k8s_io/apps/{version}/
            // k8s.io.batch -> k8s_io/batch/{version}/
            let version_dir = if api_group == "k8s.io" {
                // Core API group goes at the root
                output_dir.join(&version_name)
            } else if api_group.starts_with("k8s.io.") {
                // Other API groups get their own subdirectory
                let group_part = api_group.strip_prefix("k8s.io.").unwrap_or(api_group);
                output_dir.join(group_part).join(&version_name)
            } else {
                // Fallback for any other pattern
                output_dir
                    .join(api_group.replace('.', "/"))
                    .join(&version_name)
            };

            fs::create_dir_all(&version_dir)?;

            for (filename, content) in files {
                let file_path = version_dir.join(&filename);
                fs::write(&file_path, content)?;
                info!("Generated {:?}", file_path);
            }
        }
    }

    // Generate hierarchical mod.ncl files for the ApiGroupVersioned structure
    {
        // Generate root mod.ncl that imports all API groups
        let mut root_imports = Vec::new();

        // Handle core API versions (at root level)
        if let Some(versions) = package.types.get("k8s.io") {
            for version in versions.keys() {
                root_imports.push(format!("  {} = import \"./{}/mod.ncl\",", version, version));
            }
        }

        // Handle other API groups
        for api_group in &all_groups {
            if api_group == "k8s.io" {
                continue; // Already handled above
            }

            if api_group.starts_with("k8s.io.") {
                let group_part = api_group.strip_prefix("k8s.io.").unwrap_or(api_group);

                // Generate mod.ncl for each API group
                let group_dir = output_dir.join(group_part);
                fs::create_dir_all(&group_dir)?;

                let mut group_imports = Vec::new();
                if let Some(versions) = package.types.get(api_group) {
                    for version in versions.keys() {
                        group_imports
                            .push(format!("  {} = import \"./{}/mod.ncl\",", version, version));
                    }
                }

                let group_content = format!(
                    "# Kubernetes {} API Group\n# Generated with ApiGroupVersioned structure\n\n{{\n{}\n}}\n",
                    group_part, group_imports.join("\n")
                );

                let group_mod_path = group_dir.join("mod.ncl");
                fs::write(&group_mod_path, group_content)?;
                info!("Generated API group module {:?}", group_mod_path);

                // Add to root imports
                root_imports.push(format!(
                    "  {} = import \"./{}/mod.ncl\",",
                    group_part, group_part
                ));
            }
        }

        let root_content = format!(
            "# Kubernetes Types Package\n# Generated with ApiGroupVersioned structure\n\n{{\n{}\n}}\n",
            root_imports.join("\n")
        );

        let root_path = output_dir.join("mod.ncl");
        fs::write(&root_path, root_content)?;
        info!("Generated root package module {:?}", root_path);

        // Generate Nickel-pkg.ncl manifest using the unified pipeline
        use amalgam_codegen::nickel_manifest::{NickelManifestConfig, NickelManifestGenerator};
        use amalgam_core::IR;

        // Build IR from the package - include all API groups
        let mut ir = IR::new();
        for (api_group, group_types) in &package.types {
            for (version_name, version_types) in group_types {
                for type_def in version_types.values() {
                    // Create proper module name based on API group
                    let module_name = if api_group.starts_with("k8s.io.") {
                        // For sub-groups, use the full path: io.k8s.api.apps.v1
                        let group_part = api_group.strip_prefix("k8s.io.").unwrap_or(api_group);
                        format!("io.k8s.api.{}.{}", group_part, version_name)
                    } else if api_group == "k8s.io" {
                        // Core API group
                        format!("io.k8s.api.core.{}", version_name)
                    } else {
                        // Fallback
                        format!("{}.{}", api_group, version_name)
                    };

                    let module = amalgam_core::ir::Module {
                        name: module_name,
                        imports: Vec::new(),
                        types: vec![type_def.clone()],
                        constants: Vec::new(),
                        metadata: Default::default(),
                    };
                    ir.add_module(module);
                }
            }
        }

        let manifest_config = NickelManifestConfig {
            name: "k8s-io".to_string(),
            version: "0.1.0".to_string(),
            minimal_nickel_version: "1.9.0".to_string(),
            description: format!(
                "Kubernetes {} core type definitions generated by Amalgam for Nickel",
                version
            ),
            authors: vec!["amalgam".to_string()],
            license: "Apache-2.0".to_string(),
            keywords: vec![
                "kubernetes".to_string(),
                "k8s".to_string(),
                "types".to_string(),
            ],
            base_package_id: None,
            local_dev_mode: false,
            local_package_prefix: None,
        };

        let generator = NickelManifestGenerator::new(manifest_config);
        let manifest_content = generator
            .generate_manifest(&ir, None)
            .expect("Failed to generate Nickel manifest");

        let manifest_path = output_dir.join("Nickel-pkg.ncl");
        fs::write(&manifest_path, manifest_content)?;
        info!("Generated Nickel manifest {:?}", manifest_path);
    }

    info!(
        "✅ Successfully generated {} Kubernetes {} types with proper cross-version imports",
        total_types, version
    );

    Ok(())
}
