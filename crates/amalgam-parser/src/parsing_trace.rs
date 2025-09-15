use serde::{Deserialize, Serialize};
/// Structured tracing for package parsing operations
use std::collections::BTreeMap;

/// Complete trace of the parsing operation
#[derive(Debug, Serialize, Deserialize)]
pub struct ParsingTrace {
    /// Input characteristics
    pub input: InputTrace,

    /// Module parsing steps
    pub module_parsing: Vec<ModuleParsingStep>,

    /// Type extraction attempts
    pub type_extractions: Vec<TypeExtractionAttempt>,

    /// Final result
    pub result: ParsingResult,

    /// Any fallback operations
    pub fallbacks: Vec<FallbackOperation>,
}

/// Trace of input data
#[derive(Debug, Serialize, Deserialize)]
pub struct InputTrace {
    /// Total length of concatenated output
    pub total_length: usize,

    /// Number of modules in IR
    pub ir_module_count: usize,

    /// Module names in IR
    pub ir_modules: Vec<ModuleInfo>,

    /// First 200 chars of raw input
    pub raw_preview: String,

    /// Module markers found
    pub module_markers_found: Vec<String>,
}

/// Information about a module in the IR
#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleInfo {
    pub name: String,
    pub type_count: usize,
    pub type_names: Vec<String>,
}

/// A step in parsing a module section
#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleParsingStep {
    /// Module section being processed
    pub section_index: usize,

    /// Raw section content
    pub raw_section: String,

    /// Extracted module name
    pub extracted_name: Option<String>,

    /// Module found in IR?
    pub found_in_ir: bool,

    /// Module content after name extraction
    pub module_content: String,

    /// Line count
    pub line_count: usize,
}

/// Attempt to extract a type
#[derive(Debug, Serialize, Deserialize)]
pub struct TypeExtractionAttempt {
    /// Module name
    pub module_name: String,

    /// Type name being extracted
    pub type_name: String,

    /// Extraction strategy used
    pub strategy: String,

    /// Success status
    pub success: bool,

    /// Extracted content preview (first 100 chars)
    pub content_preview: Option<String>,

    /// Error if failed
    pub error: Option<String>,

    /// File name that would be generated
    pub target_file: String,
}

impl TypeExtractionAttempt {
    /// Create a new type extraction attempt
    pub fn new(
        module_name: &str,
        type_name: &str,
        strategy: &str,
        success: bool,
        content: Option<&str>,
        error: Option<String>,
        target_file: &str,
    ) -> Self {
        Self {
            module_name: module_name.to_string(),
            type_name: type_name.to_string(),
            strategy: strategy.to_string(),
            success,
            content_preview: content.map(|c| c.chars().take(100).collect()),
            error,
            target_file: target_file.to_string(),
        }
    }
}

/// Final parsing result
#[derive(Debug, Serialize, Deserialize)]
pub struct ParsingResult {
    /// Number of files successfully extracted
    pub files_extracted: usize,

    /// File names extracted
    pub file_names: Vec<String>,

    /// Files that failed to extract
    pub failed_extractions: Vec<String>,

    /// Whether fallback was triggered
    pub used_fallback: bool,
}

/// Fallback operation details
#[derive(Debug, Serialize, Deserialize)]
pub struct FallbackOperation {
    /// Reason for fallback
    pub reason: String,

    /// Fallback strategy used
    pub strategy: String,

    /// Files generated via fallback
    pub files_generated: Vec<String>,
}

impl Default for ParsingTrace {
    fn default() -> Self {
        Self::new()
    }
}

impl ParsingTrace {
    pub fn new() -> Self {
        Self {
            input: InputTrace {
                total_length: 0,
                ir_module_count: 0,
                ir_modules: vec![],
                raw_preview: String::new(),
                module_markers_found: vec![],
            },
            module_parsing: vec![],
            type_extractions: vec![],
            result: ParsingResult {
                files_extracted: 0,
                file_names: vec![],
                failed_extractions: vec![],
                used_fallback: false,
            },
            fallbacks: vec![],
        }
    }

    /// Record input characteristics
    pub fn record_input(&mut self, raw_output: &str, ir: &amalgam_core::IR) {
        self.input.total_length = raw_output.len();
        self.input.ir_module_count = ir.modules.len();

        // Record module info from IR
        self.input.ir_modules = ir
            .modules
            .iter()
            .map(|m| ModuleInfo {
                name: m.name.clone(),
                type_count: m.types.len(),
                type_names: m.types.iter().map(|t| t.name.clone()).collect(),
            })
            .collect();

        // Preview of raw input
        self.input.raw_preview = raw_output.chars().take(200).collect();

        // Find module markers
        for line in raw_output.lines() {
            if line.starts_with("# Module:") {
                self.input.module_markers_found.push(line.to_string());
            }
        }
    }

    /// Record a module parsing step
    pub fn record_module_parse(
        &mut self,
        section_index: usize,
        raw_section: &str,
        extracted_name: Option<String>,
        found_in_ir: bool,
        module_content: &str,
    ) {
        self.module_parsing.push(ModuleParsingStep {
            section_index,
            raw_section: raw_section.chars().take(500).collect(), // Limit size
            extracted_name,
            found_in_ir,
            module_content: module_content.chars().take(500).collect(),
            line_count: module_content.lines().count(),
        });
    }

    /// Record a type extraction attempt
    pub fn record_type_extraction(&mut self, attempt: TypeExtractionAttempt) {
        self.type_extractions.push(attempt);
    }

    /// Record final result
    pub fn record_result(&mut self, files: &BTreeMap<String, String>, used_fallback: bool) {
        self.result.files_extracted = files.len();
        self.result.file_names = files.keys().cloned().collect();
        self.result.used_fallback = used_fallback;

        // Find failed extractions by comparing with IR
        let extracted_types: Vec<String> = files
            .keys()
            .filter_map(|f| f.strip_suffix(".ncl"))
            .map(|s| s.to_string())
            .collect();

        for module_info in &self.input.ir_modules {
            for type_name in &module_info.type_names {
                if !extracted_types.contains(&type_name.to_lowercase()) {
                    self.result
                        .failed_extractions
                        .push(format!("{}.{}", module_info.name, type_name));
                }
            }
        }
    }

    /// Record a fallback operation
    pub fn record_fallback(&mut self, reason: &str, strategy: &str, files: Vec<String>) {
        self.fallbacks.push(FallbackOperation {
            reason: reason.to_string(),
            strategy: strategy.to_string(),
            files_generated: files,
        });
    }

    /// Export as JSON for analysis
    #[allow(dead_code)]
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .unwrap_or_else(|e| format!("Failed to serialize trace: {}", e))
    }

    /// Create a summary report
    pub fn summary(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Parsing Trace Summary ===\n");
        report.push_str(&format!(
            "Input: {} chars, {} IR modules\n",
            self.input.total_length, self.input.ir_module_count
        ));
        report.push_str(&format!(
            "Module markers found: {}\n",
            self.input.module_markers_found.len()
        ));

        if !self.module_parsing.is_empty() {
            report.push_str("\nModule Parsing:\n");
            for (i, step) in self.module_parsing.iter().enumerate() {
                report.push_str(&format!(
                    "  [{}] Module: {:?}, Found in IR: {}\n",
                    i, step.extracted_name, step.found_in_ir
                ));
            }
        }

        if !self.type_extractions.is_empty() {
            report.push_str("\nType Extractions:\n");
            let successful = self.type_extractions.iter().filter(|t| t.success).count();
            let failed = self.type_extractions.len() - successful;
            report.push_str(&format!(
                "  Successful: {}, Failed: {}\n",
                successful, failed
            ));

            for extraction in self.type_extractions.iter().filter(|t| !t.success) {
                report.push_str(&format!(
                    "  ✗ {}.{}: {:?}\n",
                    extraction.module_name, extraction.type_name, extraction.error
                ));
            }
        }

        report.push_str(&format!(
            "\nResult: {} files extracted\n",
            self.result.files_extracted
        ));
        if self.result.used_fallback {
            report.push_str("⚠️  Fallback was used\n");
        }

        if !self.result.failed_extractions.is_empty() {
            report.push_str(&format!(
                "Failed to extract: {:?}\n",
                self.result.failed_extractions
            ));
        }

        report
    }
}

/// Builder pattern for constructing traces
pub struct TraceBuilder {
    trace: ParsingTrace,
}

impl Default for TraceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TraceBuilder {
    pub fn new() -> Self {
        Self {
            trace: ParsingTrace::new(),
        }
    }

    pub fn with_input(mut self, raw: &str, ir: &amalgam_core::IR) -> Self {
        self.trace.record_input(raw, ir);
        self
    }

    pub fn build(self) -> ParsingTrace {
        self.trace
    }
}
