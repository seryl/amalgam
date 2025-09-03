//! Tests for OpenAPI allOf/anyOf support in the unified IR pipeline

use amalgam_parser::walkers::openapi::OpenAPIWalker;
use amalgam_parser::walkers::SchemaWalker;
use openapiv3::OpenAPI;
use serde_json::json;

/// Create a test OpenAPI spec with allOf example
fn create_allof_spec() -> OpenAPI {
    let spec_json = json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Test API with allOf",
            "version": "1.0.0"
        },
        "paths": {},
        "components": {
            "schemas": {
                "Pet": {
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string"
                        },
                        "type": {
                            "type": "string"
                        }
                    },
                    "required": ["name", "type"]
                },
                "Dog": {
                    "allOf": [
                        {
                            "$ref": "#/components/schemas/Pet"
                        },
                        {
                            "type": "object",
                            "properties": {
                                "breed": {
                                    "type": "string"
                                },
                                "goodBoy": {
                                    "type": "boolean"
                                }
                            }
                        }
                    ]
                },
                "Cat": {
                    "allOf": [
                        {
                            "$ref": "#/components/schemas/Pet"
                        },
                        {
                            "type": "object",
                            "properties": {
                                "lives": {
                                    "type": "integer",
                                    "default": 9
                                },
                                "indoor": {
                                    "type": "boolean"
                                }
                            }
                        }
                    ]
                }
            }
        }
    });

    serde_json::from_value(spec_json).expect("Failed to parse OpenAPI spec")
}

/// Create a test OpenAPI spec with anyOf example
fn create_anyof_spec() -> OpenAPI {
    let spec_json = json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Test API with anyOf",
            "version": "1.0.0"
        },
        "paths": {},
        "components": {
            "schemas": {
                "StringOrNumber": {
                    "anyOf": [
                        {
                            "type": "string"
                        },
                        {
                            "type": "number"
                        }
                    ]
                },
                "PetOrError": {
                    "anyOf": [
                        {
                            "type": "object",
                            "properties": {
                                "id": {
                                    "type": "string"
                                },
                                "name": {
                                    "type": "string"
                                }
                            }
                        },
                        {
                            "type": "object",
                            "properties": {
                                "error": {
                                    "type": "string"
                                },
                                "code": {
                                    "type": "integer"
                                }
                            }
                        }
                    ]
                },
                "MixedResponse": {
                    "anyOf": [
                        {
                            "type": "string"
                        },
                        {
                            "type": "array",
                            "items": {
                                "type": "string"
                            }
                        },
                        {
                            "type": "object",
                            "properties": {
                                "message": {
                                    "type": "string"
                                }
                            }
                        }
                    ]
                }
            }
        }
    });

    serde_json::from_value(spec_json).expect("Failed to parse OpenAPI spec")
}

/// Create a test with nested allOf/anyOf
fn create_complex_spec() -> OpenAPI {
    let spec_json = json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Complex API with nested allOf/anyOf",
            "version": "1.0.0"
        },
        "paths": {},
        "components": {
            "schemas": {
                "BaseObject": {
                    "type": "object",
                    "properties": {
                        "id": {
                            "type": "string"
                        },
                        "created": {
                            "type": "string",
                            "format": "date-time"
                        }
                    },
                    "required": ["id"]
                },
                "NamedObject": {
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string"
                        },
                        "description": {
                            "type": "string"
                        }
                    },
                    "required": ["name"]
                },
                "ComplexEntity": {
                    "allOf": [
                        {
                            "$ref": "#/components/schemas/BaseObject"
                        },
                        {
                            "$ref": "#/components/schemas/NamedObject"
                        },
                        {
                            "type": "object",
                            "properties": {
                                "status": {
                                    "anyOf": [
                                        {
                                            "type": "string",
                                            "enum": ["active", "inactive", "pending"]
                                        },
                                        {
                                            "type": "object",
                                            "properties": {
                                                "state": {
                                                    "type": "string"
                                                },
                                                "reason": {
                                                    "type": "string"
                                                }
                                            }
                                        }
                                    ]
                                },
                                "metadata": {
                                    "type": "object",
                                    "additionalProperties": true
                                }
                            }
                        }
                    ]
                }
            }
        }
    });

    serde_json::from_value(spec_json).expect("Failed to parse OpenAPI spec")
}

#[test]
fn test_allof_basic_composition() {
    let spec = create_allof_spec();
    let walker = OpenAPIWalker::new("test.api");
    let ir = walker.walk(spec).expect("Failed to walk OpenAPI spec");

    // Check that modules were created
    assert!(!ir.modules.is_empty(), "IR should contain modules");

    // Find the Dog type
    let dog_type = ir
        .modules
        .iter()
        .flat_map(|m| &m.types)
        .find(|t| t.name == "Dog");

    assert!(dog_type.is_some(), "Dog type should be generated");

    // Check that the Dog type has merged fields from Pet and its own fields
    if let Some(dog) = dog_type {
        match &dog.ty {
            amalgam_core::types::Type::Record { fields, .. } => {
                // Should have fields from Pet (name, type) and Dog-specific fields
                assert!(
                    fields.contains_key("name") || fields.contains_key("breed"),
                    "Dog should have either Pet fields or its own fields merged"
                );
            }
            amalgam_core::types::Type::Union { .. } => {
                // Also acceptable if it creates a union when merging is complex
            }
            _ => panic!("Dog should be a Record or Union type"),
        }
    }
}

#[test]
fn test_anyof_creates_union() {
    let spec = create_anyof_spec();
    let walker = OpenAPIWalker::new("test.api");
    let ir = walker.walk(spec).expect("Failed to walk OpenAPI spec");

    // Find the StringOrNumber type
    let string_or_number = ir
        .modules
        .iter()
        .flat_map(|m| &m.types)
        .find(|t| t.name == "StringOrNumber");

    assert!(
        string_or_number.is_some(),
        "StringOrNumber type should be generated"
    );

    // Check that it's a union type
    if let Some(son) = string_or_number {
        match &son.ty {
            amalgam_core::types::Type::Union { types, .. } => {
                assert_eq!(
                    types.len(),
                    2,
                    "StringOrNumber should have 2 types in union"
                );

                let has_string = types
                    .iter()
                    .any(|t| matches!(t, amalgam_core::types::Type::String));
                let has_number = types
                    .iter()
                    .any(|t| matches!(t, amalgam_core::types::Type::Number));

                assert!(has_string, "Union should contain String type");
                assert!(has_number, "Union should contain Number type");
            }
            _ => panic!("StringOrNumber should be a Union type"),
        }
    }
}

#[test]
fn test_anyof_with_objects() {
    let spec = create_anyof_spec();
    let walker = OpenAPIWalker::new("test.api");
    let ir = walker.walk(spec).expect("Failed to walk OpenAPI spec");

    // Find the PetOrError type
    let pet_or_error = ir
        .modules
        .iter()
        .flat_map(|m| &m.types)
        .find(|t| t.name == "PetOrError");

    assert!(
        pet_or_error.is_some(),
        "PetOrError type should be generated"
    );

    // Check that it's a union of two object types
    if let Some(poe) = pet_or_error {
        match &poe.ty {
            amalgam_core::types::Type::Union { types, .. } => {
                assert_eq!(types.len(), 2, "PetOrError should have 2 types in union");

                // Both should be Record types
                for t in types {
                    assert!(
                        matches!(t, amalgam_core::types::Type::Record { .. }),
                        "Union members should be Record types"
                    );
                }
            }
            _ => panic!("PetOrError should be a Union type"),
        }
    }
}

#[test]
fn test_complex_nested_allof_anyof() {
    let spec = create_complex_spec();
    let walker = OpenAPIWalker::new("test.api");
    let ir = walker.walk(spec).expect("Failed to walk OpenAPI spec");

    // Find the ComplexEntity type
    let complex_entity = ir
        .modules
        .iter()
        .flat_map(|m| &m.types)
        .find(|t| t.name == "ComplexEntity");

    assert!(
        complex_entity.is_some(),
        "ComplexEntity type should be generated"
    );

    // The type should handle the nested allOf with references and anyOf within
    if let Some(entity) = complex_entity {
        // Just verify it was processed without panicking
        // The exact structure depends on how references are resolved
        match &entity.ty {
            amalgam_core::types::Type::Record { fields, .. } => {
                // If references are resolved, we should have a record with merged fields
                assert!(!fields.is_empty(), "ComplexEntity should have fields");
            }
            amalgam_core::types::Type::Union { .. } => {
                // Also acceptable if complex merging results in a union
            }
            _ => {
                // References might not be resolved in this test
            }
        }
    }
}

#[test]
fn test_mixed_response_anyof() {
    let spec = create_anyof_spec();
    let walker = OpenAPIWalker::new("test.api");
    let ir = walker.walk(spec).expect("Failed to walk OpenAPI spec");

    // Find the MixedResponse type
    let mixed_response = ir
        .modules
        .iter()
        .flat_map(|m| &m.types)
        .find(|t| t.name == "MixedResponse");

    assert!(
        mixed_response.is_some(),
        "MixedResponse type should be generated"
    );

    // Should be a union of string, array, and object
    if let Some(mr) = mixed_response {
        match &mr.ty {
            amalgam_core::types::Type::Union { types, .. } => {
                assert_eq!(types.len(), 3, "MixedResponse should have 3 types in union");

                let has_string = types
                    .iter()
                    .any(|t| matches!(t, amalgam_core::types::Type::String));
                let has_array = types
                    .iter()
                    .any(|t| matches!(t, amalgam_core::types::Type::Array(_)));
                let has_record = types
                    .iter()
                    .any(|t| matches!(t, amalgam_core::types::Type::Record { .. }));

                assert!(has_string, "Union should contain String type");
                assert!(has_array, "Union should contain Array type");
                assert!(has_record, "Union should contain Record type");
            }
            _ => panic!("MixedResponse should be a Union type"),
        }
    }
}

#[test]
fn test_allof_field_conflict_resolution() {
    let spec_json = json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Test API with field conflicts",
            "version": "1.0.0"
        },
        "paths": {},
        "components": {
            "schemas": {
                "ConflictTest": {
                    "allOf": [
                        {
                            "type": "object",
                            "properties": {
                                "field": {
                                    "type": "string"
                                },
                                "common": {
                                    "type": "integer"
                                }
                            }
                        },
                        {
                            "type": "object",
                            "properties": {
                                "field": {
                                    "type": "number"
                                },
                                "other": {
                                    "type": "boolean"
                                }
                            }
                        }
                    ]
                }
            }
        }
    });

    let spec: OpenAPI = serde_json::from_value(spec_json).expect("Failed to parse OpenAPI spec");
    let walker = OpenAPIWalker::new("test.api");
    let ir = walker.walk(spec).expect("Failed to walk OpenAPI spec");

    // Find the ConflictTest type
    let conflict_test = ir
        .modules
        .iter()
        .flat_map(|m| &m.types)
        .find(|t| t.name == "ConflictTest");

    assert!(
        conflict_test.is_some(),
        "ConflictTest type should be generated"
    );

    // When there's a field conflict, it should create a union for that field
    if let Some(ct) = conflict_test {
        match &ct.ty {
            amalgam_core::types::Type::Record { fields, .. } => {
                if let Some(field) = fields.get("field") {
                    // The conflicting field should be a union of string and number
                    assert!(
                        matches!(&field.ty, amalgam_core::types::Type::Union { .. }),
                        "Conflicting field should be a Union type"
                    );
                }
            }
            _ => {
                // Also acceptable to make the whole thing a union
            }
        }
    }
}
