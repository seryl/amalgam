//! Library interface for amalgam CLI components

pub mod manifest;
pub mod validate;
mod vendor;

use amalgam_codegen::nickel::NickelCodegen;
use amalgam_codegen::Codegen;
use amalgam_core::ir::Module;
use amalgam_parser::k8s_types::K8sTypesFetcher;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use tracing::info;

pub async fn handle_k8s_core_import(
    version: &str,
    output_dir: &PathBuf,
    nickel_package: bool,
) -> Result<()> {
    info!("Fetching Kubernetes {} core types...", version);

    // Create fetcher
    let fetcher = K8sTypesFetcher::new();

    // Fetch the OpenAPI schema
    let openapi = fetcher.fetch_k8s_openapi(version).await?;

    // Extract core types
    let types = fetcher.extract_core_types(&openapi)?;

    let total_types = types.len();
    info!("Extracted {} core types", total_types);

    // Group types by version
    let mut types_by_version: std::collections::HashMap<
        String,
        Vec<(
            amalgam_parser::imports::TypeReference,
            amalgam_core::ir::TypeDefinition,
        )>,
    > = std::collections::HashMap::new();

    for (type_ref, type_def) in types {
        types_by_version
            .entry(type_ref.version.clone())
            .or_default()
            .push((type_ref, type_def));
    }

    // Generate files for each version
    for (version, version_types) in &types_by_version {
        let version_dir = output_dir.join(version);
        std::fs::create_dir_all(&version_dir)?;

        // Generate Nickel files for each type
        for (type_ref, type_def) in version_types {
            // Create module for this type
            let module = Module {
                name: type_ref.kind.clone(),
                imports: Vec::new(),
                types: vec![type_def.clone()],
                constants: Vec::new(),
                metadata: Default::default(),
            };

            // Create IR with the module
            let mut ir = amalgam_core::IR::new();
            ir.add_module(module);

            // Generate Nickel code
            let mut codegen = NickelCodegen::new();
            let code = codegen.generate(&ir)?;

            // Write to file
            let filename = format!("{}.ncl", type_ref.kind.to_lowercase());
            let file_path = version_dir.join(&filename);
            std::fs::write(&file_path, code)?;
        }

        // Generate mod.ncl for this version
        let type_names: Vec<String> = version_types
            .iter()
            .map(|(tr, _)| tr.kind.clone())
            .collect();
        let mut mod_content = String::from("# Module exports for this version\n{\n");
        for type_name in &type_names {
            let file_name = type_name.to_lowercase();
            mod_content.push_str(&format!(
                "  {} = import \"./{}.ncl\",\n",
                type_name, file_name
            ));
        }
        mod_content.push_str("}\n");
        fs::write(version_dir.join("mod.ncl"), mod_content)?;

        info!(
            "Generated {} types for version {}",
            version_types.len(),
            version
        );
    }

    // Generate main mod.ncl if this is a package
    if nickel_package {
        let mut mod_content = String::from("# Kubernetes core types\n{\n");
        for version in types_by_version.keys() {
            mod_content.push_str(&format!(
                "  {} = import \"./{}/mod.ncl\",\n",
                version, version
            ));
        }
        mod_content.push_str("}\n");
        fs::write(output_dir.join("mod.ncl"), mod_content)?;
    }

    info!(
        "✓ Successfully generated {} Kubernetes core types",
        total_types
    );
    Ok(())
}
