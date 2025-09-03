//! Nickel code generator with improved formatting

use crate::import_pipeline_debug::{ImportPipelineDebug, TypeReference};
use crate::package_mode::PackageMode;
use crate::resolver::{ResolutionContext, TypeResolver};
use crate::{Codegen, CodegenError};
use amalgam_core::{
    compilation_unit::CompilationUnit,
    debug::{CompilationDebugInfo, DebugConfig, ImportDebugEntry, ImportDebugInfo},
    module_registry::ModuleRegistry,
    naming::to_camel_case,
    special_cases::SpecialCasePipeline,
    types::{Field, Type},
    ImportPathCalculator, IR,
};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::sync::Arc;
use tracing::{debug, instrument, warn};

/// Debug information for tracking import generation
#[derive(Debug, Default)]
pub struct ImportGenerationDebug {
    /// Types found in symbol table: type_name -> (module, group, version)
    pub symbol_table_entries: HashMap<String, (String, String, String)>,
    /// References found during analysis: (referencing_module, referenced_type, resolved_location)
    pub references_found: Vec<(String, String, Option<String>)>,
    /// Dependencies identified for import: (from_module, to_type, reason)
    pub dependencies_identified: Vec<(String, String, String)>,
    /// Imports generated: (in_module, import_statement)
    pub imports_generated: Vec<(String, String)>,
    /// Missing types not found in symbol table
    pub missing_types: Vec<(String, String)>, // (module, type_name)
}

/// Map tracking which imports each type needs
#[derive(Debug, Clone, Default)]
pub struct TypeImportMap {
    /// Map from type name to list of import statements it needs
    type_imports: HashMap<String, Vec<String>>,
}

impl TypeImportMap {
    pub fn new() -> Self {
        Self {
            type_imports: HashMap::new(),
        }
    }

    /// Add an import for a specific type
    pub fn add_import(&mut self, type_name: &str, import_stmt: &str) {
        let imports = self.type_imports.entry(type_name.to_string()).or_default();

        // Only add if not already present (deduplicate)
        if !imports.contains(&import_stmt.to_string()) {
            imports.push(import_stmt.to_string());
        }
    }

    /// Get all imports needed by a type
    pub fn get_imports_for(&self, type_name: &str) -> Vec<String> {
        self.type_imports
            .get(type_name)
            .cloned()
            .unwrap_or_default()
    }

    /// Get total count of imports across all types
    pub fn total_import_count(&self) -> usize {
        self.type_imports
            .values()
            .map(|imports| imports.len())
            .sum()
    }
}

pub struct NickelCodegen {
    indent_size: usize,
    resolver: TypeResolver,
    package_mode: PackageMode,
    /// Module registry for import path resolution
    registry: Arc<ModuleRegistry>,
    /// Import path calculator using the registry
    import_calculator: ImportPathCalculator,
    /// Special case handler pipeline
    special_cases: Option<SpecialCasePipeline>,
    /// Track cross-module imports needed for the current module
    current_imports: HashSet<(String, String)>, // (version, type_name)
    /// Same-package dependencies for current module (Phase 2)
    same_package_deps: HashSet<String>, // type names that need imports
    /// Debug information for tracking import generation
    pub debug_info: ImportGenerationDebug,
    /// Track which imports each type needs (for extraction)
    type_import_map: TypeImportMap,
    /// Track the current type being processed (for per-type import tracking)
    current_type_name: Option<String>,
    /// Comprehensive pipeline debug
    pub pipeline_debug: ImportPipelineDebug,
    /// Debug configuration
    debug_config: DebugConfig,
    /// Compilation debug info (collected when debug_config is enabled)
    compilation_debug: CompilationDebugInfo,
    /// Track imported types for the current module being generated (Phase 2)
    /// Maps type name to whether it's a same-directory import (true) or cross-module (false)
    current_module_imports: HashMap<String, bool>,
}

impl NickelCodegen {
    pub fn new(registry: Arc<ModuleRegistry>) -> Self {
        let import_calculator = ImportPathCalculator::new(registry.clone());
        Self {
            indent_size: 2,
            resolver: TypeResolver::new(),
            package_mode: PackageMode::default(),
            registry,
            import_calculator,
            special_cases: None,
            current_imports: HashSet::new(),
            same_package_deps: HashSet::new(),
            debug_info: ImportGenerationDebug::default(),
            type_import_map: TypeImportMap::new(),
            current_type_name: None,
            pipeline_debug: ImportPipelineDebug::new(),
            debug_config: DebugConfig::default(),
            compilation_debug: CompilationDebugInfo::new(),
            current_module_imports: HashMap::new(),
        }
    }

    /// Set the special case pipeline
    pub fn set_special_cases(&mut self, pipeline: SpecialCasePipeline) {
        self.special_cases = Some(pipeline);
    }

    /// Create with a new registry built from IR
    pub fn from_ir(ir: &IR) -> Self {
        let registry = Arc::new(ModuleRegistry::from_ir(ir));
        Self::new(registry)
    }

    /// Create with an empty registry (mainly for tests)
    #[cfg(test)]
    pub fn new_for_test() -> Self {
        let registry = Arc::new(ModuleRegistry::new());
        Self::new(registry)
    }

    /// Set debug configuration
    pub fn with_debug_config(mut self, config: DebugConfig) -> Self {
        self.debug_config = config;
        self
    }

    pub fn with_package_mode(mut self, mode: PackageMode) -> Self {
        self.package_mode = mode;
        self
    }

    /// Get the compilation debug info (for testing)
    pub fn compilation_debug(&self) -> &CompilationDebugInfo {
        &self.compilation_debug
    }

    /// Get mutable compilation debug info (for export)
    pub fn compilation_debug_mut(&mut self) -> &mut CompilationDebugInfo {
        &mut self.compilation_debug
    }

    /// Sync pipeline debug data into compilation debug
    fn sync_debug_to_compilation(&mut self) {
        if !self.debug_config.should_debug_imports() {
            return;
        }

        // Transfer import generation data to compilation debug
        for (type_name, import_gen) in &self.pipeline_debug.import_generation {
            // Find the module for this type from dependency analysis
            if let Some(dep_analysis) = self.pipeline_debug.dependency_analysis.get(type_name) {
                // Normalize the module name for consistency
                let (group, version) = Self::parse_module_name(&dep_analysis.module);
                let normalized_module = format!("{}.{}", group, version);

                // Create ImportDebugInfo from pipeline debug data
                let mut imports = Vec::new();
                for stmt in &import_gen.import_statements {
                    imports.push(ImportDebugEntry {
                        dependency: stmt.dependency.clone(),
                        import_path: stmt.path.clone(),
                        import_statement: stmt.statement.clone(),
                        resolution_strategy: "pipeline".to_string(),
                    });
                }

                if !imports.is_empty() {
                    let debug_info = ImportDebugInfo {
                        module_name: normalized_module.clone(),
                        type_name: type_name.clone(),
                        imports,
                        symbol_table: HashMap::new(),
                        path_calculations: Vec::new(),
                    };

                    self.compilation_debug
                        .modules
                        .entry(normalized_module)
                        .or_default()
                        .push(debug_info);
                }
            }
        }
    }

    /// Get the correct module path for k8s.io consolidated structure
    /// Maps individual type file paths to their actual consolidated module locations
    fn get_k8s_module_path(&self, import_path: &str, type_name: &str) -> String {
        // The import_path will be something like "../../apimachinery_pkg_apis_meta/v1/ObjectMeta.ncl"
        // We need to map this to the actual consolidated module path

        // First convert underscores back to dots for k8s.io modules
        let normalized_path = import_path
            .replace("apimachinery_pkg_apis_meta", "apimachinery.pkg.apis/meta")
            .replace("apimachinery_pkg_apis", "apimachinery.pkg.apis")
            .replace("api_", "api/");

        // Extract the components from the path
        if normalized_path.contains("apimachinery.pkg.apis") {
            // This should map to apimachinery.pkg.apis/meta/v1/mod.ncl (consolidated module)
            if normalized_path.contains("/v1/") || normalized_path.contains("/v1.") {
                "../../apimachinery.pkg.apis/meta/v1/mod.ncl".to_string()
            } else if normalized_path.contains("/v1alpha1/")
                || normalized_path.contains("/v1alpha1.")
            {
                "../../apimachinery.pkg.apis/meta/v1alpha1/mod.ncl".to_string()
            } else if normalized_path.contains("/v1beta1/") || normalized_path.contains("/v1beta1.")
            {
                "../../apimachinery.pkg.apis/meta/v1beta1/mod.ncl".to_string()
            } else {
                // Default to v1 if version not clear
                "../../apimachinery.pkg.apis/meta/v1/mod.ncl".to_string()
            }
        } else if normalized_path.contains("/v0/") || normalized_path.contains("v0.ncl") {
            // v0 types are in the root v0.ncl
            "../../v0/mod.ncl".to_string()
        } else if normalized_path.ends_with(&format!("/{}.ncl", type_name)) {
            // Regular API types - convert to consolidated module
            // e.g., "../v1/Pod.ncl" -> "../v1.ncl"
            normalized_path.replace(&format!("/{}.ncl", type_name), ".ncl")
        } else {
            // Default: return normalized path
            normalized_path
        }
    }

    /// Generate code with two-phase compilation using CompilationUnit
    /// This ensures all cross-module dependencies are resolved before generation
    pub fn generate_with_compilation_unit(
        &mut self,
        compilation_unit: &CompilationUnit,
    ) -> Result<String, CodegenError> {
        let mut output = String::new();

        // Process modules in topological order to ensure dependencies are available
        let module_order = compilation_unit
            .get_modules_in_order()
            .map_err(|e| CodegenError::Generation(format!("Failed to get module order: {}", e)))?;

        for module_id in module_order {
            let analysis = compilation_unit.modules.get(&module_id).ok_or_else(|| {
                CodegenError::Generation(format!(
                    "Module {} not found in compilation unit",
                    module_id
                ))
            })?;

            let module = &analysis.module;

            // Generate module-level imports based on analysis
            let mut module_imports = Vec::new();
            self.current_module_imports.clear(); // Reset for this module

            // Track k8s module imports to consolidate them
            let mut k8s_module_imports: HashMap<String, (String, Vec<String>)> = HashMap::new();

            if let Some(required_imports) = compilation_unit.get_module_imports(&module_id) {
                for (imported_module_id, imported_types) in required_imports {
                    // Calculate the import path from current module to imported module
                    let (current_group, current_version) = Self::parse_module_name(&module_id);
                    let (import_group, import_version) =
                        Self::parse_module_name(imported_module_id);

                    // Check if this is a k8s.io or apimachinery module that needs consolidation
                    if import_group.contains("k8s.io")
                        || import_group.starts_with("io.k8s.")
                        || import_group.starts_with("apimachinery.")
                    {
                        // For k8s types, we need to consolidate by module
                        for type_name in imported_types {
                            let import_path = if let Some(ref special_cases) = self.special_cases {
                                if let Some(override_path) =
                                    special_cases.get_import_override(&module_id, type_name)
                                {
                                    override_path
                                } else {
                                    self.import_calculator.calculate(
                                        &current_group,
                                        &current_version,
                                        &import_group,
                                        &import_version,
                                        type_name,
                                    )
                                }
                            } else {
                                self.import_calculator.calculate(
                                    &current_group,
                                    &current_version,
                                    &import_group,
                                    &import_version,
                                    type_name,
                                )
                            };

                            let module_path = self.get_k8s_module_path(&import_path, type_name);
                            // Use the same alias generation logic as in generate_with_compilation_unit
                            let module_alias = Self::generate_module_alias(&module_path);

                            // Add to consolidated imports
                            k8s_module_imports
                                .entry(module_path.clone())
                                .or_insert((module_alias, Vec::new()))
                                .1
                                .push(type_name.clone());

                            // Track that this type is imported for reference generation
                            // K8s imports are always cross-module (false)
                            self.current_module_imports.insert(type_name.clone(), false);
                        }
                    } else {
                        // Import each type individually for non-k8s modules
                        for type_name in imported_types {
                            let import_path = if let Some(ref special_cases) = self.special_cases {
                                if let Some(override_path) =
                                    special_cases.get_import_override(&module_id, type_name)
                                {
                                    override_path
                                } else {
                                    self.import_calculator.calculate(
                                        &current_group,
                                        &current_version,
                                        &import_group,
                                        &import_version,
                                        type_name,
                                    )
                                }
                            } else {
                                self.import_calculator.calculate(
                                    &current_group,
                                    &current_version,
                                    &import_group,
                                    &import_version,
                                    type_name,
                                )
                            };

                            // Regular import for non-k8s types
                            // For same-directory imports (./), use PascalCase
                            // For cross-module imports, use camelCase
                            let is_same_directory = import_path.starts_with("./");
                            let import_alias = if is_same_directory {
                                type_name.clone()
                            } else {
                                to_camel_case(type_name)
                            };
                            let stmt =
                                format!("let {} = import \"{}\" in", import_alias, import_path);
                            module_imports.push(stmt);

                            // Track that this type is imported for reference generation
                            self.current_module_imports
                                .insert(type_name.clone(), is_same_directory);
                        }
                    }
                }
            }

            // Generate consolidated k8s module imports
            for (module_path, (module_alias, type_names)) in k8s_module_imports {
                // Import the module once
                module_imports.push(format!(
                    "let {} = import \"{}\" in",
                    module_alias, module_path
                ));

                // Extract each type with proper 'in' keywords
                for type_name in type_names {
                    // Use camelCase for variable (left side), PascalCase for type (right side)
                    let type_alias = to_camel_case(&type_name);
                    module_imports.push(format!(
                        "let {} = {}.{} in",
                        type_alias, module_alias, type_name
                    ));
                }
            }

            // Generate the module with hoisted imports
            writeln!(output, "# Module: {}", module_id)
                .map_err(|e| CodegenError::Generation(e.to_string()))?;
            writeln!(output).map_err(|e| CodegenError::Generation(e.to_string()))?;

            // Write module-level imports at the top
            for import in &module_imports {
                writeln!(output, "{}", import)
                    .map_err(|e| CodegenError::Generation(e.to_string()))?;
            }
            if !module_imports.is_empty() {
                writeln!(output).map_err(|e| CodegenError::Generation(e.to_string()))?;
            }

            // Generate the module content
            let is_single_type = module.types.len() == 1 && module.constants.is_empty();

            if is_single_type {
                let type_def = &module.types[0];
                if let Some(doc) = &type_def.documentation {
                    for line in doc.lines() {
                        writeln!(output, "# {}", line)
                            .map_err(|e| CodegenError::Generation(e.to_string()))?;
                    }
                }
                let type_str = self.type_to_nickel(&type_def.ty, module, 0)?;
                writeln!(output, "{}", type_str)?;
            } else {
                writeln!(output, "{{")?;
                for (idx, type_def) in module.types.iter().enumerate() {
                    let type_str = self.type_to_nickel(&type_def.ty, module, 1)?;
                    if let Some(doc) = &type_def.documentation {
                        for line in doc.lines() {
                            writeln!(output, "{}# {}", self.indent(1), line)
                                .map_err(|e| CodegenError::Generation(e.to_string()))?;
                        }
                    }
                    let is_last_item = idx == module.types.len() - 1 && module.constants.is_empty();
                    if !is_last_item {
                        writeln!(output, "  {} = {},", type_def.name, type_str)?;
                        writeln!(output)?;
                    } else {
                        writeln!(output, "  {} = {}", type_def.name, type_str)?;
                    }
                }
                writeln!(output, "}}")?;
            }

            writeln!(output)?; // Add spacing between modules
        }

        Ok(output)
    }

    fn indent(&self, level: usize) -> String {
        " ".repeat(level * self.indent_size)
    }

    /// Escape field names that are reserved keywords or start with special characters
    fn escape_field_name(&self, name: &str) -> String {
        // Fields starting with $ need to be quoted
        if name.starts_with('$') || self.is_reserved_keyword(name) {
            format!("\"{}\"", name)
        } else {
            name.to_string()
        }
    }

    /// Check if a field name is a Nickel reserved keyword
    fn is_reserved_keyword(&self, name: &str) -> bool {
        matches!(
            name,
            "and"
                | "or"
                | "not"
                | "if"
                | "then"
                | "else"
                | "let"
                | "in"
                | "fun"
                | "import"
                | "match"
                | "rec"
                | "null"
                | "true"
                | "false"
                | "switch"
                | "default"
                | "forall"
                | "doc"
                | "optional"
                | "priority"
                | "force"
                | "merge"
        )
    }

    /// Phase 2: Analyze dependencies for a type and collect required imports
    #[instrument(skip(self, ty, current_module), level = "debug")]
    fn analyze_dependencies(&mut self, ty: &Type, current_module: &amalgam_core::ir::Module) {
        match ty {
            Type::Reference {
                name,
                module: ref_module,
            } => {
                debug!("Found reference: {} (module: {:?})", name, ref_module);

                // Record the reference in debug info
                let resolved_location = self
                    .registry
                    .find_module_for_type(name)
                    .map(|module_info| module_info.name.clone());
                self.debug_info.references_found.push((
                    current_module.name.clone(),
                    name.clone(),
                    resolved_location.clone(),
                ));

                // If no module specified, it's a same-package reference
                if ref_module.is_none() {
                    // Check if this type exists in our registry but not in current module
                    if let Some(module_info) = self.registry.find_module_for_type(name) {
                        debug!(
                            "Found type in registry: {} -> {} (current module: {})",
                            name, module_info.name, current_module.name
                        );
                        let (current_group, current_version) =
                            Self::parse_module_name(&current_module.name);

                        // Same package, same version
                        if module_info.group == current_group
                            && module_info.version == current_version
                        {
                            // Only add to imports if the type is actually in a different module
                            // When all types are in the same module (like in tests), they don't need imports
                            if module_info.name != current_module.name {
                                self.same_package_deps.insert(name.clone());
                                self.debug_info.dependencies_identified.push((
                                    current_module.name.clone(),
                                    name.clone(),
                                    "same-version-different-module".to_string(),
                                ));
                            }
                            // If it's the same module, no import needed - types can reference each other directly
                        }
                        // Same package (group), different version - need import
                        else if module_info.group == current_group
                            && module_info.version != current_version
                        {
                            self.same_package_deps.insert(name.clone());
                            self.debug_info.dependencies_identified.push((
                                current_module.name.clone(),
                                name.clone(),
                                "cross-version-same-package".to_string(),
                            ));
                        }
                    } else {
                        // Type not found in symbol table
                        self.debug_info
                            .missing_types
                            .push((current_module.name.clone(), name.clone()));
                    }
                }
            }
            Type::Array(elem) => {
                self.analyze_dependencies(elem, current_module);
            }
            Type::Map { value, .. } => {
                self.analyze_dependencies(value, current_module);
            }
            Type::Optional(inner) => {
                self.analyze_dependencies(inner, current_module);
            }
            Type::Record { fields, .. } => {
                for field in fields.values() {
                    self.analyze_dependencies(&field.ty, current_module);
                }
            }
            Type::Union { types, .. } => {
                for t in types {
                    self.analyze_dependencies(t, current_module);
                }
            }
            Type::TaggedUnion { variants, .. } => {
                for variant_type in variants.values() {
                    self.analyze_dependencies(variant_type, current_module);
                }
            }
            Type::Contract { base, .. } => {
                self.analyze_dependencies(base, current_module);
            }
            // Primitive types don't need dependency analysis
            _ => {}
        }
    }

    /// Format a documentation string properly
    /// Uses triple quotes for multiline, regular quotes for single line
    /// Parse group and version from a module name
    fn parse_module_name(module_name: &str) -> (String, String) {
        // Module names can be:
        // - "group.version" (e.g., "k8s.io.v1")
        // - "Kind.version.group" (e.g., "Composition.v1.apiextensions.crossplane.io")
        // - Legacy K8s format: "io.k8s.api.core.v1" (needs conversion to "k8s.io.v1")
        // - With underscores: "io_k8s_api_core_v1" (needs special handling)

        // Normalize legacy K8s module names first
        let (normalized_name, _transform_reason) =
            if module_name.starts_with("io.k8s.api.") || module_name.starts_with("io_k8s_api") {
                // Convert io.k8s.api.core.v1 -> k8s.io.v1
                // Convert io_k8s_api_core_v1 -> k8s.io.v1
                let separator = if module_name.contains('_') { '_' } else { '.' };
                let parts: Vec<&str> = module_name.split(separator).collect();
                if let Some(version_idx) = parts.iter().position(|&p| p.starts_with("v")) {
                    let version = parts[version_idx];
                    (format!("k8s.io.{}", version), Some("Legacy K8s API format"))
                } else {
                    (module_name.to_string(), None)
                }
            } else if module_name.starts_with("io.k8s.apimachinery")
                || module_name.starts_with("io_k8s_apimachinery")
            {
                // Check if this is an unversioned runtime or util type
                let separator = if module_name.contains('_') { '_' } else { '.' };
                let parts: Vec<&str> = module_name.split(separator).collect();

                // Check for runtime or util packages (unversioned, should map to v0)
                if parts.contains(&"runtime") || parts.contains(&"util") {
                    // io.k8s.apimachinery.pkg.runtime -> k8s.io.v0
                    // io.k8s.apimachinery.pkg.util -> k8s.io.v0
                    (
                        "k8s.io.v0".to_string(),
                        Some("Unversioned K8s runtime/util type"),
                    )
                } else if let Some(version_idx) = parts.iter().position(|&p| p.starts_with("v")) {
                    // Convert io.k8s.apimachinery.pkg.apis.meta.v1 -> k8s.io.v1
                    let version = parts[version_idx];
                    (
                        format!("k8s.io.{}", version),
                        Some("Legacy K8s apimachinery format"),
                    )
                } else {
                    // No version found and not runtime/util - default to v0
                    (
                        "k8s.io.v0".to_string(),
                        Some("Unversioned K8s apimachinery type"),
                    )
                }
            } else {
                (module_name.to_string(), None)
            };

        // Record transformation if it happened (requires mutable self, so we can't do it here)
        // This will be handled by the caller if needed

        // Now parse the normalized name
        let separator = if normalized_name.contains('_') && !normalized_name.contains('.') {
            '_'
        } else {
            '.'
        };

        let parts: Vec<&str> = normalized_name.split(separator).collect();

        // Try to identify version parts (v1, v1beta1, v1alpha1, v2, etc.)
        let version_pattern = |s: &str| {
            s.starts_with("v")
                && (s[1..].chars().all(|c| c.is_ascii_digit())
                    || s.contains("alpha")
                    || s.contains("beta"))
        };

        // Find the version part
        if let Some(version_idx) = parts.iter().position(|&p| version_pattern(p)) {
            let version = parts[version_idx].to_string();

            // If version is at the end or second-to-last position, it's "group.version" format
            if version_idx == parts.len() - 1 || version_idx == parts.len() - 2 {
                // Group is everything before the version
                let group = parts[..version_idx].join(&separator.to_string());
                return (group, version);
            }

            // Otherwise it's "Kind.version.group" format
            // Group is everything after the version
            let group = parts[version_idx + 1..].join(&separator.to_string());
            return (group, version);
        }

        // Fallback: assume last part is version if no clear version pattern
        if parts.len() >= 2 {
            let version = parts[parts.len() - 1].to_string();
            let group = parts[..parts.len() - 1].join(&separator.to_string());
            (group, version)
        } else {
            // Single part, use as group with empty version
            (normalized_name, String::new())
        }
    }

    fn format_doc(&self, doc: &str) -> String {
        if doc.contains('\n') || doc.len() > 80 {
            // Use multiline string format for multiline or long docs
            let trimmed_doc = doc.trim();

            // For multiline docs, use the m%"..."%  format
            // This preserves newlines and formatting within the doc string
            format!("m%\"\n{}\n\"%", trimmed_doc)
        } else {
            // Use regular quotes for short docs, properly escaping internal quotes
            format!("\"{}\"", doc.replace('"', "\\\""))
        }
    }

    fn type_to_nickel(
        &mut self,
        ty: &Type,
        module: &amalgam_core::ir::Module,
        indent_level: usize,
    ) -> Result<String, CodegenError> {
        let result = self.type_to_nickel_impl(ty, module, indent_level)?;
        Ok(result)
    }

    fn type_to_nickel_impl(
        &mut self,
        ty: &Type,
        module: &amalgam_core::ir::Module,
        indent_level: usize,
    ) -> Result<String, CodegenError> {
        // Analyze dependencies for this type
        self.analyze_dependencies(ty, module);

        // Debug: log type processing for LabelSelector case
        if let Type::Reference { name, .. } = ty {
            if name == "LabelSelector" {
                debug!(
                    "Processing LabelSelector reference in module {}",
                    module.name
                );
            }
        }

        match ty {
            Type::String => {
                tracing::info!("Type::String in current_type: {:?}", self.current_type_name);
                Ok("String".to_string())
            }
            Type::Number => Ok("Number".to_string()),
            Type::Integer => Ok("Number".to_string()), // Nickel uses Number for all numerics
            Type::Bool => Ok("Bool".to_string()),
            Type::Null => Ok("Null".to_string()),
            Type::Any => {
                tracing::info!("Type::Any in current_type: {:?}", self.current_type_name);
                Ok("Dyn".to_string())
            }

            Type::Array(elem) => {
                let elem_type = self.type_to_nickel_impl(elem, module, indent_level)?;
                Ok(format!("Array {}", elem_type))
            }

            Type::Map { value, .. } => {
                let value_type = self.type_to_nickel_impl(value, module, indent_level)?;
                Ok(format!("{{ _ : {} }}", value_type))
            }

            Type::Optional(inner) => {
                let inner_type = self.type_to_nickel_impl(inner, module, indent_level)?;
                Ok(format!("{} | Null", inner_type))
            }

            Type::Record { fields, open } => {
                if fields.is_empty() && *open {
                    return Ok("{ .. }".to_string());
                }

                let mut result = String::from("{\n");

                // Sort fields for consistent output
                let mut sorted_fields: Vec<_> = fields.iter().collect();
                sorted_fields.sort_by_key(|(name, _)| *name);

                for (i, (name, field)) in sorted_fields.iter().enumerate() {
                    let field_str = self.field_to_nickel(name, field, module, indent_level + 1)?;
                    result.push_str(&field_str);
                    // Add comma except for the last field
                    if i < sorted_fields.len() - 1 {
                        result.push(',');
                    }
                    result.push('\n');
                }

                if *open {
                    result.push_str(&format!("{}.. | Dyn,\n", self.indent(indent_level + 1)));
                }

                result.push_str(&self.indent(indent_level));
                result.push('}');
                Ok(result)
            }

            Type::Union {
                types,
                coercion_hint,
            } => {
                // Handle union types based on coercion hint
                match coercion_hint {
                    Some(amalgam_core::types::UnionCoercion::PreferString) => {
                        // For IntOrString - need to accept both strings and numbers
                        // Check if this is specifically Integer + String union
                        let is_int_or_string = types.len() == 2
                            && types.iter().any(|t| matches!(t, Type::Integer))
                            && types.iter().any(|t| matches!(t, Type::String));

                        if is_int_or_string {
                            // Generate a proper Nickel contract for IntOrString
                            // This contract accepts either a Number or a String
                            Ok("std.contract.from_predicate (fun value => std.is_number value || std.is_string value)".to_string())
                        } else {
                            // Default to String for other string-preferring unions
                            Ok("String".to_string())
                        }
                    }
                    Some(amalgam_core::types::UnionCoercion::PreferNumber) => {
                        // For types that should be coerced to number
                        Ok("Number".to_string())
                    }
                    Some(amalgam_core::types::UnionCoercion::Custom(handler)) => {
                        // Custom handler - could be a Nickel contract
                        Ok(handler.clone())
                    }
                    Some(amalgam_core::types::UnionCoercion::NoPreference) | None => {
                        // Generate actual union type
                        let type_strs: Result<Vec<_>, _> = types
                            .iter()
                            .map(|t| self.type_to_nickel_impl(t, module, indent_level))
                            .collect();
                        Ok(type_strs?.join(" | "))
                    }
                }
            }

            Type::TaggedUnion {
                tag_field,
                variants,
            } => {
                let mut contracts = Vec::new();
                for (tag, variant_type) in variants {
                    let variant_str =
                        self.type_to_nickel_impl(variant_type, module, indent_level)?;
                    contracts.push(format!("({} == \"{}\" && {})", tag_field, tag, variant_str));
                }
                Ok(contracts.join(" | "))
            }

            Type::Reference {
                name,
                module: ref_module,
            } => {
                tracing::debug!(
                    "Processing Type::Reference - name: {}, ref_module: {:?}, current_module: {}, current_type: {:?}",
                    name,
                    ref_module,
                    module.name,
                    self.current_type_name
                );

                // Special debug for problematic cases
                if name.contains("roupVersionForDiscovery") || name.contains("APIGroup") {
                    tracing::error!(
                        "DEBUGGING PROBLEMATIC TYPE: name='{}', ref_module={:?}, current_module='{}'",
                        name, ref_module, module.name
                    );
                }

                // Check if this type was imported at the module level (Phase 2)
                if let Some(&is_same_directory) = self.current_module_imports.get(name) {
                    // Type is already imported, use appropriate casing based on import type
                    if is_same_directory {
                        // Same-directory imports keep PascalCase
                        return Ok(name.clone());
                    } else {
                        // Cross-module imports use camelCase
                        return Ok(to_camel_case(name));
                    }
                }

                // If we have module information, this is a cross-module reference
                if let Some(ref_module) = ref_module {
                    // Parse both module names to extract group and version
                    let (ref_group, ref_version) = Self::parse_module_name(ref_module);
                    let (current_group, current_version) = Self::parse_module_name(&module.name);

                    // Check if this is a cross-module reference
                    if ref_module != &module.name {
                        // Track this as a cross-module import
                        // Use camelCase for the variable name
                        let camelcased_name = to_camel_case(name);

                        // Use the ImportPathCalculator to get the correct path
                        // Pass the original name to preserve case in the filename
                        let import_path = self.import_calculator.calculate(
                            &current_group,
                            &current_version,
                            &ref_group,
                            &ref_version,
                            name, // Use original case for filename
                        );

                        // Track the import for this type - format it as a proper Nickel import statement
                        // Check if this is importing from a mod.ncl file (module with multiple types)
                        let (import_stmt, reference_name) = if import_path.ends_with("/mod.ncl") {
                            // Import the module and extract the specific type
                            let module_alias = format!("{}Module", to_camel_case(&ref_version));
                            let import =
                                format!("let {} = import \"{}\" in", module_alias, import_path);
                            let reference = format!("{}.{}", module_alias, name); // Use original case for type name
                            (import, reference)
                        } else {
                            // Regular import of a single type file
                            let import =
                                format!("let {} = import \"{}\" in", camelcased_name, import_path);
                            (import, camelcased_name.clone())
                        };

                        tracing::debug!(
                            "Adding cross-module import for type '{}': path='{}', stmt='{}'",
                            self.current_type_name.as_deref().unwrap_or(""),
                            import_path,
                            import_stmt
                        );
                        let current_type = self.current_type_name.as_deref().unwrap_or("");
                        self.type_import_map.add_import(current_type, &import_stmt);

                        // Generate the reference
                        // Return the appropriate reference (either module.Type or just the alias)
                        return Ok(reference_name);
                    }
                } else {
                    // Same-package reference - check if it needs an import
                    tracing::debug!(
                        "Checking same-package reference: name='{}', module='{}', type_exists={}, current_type='{}'",
                        name,
                        module.name,
                        self.registry.find_module_for_type(name).is_some(),
                        self.current_type_name.as_deref().unwrap_or("unknown")
                    );
                    if let Some(module_info) = self.registry.find_module_for_type(name) {
                        let (current_group, current_version) =
                            Self::parse_module_name(&module.name);

                        tracing::debug!(
                            "Type found: name='{}', module_info.name='{}', module_info.group='{}', module_info.version='{}', current_group='{}', current_version='{}', different_module={}",
                            name,
                            module_info.name,
                            module_info.group,
                            module_info.version,
                            current_group,
                            current_version,
                            module_info.name != module.name
                        );

                        // If it's same package, same version, but different module - need import
                        if module_info.group == current_group
                            && module_info.version == current_version
                            && module_info.name != module.name
                        {
                            // Generate import statement for same-package reference
                            // Use camelCase for the variable name but proper case for the filename
                            // Use camelCase for the variable name
                            let camelcased_name = to_camel_case(name);
                            let import_path = format!("./{}.ncl", name); // Use original case for filename
                            let import_stmt =
                                format!("let {} = import \"{}\" in", camelcased_name, import_path);

                            tracing::debug!(
                                "Adding same-package import for type '{}': path='{}', stmt='{}'",
                                self.current_type_name.as_deref().unwrap_or(""),
                                import_path,
                                import_stmt
                            );

                            self.type_import_map.add_import(
                                self.current_type_name.as_deref().unwrap_or(""),
                                &import_stmt,
                            );

                            // Use the camelCase alias that matches the import
                            let result = camelcased_name.clone();
                            return Ok(result);
                        }
                        // If it's same package but different version, use imported alias
                        else if module_info.group == current_group
                            && module_info.version != current_version
                        {
                            // Use consistent camelCase alias generation
                            let import_alias =
                                to_camel_case(&format!("{}_{}", module_info.version, name));
                            let result = format!("{}.{}", import_alias, name);
                            return Ok(result);
                        }
                    } else {
                        // Symbol not found in table - check if this is an external reference
                        // that needs special handling (e.g., k8s.io/api/core/v1.EnvVar)
                        // Strip array prefix if present (e.g., "[]k8s.io/api/core/v1.EnvVar" -> "k8s.io/api/core/v1.EnvVar")
                        let clean_name = name.strip_prefix("[]").unwrap_or(name);

                        // Check if this is a same-module FQN (e.g., "io.k8s.api.coordination.v1alpha2.LeaseCandidateSpec")
                        // that should be treated as a local type
                        if clean_name.starts_with("io.k8s.") {
                            // Extract the simple type name from the FQN
                            let extracted_type_name =
                                clean_name.split('.').next_back().unwrap_or(clean_name);

                            // Check if this type exists in the current module (case-sensitive match)
                            if let Some(local_type) =
                                module.types.iter().find(|t| t.name == extracted_type_name)
                            {
                                tracing::debug!(
                                    "Detected same-module FQN '{}' -> local type '{}' in module '{}'",
                                    clean_name, local_type.name, module.name
                                );
                                // Return the exact type name as defined in the module (preserving case)
                                return Ok(local_type.name.clone());
                            }
                        }

                        if clean_name.contains('/')
                            || clean_name.starts_with("io.k8s.")
                            || clean_name.starts_with("k8s.io")
                        {
                            // This is an external k8s reference that needs proper parsing
                            // Parse it to get the actual type name and module
                            // Parse the external reference to extract group, version, and kind
                            let (ext_group, ext_version, ext_kind) = if clean_name
                                .starts_with("k8s.io/api/core/")
                            {
                                // Format: k8s.io/api/core/v1.EnvVar
                                if let Some(rest) = clean_name.strip_prefix("k8s.io/api/core/") {
                                    let parts: Vec<&str> = rest.split('.').collect();
                                    if parts.len() == 2 {
                                        (
                                            "k8s.io".to_string(),
                                            parts[0].to_string(),
                                            parts[1].to_string(),
                                        )
                                    } else {
                                        // Can't parse, skip
                                        return Ok(clean_name.to_string());
                                    }
                                } else {
                                    return Ok(clean_name.to_string());
                                }
                            } else if clean_name.starts_with("k8s.io/apimachinery/pkg/apis/meta/") {
                                // Format: k8s.io/apimachinery/pkg/apis/meta/v1.ObjectMeta
                                if let Some(rest) =
                                    clean_name.strip_prefix("k8s.io/apimachinery/pkg/apis/meta/")
                                {
                                    let parts: Vec<&str> = rest.split('.').collect();
                                    if parts.len() == 2 {
                                        (
                                            "k8s.io".to_string(),
                                            parts[0].to_string(),
                                            parts[1].to_string(),
                                        )
                                    } else {
                                        return Ok(clean_name.to_string());
                                    }
                                } else {
                                    return Ok(clean_name.to_string());
                                }
                            } else if clean_name.starts_with("io.k8s.api.core.") {
                                // Format: io.k8s.api.core.v1.EnvVar
                                let parts: Vec<&str> = clean_name.split('.').collect();
                                if parts.len() >= 6 {
                                    let version = parts[parts.len() - 2].to_string();
                                    let kind = parts[parts.len() - 1].to_string();
                                    ("k8s.io".to_string(), version, kind)
                                } else {
                                    return Ok(clean_name.to_string());
                                }
                            } else if clean_name.starts_with("io.k8s.api.discovery.") {
                                // Format: io.k8s.api.discovery.v1.EndpointConditions
                                let parts: Vec<&str> = clean_name.split('.').collect();
                                if parts.len() >= 6 {
                                    let version = parts[parts.len() - 2].to_string();
                                    let kind = parts[parts.len() - 1].to_string();
                                    ("k8s.io".to_string(), version, kind)
                                } else {
                                    return Ok(clean_name.to_string());
                                }
                            } else if clean_name.starts_with("io.k8s.apimachinery.pkg.apis.meta.") {
                                // Format: io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta
                                let parts: Vec<&str> = clean_name.split('.').collect();
                                if parts.len() >= 8 {
                                    let version = parts[parts.len() - 2].to_string();
                                    let kind = parts[parts.len() - 1].to_string();
                                    ("k8s.io".to_string(), version, kind)
                                } else {
                                    return Ok(clean_name.to_string());
                                }
                            } else if clean_name.starts_with("io.k8s.apimachinery.pkg.runtime.") {
                                // Format: io.k8s.apimachinery.pkg.runtime.RawExtension
                                // Note: runtime types don't have version in their path
                                let parts: Vec<&str> = clean_name.split('.').collect();
                                if parts.len() >= 6 {
                                    let kind = parts[parts.len() - 1].to_string();
                                    // Runtime types are typically unversioned or use 'v1'
                                    ("k8s.io".to_string(), "v1".to_string(), kind)
                                } else {
                                    return Ok(clean_name.to_string());
                                }
                            } else {
                                return Ok(clean_name.to_string());
                            };

                            // Use the ImportPathCalculator to get the correct path
                            let (current_group, current_version) =
                                Self::parse_module_name(&module.name);
                            let import_path = self.import_calculator.calculate(
                                &current_group,
                                &current_version,
                                &ext_group,
                                &ext_version,
                                &ext_kind,
                            );

                            // Use camelCase for variable name
                            let camelcased_name = to_camel_case(&ext_kind);

                            // Check if this is importing from a mod.ncl file (module with multiple types)
                            let (import_stmt, reference_name) = if import_path.ends_with("/mod.ncl")
                            {
                                // Import the module and extract the specific type
                                let module_alias = format!("{}Module", to_camel_case(&ext_version));
                                let import =
                                    format!("let {} = import \"{}\" in", module_alias, import_path);
                                let reference = format!("{}.{}", module_alias, ext_kind); // Use original case for type name
                                (import, reference)
                            } else {
                                // Regular import of a single type file
                                let import = format!(
                                    "let {} = import \"{}\" in",
                                    camelcased_name, import_path
                                );
                                (import, camelcased_name.clone())
                            };

                            tracing::debug!(
                                "External reference '{}' parsed to group='{}', version='{}', kind='{}', generating cross-package import",
                                clean_name, ext_group, ext_version, ext_kind
                            );

                            self.type_import_map.add_import(
                                self.current_type_name.as_deref().unwrap_or(""),
                                &import_stmt,
                            );

                            // Return the appropriate reference
                            return Ok(reference_name);
                        }

                        // Only generate same-package import for simple type names
                        // that don't contain path separators or package prefixes
                        if !name.contains('/') && !name.contains('.') {
                            let (_current_group, _current_version) =
                                Self::parse_module_name(&module.name);

                            // For same-package references, assume they exist and generate import
                            // This handles cases where the symbol table might be incomplete
                            // Use camelCase for variable name
                            let camelcased_name = to_camel_case(name);
                            let import_path = format!("./{}.ncl", name); // Use original case for filename
                            let import_stmt =
                                format!("let {} = import \"{}\" in", camelcased_name, import_path);

                            tracing::debug!(
                                "Symbol '{}' not in table, generating speculative import for same-package reference",
                                name
                            );

                            let current_type = self.current_type_name.as_deref().unwrap_or("");
                            self.type_import_map.add_import(current_type, &import_stmt);

                            // For same-package imports, return the camelCase variable name
                            return Ok(camelcased_name);
                        } else {
                            // This is a complex name that we don't know how to handle
                            // Just return it as-is and hope for the best
                            return Ok(name.to_string());
                        }
                    }
                }

                // For local same-module references, check if the type exists in the current module first
                // This should preserve the original case for local types
                for type_def in &module.types {
                    if type_def.name == *name {
                        tracing::debug!(
                            "Found local type '{}' in module '{}', using original case",
                            name,
                            module.name
                        );
                        return Ok(name.to_string());
                    }
                }

                // Use the resolver for other references
                let context = ResolutionContext {
                    current_group: None,
                    current_version: None,
                    current_kind: None,
                };
                tracing::debug!(
                    "Using resolver for type '{}' in module '{}'",
                    name,
                    module.name
                );
                Ok(self.resolver.resolve(name, module, &context))
            }

            Type::Contract { base, predicate } => {
                let base_type = self.type_to_nickel_impl(base, module, indent_level)?;
                Ok(format!("{} | Contract({})", base_type, predicate))
            }
        }
    }

    fn field_to_nickel(
        &mut self,
        name: &str,
        field: &Field,
        module: &amalgam_core::ir::Module,
        indent_level: usize,
    ) -> Result<String, CodegenError> {
        let indent = self.indent(indent_level);
        let type_str = self.type_to_nickel(&field.ty, module, indent_level)?;

        // Start with field name - escape reserved keywords and fields starting with $
        let field_name = self.escape_field_name(name);
        let mut result = format!("{}{}", indent, field_name);

        // 1. Type annotation
        result.push_str(&format!("\n{}{} | {}", indent, " ".repeat(2), type_str));

        // 2. Documentation (with proper multiline handling)
        if let Some(desc) = &field.description {
            result.push_str(&format!(
                "\n{}{} | doc {}",
                indent,
                " ".repeat(2),
                self.format_doc(desc)
            ));
        }

        // 3. Required/Optional marker
        // In Nickel, a field with a default value is implicitly optional
        // For required fields, don't add 'optional' marker
        // For optional fields without defaults, add 'optional' marker
        if !field.required && field.default.is_none() {
            result.push_str(&format!("\n{}{} | optional", indent, " ".repeat(2)));
        }

        // 4. Default value (comes last in the type pipeline)
        if let Some(default) = &field.default {
            let default_str = format_json_value_impl(default, indent_level, self);
            result.push_str(&format!("\n{}{} = {}", indent, " ".repeat(2), default_str));
        }

        Ok(result)
    }
}

/// Format a JSON value for Nickel with proper field name escaping
fn format_json_value_impl(
    value: &serde_json::Value,
    indent_level: usize,
    codegen: &NickelCodegen,
) -> String {
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
        serde_json::Value::Array(arr) => {
            let items: Vec<String> = arr
                .iter()
                .map(|v| format_json_value_impl(v, indent_level, codegen))
                .collect();
            format!("[{}]", items.join(", "))
        }
        serde_json::Value::Object(obj) => {
            if obj.is_empty() {
                "{}".to_string()
            } else {
                let indent = " ".repeat((indent_level + 1) * 2);
                let mut items = Vec::new();
                for (k, v) in obj {
                    let escaped_key = codegen.escape_field_name(k);
                    items.push(format!(
                        "{}{} = {}",
                        indent,
                        escaped_key,
                        format_json_value_impl(v, indent_level + 1, codegen)
                    ));
                }
                format!(
                    "{{\n{}\n{}}}",
                    items.join(",\n"),
                    " ".repeat(indent_level * 2)
                )
            }
        }
    }
}

impl Default for NickelCodegen {
    fn default() -> Self {
        Self::new(Arc::new(ModuleRegistry::new()))
    }
}

impl Codegen for NickelCodegen {
    #[instrument(skip(self, ir), level = "info")]
    fn generate(&mut self, ir: &IR) -> Result<String, CodegenError> {
        let mut output = String::new();

        for module in &ir.modules {
            // Clear imports for this module
            self.current_imports.clear();
            self.same_package_deps.clear();

            // Debug: Check if this module contains TopologySpreadConstraint
            let has_topology = module
                .types
                .iter()
                .any(|t| t.name == "TopologySpreadConstraint");
            if has_topology {
                debug!(
                    "Processing TopologySpreadConstraint module: {}",
                    module.name
                );
                for type_def in &module.types {
                    debug!("Type in module: {} -> {:?}", type_def.name, type_def.ty);
                }
            }

            // Phase 2: Analyze dependencies by processing all types
            // This populates same_package_deps with types that need imports
            let mut type_strings = Vec::new();
            for type_def in &module.types {
                let type_str = self.type_to_nickel_impl(&type_def.ty, module, 1)?;
                type_strings.push((type_def.clone(), type_str));
            }

            // Check if this is a single-type module first to decide on header
            let is_single_type = module.types.len() == 1 && module.constants.is_empty();

            // Module header comment (skip for single-type modules that export directly)
            if !is_single_type {
                // Normalize module name for display
                let (group, version) = Self::parse_module_name(&module.name);
                let display_name = format!("{}.{}", group, version);
                writeln!(output, "# Module: {}", display_name)
                    .map_err(|e| CodegenError::Generation(e.to_string()))?;
                writeln!(output).map_err(|e| CodegenError::Generation(e.to_string()))?;
            }

            // Phase 3: Generate imports for same-package dependencies
            if !self.same_package_deps.is_empty() {
                let (current_group, current_version) = Self::parse_module_name(&module.name);

                let mut same_pkg_imports: Vec<_> = self.same_package_deps.iter().collect();
                same_pkg_imports.sort();

                for type_name in same_pkg_imports {
                    if let Some(module_info) = self.registry.find_module_for_type(type_name) {
                        // Generate appropriate alias and path based on whether it's same or different version
                        let (import_alias, path) = if module_info.version == current_version {
                            // Same version, different module - use camelCase for variable name
                            let alias = to_camel_case(type_name);
                            let path = self.import_calculator.calculate(
                                &current_group,
                                &current_version,
                                &module_info.group,
                                &module_info.version,
                                type_name, // Use original case for filename
                            );
                            (alias, path)
                        } else {
                            // Different version - use camelCase with version prefix
                            let alias =
                                to_camel_case(&format!("{}_{}", module_info.version, type_name));
                            let path = self.import_calculator.calculate(
                                &current_group,
                                &current_version,
                                &module_info.group,
                                &module_info.version,
                                type_name, // Use original case for filename
                            );
                            (alias, path)
                        };

                        let import_stmt = format!("let {} = import \"{}\" in", import_alias, path);
                        writeln!(output, "{}", import_stmt)
                            .map_err(|e| CodegenError::Generation(e.to_string()))?;

                        // Record in debug info
                        self.debug_info
                            .imports_generated
                            .push((module.name.clone(), import_stmt));
                    }
                }
                writeln!(output).map_err(|e| CodegenError::Generation(e.to_string()))?;
            }

            // Generate cross-module imports that were discovered
            if !self.current_imports.is_empty() {
                let mut imports: Vec<_> = self.current_imports.iter().collect();
                imports.sort_by_key(|(ver, name)| (ver.clone(), name.clone()));

                // Parse group and version from module name
                // Module names can be:
                // - "group.version" (e.g., "k8s.io.v1")
                // - "Kind.version.group" (e.g., "Composition.v1.apiextensions.crossplane.io")
                let (from_group, from_version) = Self::parse_module_name(&module.name);

                for (version, type_name) in imports {
                    // Use camelCase for variable names with version prefix
                    let import_alias = to_camel_case(&format!("{}_{}", version, type_name));

                    // Use unified calculator for cross-module imports within same package
                    let path = self.import_calculator.calculate(
                        &from_group,
                        &from_version,
                        &from_group, // Same group, different version
                        version,
                        type_name,
                    );

                    writeln!(output, "let {} = import \"{}\" in", import_alias, path)
                        .map_err(|e| CodegenError::Generation(e.to_string()))?;
                }
                writeln!(output).map_err(|e| CodegenError::Generation(e.to_string()))?;
            }

            // Generate original imports if any
            if !module.imports.is_empty() {
                for import in &module.imports {
                    // Convert import path based on package mode
                    let import_path = self.package_mode.convert_import(&import.path);

                    // Generate import statement
                    // If the path is a package name (no slashes), use package import syntax
                    let import_statement =
                        if !import_path.contains('/') && import_path.starts_with('"') {
                            // Package import: import "package_name"
                            format!(
                                "let {} = import {} in",
                                import
                                    .alias
                                    .as_ref()
                                    .unwrap_or(&import.path.replace('/', "_")),
                                import_path
                            )
                        } else {
                            // Regular file import
                            format!(
                                "let {} = import \"{}\" in",
                                import
                                    .alias
                                    .as_ref()
                                    .unwrap_or(&import.path.replace('/', "_")),
                                import_path
                            )
                        };

                    writeln!(output, "{}", import_statement)
                        .map_err(|e| CodegenError::Generation(e.to_string()))?;
                }
                writeln!(output).map_err(|e| CodegenError::Generation(e.to_string()))?;
            }

            if is_single_type {
                // Single type - export directly without wrapping in a record
                let type_def = &module.types[0];

                // Add type documentation as a comment if present
                if let Some(doc) = &type_def.documentation {
                    for line in doc.lines() {
                        writeln!(output, "# {}", line)
                            .map_err(|e| CodegenError::Generation(e.to_string()))?;
                    }
                }

                // Generate just the type definition, no record wrapper
                let type_str = self.type_to_nickel(&type_def.ty, module, 0)?;
                writeln!(output, "{}", type_str)?;
            } else {
                // Multiple types or has constants - use record structure
                writeln!(output, "{{")?;

                for (idx, type_def) in module.types.iter().enumerate() {
                    // Generate the type string
                    let type_str = self.type_to_nickel(&type_def.ty, module, 1)?;
                    // Add type documentation as a comment if present
                    if let Some(doc) = &type_def.documentation {
                        for line in doc.lines() {
                            writeln!(output, "{}# {}", self.indent(1), line)
                                .map_err(|e| CodegenError::Generation(e.to_string()))?;
                        }
                    }

                    // Check if type is a record that needs special formatting
                    // Write the type definition with proper formatting
                    // Add comma if not the last item (considering constants might follow)
                    let is_last_item = idx == module.types.len() - 1 && module.constants.is_empty();
                    if !is_last_item {
                        writeln!(output, "  {} = {},", type_def.name, type_str)?;
                    } else {
                        writeln!(output, "  {} = {}", type_def.name, type_str)?;
                    }

                    // Add spacing between types for readability
                    if idx < module.types.len() - 1 {
                        writeln!(output)?;
                    }
                }

                // Generate constants with proper formatting
                if !module.constants.is_empty() {
                    writeln!(output)?; // Add spacing before constants

                    for (idx, constant) in module.constants.iter().enumerate() {
                        if let Some(doc) = &constant.documentation {
                            writeln!(output, "  # {}", doc)
                                .map_err(|e| CodegenError::Generation(e.to_string()))?;
                        }

                        let value_str = format_json_value_impl(&constant.value, 1, self);
                        // Only add comma if not the last constant
                        if idx < module.constants.len() - 1 {
                            writeln!(output, "  {} = {},", constant.name, value_str)
                                .map_err(|e| CodegenError::Generation(e.to_string()))?;
                        } else {
                            writeln!(output, "  {} = {}", constant.name, value_str)
                                .map_err(|e| CodegenError::Generation(e.to_string()))?;
                        }
                    }
                }

                writeln!(output, "}}")?;
            }
        }

        // Sync pipeline debug to compilation debug before returning
        self.sync_debug_to_compilation();

        Ok(output)
    }
}

impl NickelCodegen {
    /// Generate a unique module alias from an import path using pattern matching
    fn generate_module_alias(path: &str) -> String {
        // Extract meaningful parts from the path
        if let Some(alias) = Self::extract_alias_from_path(path) {
            return alias;
        }

        // Fallback: generate from path segments
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() >= 2 {
            let last_two = format!(
                "{}{}",
                parts[parts.len() - 2].replace('.', "_"),
                parts[parts.len() - 1]
                    .replace(".ncl", "")
                    .replace("mod", "")
            );
            to_camel_case(&last_two)
        } else {
            "importedModule".to_string()
        }
    }

    /// Extract a meaningful alias from known path patterns
    fn extract_alias_from_path(path: &str) -> Option<String> {
        // Match patterns like "apimachinery.pkg.apis/meta/v1"
        if path.contains("apimachinery.pkg.apis/meta/") {
            if let Some(version) = path.split("meta/").nth(1) {
                let version = version
                    .trim_end_matches("/mod.ncl")
                    .trim_end_matches(".ncl");
                return Some(format!("meta{}", version));
            }
        }

        // Match patterns like "../core/v1" or "api/core/v1"
        if path.contains("/core/") {
            if let Some(version) = path.split("/core/").nth(1) {
                let version = version
                    .trim_end_matches("/mod.ncl")
                    .trim_end_matches(".ncl");
                return Some(format!("core{}", version));
            }
        }

        // Match patterns like "api/apps/v1", "api/batch/v1", etc.
        if path.contains("api/") {
            if let Some(api_part) = path.split("api/").nth(1) {
                let parts: Vec<&str> = api_part.split('/').collect();
                if parts.len() >= 2 {
                    let group = parts[0].replace('.', "_");
                    let version = parts[1]
                        .trim_end_matches("/mod.ncl")
                        .trim_end_matches(".ncl");
                    return Some(format!("{}{}", group, version));
                }
            }
        }

        // Match v0 module
        if path.contains("v0/mod.ncl") || path.contains("v0.ncl") {
            return Some("v0Module".to_string());
        }

        None
    }

    /// Generate code with per-type import tracking
    /// Returns both the generated code and a map of which imports each type needs
    pub fn generate_with_import_tracking(
        &mut self,
        ir: &IR,
    ) -> Result<(String, TypeImportMap), CodegenError> {
        // Clear the type import map for this generation
        self.type_import_map = TypeImportMap::new();

        let mut output = String::new();

        for module in &ir.modules {
            // Clear imports for this module
            self.current_imports.clear();
            self.same_package_deps.clear();

            // First pass: collect ALL dependencies for ALL types in this module
            // This allows us to consolidate imports by module path
            let mut module_deps_by_path: HashMap<String, HashSet<String>> = HashMap::new();
            // Track which types need which dependencies
            let mut type_dependencies: HashMap<String, HashSet<String>> = HashMap::new();

            // Process each type and collect its dependencies
            for type_def in &module.types {
                // Set current type being processed
                self.current_type_name = Some(type_def.name.clone());

                // Start dependency analysis for this type
                let _analysis = self
                    .pipeline_debug
                    .start_dependency_analysis(&type_def.name, &module.name);

                // Clear per-type tracking
                let mut type_specific_deps: HashSet<String> = HashSet::new();

                // Analyze this type's dependencies
                self.analyze_type_dependencies_with_debug(
                    &type_def.ty,
                    module,
                    &mut type_specific_deps,
                    &type_def.name,
                    "",
                );

                if !type_specific_deps.is_empty() {
                    tracing::debug!(
                        "Type {} has {} dependencies: {:?}",
                        type_def.name,
                        type_specific_deps.len(),
                        type_specific_deps
                    );
                }

                // Collect dependencies by module path for consolidation
                // Also track which types need which imports for the TypeImportMap
                if !type_specific_deps.is_empty() {
                    // Track that this type has these dependencies
                    type_dependencies.insert(type_def.name.clone(), type_specific_deps.clone());

                    let (current_group, current_version) = Self::parse_module_name(&module.name);

                    for dep_type_name in &type_specific_deps {
                        if let Some(module_info) = self.registry.find_module_for_type(dep_type_name)
                        {
                            let path = self.import_calculator.calculate(
                                &current_group,
                                &current_version,
                                &module_info.group,
                                &module_info.version,
                                dep_type_name,
                            );

                            // Track types by their module path for consolidation
                            module_deps_by_path
                                .entry(path.clone())
                                .or_default()
                                .insert(dep_type_name.clone());
                        }
                    }
                }
            }

            // Clear current type
            self.current_type_name = None;

            // Now generate the module code as before
            // ALWAYS output module markers for extraction to work
            // Normalize module name for display
            let (group, version) = Self::parse_module_name(&module.name);
            let display_name = format!("{}.{}", group, version);
            writeln!(output, "# Module: {}", display_name)
                .map_err(|e| CodegenError::Generation(e.to_string()))?;
            writeln!(output).map_err(|e| CodegenError::Generation(e.to_string()))?;

            // Generate consolidated imports for this module
            let mut consolidated_imports = Vec::new();
            // Track generated imports by dependency name for type import map
            let mut dependency_imports: HashMap<String, String> = HashMap::new();

            // Debug: log the module_deps_by_path to understand what we're working with

            for (path, type_names) in &module_deps_by_path {
                // Check if this is a k8s.io consolidated module path
                let is_consolidated = path.contains(".ncl")
                    && (path.contains("apimachinery.pkg.apis")
                        || path.contains("api/core/")
                        || path.contains("api/")
                        || path.contains("kube-aggregator.pkg.apis")
                        || path.contains("apiextensions-apiserver.pkg.apis"))
                    && !type_names
                        .iter()
                        .any(|name| path.contains(&format!("/{}.ncl", name)));

                if is_consolidated {
                    // Generate a single module import with multiple type extractions
                    // Generate a unique module alias based on the path pattern
                    let module_alias = Self::generate_module_alias(path);

                    // Import the module once
                    consolidated_imports
                        .push(format!("let {} = import \"{}\" in", module_alias, path));

                    // Extract each type from the module (ALL need 'in' because there might be more imports after this)
                    for type_name in type_names {
                        let sanitized_var = sanitize_import_variable_name(type_name);
                        // ALL extractions need 'in' because there might be more imports/extractions
                        consolidated_imports.push(format!(
                            "let {} = {}.{} in",
                            sanitized_var, module_alias, type_name
                        ));
                        // Track the import for this dependency
                        let import_stmt =
                            format!("let {} = {}.{} in", sanitized_var, module_alias, type_name);
                        dependency_imports.insert(type_name.clone(), import_stmt);
                    }
                } else {
                    // Regular imports for individual type files
                    for type_name in type_names {
                        let sanitized_var = sanitize_import_variable_name(type_name);
                        let import_stmt = format!("let {} = import \"{}\" in", sanitized_var, path);
                        consolidated_imports.push(import_stmt.clone());
                        dependency_imports.insert(type_name.clone(), import_stmt);
                    }
                }
            }

            // Now populate the type import map based on which types need which dependencies
            for (type_name, deps) in &type_dependencies {
                let mut import_statements = Vec::new();
                for dep_name in deps {
                    if let Some(import_stmt) = dependency_imports.get(dep_name) {
                        self.type_import_map.add_import(type_name, import_stmt);

                        // Create ImportStatement for debugging
                        use crate::import_pipeline_debug::ImportStatement;

                        // Extract path from import statement (format: "let name = import \"path\" in")
                        let path = if let Some(start) = import_stmt.find("\"") {
                            if let Some(end) = import_stmt.rfind("\"") {
                                import_stmt[start + 1..end].to_string()
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        };

                        import_statements.push(ImportStatement {
                            dependency: dep_name.clone(),
                            statement: import_stmt.clone(),
                            path,
                        });
                    }
                }

                // Record import generation for this type (for debugging)
                if !import_statements.is_empty() {
                    use crate::import_pipeline_debug::ImportGeneration;
                    self.pipeline_debug.record_import_generation(
                        type_name,
                        ImportGeneration {
                            type_name: type_name.clone(),
                            dependencies: deps.iter().cloned().collect(),
                            import_statements,
                            path_calculations: vec![],
                        },
                    );
                }
            }

            // Write consolidated imports
            for import in &consolidated_imports {
                writeln!(output, "{}", import)
                    .map_err(|e| CodegenError::Generation(e.to_string()))?;
            }
            if !consolidated_imports.is_empty() {
                writeln!(output).map_err(|e| CodegenError::Generation(e.to_string()))?;
            }

            let is_single_type = module.types.len() == 1 && module.constants.is_empty();

            // Generate module-level imports (for backward compatibility)
            // ... (rest of the generation logic remains the same)

            if is_single_type {
                let type_def = &module.types[0];
                // Set current type for import tracking
                self.current_type_name = Some(type_def.name.clone());
                if let Some(doc) = &type_def.documentation {
                    for line in doc.lines() {
                        writeln!(output, "# {}", line)
                            .map_err(|e| CodegenError::Generation(e.to_string()))?;
                    }
                }
                let type_str = self.type_to_nickel(&type_def.ty, module, 0)?;
                writeln!(output, "{}", type_str)?;
                self.current_type_name = None;
            } else {
                writeln!(output, "{{")?;
                for (idx, type_def) in module.types.iter().enumerate() {
                    // Set current type for import tracking
                    self.current_type_name = Some(type_def.name.clone());
                    let type_str = self.type_to_nickel(&type_def.ty, module, 1)?;
                    self.current_type_name = None;
                    if let Some(doc) = &type_def.documentation {
                        for line in doc.lines() {
                            writeln!(output, "{}# {}", self.indent(1), line)
                                .map_err(|e| CodegenError::Generation(e.to_string()))?;
                        }
                    }
                    // Write the type definition with proper formatting
                    // Add comma if not the last item (considering constants might follow)
                    let is_last_item = idx == module.types.len() - 1 && module.constants.is_empty();
                    if !is_last_item {
                        writeln!(output, "  {} = {},", type_def.name, type_str)?;
                        // Add newline after comma for better readability
                        writeln!(output)?;
                    } else {
                        writeln!(output, "  {} = {}", type_def.name, type_str)?;
                    }
                    if idx < module.types.len() - 1 && !is_last_item {
                        // Add another newline between types (double spacing)
                        writeln!(output)?;
                    }
                }
                if !module.constants.is_empty() {
                    writeln!(output)?;
                    for (idx, constant) in module.constants.iter().enumerate() {
                        if let Some(doc) = &constant.documentation {
                            writeln!(output, "  # {}", doc)
                                .map_err(|e| CodegenError::Generation(e.to_string()))?;
                        }
                        let value_str = format_json_value_impl(&constant.value, 1, self);
                        // Only add comma if not the last constant
                        if idx < module.constants.len() - 1 {
                            writeln!(output, "  {} = {},", constant.name, value_str)
                                .map_err(|e| CodegenError::Generation(e.to_string()))?;
                            // Add newline after comma for better readability
                            writeln!(output)?;
                        } else {
                            writeln!(output, "  {} = {}", constant.name, value_str)
                                .map_err(|e| CodegenError::Generation(e.to_string()))?;
                        }
                    }
                }
                writeln!(output, "}}")?;
            }
        }

        // Finalize the pipeline debug summary
        self.pipeline_debug.finalize_summary();

        Ok((output, self.type_import_map.clone()))
    }

    /// Analyze dependencies for a specific type with debug tracking
    fn analyze_type_dependencies_with_debug(
        &mut self,
        ty: &Type,
        module: &amalgam_core::ir::Module,
        deps: &mut HashSet<String>,
        current_type: &str,
        context: &str,
    ) {
        match ty {
            Type::Reference {
                name,
                module: ref_module,
            } => {
                // Record the reference
                if let Some(analysis) = self
                    .pipeline_debug
                    .dependency_analysis
                    .get_mut(current_type)
                {
                    analysis.references_found.push(TypeReference {
                        name: name.clone(),
                        context: context.to_string(),
                        has_module: ref_module.is_some(),
                        module: ref_module.clone(),
                    });
                }

                // Check if this is a reference to another type
                if ref_module.is_none() {
                    // Same-package reference - check if it's in the registry
                    if let Some(module_info) = self.registry.find_module_for_type(name) {
                        let (current_group, current_version) =
                            Self::parse_module_name(&module.name);

                        // Check if it's in the same group/version
                        // With the unified module approach (one module per version),
                        // all types are in the same module but in different files
                        // So we need imports for any reference to another type
                        if module_info.group == current_group
                            && module_info.version == current_version
                        {
                            // Check if it's NOT a self-reference
                            if let Some(current_type_name) = &self.current_type_name {
                                if name != current_type_name {
                                    // Different type, needs import even though same module
                                    deps.insert(name.clone());
                                    if let Some(analysis) = self
                                        .pipeline_debug
                                        .dependency_analysis
                                        .get_mut(current_type)
                                    {
                                        analysis.dependencies_identified.insert(name.clone());
                                    }
                                }
                            } else {
                                // Same module - only add if not self-reference
                                if let Some(current_type_name) = &self.current_type_name {
                                    if name != current_type_name {
                                        deps.insert(name.clone());
                                        if let Some(analysis) = self
                                            .pipeline_debug
                                            .dependency_analysis
                                            .get_mut(current_type)
                                        {
                                            analysis.dependencies_identified.insert(name.clone());
                                        }
                                    } else if let Some(analysis) = self
                                        .pipeline_debug
                                        .dependency_analysis
                                        .get_mut(current_type)
                                    {
                                        analysis.self_references_filtered.push(name.clone());
                                    }
                                } else {
                                    deps.insert(name.clone());
                                    if let Some(analysis) = self
                                        .pipeline_debug
                                        .dependency_analysis
                                        .get_mut(current_type)
                                    {
                                        analysis.dependencies_identified.insert(name.clone());
                                    }
                                }
                            }
                        }
                    } else {
                        // Reference not found in symbol table
                        if let Some(analysis) = self
                            .pipeline_debug
                            .dependency_analysis
                            .get_mut(current_type)
                        {
                            analysis.unresolved_references.push(name.clone());
                        }
                    }
                }
            }
            Type::Array(inner) => self.analyze_type_dependencies_with_debug(
                inner,
                module,
                deps,
                current_type,
                &format!("{}[array]", context),
            ),
            Type::Optional(inner) => self.analyze_type_dependencies_with_debug(
                inner,
                module,
                deps,
                current_type,
                &format!("{}[optional]", context),
            ),
            Type::Map { value, .. } => self.analyze_type_dependencies_with_debug(
                value,
                module,
                deps,
                current_type,
                &format!("{}[map-value]", context),
            ),
            Type::Record { fields, .. } => {
                for (field_name, field) in fields {
                    self.analyze_type_dependencies_with_debug(
                        &field.ty,
                        module,
                        deps,
                        current_type,
                        &format!("{}field:{}", context, field_name),
                    );
                }
            }
            Type::Union { types, .. } => {
                for (i, union_ty) in types.iter().enumerate() {
                    self.analyze_type_dependencies_with_debug(
                        union_ty,
                        module,
                        deps,
                        current_type,
                        &format!("{}[union-variant-{}]", context, i),
                    );
                }
            }
            _ => {}
        }
    }
}

/// Sanitize a string to be a valid Nickel variable name
/// Converts special characters to underscores and converts to camelCase
fn sanitize_import_variable_name(name: &str) -> String {
    // First clean up special characters
    let cleaned = name.replace(['-', '.', '/', ':', '\\'], "_");

    // Then convert to camelCase (lowercase first letter, keep rest as-is)
    to_camel_case(&cleaned)
}

#[cfg(test)]
mod tests {
    use super::*;
    use amalgam_core::ir::{Metadata, Module};
    use std::collections::BTreeMap;

    fn create_test_module() -> Module {
        Module {
            name: "test".to_string(),
            imports: Vec::new(),
            types: Vec::new(),
            constants: Vec::new(),
            metadata: Metadata {
                source_language: None,
                source_file: None,
                version: None,
                generated_at: None,
                custom: BTreeMap::new(),
            },
        }
    }

    #[test]
    fn test_simple_type_generation() {
        let mut codegen = NickelCodegen::new_for_test();
        let module = create_test_module();

        assert_eq!(
            codegen.type_to_nickel(&Type::String, &module, 0).unwrap(),
            "String"
        );
        assert_eq!(
            codegen.type_to_nickel(&Type::Number, &module, 0).unwrap(),
            "Number"
        );
        assert_eq!(
            codegen.type_to_nickel(&Type::Bool, &module, 0).unwrap(),
            "Bool"
        );
        assert_eq!(
            codegen.type_to_nickel(&Type::Any, &module, 0).unwrap(),
            "Dyn"
        );
    }

    #[test]
    fn test_array_generation() {
        let mut codegen = NickelCodegen::new_for_test();
        let module = create_test_module();
        let array_type = Type::Array(Box::new(Type::String));
        assert_eq!(
            codegen.type_to_nickel(&array_type, &module, 0).unwrap(),
            "Array String"
        );
    }

    #[test]
    fn test_optional_generation() {
        let mut codegen = NickelCodegen::new_for_test();
        let module = create_test_module();
        let optional_type = Type::Optional(Box::new(Type::String));
        assert_eq!(
            codegen.type_to_nickel(&optional_type, &module, 0).unwrap(),
            "String | Null"
        );
    }

    #[test]
    fn test_doc_formatting() {
        let codegen = NickelCodegen::new_for_test();

        // Short doc uses regular quotes
        assert_eq!(codegen.format_doc("Short doc"), "\"Short doc\"");

        // Multiline doc uses triple quotes
        let multiline = "This is a\nmultiline doc";
        assert_eq!(
            codegen.format_doc(multiline),
            "m%\"\nThis is a\nmultiline doc\n\"%"
        );

        // Escapes quotes in short docs
        assert_eq!(
            codegen.format_doc("Doc with \"quotes\""),
            "\"Doc with \\\"quotes\\\"\""
        );
    }
}
