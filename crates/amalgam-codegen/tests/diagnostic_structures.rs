/// Diagnostic structures for understanding IR and codegen output
use amalgam_codegen::{nickel::NickelCodegen, Codegen};
use amalgam_core::ir::{Module, TypeDefinition, IR};
use amalgam_core::types::{Field, Type};
use amalgam_core::ModuleRegistry;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Diagnostic output structure that captures everything about the codegen process
#[derive(Debug, Serialize, Deserialize)]
pub struct CodegenDiagnostics {
    /// Input IR structure
    pub input_ir: IRSnapshot,

    /// Symbol table after construction
    pub symbol_table: Vec<SymbolTableEntry>,

    /// Dependencies found for each type
    pub dependencies: BTreeMap<String, Vec<DependencyInfo>>,

    /// Generated output for each module
    pub module_outputs: Vec<ModuleOutput>,

    /// Final concatenated output
    pub final_output: String,

    /// Parsing attempts to split output
    pub parsing_attempts: Vec<ParsingAttempt>,
}

/// Snapshot of IR structure
#[derive(Debug, Serialize, Deserialize)]
pub struct IRSnapshot {
    pub module_count: usize,
    pub modules: Vec<ModuleSnapshot>,
}

/// Snapshot of a single module
#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleSnapshot {
    pub name: String,
    pub import_count: usize,
    pub type_count: usize,
    pub types: Vec<TypeSnapshot>,
}

/// Snapshot of a type definition
#[derive(Debug, Serialize, Deserialize)]
pub struct TypeSnapshot {
    pub name: String,
    pub kind: String,            // "Record", "Enum", "Alias", etc.
    pub references: Vec<String>, // Other types this type references
}

/// Symbol table entry for diagnostics
#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolTableEntry {
    pub type_name: String,
    pub module: String,
    pub file_path: String,
}

/// Dependency information
#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub referenced_type: String,
    pub reference_location: String, // Where in the type the reference occurs
    pub is_same_package: bool,
    pub calculated_import_path: Option<String>,
}

/// Output for a single module
#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleOutput {
    pub module_name: String,
    pub generated_imports: Vec<String>,
    pub generated_content: String,
    pub output_length: usize,
}

/// Attempt to parse the concatenated output
#[derive(Debug, Serialize, Deserialize)]
pub struct ParsingAttempt {
    pub strategy: String,
    pub success: bool,
    pub error: Option<String>,
    pub extracted_files: BTreeMap<String, String>,
}

/// Create diagnostic data from an IR and codegen run
pub fn create_diagnostics(ir: &IR) -> CodegenDiagnostics {
    // Create IR snapshot
    let ir_snapshot = create_ir_snapshot(ir);

    // Run codegen and capture output
    let mut codegen = NickelCodegen::new(Arc::new(ModuleRegistry::new()));
    let final_output = codegen
        .generate(ir)
        .unwrap_or_else(|e| format!("ERROR: {}", e));

    // Try different parsing strategies
    let parsing_attempts = try_parsing_strategies(&final_output, &ir_snapshot);

    CodegenDiagnostics {
        input_ir: ir_snapshot,
        symbol_table: vec![], // Would be populated from codegen internals
        dependencies: BTreeMap::new(), // Would be populated from codegen internals
        module_outputs: vec![], // Would be populated from codegen internals
        final_output,
        parsing_attempts,
    }
}

fn create_ir_snapshot(ir: &IR) -> IRSnapshot {
    let modules = ir
        .modules
        .iter()
        .map(|module| {
            let types = module
                .types
                .iter()
                .map(|typ| TypeSnapshot {
                    name: typ.name.clone(),
                    kind: match &typ.ty {
                        Type::Record { .. } => "Record".to_string(),
                        Type::Reference { .. } => "Reference".to_string(),
                        Type::String => "String".to_string(),
                        Type::Number => "Number".to_string(),
                        Type::Integer => "Integer".to_string(),
                        Type::Bool => "Bool".to_string(),
                        Type::Array(_) => "Array".to_string(),
                        Type::Optional(_) => "Optional".to_string(),
                        Type::Map { .. } => "Map".to_string(),
                        Type::Any => "Any".to_string(),
                        Type::Null => "Null".to_string(),
                        Type::Union { .. } => "Union".to_string(),
                        Type::TaggedUnion { .. } => "TaggedUnion".to_string(),
                        Type::Contract { .. } => "Contract".to_string(),
                    },
                    references: extract_references(&typ.ty),
                })
                .collect();

            ModuleSnapshot {
                name: module.name.clone(),
                import_count: module.imports.len(),
                type_count: module.types.len(),
                types,
            }
        })
        .collect();

    IRSnapshot {
        module_count: ir.modules.len(),
        modules,
    }
}

fn extract_references(ty: &Type) -> Vec<String> {
    let mut refs = Vec::new();
    extract_references_recursive(ty, &mut refs);
    refs
}

fn extract_references_recursive(ty: &Type, refs: &mut Vec<String>) {
    match ty {
        Type::Reference { name, .. } => {
            refs.push(name.clone());
        }
        Type::Optional(inner) => {
            extract_references_recursive(inner, refs);
        }
        Type::Array(inner) => {
            extract_references_recursive(inner, refs);
        }
        Type::Record { fields, .. } => {
            for field in fields.values() {
                extract_references_recursive(&field.ty, refs);
            }
        }
        Type::Map { key, value } => {
            extract_references_recursive(key, refs);
            extract_references_recursive(value, refs);
        }
        _ => {}
    }
}

fn try_parsing_strategies(output: &str, ir_snapshot: &IRSnapshot) -> Vec<ParsingAttempt> {
    let mut attempts = vec![
        // Strategy 1: Split by module comments
        try_module_comment_split(output),
        // Strategy 2: Split by known type names
        try_type_name_split(output, ir_snapshot),
        // Strategy 3: Split by "# File: " markers
        try_file_marker_split(output),
    ];

    // Strategy 4: Split by module boundaries (looking for record open/close)
    attempts.push(try_record_boundary_split(output));

    attempts
}

fn try_module_comment_split(output: &str) -> ParsingAttempt {
    let mut files = BTreeMap::new();
    let mut current_file = String::new();
    let mut current_content = String::new();

    for line in output.lines() {
        if line.starts_with("# Module:") || line.starts_with("# File:") {
            if !current_file.is_empty() {
                files.insert(current_file.clone(), current_content.clone());
            }
            current_file = line
                .trim_start_matches("# Module:")
                .trim_start_matches("# File:")
                .trim()
                .to_string();
            current_content.clear();
        } else {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    if !current_file.is_empty() {
        files.insert(current_file, current_content);
    }

    ParsingAttempt {
        strategy: "Module comment split".to_string(),
        success: !files.is_empty(),
        error: if files.is_empty() {
            Some("No module markers found".to_string())
        } else {
            None
        },
        extracted_files: files,
    }
}

fn try_type_name_split(output: &str, ir_snapshot: &IRSnapshot) -> ParsingAttempt {
    let mut files = BTreeMap::new();

    // Collect all type names from IR
    let type_names: Vec<String> = ir_snapshot
        .modules
        .iter()
        .flat_map(|m| m.types.iter().map(|t| t.name.clone()))
        .collect();

    // Try to find each type in the output
    for type_name in &type_names {
        let filename = format!("{}.ncl", type_name.to_lowercase());

        // Look for patterns like "TypeName = " at the start of a line
        let pattern = format!("{} = ", type_name);
        if let Some(start) = output.find(&pattern) {
            // Extract content until next type or end
            let content_start = start;
            let mut content_end = output.len();

            for other_type in &type_names {
                if other_type != type_name {
                    let other_pattern = format!("\n{} = ", other_type);
                    if let Some(pos) = output[content_start..].find(&other_pattern) {
                        content_end = content_end.min(content_start + pos);
                    }
                }
            }

            let content = &output[content_start..content_end];
            files.insert(filename, content.to_string());
        }
    }

    ParsingAttempt {
        strategy: "Type name split".to_string(),
        success: !files.is_empty(),
        error: if files.is_empty() {
            Some("No type definitions found".to_string())
        } else {
            None
        },
        extracted_files: files,
    }
}

fn try_file_marker_split(output: &str) -> ParsingAttempt {
    let mut files = BTreeMap::new();
    let parts: Vec<&str> = output.split("# File: ").collect();

    for part in parts.iter().skip(1) {
        if let Some(newline_pos) = part.find('\n') {
            let filename = part[..newline_pos].trim().to_string();
            let content = part[newline_pos + 1..].to_string();
            files.insert(filename, content);
        }
    }

    ParsingAttempt {
        strategy: "File marker split".to_string(),
        success: !files.is_empty(),
        error: if files.is_empty() {
            Some("No '# File: ' markers found".to_string())
        } else {
            None
        },
        extracted_files: files,
    }
}

fn try_record_boundary_split(output: &str) -> ParsingAttempt {
    let mut files = BTreeMap::new();
    let mut in_record = false;
    let mut brace_depth: i32 = 0;
    let mut current_content = String::new();
    let mut file_counter = 0;

    for line in output.lines() {
        for ch in line.chars() {
            match ch {
                '{' => {
                    brace_depth += 1;
                    in_record = true;
                }
                '}' => {
                    brace_depth = brace_depth.saturating_sub(1);
                    if brace_depth == 0 && in_record {
                        current_content.push(ch);
                        // End of a complete record
                        files.insert(
                            format!("module_{}.ncl", file_counter),
                            current_content.clone(),
                        );
                        file_counter += 1;
                        current_content.clear();
                        in_record = false;
                        continue;
                    }
                }
                _ => {}
            }
            current_content.push(ch);
        }
        current_content.push('\n');
    }

    ParsingAttempt {
        strategy: "Record boundary split".to_string(),
        success: !files.is_empty(),
        error: if files.is_empty() {
            Some("No complete records found".to_string())
        } else {
            None
        },
        extracted_files: files,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_structure_generation() -> Result<(), Box<dyn std::error::Error>> {
        // Create a simple IR for testing
        let mut ir = IR::new();

        // Add a module with types that reference each other
        let mut module = Module {
            name: "test.v1".to_string(),
            imports: vec![],
            types: vec![],
            constants: vec![],
            metadata: Default::default(),
        };

        // Add LabelSelector type
        let mut label_selector_fields = BTreeMap::new();
        label_selector_fields.insert(
            "matchLabels".to_string(),
            Field {
                ty: Type::Map {
                    key: Box::new(Type::String),
                    value: Box::new(Type::String),
                },
                required: false,
                description: Some("Labels to match".to_string()),
                default: None,
            },
        );

        module.types.push(TypeDefinition {
            name: "LabelSelector".to_string(),
            ty: Type::Record {
                fields: label_selector_fields,
                open: false,
            },
            documentation: Some("A label selector".to_string()),
            annotations: BTreeMap::new(),
        });

        // Add type that references LabelSelector
        let mut topology_fields = BTreeMap::new();
        topology_fields.insert(
            "labelSelector".to_string(),
            Field {
                ty: Type::Optional(Box::new(Type::Reference {
                    name: "LabelSelector".to_string(),
                    module: None,
                })),
                required: false,
                description: Some("Label selector".to_string()),
                default: None,
            },
        );

        module.types.push(TypeDefinition {
            name: "TopologySpreadConstraint".to_string(),
            ty: Type::Record {
                fields: topology_fields,
                open: false,
            },
            documentation: Some("Topology constraint".to_string()),
            annotations: BTreeMap::new(),
        });

        ir.modules.push(module);

        // Generate diagnostics
        let diagnostics = create_diagnostics(&ir);

        // Export as JSON for analysis
        let json = serde_json::to_string_pretty(&diagnostics)?;
        println!("==== Diagnostic Output ====");
        println!("{}", json);
        println!("==== End Diagnostic Output ====");

        // Verify the structure captures what we need
        assert_eq!(diagnostics.input_ir.module_count, 1);
        assert_eq!(diagnostics.input_ir.modules[0].type_count, 2);

        // Check that references were extracted
        let topology_type = &diagnostics.input_ir.modules[0].types[1];
        assert_eq!(topology_type.name, "TopologySpreadConstraint");
        assert!(topology_type
            .references
            .contains(&"LabelSelector".to_string()));

        // Check parsing attempts were made
        assert!(!diagnostics.parsing_attempts.is_empty());

        // Print analysis summary
        println!("\n==== Analysis Summary ====");
        println!("Modules in IR: {}", diagnostics.input_ir.module_count);
        for module in &diagnostics.input_ir.modules {
            println!("  Module '{}': {} types", module.name, module.type_count);
            for typ in &module.types {
                println!("    - {} ({})", typ.name, typ.kind);
                if !typ.references.is_empty() {
                    println!("      References: {:?}", typ.references);
                }
            }
        }

        println!("\nParsing Attempts:");
        for attempt in &diagnostics.parsing_attempts {
            println!(
                "  Strategy: {} - Success: {}",
                attempt.strategy, attempt.success
            );
            if let Some(err) = &attempt.error {
                println!("    Error: {}", err);
            }
            if attempt.success {
                println!("    Extracted {} files", attempt.extracted_files.len());
            }
        }
        Ok(())
    }
}
