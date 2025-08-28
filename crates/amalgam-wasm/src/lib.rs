use amalgam_codegen::{nickel::NickelGenerator, CodeGenerator};
use amalgam_core::ir::{Module, TypeDefinition};
use amalgam_parser::{crd::CrdParser, openapi::OpenApiParser, Parser};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys;

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
}

/// Main WASM interface for amalgam
#[wasm_bindgen]
pub struct AmalgamWasm {
    module: Module,
}

#[wasm_bindgen]
impl AmalgamWasm {
    /// Create a new AmalgamWasm instance
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> Self {
        console_log!("Creating new AmalgamWasm module: {}", name);
        Self {
            module: Module {
                name,
                types: Vec::new(),
                imports: Vec::new(),
                metadata: Default::default(),
            },
        }
    }

    /// Parse a Kubernetes CRD from YAML
    #[wasm_bindgen]
    pub async fn parse_crd(yaml_content: String) -> Result<AmalgamWasm, JsValue> {
        console_log!("Parsing CRD from YAML");
        
        let parser = CrdParser::new();
        match parser.parse_str(&yaml_content).await {
            Ok(module) => {
                console_log!("Successfully parsed CRD with {} types", module.types.len());
                Ok(AmalgamWasm { module })
            }
            Err(e) => {
                console_error!("Failed to parse CRD: {}", e);
                Err(JsValue::from_str(&format!("Parse error: {}", e)))
            }
        }
    }

    /// Parse an OpenAPI specification from JSON/YAML
    #[wasm_bindgen]
    pub async fn parse_openapi(spec_content: String) -> Result<AmalgamWasm, JsValue> {
        console_log!("Parsing OpenAPI specification");
        
        let parser = OpenApiParser::new();
        match parser.parse_str(&spec_content).await {
            Ok(module) => {
                console_log!("Successfully parsed OpenAPI with {} types", module.types.len());
                Ok(AmalgamWasm { module })
            }
            Err(e) => {
                console_error!("Failed to parse OpenAPI: {}", e);
                Err(JsValue::from_str(&format!("Parse error: {}", e)))
            }
        }
    }

    /// Generate Nickel code from the loaded module
    #[wasm_bindgen]
    pub fn generate_nickel(&self) -> Result<String, JsValue> {
        console_log!("Generating Nickel code");
        
        let generator = NickelGenerator::new();
        match generator.generate(&self.module) {
            Ok(code) => {
                console_log!("Successfully generated {} bytes of Nickel code", code.len());
                Ok(code)
            }
            Err(e) => {
                console_error!("Failed to generate Nickel: {}", e);
                Err(JsValue::from_str(&format!("Generation error: {}", e)))
            }
        }
    }

    /// Add a type definition to the module
    #[wasm_bindgen]
    pub fn add_type(&mut self, type_def: JsValue) -> Result<(), JsValue> {
        let type_def: TypeDefinition = from_value(type_def)
            .map_err(|e| JsValue::from_str(&format!("Invalid type definition: {}", e)))?;
        
        console_log!("Adding type: {}", type_def.name);
        self.module.types.push(type_def);
        Ok(())
    }

    /// Get all type definitions as JSON
    #[wasm_bindgen]
    pub fn get_types(&self) -> Result<JsValue, JsValue> {
        to_value(&self.module.types)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get the module metadata as JSON
    #[wasm_bindgen]
    pub fn get_metadata(&self) -> Result<JsValue, JsValue> {
        to_value(&self.module.metadata)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Validate Nickel code using nickel-lang-core
    #[wasm_bindgen]
    pub fn validate_nickel(code: String) -> Result<bool, JsValue> {
        console_log!("Validating Nickel code");
        
        // Create a temporary program from the code string
        match nickel_lang_core::program::Program::new_from_source(
            code.into(),
            "<wasm>".into(),
            std::io::stderr(),
        ) {
            Ok(mut program) => {
                // Try to typecheck the program
                match program.typecheck() {
                    Ok(()) => {
                        console_log!("Nickel code is valid");
                        Ok(true)
                    }
                    Err(e) => {
                        console_error!("Nickel validation failed: {}", e);
                        Ok(false)
                    }
                }
            }
            Err(e) => {
                console_error!("Failed to parse Nickel code: {}", e);
                Err(JsValue::from_str(&format!("Parse error: {}", e)))
            }
        }
    }

    /// Evaluate Nickel code and return the result as JSON
    #[wasm_bindgen]
    pub fn eval_nickel(code: String) -> Result<JsValue, JsValue> {
        console_log!("Evaluating Nickel code");
        
        match nickel_lang_core::program::Program::new_from_source(
            code.into(),
            "<wasm>".into(),
            std::io::stderr(),
        ) {
            Ok(mut program) => {
                // Evaluate the program
                match program.eval_full() {
                    Ok(term) => {
                        // Serialize the result to JSON
                        match program.serialize(term, nickel_lang_core::serialize::ExportFormat::Json) {
                            Ok(json_str) => {
                                console_log!("Successfully evaluated Nickel code");
                                // Parse JSON string to JsValue
                                match js_sys::JSON::parse(&json_str) {
                                    Ok(val) => Ok(val),
                                    Err(e) => Err(JsValue::from_str(&format!("JSON parse error: {:?}", e)))
                                }
                            }
                            Err(e) => {
                                console_error!("Failed to serialize result: {}", e);
                                Err(JsValue::from_str(&format!("Serialization error: {}", e)))
                            }
                        }
                    }
                    Err(e) => {
                        console_error!("Evaluation failed: {}", e);
                        Err(JsValue::from_str(&format!("Evaluation error: {}", e)))
                    }
                }
            }
            Err(e) => {
                console_error!("Failed to parse Nickel code: {}", e);
                Err(JsValue::from_str(&format!("Parse error: {}", e)))
            }
        }
    }
}

/// Standalone function to convert between schema formats
#[wasm_bindgen]
pub async fn convert_schema(
    input: String,
    from_format: String,
    to_format: String,
) -> Result<String, JsValue> {
    console_log!("Converting from {} to {}", from_format, to_format);
    
    // Parse input based on format
    let module = match from_format.as_str() {
        "crd" => {
            let parser = CrdParser::new();
            parser.parse_str(&input).await
                .map_err(|e| JsValue::from_str(&format!("CRD parse error: {}", e)))?
        }
        "openapi" => {
            let parser = OpenApiParser::new();
            parser.parse_str(&input).await
                .map_err(|e| JsValue::from_str(&format!("OpenAPI parse error: {}", e)))?
        }
        _ => {
            return Err(JsValue::from_str(&format!("Unknown input format: {}", from_format)));
        }
    };

    // Generate output based on format
    match to_format.as_str() {
        "nickel" => {
            let generator = NickelGenerator::new();
            generator.generate(&module)
                .map_err(|e| JsValue::from_str(&format!("Nickel generation error: {}", e)))
        }
        "json" => {
            serde_json::to_string_pretty(&module)
                .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
        }
        _ => {
            Err(JsValue::from_str(&format!("Unknown output format: {}", to_format)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_module_creation() {
        let module = AmalgamWasm::new("test".to_string());
        assert_eq!(module.module.name, "test");
    }

    #[wasm_bindgen_test]
    fn test_nickel_validation() {
        let valid_code = "{ foo = 42 }";
        assert!(AmalgamWasm::validate_nickel(valid_code.to_string()).is_ok());
        
        let invalid_code = "{ foo = }";
        assert!(AmalgamWasm::validate_nickel(invalid_code.to_string()).is_ok());
    }
}