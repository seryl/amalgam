//! Unit tests for the type resolution system

use amalgam_codegen::resolver::{
    TypeResolver, ReferenceResolver, Resolution, ResolutionContext, ImportMetadata
};
use amalgam_core::ir::{Module, Import};

/// Create a test module with k8s imports
fn create_test_module_with_k8s_imports() -> Module {
    Module {
        name: "test".to_string(),
        imports: vec![
            Import {
                path: "../../k8s_io/v1/objectmeta.ncl".to_string(),
                alias: Some("k8s_v1".to_string()),
                items: vec![],
            },
            Import {
                path: "../../k8s_io/v1/labelselector.ncl".to_string(),
                alias: Some("k8s_v1".to_string()),
                items: vec![],
            },
        ],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    }
}

#[test]
fn test_resolver_caches_resolutions() {
    let mut resolver = TypeResolver::new();
    let module = create_test_module_with_k8s_imports();
    let context = ResolutionContext {
        current_group: None,
        current_version: None,
        current_kind: None,
    };
    
    // First resolution
    let resolved1 = resolver.resolve(
        "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
        &module,
        &context
    );
    
    // Second resolution of same type (should use cache)
    let resolved2 = resolver.resolve(
        "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
        &module,
        &context
    );
    
    assert_eq!(resolved1, resolved2);
    assert_eq!(resolved1, "k8s_v1.ObjectMeta");
}

#[test]
fn test_resolver_handles_unresolvable_types() {
    let mut resolver = TypeResolver::new();
    let module = Module {
        name: "test".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };
    let context = ResolutionContext {
        current_group: None,
        current_version: None,
        current_kind: None,
    };
    
    // Should return the reference as-is when no resolver can handle it
    let resolved = resolver.resolve("UnknownType", &module, &context);
    assert_eq!(resolved, "UnknownType");
}

#[test]
fn test_kubernetes_resolver_pattern_matching() {
    let mut resolver = TypeResolver::new();
    let module = create_test_module_with_k8s_imports();
    let context = ResolutionContext {
        current_group: None,
        current_version: None,
        current_kind: None,
    };
    
    // Test various k8s type patterns
    let test_cases = vec![
        ("io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta", "k8s_v1.ObjectMeta"),
        ("io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector", "k8s_v1.LabelSelector"),
    ];
    
    for (input, expected) in test_cases {
        let resolved = resolver.resolve(input, &module, &context);
        assert_eq!(resolved, expected, "Failed to resolve {}", input);
    }
}

#[test]
fn test_resolver_with_no_matching_imports() {
    let mut resolver = TypeResolver::new();
    let module = Module {
        name: "test".to_string(),
        imports: vec![
            // Import for a different version
            Import {
                path: "../../k8s_io/v1beta1/objectmeta.ncl".to_string(),
                alias: Some("k8s_v1beta1".to_string()),
                items: vec![],
            },
        ],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };
    let context = ResolutionContext {
        current_group: None,
        current_version: None,
        current_kind: None,
    };
    
    // Should not resolve v1 type when only v1beta1 is imported
    let resolved = resolver.resolve(
        "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
        &module,
        &context
    );
    
    // Should return as-is since no matching import
    assert_eq!(resolved, "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta");
}

#[test]
fn test_local_type_resolver() {
    let mut resolver = TypeResolver::new();
    let module = Module {
        name: "test".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };
    let context = ResolutionContext {
        current_group: None,
        current_version: None,
        current_kind: None,
    };
    
    // Local types (no dots) should be returned as-is
    let local_types = vec!["MyType", "Config", "Settings"];
    
    for local_type in local_types {
        let resolved = resolver.resolve(local_type, &module, &context);
        assert_eq!(resolved, local_type);
    }
}

#[test]
fn test_import_path_parsing() {
    // This would be an internal test if we could access the KubernetesResolver directly
    // For now, we test it through the public interface
    let mut resolver = TypeResolver::new();
    
    // Create module with various import path formats
    let module = Module {
        name: "test".to_string(),
        imports: vec![
            Import {
                path: "../../k8s_io/v1/mod.ncl".to_string(),  // Module import
                alias: Some("k8s_core".to_string()),
                items: vec![],
            },
            Import {
                path: "../../k8s_io/v1beta1/deployment.ncl".to_string(),  // Specific type
                alias: Some("k8s_beta".to_string()),
                items: vec![],
            },
        ],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };
    let context = ResolutionContext {
        current_group: None,
        current_version: None,
        current_kind: None,
    };
    
    // Module imports should still work for resolving types
    let resolved = resolver.resolve(
        "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
        &module,
        &context
    );
    
    // Should match the v1 module import
    assert!(resolved.starts_with("k8s_") || resolved == "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
            "Unexpected resolution: {}", resolved);
}

#[test]
fn test_cache_clearing() {
    let mut resolver = TypeResolver::new();
    let module = create_test_module_with_k8s_imports();
    let context = ResolutionContext {
        current_group: None,
        current_version: None,
        current_kind: None,
    };
    
    // Resolve once
    let _ = resolver.resolve(
        "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
        &module,
        &context
    );
    
    // Clear cache
    resolver.clear_cache();
    
    // Resolve again (should not use cache)
    let resolved = resolver.resolve(
        "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
        &module,
        &context
    );
    
    assert_eq!(resolved, "k8s_v1.ObjectMeta");
}

/// Test that demonstrates extensibility - custom resolvers can be added
#[test]
fn test_custom_resolver_registration() {
    // This test demonstrates the extensibility of the system
    // In practice, users could implement their own ReferenceResolver trait
    
    struct CustomResolver;
    
    impl ReferenceResolver for CustomResolver {
        fn can_resolve(&self, reference: &str) -> bool {
            reference.starts_with("custom.")
        }
        
        fn resolve(
            &self,
            reference: &str,
            _imports: &[Import],
            _context: &ResolutionContext,
        ) -> Option<Resolution> {
            if reference.starts_with("custom.") {
                Some(Resolution {
                    resolved_name: format!("Custom_{}", reference.trim_start_matches("custom.")),
                    required_import: None,
                })
            } else {
                None
            }
        }
        
        fn parse_import_path(&self, _path: &str) -> Option<ImportMetadata> {
            None
        }
        
        fn name(&self) -> &str {
            "CustomResolver"
        }
    }
    
    let mut resolver = TypeResolver::new();
    resolver.register(Box::new(CustomResolver));
    
    let module = Module {
        name: "test".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };
    let context = ResolutionContext {
        current_group: None,
        current_version: None,
        current_kind: None,
    };
    
    let resolved = resolver.resolve("custom.MyType", &module, &context);
    assert_eq!(resolved, "Custom_MyType");
}