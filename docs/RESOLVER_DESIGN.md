# Type Resolver Design

## Overview

This document describes amalgam's type resolution system that handles cross-package references for any schema source using a simple, generic pattern-matching approach.

## Core Architecture

```rust
pub struct TypeResolver {
    cache: HashMap<String, Resolution>,
    type_registry: HashMap<String, String>,
}

pub struct Resolution {
    pub import_needed: bool,
    pub import_info: Option<ImportInfo>,
    pub resolved_reference: String,
}

pub struct ImportInfo {
    pub namespace: String,
    pub path: String,
    pub alias: String,
}
```

## Resolution Algorithm

The resolver uses a straightforward approach:

1. **Check Cache**: Return cached resolution if available
2. **Check Local Types**: If type is in current module, no import needed
3. **Check Existing Imports**: Reuse imports that already cover the type
4. **Create New Import**: Generate import info for cross-package references

```rust
pub fn resolve(&mut self, reference: &str, context: &ResolutionContext) -> Resolution {
    // Check cache first
    if let Some(cached) = self.cache.get(reference) {
        return cached.clone();
    }

    // Check if it's a local type
    if self.is_local_type(reference, context) {
        return Resolution {
            import_needed: false,
            import_info: None,
            resolved_reference: self.extract_type_name(reference),
        };
    }

    // Check existing imports
    if let Some(import) = self.find_matching_import(reference, &context.imports) {
        return Resolution {
            import_needed: false,
            import_info: None,
            resolved_reference: format!("{}.{}", 
                import.alias, 
                self.extract_type_name(reference)
            ),
        };
    }

    // Create new import
    let import_info = self.create_import_info(reference);
    Resolution {
        import_needed: true,
        import_info: Some(import_info.clone()),
        resolved_reference: format!("{}.{}", 
            import_info.alias, 
            self.extract_type_name(reference)
        ),
    }
}
```

## Key Features

### Universal Pattern Matching

The resolver uses namespace-based pattern matching that works for all schema formats:

```rust
fn import_matches_reference(&self, import_info: &ImportInfo, reference: &str) -> bool {
    let namespace_parts: Vec<&str> = import_info.namespace.split('.').collect();
    namespace_parts.iter().any(|&part| reference.contains(part))
}
```

This handles:
- Kubernetes: `io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta`
- OpenAPI: `#/components/schemas/User`  
- Protobuf: `google.protobuf.Timestamp`
- Any future format with hierarchical namespaces

### Type Registry

Maintains a flat map of all known types and their namespaces:

```rust
type_registry: {
    "Pod": "io.k8s.api.core.v1",
    "Container": "io.k8s.api.core.v1", 
    "ObjectMeta": "io.k8s.apimachinery.pkg.apis.meta.v1",
    "User": "com.example.api.v1",
}
```

### Import Path Calculation

Calculates relative import paths between modules:

```rust
fn calculate_import_path(&self, from: &Path, to: &str) -> String {
    let namespace_parts: Vec<&str> = to.split('.').collect();
    let mut path_parts = Vec::new();
    
    // Navigate up to common root
    let depth = from.components().count();
    for _ in 0..depth {
        path_parts.push("..");
    }
    
    // Navigate down to target
    for part in namespace_parts {
        path_parts.push(part);
    }
    
    path_parts.join("/")
}
```

## Usage Examples

### Kubernetes CRD Resolution

```rust
let mut resolver = TypeResolver::new();
let context = ResolutionContext {
    current_module: "apiextensions.crossplane.io.v1",
    imports: vec![],
};

let resolution = resolver.resolve(
    "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta",
    &context
);

// Produces:
// Import: let meta_v1 = import "../../io/k8s/apimachinery/pkg/apis/meta/v1.ncl" in
// Reference: meta_v1.ObjectMeta
```

### Local Type Resolution

```rust
let context = ResolutionContext {
    current_module: "io.k8s.api.core.v1",
    imports: vec![],
};

let resolution = resolver.resolve("Pod", &context);

// Produces:
// No import needed
// Reference: Pod
```

### Reusing Existing Imports

```rust
let context = ResolutionContext {
    current_module: "my.package",
    imports: vec![ImportInfo {
        namespace: "io.k8s.api.core.v1",
        path: "../io/k8s/api/core/v1.ncl",
        alias: "core_v1",
    }],
};

let resolution = resolver.resolve("io.k8s.api.core.v1.Container", &context);

// Produces:
// No new import needed (reuses existing)
// Reference: core_v1.Container
```

## Performance

- **O(1)** - Cached lookups
- **O(n)** - First resolution (where n = number of existing imports)
- **O(1)** - Type registry lookups
- Memory usage proportional to number of unique types

## Testing

### Unit Tests

```rust
#[test]
fn test_local_type_resolution() {
    let mut resolver = TypeResolver::new();
    let context = ResolutionContext {
        current_module: "test.module",
        imports: vec![],
    };
    
    resolver.register_type("MyType", "test.module");
    let resolution = resolver.resolve("MyType", &context);
    
    assert!(!resolution.import_needed);
    assert_eq!(resolution.resolved_reference, "MyType");
}

#[test]
fn test_cross_package_resolution() {
    let mut resolver = TypeResolver::new();
    let context = ResolutionContext {
        current_module: "my.module",
        imports: vec![],
    };
    
    let resolution = resolver.resolve("other.module.Type", &context);
    
    assert!(resolution.import_needed);
    assert_eq!(resolution.import_info.unwrap().alias, "module");
    assert_eq!(resolution.resolved_reference, "module.Type");
}
```

### Property-Based Testing

```rust
#[proptest]
fn resolver_is_deterministic(reference: String) {
    let mut resolver1 = TypeResolver::new();
    let mut resolver2 = TypeResolver::new();
    let context = ResolutionContext::default();
    
    let res1 = resolver1.resolve(&reference, &context);
    let res2 = resolver2.resolve(&reference, &context);
    
    assert_eq!(res1, res2);
}
```

## Integration with Code Generation

The resolver integrates seamlessly with the Nickel code generator:

```rust
impl NickelGenerator {
    pub fn generate_with_imports(&self, module: &Module) -> Result<String> {
        let mut resolver = TypeResolver::new();
        let mut imports = Vec::new();
        
        // Collect all type references
        for type_def in &module.types {
            for field in &type_def.fields {
                if let Some(ref_type) = extract_reference(&field.type_) {
                    let resolution = resolver.resolve(&ref_type, &context);
                    if resolution.import_needed {
                        imports.push(resolution.import_info.unwrap());
                    }
                }
            }
        }
        
        // Generate Nickel with imports
        self.generate_nickel(imports, module)
    }
}
```

## Future Enhancements

- **Configurable Aliasing**: Allow custom alias patterns
- **Import Optimization**: Combine related imports
- **Circular Dependency Detection**: Prevent import cycles
- **Smart Suggestions**: Suggest corrections for unresolved types