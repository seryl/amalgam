//! Test fixture loader for CRDs
//! 
//! Provides easy access to test CRD fixtures stored as YAML files

use amalgam_parser::crd::CRD;
use std::path::PathBuf;

/// Load a fixture CRD from the fixtures directory
pub fn load_fixture(name: &str) -> CRD {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(format!("{}.yaml", name));
    
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read fixture {}: {}", path.display(), e));
    
    serde_yaml::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse fixture {}: {}", name, e))
}

/// Available test fixtures
pub struct Fixtures;

impl Fixtures {
    pub fn simple_with_metadata() -> CRD {
        load_fixture("simple_with_metadata")
    }
    
    pub fn multiple_k8s_refs() -> CRD {
        load_fixture("multiple_k8s_refs")
    }
    
    pub fn nested_objects() -> CRD {
        load_fixture("nested_objects")
    }
    
    pub fn with_arrays() -> CRD {
        load_fixture("with_arrays")
    }
    
    pub fn with_validation() -> CRD {
        load_fixture("with_validation")
    }
    
    pub fn multi_version() -> CRD {
        load_fixture("multi_version")
    }
}

/// List all available fixture names
pub fn list_fixtures() -> Vec<&'static str> {
    vec![
        "simple_with_metadata",
        "multiple_k8s_refs",
        "nested_objects",
        "with_arrays",
        "with_validation",
        "multi_version",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_all_fixtures_load() {
        // Ensure all fixtures can be loaded
        for fixture_name in list_fixtures() {
            let crd = load_fixture(fixture_name);
            assert!(!crd.metadata.name.is_empty(), 
                    "Fixture {} should have metadata.name", fixture_name);
        }
    }
    
    #[test]
    fn test_fixtures_helper_methods() {
        // Test each helper method
        let _ = Fixtures::simple_with_metadata();
        let _ = Fixtures::multiple_k8s_refs();
        let _ = Fixtures::nested_objects();
        let _ = Fixtures::with_arrays();
        let _ = Fixtures::with_validation();
        let _ = Fixtures::multi_version();
    }
}