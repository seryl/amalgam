//! Tests for CRD cross-package type references
//!
//! These tests verify that CRDs correctly import types from other packages,
//! particularly k8s core types like ObjectMeta, Volume, ResourceRequirements, etc.

use amalgam_codegen::resolver::{ResolutionContext, TypeResolver};
use amalgam_core::ir::{Import, Metadata, Module, TypeDefinition};
use amalgam_core::types::{Field, Type};
use std::collections::BTreeMap;

/// Test that a CRD referencing k8s types generates correct imports
#[test]
fn test_crd_with_k8s_type_references() {
    let mut resolver = TypeResolver::new();

    // Simulate a CRD that references k8s types
    // This would be like a CrossPlane Composition that uses ObjectMeta
    let module = Module {
        name: "apiextensions.crossplane.io.v1.composition".to_string(),
        imports: vec![
            Import {
                path: "../../../k8s_io/v1/objectmeta.ncl".to_string(),
                alias: Some("objectmeta".to_string()),
                items: vec!["ObjectMeta".to_string()],
            },
            Import {
                path: "../../../k8s_io/v1/volume.ncl".to_string(),
                alias: Some("volume".to_string()),
                items: vec!["Volume".to_string()],
            },
            Import {
                path: "../../../k8s_io/v1/resourcerequirements.ncl".to_string(),
                alias: Some("resourcerequirements".to_string()),
                items: vec!["ResourceRequirements".to_string()],
            },
        ],
        types: vec![TypeDefinition {
            name: "Composition".to_string(),
            ty: Type::Record {
                fields: BTreeMap::from([
                    (
                        "metadata".to_string(),
                        Field {
                            ty: Type::Reference {
                                name: "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta".to_string(),
                                module: None,
                            },
                            required: false,
                            description: None,
                            default: None,
                        },
                    ),
                    (
                        "spec".to_string(),
                        Field {
                            ty: Type::Record {
                                fields: BTreeMap::from([
                                    (
                                        "volumes".to_string(),
                                        Field {
                                            ty: Type::Array(Box::new(Type::Reference {
                                                name: "io.k8s.api.core.v1.Volume".to_string(),
                                                module: None,
                                            })),
                                            required: false,
                                            description: None,
                                            default: None,
                                        },
                                    ),
                                    (
                                        "resources".to_string(),
                                        Field {
                                            ty: Type::Reference {
                                                name: "io.k8s.api.core.v1.ResourceRequirements"
                                                    .to_string(),
                                                module: None,
                                            },
                                            required: false,
                                            description: None,
                                            default: None,
                                        },
                                    ),
                                ]),
                                open: false,
                            },
                            required: true,
                            description: None,
                            default: None,
                        },
                    ),
                ]),
                open: false,
            },
            documentation: Some("CrossPlane Composition CRD".to_string()),
            annotations: BTreeMap::new(),
        }],
        constants: vec![],
        metadata: Metadata {
            source_language: Some("openapi".to_string()),
            source_file: None,
            version: None,
            generated_at: None,
            custom: BTreeMap::new(),
        },
    };

    let context = ResolutionContext::default();

    // Resolve k8s ObjectMeta reference
    let resolved = resolver.resolve(
        "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
        &module,
        &context,
    );
    assert_eq!(
        resolved, "objectmeta.ObjectMeta",
        "ObjectMeta should resolve to the imported alias"
    );

    // Resolve k8s Volume reference
    let resolved = resolver.resolve("io.k8s.api.core.v1.Volume", &module, &context);
    assert_eq!(
        resolved, "volume.Volume",
        "Volume should resolve to the imported alias"
    );

    // Resolve k8s ResourceRequirements reference
    let resolved = resolver.resolve("io.k8s.api.core.v1.ResourceRequirements", &module, &context);
    assert_eq!(
        resolved, "resourcerequirements.ResourceRequirements",
        "ResourceRequirements should resolve to the imported alias"
    );
}

/// Test CRD with mixed local and external type references
#[test]
fn test_crd_with_mixed_type_references() {
    let mut resolver = TypeResolver::new();

    let module = Module {
        name: "example.io.v1.customresource".to_string(),
        imports: vec![
            Import {
                path: "../../../k8s_io/v1/objectmeta.ncl".to_string(),
                alias: Some("k8s_meta".to_string()),
                items: vec!["ObjectMeta".to_string()],
            },
            Import {
                path: "../../../k8s_io/v1/labelselector.ncl".to_string(),
                alias: Some("k8s_selector".to_string()),
                items: vec!["LabelSelector".to_string()],
            },
        ],
        types: vec![
            // Local type defined in this CRD
            TypeDefinition {
                name: "CustomSpec".to_string(),
                ty: Type::Record {
                    fields: BTreeMap::from([
                        (
                            "field1".to_string(),
                            Field {
                                ty: Type::String,
                                required: true,
                                description: None,
                                default: None,
                            },
                        ),
                        (
                            "field2".to_string(),
                            Field {
                                ty: Type::Number,
                                required: true,
                                description: None,
                                default: None,
                            },
                        ),
                    ]),
                    open: false,
                },
                documentation: None,
                annotations: BTreeMap::new(),
            },
            // Main CRD type that references both local and external types
            TypeDefinition {
                name: "CustomResource".to_string(),
                ty: Type::Record {
                    fields: BTreeMap::from([
                        (
                            "metadata".to_string(),
                            Field {
                                ty: Type::Reference {
                                    name: "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"
                                        .to_string(),
                                    module: None,
                                },
                                required: false,
                                description: None,
                                default: None,
                            },
                        ),
                        (
                            "spec".to_string(),
                            Field {
                                ty: Type::Reference {
                                    name: "CustomSpec".to_string(),
                                    module: None,
                                },
                                required: true,
                                description: None,
                                default: None,
                            },
                        ),
                        (
                            "selector".to_string(),
                            Field {
                                ty: Type::Reference {
                                    name: "io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector"
                                        .to_string(),
                                    module: None,
                                },
                                required: false,
                                description: None,
                                default: None,
                            },
                        ),
                    ]),
                    open: false,
                },
                documentation: None,
                annotations: BTreeMap::new(),
            },
        ],
        constants: vec![],
        metadata: Metadata {
            source_language: Some("crd".to_string()),
            source_file: None,
            version: None,
            generated_at: None,
            custom: BTreeMap::new(),
        },
    };

    let context = ResolutionContext::default();

    // Resolve external k8s reference
    let resolved = resolver.resolve(
        "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
        &module,
        &context,
    );
    assert_eq!(
        resolved, "k8s_meta.ObjectMeta",
        "External k8s type should resolve to import alias"
    );

    // Resolve local type reference
    let resolved = resolver.resolve("CustomSpec", &module, &context);
    assert_eq!(
        resolved, "CustomSpec",
        "Local type should resolve to itself without prefix"
    );

    // Resolve another external reference
    let resolved = resolver.resolve(
        "io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector",
        &module,
        &context,
    );
    assert_eq!(
        resolved, "k8s_selector.LabelSelector",
        "LabelSelector should resolve to its import alias"
    );
}

/// Test that unresolvable CRD references are returned as-is
#[test]
fn test_crd_with_unresolvable_references() {
    let mut resolver = TypeResolver::new();

    let module = Module {
        name: "test.io.v1.resource".to_string(),
        imports: vec![
            // Only import ObjectMeta, not PodSpec
            Import {
                path: "../../../k8s_io/v1/objectmeta.ncl".to_string(),
                alias: Some("meta".to_string()),
                items: vec!["ObjectMeta".to_string()],
            },
        ],
        types: vec![TypeDefinition {
            name: "TestResource".to_string(),
            ty: Type::Record {
                fields: BTreeMap::from([
                    (
                        "metadata".to_string(),
                        Field {
                            ty: Type::Reference {
                                name: "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta".to_string(),
                                module: None,
                            },
                            required: false,
                            description: None,
                            default: None,
                        },
                    ),
                    // This type is not imported, should remain as-is
                    (
                        "podSpec".to_string(),
                        Field {
                            ty: Type::Reference {
                                name: "io.k8s.api.core.v1.PodSpec".to_string(),
                                module: None,
                            },
                            required: false,
                            description: None,
                            default: None,
                        },
                    ),
                ]),
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        }],
        constants: vec![],
        metadata: Metadata {
            source_language: Some("crd".to_string()),
            source_file: None,
            version: None,
            generated_at: None,
            custom: BTreeMap::new(),
        },
    };

    let context = ResolutionContext::default();

    // Imported type should resolve
    let resolved = resolver.resolve(
        "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
        &module,
        &context,
    );
    assert_eq!(resolved, "meta.ObjectMeta");

    // Non-imported type should be returned as-is
    let resolved = resolver.resolve("io.k8s.api.core.v1.PodSpec", &module, &context);
    assert_eq!(
        resolved, "io.k8s.api.core.v1.PodSpec",
        "Non-imported type should be returned unchanged"
    );
}

/// Test CRD with versioned imports (v1, v1beta1, etc.)
#[test]
fn test_crd_with_versioned_imports() {
    let mut resolver = TypeResolver::new();

    let module = Module {
        name: "networking.k8s.io.v1beta1.ingress".to_string(),
        imports: vec![
            // Import from v1 (stable)
            Import {
                path: "../../../k8s_io/v1/objectmeta.ncl".to_string(),
                alias: Some("meta_v1".to_string()),
                items: vec!["ObjectMeta".to_string()],
            },
            // Import from v1beta1 (same version as CRD)
            Import {
                path: "../v1beta1/ingressbackend.ncl".to_string(),
                alias: Some("backend".to_string()),
                items: vec!["IngressBackend".to_string()],
            },
        ],
        types: vec![TypeDefinition {
            name: "Ingress".to_string(),
            ty: Type::Record {
                fields: BTreeMap::from([
                    // Reference to v1 type
                    (
                        "metadata".to_string(),
                        Field {
                            ty: Type::Reference {
                                name: "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta".to_string(),
                                module: None,
                            },
                            required: false,
                            description: None,
                            default: None,
                        },
                    ),
                    // Reference to v1beta1 type (same version)
                    (
                        "backend".to_string(),
                        Field {
                            ty: Type::Reference {
                                name: "networking.k8s.io.v1beta1.IngressBackend".to_string(),
                                module: None,
                            },
                            required: false,
                            description: None,
                            default: None,
                        },
                    ),
                ]),
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        }],
        constants: vec![],
        metadata: Metadata {
            source_language: Some("crd".to_string()),
            source_file: None,
            version: None,
            generated_at: None,
            custom: BTreeMap::new(),
        },
    };

    let context = ResolutionContext::default();

    // v1 type should resolve with its alias
    let resolved = resolver.resolve(
        "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
        &module,
        &context,
    );
    assert_eq!(
        resolved, "meta_v1.ObjectMeta",
        "v1 ObjectMeta should use v1 import alias"
    );

    // v1beta1 type should resolve with its alias
    let resolved = resolver.resolve(
        "networking.k8s.io.v1beta1.IngressBackend",
        &module,
        &context,
    );
    assert_eq!(
        resolved, "backend.IngressBackend",
        "v1beta1 IngressBackend should use backend alias"
    );
}
