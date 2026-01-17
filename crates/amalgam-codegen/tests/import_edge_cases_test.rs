/// Edge case tests for import resolution
/// Tests unusual but valid scenarios that could break import resolution
use amalgam_codegen::nickel::NickelCodegen;
use amalgam_codegen::Codegen;
use amalgam_core::ir::{Module, TypeDefinition, IR};
use amalgam_core::types::{Field, Type};
use amalgam_core::ModuleRegistry;
use std::collections::BTreeMap;
use std::sync::Arc;

#[test]
fn test_empty_module_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Edge case: Module with no types
    let mut ir = IR::new();

    let empty_module = Module {
        name: "empty.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    let mut normal_module = Module {
        name: "normal.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    normal_module.types.push(TypeDefinition {
        name: "NormalType".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    ir.modules.push(empty_module);
    ir.modules.push(normal_module);

    // Should not panic with empty modules
    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    assert!(
        output.contains("NormalType"),
        "Normal type should be generated"
    );

    Ok(())
}

#[test]
fn test_single_letter_module_names() -> Result<(), Box<dyn std::error::Error>> {
    // Edge case: Very short module names
    let mut ir = IR::new();

    let mut module_a = Module {
        name: "a.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    module_a.types.push(TypeDefinition {
        name: "TypeA".to_string(),
        ty: Type::Reference {
            name: "TypeB".to_string(),
            module: Some("b.v1".to_string()),
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    let mut module_b = Module {
        name: "b.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    module_b.types.push(TypeDefinition {
        name: "TypeB".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    ir.modules.push(module_a);
    ir.modules.push(module_b);

    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    assert!(output.contains("TypeA"), "TypeA should be generated");
    assert!(output.contains("TypeB"), "TypeB should be generated");

    Ok(())
}

#[test]
fn test_special_characters_in_module_names() -> Result<(), Box<dyn std::error::Error>> {
    // Edge case: Module names with hyphens, dots, underscores
    let mut ir = IR::new();

    let mut module = Module {
        name: "example-corp.cloud-api.v1-beta2".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    module.types.push(TypeDefinition {
        name: "CloudResource".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    ir.modules.push(module);

    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    // Should handle special characters by sanitizing them
    assert!(output.contains("CloudResource"), "Type should be generated");
    // Module name should be sanitized (hyphens to underscores)
    assert!(
        !output.contains("example-corp") || output.contains("example_corp"),
        "Module names should be sanitized"
    );

    Ok(())
}

#[test]
fn test_deeply_nested_type_references() -> Result<(), Box<dyn std::error::Error>> {
    // Edge case: Types with deeply nested references
    let mut ir = IR::new();

    let mut module = Module {
        name: "nested.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    // Create a deeply nested type structure
    let deep_type = Type::Optional(Box::new(Type::Array(Box::new(Type::Map {
        key: Box::new(Type::String),
        value: Box::new(Type::Optional(Box::new(Type::Array(Box::new(
            Type::Reference {
                name: "ExternalType".to_string(),
                module: Some("external.v1".to_string()),
            },
        ))))),
    }))));

    let mut fields = BTreeMap::new();
    fields.insert(
        "deep_field".to_string(),
        Field {
            ty: deep_type,
            required: false,
            description: None,
            default: None,
            validation: None,
            contracts: vec![],
        },
    );

    module.types.push(TypeDefinition {
        name: "DeeplyNestedType".to_string(),
        ty: Type::Record {
            fields,
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    // Add the external module
    let mut external = Module {
        name: "external.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    external.types.push(TypeDefinition {
        name: "ExternalType".to_string(),
        ty: Type::Record {
            fields: BTreeMap::new(),
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    ir.modules.push(module);
    ir.modules.push(external);

    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    assert!(
        output.contains("DeeplyNestedType"),
        "Deeply nested type should be generated"
    );
    assert!(
        output.contains("ExternalType"),
        "External type should be generated"
    );

    Ok(())
}

#[test]
fn test_self_referential_type() -> Result<(), Box<dyn std::error::Error>> {
    // Edge case: Type that references itself (tree structure)
    let mut ir = IR::new();

    let mut module = Module {
        name: "tree.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    let mut fields = BTreeMap::new();
    fields.insert(
        "value".to_string(),
        Field {
            ty: Type::String,
            required: true,
            description: None,
            default: None,
            validation: None,
            contracts: vec![],
        },
    );
    fields.insert(
        "children".to_string(),
        Field {
            ty: Type::Optional(Box::new(Type::Array(Box::new(Type::Reference {
                name: "TreeNode".to_string(),
                module: None, // Self-reference
            })))),
            required: false,
            description: None,
            default: None,
            validation: None,
            contracts: vec![],
        },
    );

    module.types.push(TypeDefinition {
        name: "TreeNode".to_string(),
        ty: Type::Record {
            fields,
            open: false,
        },
        documentation: Some("Recursive tree structure".to_string()),
        annotations: BTreeMap::new(),
    });

    ir.modules.push(module);

    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    assert!(
        output.contains("TreeNode"),
        "Tree node type should be generated"
    );
    assert!(
        output.contains("children"),
        "Children field should be present"
    );
    // Self-references shouldn't generate imports
    assert!(
        !output.contains("import") || !output.contains("tree_v1"),
        "Self-references should not generate imports"
    );

    Ok(())
}

#[test]
fn test_reserved_keywords_as_names() -> Result<(), Box<dyn std::error::Error>> {
    // Edge case: Type/field names that are Nickel reserved keywords
    let mut ir = IR::new();

    let mut module = Module {
        name: "reserved.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    let mut fields = BTreeMap::new();
    // Use Nickel reserved keywords as field names
    for keyword in &["let", "in", "if", "then", "else", "fun", "import", "as"] {
        fields.insert(
            keyword.to_string(),
            Field {
                ty: Type::String,
                required: false,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );
    }

    module.types.push(TypeDefinition {
        name: "let".to_string(), // Type name is also a keyword
        ty: Type::Record {
            fields,
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    ir.modules.push(module);

    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    // Should escape reserved keywords properly
    // The exact escaping strategy depends on implementation
    // but it should not produce invalid Nickel code
    assert!(
        output.contains("`let`") || output.contains("let_") || output.contains(r#""let""#),
        "Reserved keywords should be escaped"
    );

    Ok(())
}

#[test]
fn test_massive_module_count() -> Result<(), Box<dyn std::error::Error>> {
    // Edge case: Large number of modules (performance test)
    let mut ir = IR::new();

    // Create 100 modules
    for i in 0..100 {
        let mut module = Module {
            name: format!("module{}.v1", i),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        // Each module has a type that references the next module
        let next_module = if i < 99 {
            Some(format!("module{}.v1", i + 1))
        } else {
            None
        };

        let mut fields = BTreeMap::new();
        if let Some(next) = next_module {
            fields.insert(
                "next".to_string(),
                Field {
                    ty: Type::Optional(Box::new(Type::Reference {
                        name: format!("Type{}", i + 1),
                        module: Some(next),
                    })),
                    required: false,
                    description: None,
                    default: None,
                    validation: None,
                    contracts: vec![],
                },
            );
        }

        module.types.push(TypeDefinition {
            name: format!("Type{}", i),
            ty: Type::Record {
                fields,
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.modules.push(module);
    }

    let start = std::time::Instant::now();
    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;
    let duration = start.elapsed();

    // Should complete in reasonable time (< 5 seconds for 100 modules)
    assert!(
        duration.as_secs() < 5,
        "Should handle 100 modules in less than 5 seconds, took {:?}",
        duration
    );

    // Spot check some types
    assert!(output.contains("Type0"), "First type should be generated");
    assert!(output.contains("Type99"), "Last type should be generated");

    Ok(())
}

#[test]
fn test_union_type_with_references() -> Result<(), Box<dyn std::error::Error>> {
    // Edge case: Union types containing references
    let mut ir = IR::new();

    let mut module = Module {
        name: "union.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    let union_type = Type::Union {
        types: vec![
            Type::String,
            Type::Number,
            Type::Reference {
                name: "ExternalType".to_string(),
                module: Some("external.v1".to_string()),
            },
            Type::Array(Box::new(Type::Reference {
                name: "AnotherExternal".to_string(),
                module: Some("another.v1".to_string()),
            })),
        ],
        coercion_hint: None,
    };

    let mut fields = BTreeMap::new();
    fields.insert(
        "union_field".to_string(),
        Field {
            ty: union_type,
            required: true,
            description: None,
            default: None,
            validation: None,
            contracts: vec![],
        },
    );

    module.types.push(TypeDefinition {
        name: "UnionType".to_string(),
        ty: Type::Record {
            fields,
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    // Add external modules
    for (module_name, type_name) in &[
        ("external.v1", "ExternalType"),
        ("another.v1", "AnotherExternal"),
    ] {
        let mut ext_module = Module {
            name: module_name.to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        ext_module.types.push(TypeDefinition {
            name: type_name.to_string(),
            ty: Type::Record {
                fields: BTreeMap::new(),
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.modules.push(ext_module);
    }

    ir.modules.push(module);

    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    assert!(
        output.contains("UnionType"),
        "Union type should be generated"
    );
    assert!(
        output.contains("ExternalType"),
        "External type should be generated"
    );
    assert!(
        output.contains("AnotherExternal"),
        "Another external type should be generated"
    );

    Ok(())
}

#[test]
fn test_duplicate_type_names_across_modules() -> Result<(), Box<dyn std::error::Error>> {
    // Edge case: Same type name in different modules
    let mut ir = IR::new();

    // Multiple modules with the same type name
    for i in 1..=3 {
        let mut module = Module {
            name: format!("namespace{}.v1", i),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        module.types.push(TypeDefinition {
            name: "Config".to_string(), // Same name in all modules
            ty: Type::Record {
                fields: {
                    let mut fields = BTreeMap::new();
                    fields.insert(
                        "namespace_id".to_string(),
                        Field {
                            ty: Type::Number,
                            required: true,
                            description: None,
                            default: Some(serde_json::json!(i)),
                            validation: None,
                            contracts: vec![],
                        },
                    );
                    fields
                },
                open: false,
            },
            documentation: None,
            annotations: BTreeMap::new(),
        });

        ir.modules.push(module);
    }

    // Module that references all the Config types
    let mut consumer_module = Module {
        name: "consumer.v1".to_string(),
        imports: vec![],
        types: vec![],
        constants: vec![],
        metadata: Default::default(),
    };

    let mut fields = BTreeMap::new();
    for i in 1..=3 {
        fields.insert(
            format!("config{}", i),
            Field {
                ty: Type::Reference {
                    name: "Config".to_string(),
                    module: Some(format!("namespace{}.v1", i)),
                },
                required: false,
                description: None,
                default: None,
                validation: None,
                contracts: vec![],
            },
        );
    }

    consumer_module.types.push(TypeDefinition {
        name: "ConfigConsumer".to_string(),
        ty: Type::Record {
            fields,
            open: false,
        },
        documentation: None,
        annotations: BTreeMap::new(),
    });

    ir.modules.push(consumer_module);

    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let output = codegen.generate(&ir)?;

    // All Config types should be generated
    assert!(
        output.contains("Config"),
        "Config types should be generated"
    );
    assert!(
        output.contains("ConfigConsumer"),
        "ConfigConsumer should be generated"
    );

    // Should handle namespace conflicts properly
    assert!(
        output.contains("namespace") || output.contains("module"),
        "Should include module context for disambiguation"
    );

    Ok(())
}
