//! Rich package generation for Phase 11

use amalgam_codegen::nickel_rich::{RichNickelGenerator, RichPackageConfig};
use amalgam_core::IR;
use anyhow::{Context, Result};
use std::path::Path;
use tracing::info;

/// Generate a rich Nickel package with enhanced features
pub async fn generate_rich_package(
    input_ir: &Path,
    output_dir: &Path,
    config: RichPackageConfig,
) -> Result<()> {
    info!("Generating rich Nickel package: {}", config.name);

    // Load the IR
    let ir_content = std::fs::read_to_string(input_ir)
        .with_context(|| format!("Failed to read IR file: {:?}", input_ir))?;

    let ir: IR = serde_json::from_str(&ir_content).with_context(|| "Failed to parse IR JSON")?;

    // Create generator
    let mut generator = RichNickelGenerator::new(config.clone());

    // Analyze the IR
    generator
        .analyze(&ir)
        .with_context(|| "Failed to analyze IR for rich package generation")?;

    // Generate the package structure
    generator
        .generate_package(output_dir)
        .with_context(|| format!("Failed to generate rich package at {:?}", output_dir))?;

    info!("✓ Generated rich Nickel package at {:?}", output_dir);

    // Report statistics
    info!("Package statistics:");
    info!("  - Name: {}", config.name);
    info!("  - Version: {}", config.version);
    info!("  - Pattern generation: {}", config.generate_patterns);
    info!("  - Examples included: {}", config.include_examples);
    info!("  - LSP-friendly: {}", config.lsp_friendly);

    Ok(())
}

/// Create default config for K8s packages
pub fn default_k8s_config() -> RichPackageConfig {
    RichPackageConfig {
        name: "k8s_io".to_string(),
        version: "1.31.0".to_string(),
        description: "Kubernetes API types with contracts and validation".to_string(),
        generate_patterns: true,
        include_examples: true,
        lsp_friendly: true,
        promoted_types: vec![
            "Pod".to_string(),
            "Service".to_string(),
            "Deployment".to_string(),
            "ConfigMap".to_string(),
            "Secret".to_string(),
            "Namespace".to_string(),
            "ServiceAccount".to_string(),
            "PersistentVolumeClaim".to_string(),
        ],
        api_groups: vec![
            "core".to_string(),
            "apps".to_string(),
            "batch".to_string(),
            "networking".to_string(),
            "storage".to_string(),
            "policy".to_string(),
            "rbac".to_string(),
        ],
    }
}

/// Create default config for CrossPlane packages
pub fn default_crossplane_config(provider: &str) -> RichPackageConfig {
    RichPackageConfig {
        name: format!("crossplane_{}", provider),
        version: "0.1.0".to_string(),
        description: format!("CrossPlane {} provider types", provider),
        generate_patterns: true,
        include_examples: true,
        lsp_friendly: true,
        promoted_types: match provider {
            "aws" => vec![
                "Instance".to_string(),
                "Bucket".to_string(),
                "DBInstance".to_string(),
                "SecurityGroup".to_string(),
                "VPC".to_string(),
            ],
            "gcp" => vec![
                "Instance".to_string(),
                "Bucket".to_string(),
                "CloudSQLInstance".to_string(),
                "Network".to_string(),
            ],
            "azure" => vec![
                "VirtualMachine".to_string(),
                "StorageAccount".to_string(),
                "SQLServer".to_string(),
                "VirtualNetwork".to_string(),
            ],
            _ => vec![],
        },
        api_groups: vec![
            "compute".to_string(),
            "storage".to_string(),
            "database".to_string(),
            "networking".to_string(),
        ],
    }
}
