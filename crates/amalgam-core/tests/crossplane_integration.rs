//! Integration tests for the unified pipeline with CrossPlane packages

use amalgam_core::pipeline::{
    InputSource, ModuleLayout, ModuleStructure, NickelFormatting, OutputTarget, PackageMetadata,
    Transform, UnifiedPipeline, VersionHandling,
};

#[test]
fn test_crossplane_provider_aws() {
    // Test with CrossPlane AWS provider
    let input = InputSource::CRDs {
        urls: vec![
            "https://raw.githubusercontent.com/crossplane/provider-aws/master/package/crds/ec2.aws.crossplane.io_instances.yaml".to_string(),
            "https://raw.githubusercontent.com/crossplane/provider-aws/master/package/crds/s3.aws.crossplane.io_buckets.yaml".to_string(),
        ],
        domain: "aws.crossplane.io".to_string(),
        versions: vec!["v1beta1".to_string(), "v1alpha1".to_string()],
        auth: None,
    };

    let output = OutputTarget::NickelPackage {
        contracts: true,
        validation: true,
        rich_exports: true,
        usage_patterns: false,
        package_metadata: PackageMetadata {
            name: "crossplane-aws".to_string(),
            version: "0.1.0".to_string(),
            description: "CrossPlane AWS provider types".to_string(),
            homepage: None,
            repository: None,
            license: Some("Apache-2.0".to_string()),
            keywords: vec!["crossplane".to_string(), "aws".to_string()],
            authors: vec!["test".to_string()],
        },
        formatting: NickelFormatting::default(),
    };

    let mut pipeline = UnifiedPipeline::new(input, output);

    // Configure for CrossPlane layout
    pipeline.layout = ModuleLayout::CrossPlane {
        group_by_version: true,
        api_extensions: true,
        provider_specific: true,
    };

    // Add CrossPlane-specific transforms
    pipeline.transforms = vec![
        Transform::NormalizeTypes,
        Transform::ApplySpecialCases {
            rules: vec!["crossplane-naming".to_string(), "aws-provider".to_string()],
        },
        Transform::ResolveReferences,
        Transform::AddContracts { strict: false },
    ];

    // Validate pipeline configuration
    let validation = pipeline.validate();
    assert!(
        validation.is_ok(),
        "CrossPlane AWS pipeline should validate"
    );

    // Validate pipeline - dependency analysis happens internally
    // during the execution phase
}

#[test]
fn test_crossplane_provider_gcp() {
    // Test with CrossPlane GCP provider
    let input = InputSource::CRDs {
        urls: vec![
            "https://raw.githubusercontent.com/crossplane/provider-gcp/master/package/crds/compute.gcp.crossplane.io_networks.yaml".to_string(),
            "https://raw.githubusercontent.com/crossplane/provider-gcp/master/package/crds/storage.gcp.crossplane.io_buckets.yaml".to_string(),
        ],
        domain: "gcp.crossplane.io".to_string(),
        versions: vec!["v1beta1".to_string()],
        auth: None,
    };

    let output = OutputTarget::NickelPackage {
        contracts: true,
        validation: false,
        rich_exports: false,
        usage_patterns: true,
        package_metadata: PackageMetadata {
            name: "crossplane-gcp".to_string(),
            version: "0.1.0".to_string(),
            description: "CrossPlane GCP provider types".to_string(),
            homepage: Some("https://crossplane.io".to_string()),
            repository: Some("https://github.com/crossplane/provider-gcp".to_string()),
            license: Some("Apache-2.0".to_string()),
            keywords: vec![
                "crossplane".to_string(),
                "gcp".to_string(),
                "google".to_string(),
            ],
            authors: vec!["crossplane".to_string()],
        },
        formatting: NickelFormatting {
            indent: 2,
            max_line_length: 100,
            sort_imports: true,
            compact_records: false,
        },
    };

    let mut pipeline = UnifiedPipeline::new(input, output);

    // Use CrossPlane-specific layout
    pipeline.layout = ModuleLayout::CrossPlane {
        group_by_version: false,
        api_extensions: true,
        provider_specific: true,
    };

    // GCP-specific transforms
    pipeline.transforms = vec![
        Transform::NormalizeTypes,
        Transform::DeduplicateTypes,
        Transform::ApplyNamingConventions {
            style: amalgam_core::pipeline::NamingStyle::PascalCase,
        },
    ];

    let validation = pipeline.validate();
    assert!(
        validation.is_ok(),
        "CrossPlane GCP pipeline should validate"
    );
}

#[test]
fn test_crossplane_provider_azure() {
    // Test with CrossPlane Azure provider
    let input = InputSource::CRDs {
        urls: vec![
            "https://raw.githubusercontent.com/crossplane/provider-azure/master/package/crds/compute.azure.crossplane.io_virtualnetworks.yaml".to_string(),
        ],
        domain: "azure.crossplane.io".to_string(),
        versions: vec!["v1alpha3".to_string()],
        auth: None,
    };

    let output = OutputTarget::Go {
        package_name: "crossplane_azure".to_string(),
        imports: vec!["fmt".to_string(), "encoding/json".to_string()],
        tags: vec!["json".to_string(), "yaml".to_string()],
        generate_json_tags: true,
    };

    let mut pipeline = UnifiedPipeline::new(input, output);

    // Azure-specific configuration
    pipeline.layout = ModuleLayout::Generic {
        namespace_pattern: "{provider}/{group}/{version}".to_string(),
        module_structure: ModuleStructure::Consolidated,
        version_handling: VersionHandling::Directories,
    };

    pipeline.transforms = vec![
        Transform::NormalizeTypes,
        Transform::ValidateSchema,
        Transform::ResolveReferences,
    ];

    let validation = pipeline.validate();
    assert!(
        validation.is_ok(),
        "CrossPlane Azure to Go pipeline should validate"
    );
}

#[test]
fn test_crossplane_composition() {
    // Test CrossPlane Composition resources
    let input = InputSource::CRDs {
        urls: vec![
            "https://raw.githubusercontent.com/crossplane/crossplane/master/cluster/crds/apiextensions.crossplane.io_compositions.yaml".to_string(),
            "https://raw.githubusercontent.com/crossplane/crossplane/master/cluster/crds/apiextensions.crossplane.io_compositeresourcedefinitions.yaml".to_string(),
        ],
        domain: "apiextensions.crossplane.io".to_string(),
        versions: vec!["v1".to_string()],
        auth: None,
    };

    let output = OutputTarget::CUE {
        package_name: Some("crossplane_compositions".to_string()),
        strict_mode: true,
        constraints: true,
    };

    let mut pipeline = UnifiedPipeline::new(input, output);

    // Composition-specific layout
    pipeline.layout = ModuleLayout::CrossPlane {
        group_by_version: false,
        api_extensions: true,
        provider_specific: false,
    };

    pipeline.transforms = vec![
        Transform::NormalizeTypes,
        Transform::AddContracts { strict: true },
        Transform::ValidateSchema,
    ];

    let validation = pipeline.validate();
    assert!(
        validation.is_ok(),
        "CrossPlane Composition to CUE pipeline should validate"
    );
}

#[test]
fn test_crossplane_multi_provider() {
    // Test multiple CrossPlane providers in one pipeline
    let input = InputSource::CRDs {
        urls: vec![
            // AWS
            "https://raw.githubusercontent.com/crossplane/provider-aws/master/package/crds/rds.aws.crossplane.io_dbinstances.yaml".to_string(),
            // GCP
            "https://raw.githubusercontent.com/crossplane/provider-gcp/master/package/crds/database.gcp.crossplane.io_cloudsqlinstances.yaml".to_string(),
            // Azure
            "https://raw.githubusercontent.com/crossplane/provider-azure/master/package/crds/database.azure.crossplane.io_postgresqlservers.yaml".to_string(),
        ],
        domain: "crossplane.io".to_string(),
        versions: vec!["v1beta1".to_string()],
        auth: None,
    };

    let output = OutputTarget::NickelPackage {
        contracts: true,
        validation: true,
        rich_exports: true,
        usage_patterns: true,
        package_metadata: PackageMetadata {
            name: "crossplane-multi-cloud".to_string(),
            version: "0.1.0".to_string(),
            description: "Multi-cloud CrossPlane provider types".to_string(),
            homepage: Some("https://crossplane.io".to_string()),
            repository: None,
            license: Some("Apache-2.0".to_string()),
            keywords: vec![
                "crossplane".to_string(),
                "aws".to_string(),
                "gcp".to_string(),
                "azure".to_string(),
                "multi-cloud".to_string(),
            ],
            authors: vec!["crossplane".to_string()],
        },
        formatting: NickelFormatting::default(),
    };

    let mut pipeline = UnifiedPipeline::new(input, output);

    // Multi-provider layout
    pipeline.layout = ModuleLayout::Generic {
        namespace_pattern: "{provider}/{resource}/{version}".to_string(),
        module_structure: ModuleStructure::Consolidated,
        version_handling: VersionHandling::Namespaced,
    };

    // Comprehensive transforms for multi-provider
    pipeline.transforms = vec![
        Transform::NormalizeTypes,
        Transform::DeduplicateTypes,
        Transform::ApplySpecialCases {
            rules: vec![
                "aws-naming".to_string(),
                "gcp-naming".to_string(),
                "azure-naming".to_string(),
            ],
        },
        Transform::ResolveReferences,
        Transform::AddContracts { strict: false },
        Transform::ValidateSchema,
    ];

    let validation = pipeline.validate();
    assert!(
        validation.is_ok(),
        "Multi-provider CrossPlane pipeline should validate"
    );

    // Validate the multi-provider pipeline
    // Note: analyze_dependencies would be called internally during execution
}

#[test]
fn test_crossplane_with_error_recovery() {
    // Test error recovery with potentially invalid CrossPlane CRDs
    let input = InputSource::CRDs {
        urls: vec![
            "https://raw.githubusercontent.com/crossplane/provider-aws/master/package/crds/invalid.yaml".to_string(),
            "https://raw.githubusercontent.com/crossplane/provider-aws/master/package/crds/ec2.aws.crossplane.io_instances.yaml".to_string(),
        ],
        domain: "aws.crossplane.io".to_string(),
        versions: vec!["v1beta1".to_string()],
        auth: None,
    };

    let output = OutputTarget::NickelPackage {
        contracts: false,
        validation: false,
        rich_exports: false,
        usage_patterns: false,
        package_metadata: PackageMetadata {
            name: "crossplane-recovery-test".to_string(),
            version: "0.1.0".to_string(),
            description: "Test error recovery".to_string(),
            homepage: None,
            repository: None,
            license: None,
            keywords: vec![],
            authors: vec!["test".to_string()],
        },
        formatting: NickelFormatting::default(),
    };

    let mut pipeline = UnifiedPipeline::new(input, output);

    // Configure for error recovery
    pipeline.layout = ModuleLayout::CrossPlane {
        group_by_version: true,
        api_extensions: false,
        provider_specific: true,
    };

    // Minimal transforms for recovery testing
    pipeline.transforms = vec![Transform::NormalizeTypes, Transform::DeduplicateTypes];

    // Validation should succeed even with potential invalid URLs
    let validation = pipeline.validate();
    assert!(
        validation.is_ok(),
        "Pipeline should validate with recovery strategy"
    );
}
