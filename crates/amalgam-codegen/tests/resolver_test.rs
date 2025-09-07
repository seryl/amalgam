//! Integration tests for the type resolution system

use amalgam_codegen::resolver::{ResolutionContext, TypeResolver};
use amalgam_core::ir::{Import, Metadata, Module};
use std::collections::BTreeMap;

/// Create a test module with k8s imports
fn create_test_module_with_k8s_imports() -> Module {
    Module {
        name: "test".to_string(),
        imports: vec![
            Import {
                path: "../../k8s.io/apimachinery/v1/mod.ncl".to_string(),
                alias: Some("k8s_v1".to_string()),
                items: vec![],
            },
            Import {
                path: "../../k8s.io/api/core/v1/mod.ncl".to_string(),
                alias: Some("core_v1".to_string()),
                items: vec![],
            },
        ],
        types: vec![],
        constants: vec![],
        metadata: Metadata {
            source_language: None,
            source_file: None,
            version: None,
            generated_at: None,
            custom: BTreeMap::new(),
        },
    }
}

#[test]
fn test_k8s_type_resolution() -> Result<(), Box<dyn std::error::Error>> {
    let mut resolver = TypeResolver::new();
    let module = create_test_module_with_k8s_imports();
    let context = ResolutionContext::default();

    // Test resolving ObjectMeta (should use k8s_v1 alias)
    let resolved = resolver.resolve("ObjectMeta", &module, &context);
    assert_eq!(resolved, "k8s_v1.ObjectMeta");

    // Test resolving with full name
    let resolved = resolver.resolve(
        "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
        &module,
        &context,
    );
    assert_eq!(resolved, "k8s_v1.ObjectMeta");
    Ok(())
}

#[test]
fn test_crossplane_type_resolution() -> Result<(), Box<dyn std::error::Error>> {
    let mut resolver = TypeResolver::new();
    let module = Module {
        name: "test".to_string(),
        imports: vec![Import {
            path: "../../apiextensions.crossplane.io/v1/composition.ncl".to_string(),
            alias: Some("crossplane".to_string()),
            items: vec![],
        }],
        types: vec![],
        constants: vec![],
        metadata: Metadata {
            source_language: None,
            source_file: None,
            version: None,
            generated_at: None,
            custom: BTreeMap::new(),
        },
    };
    let context = ResolutionContext::default();

    // Should resolve based on v1 in the import path
    let resolved = resolver.resolve(
        "apiextensions.crossplane.io/v1/Composition",
        &module,
        &context,
    );
    assert_eq!(resolved, "crossplane.Composition");
    Ok(())
}

#[test]
fn test_unknown_type_resolution() -> Result<(), Box<dyn std::error::Error>> {
    let mut resolver = TypeResolver::new();
    let module = Module {
        name: "test".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Metadata {
            source_language: None,
            source_file: None,
            version: None,
            generated_at: None,
            custom: BTreeMap::new(),
        },
    };
    let context = ResolutionContext::default();

    // Unknown type should be returned as-is
    let resolved = resolver.resolve("SomeUnknownType", &module, &context);
    assert_eq!(resolved, "SomeUnknownType");
    Ok(())
}

#[test]
fn test_cache_behavior() -> Result<(), Box<dyn std::error::Error>> {
    let mut resolver = TypeResolver::new();
    let module = create_test_module_with_k8s_imports();
    let context = ResolutionContext::default();

    // First resolution
    let resolved1 = resolver.resolve("ObjectMeta", &module, &context);

    // Second resolution should hit cache
    let resolved2 = resolver.resolve("ObjectMeta", &module, &context);

    assert_eq!(resolved1, resolved2);
    assert_eq!(resolved1, "k8s_v1.ObjectMeta");
    Ok(())
}
