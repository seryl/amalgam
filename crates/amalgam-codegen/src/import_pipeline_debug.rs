use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Comprehensive debug structure for the entire import generation pipeline
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImportPipelineDebug {
    /// Stage 1: Symbol table construction
    pub symbol_table: SymbolTableDebug,

    /// Stage 2: Dependency analysis per type
    pub dependency_analysis: HashMap<String, DependencyAnalysis>,

    /// Stage 3: Import path generation
    pub import_generation: HashMap<String, ImportGeneration>,

    /// Stage 4: Module generation
    pub module_generation: ModuleGenerationDebug,

    /// Pipeline summary
    pub summary: PipelineSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SymbolTableDebug {
    /// Total symbols registered
    pub total_symbols: usize,

    /// Symbols by module
    pub symbols_by_module: HashMap<String, Vec<String>>,

    /// Symbol entries (type_name -> (module, group, version))
    pub symbol_entries: HashMap<String, (String, String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DependencyAnalysis {
    /// The type being analyzed
    pub type_name: String,

    /// Module this type belongs to
    pub module: String,

    /// All type references found in this type's definition
    pub references_found: Vec<TypeReference>,

    /// Dependencies identified (type names that need imports)
    pub dependencies_identified: HashSet<String>,

    /// Self-references that were filtered out
    pub self_references_filtered: Vec<String>,

    /// References not found in symbol table
    pub unresolved_references: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeReference {
    /// Name of the referenced type
    pub name: String,

    /// Context where it was found (e.g., "field: containerUser", "array element")
    pub context: String,

    /// Whether it has an explicit module
    pub has_module: bool,

    /// The module if specified
    pub module: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImportGeneration {
    /// Type name
    pub type_name: String,

    /// Dependencies that need imports
    pub dependencies: Vec<String>,

    /// Import statements generated
    pub import_statements: Vec<ImportStatement>,

    /// Path calculation details
    pub path_calculations: Vec<PathCalculation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportStatement {
    /// The dependency being imported
    pub dependency: String,

    /// The import statement generated
    pub statement: String,

    /// Path used in the import
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathCalculation {
    /// From module
    pub from_module: String,

    /// To module  
    pub to_module: String,

    /// Calculated path
    pub calculated_path: String,

    /// Path type (same-version, cross-version, cross-package)
    pub path_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModuleGenerationDebug {
    /// Modules processed
    pub modules_processed: Vec<String>,

    /// Module content sizes (module -> character count)
    pub module_sizes: HashMap<String, usize>,

    /// Whether module markers were added
    pub module_markers_added: HashMap<String, bool>,

    /// Types per module
    pub types_per_module: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PipelineSummary {
    /// Total types processed
    pub total_types: usize,

    /// Types with dependencies
    pub types_with_dependencies: usize,

    /// Types with imports generated
    pub types_with_imports: usize,

    /// Total imports generated
    pub total_imports: usize,

    /// Unresolved references
    pub total_unresolved: usize,

    /// Issues found
    pub issues: Vec<PipelineIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineIssue {
    /// Stage where issue occurred
    pub stage: String,

    /// Type affected
    pub type_name: String,

    /// Description of the issue
    pub description: String,

    /// Severity (error, warning, info)
    pub severity: String,
}

impl ImportPipelineDebug {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_symbol(&mut self, type_name: &str, module: &str, group: &str, version: &str) {
        self.symbol_table.total_symbols += 1;
        self.symbol_table
            .symbols_by_module
            .entry(module.to_string())
            .or_default()
            .push(type_name.to_string());
        self.symbol_table.symbol_entries.insert(
            type_name.to_string(),
            (module.to_string(), group.to_string(), version.to_string()),
        );
    }

    pub fn start_dependency_analysis(
        &mut self,
        type_name: &str,
        module: &str,
    ) -> &mut DependencyAnalysis {
        self.dependency_analysis
            .entry(type_name.to_string())
            .or_insert_with(|| DependencyAnalysis {
                type_name: type_name.to_string(),
                module: module.to_string(),
                ..Default::default()
            })
    }

    pub fn record_import_generation(&mut self, type_name: &str, generation: ImportGeneration) {
        self.import_generation
            .insert(type_name.to_string(), generation);
    }

    pub fn record_module_generation(
        &mut self,
        module: &str,
        types: &[String],
        content_size: usize,
        has_marker: bool,
    ) {
        self.module_generation
            .modules_processed
            .push(module.to_string());
        self.module_generation
            .module_sizes
            .insert(module.to_string(), content_size);
        self.module_generation
            .module_markers_added
            .insert(module.to_string(), has_marker);
        self.module_generation
            .types_per_module
            .insert(module.to_string(), types.to_vec());
    }

    pub fn finalize_summary(&mut self) {
        self.summary.total_types = self.dependency_analysis.len();
        self.summary.types_with_dependencies = self
            .dependency_analysis
            .values()
            .filter(|d| !d.dependencies_identified.is_empty())
            .count();
        self.summary.types_with_imports = self
            .import_generation
            .values()
            .filter(|g| !g.import_statements.is_empty())
            .count();
        self.summary.total_imports = self
            .import_generation
            .values()
            .map(|g| g.import_statements.len())
            .sum();
        self.summary.total_unresolved = self
            .dependency_analysis
            .values()
            .map(|d| d.unresolved_references.len())
            .sum();

        // Check for issues
        for (type_name, analysis) in &self.dependency_analysis {
            if !analysis.dependencies_identified.is_empty() {
                // Check if imports were generated
                if let Some(generation) = self.import_generation.get(type_name) {
                    if generation.import_statements.is_empty() {
                        self.summary.issues.push(PipelineIssue {
                            stage: "import_generation".to_string(),
                            type_name: type_name.clone(),
                            description: format!(
                                "Type has {} dependencies but no imports generated",
                                analysis.dependencies_identified.len()
                            ),
                            severity: "error".to_string(),
                        });
                    }
                } else {
                    self.summary.issues.push(PipelineIssue {
                        stage: "import_generation".to_string(),
                        type_name: type_name.clone(),
                        description: "Type has dependencies but no import generation record"
                            .to_string(),
                        severity: "error".to_string(),
                    });
                }
            }

            if !analysis.unresolved_references.is_empty() {
                self.summary.issues.push(PipelineIssue {
                    stage: "dependency_analysis".to_string(),
                    type_name: type_name.clone(),
                    description: format!(
                        "Has {} unresolved references: {:?}",
                        analysis.unresolved_references.len(),
                        analysis.unresolved_references
                    ),
                    severity: "warning".to_string(),
                });
            }
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("Error serializing: {}", e))
    }

    pub fn summary_string(&self) -> String {
        let mut s = String::new();
        s.push_str("=== Import Pipeline Debug Summary ===\n");
        s.push_str(&format!(
            "Symbol Table: {} symbols across {} modules\n",
            self.symbol_table.total_symbols,
            self.symbol_table.symbols_by_module.len()
        ));
        s.push_str(&format!(
            "Dependency Analysis: {} types analyzed\n",
            self.dependency_analysis.len()
        ));
        s.push_str(&format!(
            "  - {} types have dependencies\n",
            self.summary.types_with_dependencies
        ));
        s.push_str(&format!(
            "  - {} types have imports generated\n",
            self.summary.types_with_imports
        ));
        s.push_str(&format!(
            "  - {} total imports generated\n",
            self.summary.total_imports
        ));
        s.push_str(&format!(
            "  - {} unresolved references\n",
            self.summary.total_unresolved
        ));

        if !self.summary.issues.is_empty() {
            s.push_str(&format!(
                "\n⚠️  {} issues found:\n",
                self.summary.issues.len()
            ));
            for issue in &self.summary.issues {
                s.push_str(&format!(
                    "  [{} {}] {}: {}\n",
                    issue.severity.to_uppercase(),
                    issue.stage,
                    issue.type_name,
                    issue.description
                ));
            }
        }

        s.push_str(&format!(
            "\nModule Generation: {} modules\n",
            self.module_generation.modules_processed.len()
        ));
        let with_markers = self
            .module_generation
            .module_markers_added
            .values()
            .filter(|&&v| v)
            .count();
        s.push_str(&format!("  - {} with module markers\n", with_markers));

        s
    }

    pub fn type_report(&self, type_name: &str) -> String {
        let mut s = String::new();
        s.push_str(&format!("=== Report for Type: {} ===\n", type_name));

        // Symbol table entry
        if let Some(entry) = self.symbol_table.symbol_entries.get(type_name) {
            s.push_str(&format!(
                "Symbol Table: module={}, group={}, version={}\n",
                entry.0, entry.1, entry.2
            ));
        } else {
            s.push_str("Symbol Table: NOT FOUND\n");
        }

        // Dependency analysis
        if let Some(analysis) = self.dependency_analysis.get(type_name) {
            s.push_str(&format!(
                "Dependencies Found: {}\n",
                analysis.dependencies_identified.len()
            ));
            for dep in &analysis.dependencies_identified {
                s.push_str(&format!("  - {}\n", dep));
            }
            if !analysis.unresolved_references.is_empty() {
                s.push_str(&format!(
                    "Unresolved: {:?}\n",
                    analysis.unresolved_references
                ));
            }
        } else {
            s.push_str("Dependency Analysis: NOT FOUND\n");
        }

        // Import generation
        if let Some(generation) = self.import_generation.get(type_name) {
            s.push_str(&format!(
                "Imports Generated: {}\n",
                generation.import_statements.len()
            ));
            for stmt in &generation.import_statements {
                s.push_str(&format!("  - {}\n", stmt.statement));
            }
        } else {
            s.push_str("Import Generation: NOT FOUND\n");
        }

        s
    }
}
