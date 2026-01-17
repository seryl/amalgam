//! WebAssembly bindings for Amalgam schema compiler
//!
//! This crate provides WASM bindings to use Amalgam in the browser or Node.js
//! for converting schemas (CRD, OpenAPI) to Nickel configuration language.

use amalgam_codegen::{nickel::NickelCodegen, Codegen};
use amalgam_core::ir::IR;
use amalgam_parser::walkers::{crd::{CRDWalker, CRDInput, CRDVersion as WalkerCRDVersion}, openapi::OpenAPIWalker, SchemaWalker};
use wasm_bindgen::prelude::*;
use serde_json::Value as JsonValue;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

// Macro for easier console logging
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

macro_rules! console_error {
    ($($t:tt)*) => (error(&format_args!($($t)*).to_string()))
}

/// Initialize panic hook for better error messages in browser console
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
    console_log!("Amalgam WASM initialized");
}

/// Convert a CRD YAML string to Nickel configuration
#[wasm_bindgen]
pub fn crd_to_nickel(crd_yaml: &str, base_module: Option<String>) -> Result<String, JsValue> {
    console_log!("Converting CRD to Nickel");
    
    // Parse the YAML into a generic Value
    let crd_value: serde_yaml::Value = serde_yaml::from_str(crd_yaml)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse YAML: {}", e)))?;
    
    // Extract the CRD metadata
    let crd_obj = crd_value.as_mapping()
        .ok_or_else(|| JsValue::from_str("CRD must be a YAML object"))?;
    
    // Get the spec field
    let spec = crd_obj.get(&serde_yaml::Value::String("spec".to_string()))
        .ok_or_else(|| JsValue::from_str("CRD missing 'spec' field"))?;
    
    // Extract group
    let group = spec.get("group")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JsValue::from_str("CRD spec missing 'group' field"))?;
    
    // Extract versions
    let versions = spec.get("versions")
        .and_then(|v| v.as_sequence())
        .ok_or_else(|| JsValue::from_str("CRD spec missing or invalid 'versions' field"))?;
    
    // Convert versions to walker format
    let mut walker_versions = Vec::new();
    for version in versions {
        let name = version.get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsValue::from_str("Version missing 'name' field"))?;
        
        // Get the OpenAPI v3 schema
        let schema = version.get("schema")
            .and_then(|s| s.get("openAPIV3Schema"))
            .ok_or_else(|| JsValue::from_str(&format!("Version {} missing schema.openAPIV3Schema", name)))?;
        
        // Convert YAML value to JSON value for the walker
        let schema_json = serde_json::to_value(schema)
            .map_err(|e| JsValue::from_str(&format!("Failed to convert schema: {}", e)))?;
        
        walker_versions.push(WalkerCRDVersion {
            name: name.to_string(),
            schema: schema_json,
        });
    }
    
    // Create CRDInput for the walker
    let crd_input = CRDInput {
        group: group.to_string(),
        versions: walker_versions,
    };
    
    // Determine base module name
    let module_name = if let Some(base) = base_module {
        base
    } else {
        // Extract from metadata.name if possible
        crd_obj.get(&serde_yaml::Value::String("metadata".to_string()))
            .and_then(|m| m.get("name"))
            .and_then(|n| n.as_str())
            .map(|s| s.replace('.', "_"))
            .unwrap_or_else(|| group.replace('.', "_"))
    };
    
    // Create walker and process
    let walker = CRDWalker::new(module_name);
    let ir = walker.walk(crd_input)
        .map_err(|e| JsValue::from_str(&format!("Failed to process CRD: {}", e)))?;
    
    // Generate Nickel code
    let mut codegen = NickelCodegen::from_ir(&ir);
    let output = codegen.generate(&ir)
        .map_err(|e| JsValue::from_str(&format!("Failed to generate Nickel: {}", e)))?;
    
    Ok(output)
}

/// Convert an OpenAPI JSON string to Nickel configuration
#[wasm_bindgen]
pub fn openapi_to_nickel(openapi_json: &str, base_module: Option<String>) -> Result<String, JsValue> {
    console_log!("Converting OpenAPI to Nickel");
    
    // Parse the OpenAPI spec using the openapiv3 crate
    let spec: openapiv3::OpenAPI = serde_json::from_str(openapi_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse OpenAPI JSON: {}", e)))?;
    
    // Determine base module name
    let module_name = base_module.unwrap_or_else(|| {
        // Try to derive from the title
        spec.info.title.replace(' ', "_").replace('-', "_").to_lowercase()
    });
    
    // Create walker and process
    let walker = OpenAPIWalker::new(module_name);
    let ir = walker.walk(spec)
        .map_err(|e| JsValue::from_str(&format!("Failed to process OpenAPI: {}", e)))?;
    
    // Generate Nickel code
    let mut codegen = NickelCodegen::from_ir(&ir);
    let output = codegen.generate(&ir)
        .map_err(|e| JsValue::from_str(&format!("Failed to generate Nickel: {}", e)))?;
    
    Ok(output)
}

/// Process multiple schemas and generate a package
#[wasm_bindgen]
pub struct AmalgamPackage {
    ir: IR,
}

#[wasm_bindgen]
impl AmalgamPackage {
    /// Create a new package
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_log!("Creating new AmalgamPackage");
        Self {
            ir: IR::new(),
        }
    }
    
    /// Add a CRD to the package
    pub fn add_crd(&mut self, crd_yaml: &str, base_module: Option<String>) -> Result<(), JsValue> {
        let crd_value: serde_yaml::Value = serde_yaml::from_str(crd_yaml)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse YAML: {}", e)))?;
        
        // Extract the CRD metadata
        let crd_obj = crd_value.as_mapping()
            .ok_or_else(|| JsValue::from_str("CRD must be a YAML object"))?;
        
        // Get the spec field
        let spec = crd_obj.get(&serde_yaml::Value::String("spec".to_string()))
            .ok_or_else(|| JsValue::from_str("CRD missing 'spec' field"))?;
        
        // Extract group
        let group = spec.get("group")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsValue::from_str("CRD spec missing 'group' field"))?;
        
        // Extract versions
        let versions = spec.get("versions")
            .and_then(|v| v.as_sequence())
            .ok_or_else(|| JsValue::from_str("CRD spec missing or invalid 'versions' field"))?;
        
        // Convert versions to walker format
        let mut walker_versions = Vec::new();
        for version in versions {
            let name = version.get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| JsValue::from_str("Version missing 'name' field"))?;
            
            // Get the OpenAPI v3 schema
            let schema = version.get("schema")
                .and_then(|s| s.get("openAPIV3Schema"))
                .ok_or_else(|| JsValue::from_str(&format!("Version {} missing schema.openAPIV3Schema", name)))?;
            
            // Convert YAML value to JSON value for the walker
            let schema_json = serde_json::to_value(schema)
                .map_err(|e| JsValue::from_str(&format!("Failed to convert schema: {}", e)))?;
            
            walker_versions.push(WalkerCRDVersion {
                name: name.to_string(),
                schema: schema_json,
            });
        }
        
        // Create CRDInput for the walker
        let crd_input = CRDInput {
            group: group.to_string(),
            versions: walker_versions,
        };
        
        // Determine base module name
        let module_name = if let Some(base) = base_module {
            base
        } else {
            // Extract from metadata.name if possible
            crd_obj.get(&serde_yaml::Value::String("metadata".to_string()))
                .and_then(|m| m.get("name"))
                .and_then(|n| n.as_str())
                .map(|s| s.replace('.', "_"))
                .unwrap_or_else(|| group.replace('.', "_"))
        };
        
        // Create walker and process
        let walker = CRDWalker::new(module_name);
        let module_ir = walker.walk(crd_input)
            .map_err(|e| JsValue::from_str(&format!("Failed to process CRD: {}", e)))?;
        
        // Merge modules into the package IR
        for module in module_ir.modules {
            self.ir.add_module(module);
        }
        
        Ok(())
    }
    
    /// Add an OpenAPI spec to the package
    pub fn add_openapi(&mut self, openapi_json: &str, base_module: Option<String>) -> Result<(), JsValue> {
        // Parse the OpenAPI spec using the openapiv3 crate
        let spec: openapiv3::OpenAPI = serde_json::from_str(openapi_json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse OpenAPI JSON: {}", e)))?;
        
        // Determine base module name
        let module_name = base_module.unwrap_or_else(|| {
            // Try to derive from the title
            spec.info.title.replace(' ', "_").replace('-', "_").to_lowercase()
        });
        
        // Create walker and process
        let walker = OpenAPIWalker::new(module_name);
        let module_ir = walker.walk(spec)
            .map_err(|e| JsValue::from_str(&format!("Failed to process OpenAPI: {}", e)))?;
        
        // Merge modules into the package IR
        for module in module_ir.modules {
            self.ir.add_module(module);
        }
        
        Ok(())
    }
    
    /// Generate Nickel code for the entire package
    pub fn generate(&mut self) -> Result<String, JsValue> {
        let mut codegen = NickelCodegen::from_ir(&self.ir);
        let output = codegen.generate(&self.ir)
            .map_err(|e| JsValue::from_str(&format!("Failed to generate Nickel: {}", e)))?;
        
        Ok(output)
    }
    
    /// Get the number of modules in the package
    pub fn module_count(&self) -> usize {
        self.ir.modules.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_package_creation() {
        let package = AmalgamPackage::new();
        assert_eq!(package.module_count(), 0);
    }
}