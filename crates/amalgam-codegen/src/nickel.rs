//! Nickel code generator with improved formatting

use crate::import_pipeline_debug::{
    ImportGeneration, ImportPipelineDebug, ImportStatement, PathCalculation, TypeReference,
};
use crate::package_mode::PackageMode;
use crate::resolver::{ResolutionContext, TypeResolver};
use crate::{Codegen, CodegenError};
use amalgam_core::{
    debug::{CompilationDebugInfo, DebugConfig, ImportDebugEntry, ImportDebugInfo, ModuleNameTransform},
    types::{Field, Type},
    ImportPathCalculator, IR,
};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
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

/// Symbol table entry for dependency analysis
#[derive(Debug, Clone)]
struct SymbolEntry {
    #[allow(dead_code)]
    name: String,
    module: String,
    version: String,
    group: String,
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
        let imports = self.type_imports
            .entry(type_name.to_string())
            .or_default();
        
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
    /// Track cross-module imports needed for the current module
    current_imports: HashSet<(String, String)>, // (version, type_name)
    /// Symbol table for dependency analysis (Phase 1)
    symbol_table: HashMap<String, SymbolEntry>,
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
}

impl NickelCodegen {
    pub fn new() -> Self {
        Self {
            indent_size: 2,
            resolver: TypeResolver::new(),
            package_mode: PackageMode::default(),
            current_imports: HashSet::new(),
            symbol_table: HashMap::new(),
            same_package_deps: HashSet::new(),
            debug_info: ImportGenerationDebug::default(),
            type_import_map: TypeImportMap::new(),
            current_type_name: None,
            pipeline_debug: ImportPipelineDebug::new(),
            debug_config: DebugConfig::default(),
            compilation_debug: CompilationDebugInfo::new(),
        }
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
                    
                    self.compilation_debug.modules
                        .entry(normalized_module)
                        .or_default()
                        .push(debug_info);
                }
            }
        }
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

    /// Phase 1: Build symbol table from all modules in IR
    #[instrument(skip(self, ir), level = "debug")]
    fn build_symbol_table(&mut self, ir: &IR) {
        self.symbol_table.clear();
        self.debug_info.symbol_table_entries.clear();

        for module in &ir.modules {
            let (group, version) = Self::parse_module_name(&module.name);
            debug!(
                "Processing module: {} (group: {}, version: {})",
                module.name, group, version
            );
            
            // Record module name transformation if debugging
            if self.debug_config.should_debug_imports() {
                if module.name.starts_with("io.k8s") || module.name.contains("_") {
                    let normalized = format!("{}.{}", group, version);
                    self.compilation_debug.add_module_transform(ModuleNameTransform {
                        original: module.name.clone(),
                        normalized: normalized.clone(),
                        group: group.clone(),
                        version: version.clone(),
                        reason: if module.name.starts_with("io.k8s.api") {
                            "Legacy K8s API format".to_string()
                        } else if module.name.starts_with("io.k8s.apimachinery") {
                            "Legacy K8s apimachinery format".to_string()
                        } else if module.name.contains("_") {
                            "Underscore separator normalization".to_string()
                        } else {
                            "Standard format".to_string()
                        },
                    });
                }
                
                tracing::info!(
                    "Building symbol table for module: '{}' -> group='{}', version='{}'",
                    module.name,
                    group,
                    version
                );
            }

            for type_def in &module.types {
                let entry = SymbolEntry {
                    name: type_def.name.clone(),
                    module: module.name.clone(),
                    version: version.clone(),
                    group: group.clone(),
                };
                // Use fully qualified name as key for symbol table
                let fqn = format!("{}.{}.{}", group, version, type_def.name);
                debug!(
                    "Adding symbol: {} -> {} (fqn: {})",
                    type_def.name, module.name, fqn
                );
                self.symbol_table.insert(fqn, entry.clone());

                // Also add with just type name for same-module resolution
                self.symbol_table
                    .insert(type_def.name.clone(), entry.clone());

                // Record in debug structure
                self.debug_info.symbol_table_entries.insert(
                    type_def.name.clone(),
                    (module.name.clone(), group.clone(), version.clone()),
                );

                // Pipeline debug: record symbol
                self.pipeline_debug
                    .record_symbol(&type_def.name, &module.name, &group, &version);
            }
        }
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
                let resolved_location = self.symbol_table.get(name).map(|symbol| symbol.module.clone());
                self.debug_info.references_found.push((
                    current_module.name.clone(),
                    name.clone(),
                    resolved_location.clone(),
                ));

                // If no module specified, it's a same-package reference
                if ref_module.is_none() {
                    // Check if this type exists in our symbol table but not in current module
                    if let Some(symbol) = self.symbol_table.get(name) {
                        debug!(
                            "Found symbol in table: {} -> {} (current module: {})",
                            name, symbol.module, current_module.name
                        );
                        let (current_group, current_version) =
                            Self::parse_module_name(&current_module.name);

                        // Same package, same version - need import even if same module
                        // because types will be split into separate files
                        if symbol.group == current_group && symbol.version == current_version {
                            // Check if this is a reference to a different type (will be in different file)
                            // We need to check the current type being processed to avoid self-references
                            // For now, always add the import - the extraction will handle whether it's needed
                            self.same_package_deps.insert(name.clone());
                            self.debug_info.dependencies_identified.push((
                                current_module.name.clone(),
                                name.clone(),
                                "same-version-same-module-different-file".to_string(),
                            ));
                        }
                        // Same package (group), different version - need import
                        else if symbol.group == current_group && symbol.version != current_version
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
                    (format!("k8s.io.v0"), Some("Unversioned K8s runtime/util type"))
                } else if let Some(version_idx) = parts.iter().position(|&p| p.starts_with("v")) {
                    // Convert io.k8s.apimachinery.pkg.apis.meta.v1 -> k8s.io.v1
                    let version = parts[version_idx];
                    (format!("k8s.io.{}", version), Some("Legacy K8s apimachinery format"))
                } else {
                    // No version found and not runtime/util - default to v0
                    (format!("k8s.io.v0"), Some("Unversioned K8s apimachinery type"))
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
            // Use triple quotes for multiline or long docs
            format!("m%\"\n{}\n\"%", doc.trim())
        } else {
            // Use regular quotes for short docs
            format!("\"{}\"", doc.replace('"', "\\\""))
        }
    }

    fn type_to_nickel(
        &mut self,
        ty: &Type,
        module: &amalgam_core::ir::Module,
        indent_level: usize,
    ) -> Result<String, CodegenError> {
        // Debug log if this produces the problematic output
        let result = self.type_to_nickel_impl(ty, module, indent_level)?;
        if result.contains("managedfieldsentry") || result.contains("ManagedFieldsEntry") {
            eprintln!(
                "WARNING: Generated problematic output '{}' from type: {:?}, current_type: {:?}",
                result,
                ty,
                self.current_type_name
            );
        }
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
                    let variant_str = self.type_to_nickel_impl(variant_type, module, indent_level)?;
                    contracts.push(format!("({} == \"{}\" && {})", tag_field, tag, variant_str));
                }
                Ok(contracts.join(" | "))
            }

            Type::Reference {
                name,
                module: ref_module,
            } => {
                tracing::info!(
                    "Processing Type::Reference - name: {}, ref_module: {:?}, current_module: {}, current_type: {:?}",
                    name,
                    ref_module,
                    module.name,
                    self.current_type_name
                );
                // If we have module information, this is a cross-module reference
                if let Some(ref_module) = ref_module {
                    // Parse both module names to extract group and version
                    let (ref_group, ref_version) = Self::parse_module_name(ref_module);
                    let (current_group, current_version) = Self::parse_module_name(&module.name);

                    // Check if this is a cross-module reference
                    if ref_module != &module.name {
                        // Track this as a cross-module import
                        let snake_name = name.to_lowercase();

                        // Use the ImportPathCalculator to get the correct path
                        let calc = ImportPathCalculator::new();
                        let import_path = calc.calculate(
                            &current_group,
                            &current_version,
                            &ref_group,
                            &ref_version,
                            &snake_name,
                        );

                        // Track the import for this type - format it as a proper Nickel import statement
                        let import_stmt =
                            format!("let {} = import \"{}\" in", snake_name, import_path);
                        eprintln!("🔍 IMPORT SOURCE 1: Generated import: '{}'", import_stmt);
                        tracing::debug!(
                            "Adding cross-module import for type '{}': path='{}', stmt='{}'",
                            self.current_type_name.as_deref().unwrap_or(""),
                            import_path,
                            import_stmt
                        );
                        let current_type = self.current_type_name.as_deref().unwrap_or("");
                        eprintln!("🔍 IMPORT: Adding to TypeImportMap for type '{}': '{}'", current_type, import_stmt);
                        self.type_import_map.add_import(
                            current_type,
                            &import_stmt,
                        );

                        // Generate the reference
                        // For same-package references, just use the import alias (the file exports the type directly)
                        // For cross-package references, use module.type format
                        let result = if ref_group == current_group {
                            // Same package - the imported file directly exports the type
                            snake_name.clone()
                        } else {
                            // Different package - use qualified name
                            format!("{}.{}", snake_name, name)
                        };
                        eprintln!("🔍 TRACE: Generated qualified reference: '{}' (same_package={}, module_alias='{}')", result, ref_group == current_group, snake_name);
                        return Ok(result);
                    }
                } else {
                    // Same-package reference - check if it needs an import
                    tracing::debug!(
                        "Checking same-package reference: name='{}', module='{}', symbol_exists={}, current_type='{}'",
                        name,
                        module.name,
                        self.symbol_table.contains_key(name),
                        self.current_type_name.as_deref().unwrap_or("unknown")
                    );
                    if let Some(symbol) = self.symbol_table.get(name) {
                        let (current_group, current_version) =
                            Self::parse_module_name(&module.name);

                        tracing::debug!(
                            "Symbol found: name='{}', symbol.module='{}', symbol.group='{}', symbol.version='{}', current_group='{}', current_version='{}', different_module={}",
                            name,
                            symbol.module,
                            symbol.group,
                            symbol.version,
                            current_group,
                            current_version,
                            symbol.module != module.name
                        );

                        // If it's same package, same version, but different module - need import
                        if symbol.group == current_group
                            && symbol.version == current_version
                            && symbol.module != module.name
                        {
                            // Generate import statement for same-package reference
                            let snake_name = name.to_lowercase();
                            let import_path = format!("./{}.ncl", snake_name);
                            let import_stmt = format!("let {} = import \"{}\" in", snake_name, import_path);
                            eprintln!("🔍 IMPORT SOURCE 2: Generated import: '{}'", import_stmt);
                            
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

                            // Use import alias directly for same-package reference
                            // The imported file exports the type directly, not as a field
                            let result = snake_name.clone();
                            eprintln!("🔍 TRACE: Generated qualified reference for same-package: '{}' (using import alias directly)", result);
                            return Ok(result);
                        }
                        // If it's same package but different version, use imported alias
                        else if symbol.group == current_group && symbol.version != current_version
                        {
                            let import_alias =
                                format!("{}_{}_{}", symbol.version, name.to_lowercase(), "import");
                            let result = format!("{}.{}", import_alias, name);
                            eprintln!("🔍 TRACE: Generated qualified reference at line 747: '{}' (import_alias='{}', name='{}')", result, import_alias, name);
                            return Ok(result);
                        }
                    } else {
                        // Symbol not found in table - but we should still try to generate import
                        // for same-package references
                        let (current_group, current_version) = Self::parse_module_name(&module.name);
                        
                        // For same-package references, assume they exist and generate import
                        // This handles cases where the symbol table might be incomplete
                        let snake_name = sanitize_import_variable_name(name);
                        let import_path = format!("./{}.ncl", snake_name);
                        let import_stmt = format!("let {} = import \"{}\" in", snake_name, import_path);
                        eprintln!("🔍 IMPORT SOURCE 3: Generated import: '{}'", import_stmt);
                        
                        tracing::debug!(
                            "Symbol '{}' not in table, generating speculative import for same-package reference",
                            name
                        );
                        
                        let current_type = self.current_type_name.as_deref().unwrap_or("");
                        eprintln!("🔍 IMPORT: Adding to TypeImportMap for type '{}': '{}'", current_type, import_stmt);
                        self.type_import_map.add_import(
                            current_type,
                            &import_stmt,
                        );
                        
                        // Return qualified reference - extract just the type name, not the full path
                        let type_name = name.split('.').last().unwrap_or(name);
                        let result = format!("{}.{}", snake_name, type_name);
                        eprintln!("🔍 TRACE: Generated qualified reference at line 777: '{}' (snake_name='{}', name='{}')", result, snake_name, name);
                        return Ok(result);
                    }
                }

                // Use the resolver for same-module references or fallback
                let context = ResolutionContext {
                    current_group: None,
                    current_version: None,
                    current_kind: None,
                };
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

        let mut parts = Vec::new();

        // Field name - escape reserved keywords and fields starting with $
        let field_name = self.escape_field_name(name);
        parts.push(format!("{}{}", indent, field_name));

        // In Nickel, a field with a default value is implicitly optional
        // For Kubernetes types, we make most fields optional to enable gradual construction
        // Only skip 'optional' if there's an explicit default value
        if field.default.is_none() {
            parts.push("optional".to_string());
        }

        // Type
        parts.push(type_str);

        // Documentation (must come BEFORE default in Nickel)
        if let Some(desc) = &field.description {
            parts.push(format!("doc {}", self.format_doc(desc)));
        }

        // Default value (must come AFTER doc in Nickel)
        if let Some(default) = &field.default {
            let default_str = format_json_value_impl(default, indent_level, self);
            parts.push(format!("default = {}", default_str));
        }

        Ok(parts.join(" | "))
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
        Self::new()
    }
}

impl Codegen for NickelCodegen {
    #[instrument(skip(self, ir), level = "info")]
    fn generate(&mut self, ir: &IR) -> Result<String, CodegenError> {
        let mut output = String::new();

        // Phase 1: Build symbol table for all types in the IR
        self.build_symbol_table(ir);

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
                let calc = ImportPathCalculator::new();

                let mut same_pkg_imports: Vec<_> = self.same_package_deps.iter().collect();
                same_pkg_imports.sort();

                for type_name in same_pkg_imports {
                    if let Some(symbol) = self.symbol_table.get(type_name) {
                        // Generate appropriate alias and path based on whether it's same or different version
                        let (import_alias, path) = if symbol.version == current_version {
                            // Same version, different module - import directly as the type
                            let alias = format!("{}_{}", type_name, "type");
                            let path = calc.calculate(
                                &current_group,
                                &current_version,
                                &symbol.group,
                                &symbol.version,
                                &type_name.to_lowercase(),
                            );
                            (alias, path)
                        } else {
                            // Different version - include version in alias
                            let alias = format!(
                                "{}_{}_{}",
                                symbol.version,
                                type_name.to_lowercase(),
                                "import"
                            );
                            let path = calc.calculate(
                                &current_group,
                                &current_version,
                                &symbol.group,
                                &symbol.version,
                                &type_name.to_lowercase(),
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

                let calc = ImportPathCalculator::new();

                for (version, type_name) in imports {
                    let import_alias = format!("{}_{}", version, type_name);

                    // Use unified calculator for cross-module imports within same package
                    let path = calc.calculate(
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
    /// Generate code with per-type import tracking
    /// Returns both the generated code and a map of which imports each type needs
    pub fn generate_with_import_tracking(
        &mut self,
        ir: &IR,
    ) -> Result<(String, TypeImportMap), CodegenError> {
        // Clear the type import map for this generation
        self.type_import_map = TypeImportMap::new();

        let mut output = String::new();

        // Phase 1: Build symbol table for all types in the IR
        self.build_symbol_table(ir);

        for module in &ir.modules {
            // Clear imports for this module
            self.current_imports.clear();
            self.same_package_deps.clear();

            // Process each type and track its specific imports
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

                // Generate import statements for this type's dependencies
                if !type_specific_deps.is_empty() {
                    let (current_group, current_version) = Self::parse_module_name(&module.name);
                    let calc = ImportPathCalculator::new();

                    let mut import_gen = ImportGeneration {
                        type_name: type_def.name.clone(),
                        dependencies: type_specific_deps.iter().cloned().collect(),
                        import_statements: Vec::new(),
                        path_calculations: Vec::new(),
                    };

                    for dep_type_name in &type_specific_deps {
                        if let Some(symbol) = self.symbol_table.get(dep_type_name) {
                            // Generate import statement
                            tracing::debug!(
                                "Calculating import path: from {}/{} to {}/{} for type {}",
                                current_group,
                                current_version,
                                symbol.group,
                                symbol.version,
                                dep_type_name
                            );
                            tracing::debug!(
                                "Symbol details: name={}, module={}, group={}, version={}",
                                symbol.name,
                                symbol.module,
                                symbol.group,
                                symbol.version
                            );
                            let path = calc.calculate(
                                &current_group,
                                &current_version,
                                &symbol.group,
                                &symbol.version,
                                &dep_type_name.to_lowercase(),
                            );
                            tracing::debug!("Calculated path: {}", path);

                            // Sanitize dependency type name for valid Nickel variable names
                            let sanitized_var_name = sanitize_import_variable_name(dep_type_name);
                            let import_stmt =
                                format!("let {} = import \"{}\" in", sanitized_var_name, path);

                            // Record in debug
                            import_gen.import_statements.push(ImportStatement {
                                dependency: dep_type_name.clone(),
                                statement: import_stmt.clone(),
                                path: path.clone(),
                            });

                            import_gen.path_calculations.push(PathCalculation {
                                from_module: module.name.clone(),
                                to_module: symbol.module.clone(),
                                calculated_path: path.clone(),
                                path_type: if symbol.group == current_group
                                    && symbol.version == current_version
                                {
                                    "same-version".to_string()
                                } else if symbol.group == current_group {
                                    "cross-version".to_string()
                                } else {
                                    "cross-package".to_string()
                                },
                            });

                            // Add to the type's import map
                            self.type_import_map
                                .add_import(&type_def.name, &import_stmt);
                        }
                    }

                    // Record the import generation
                    self.pipeline_debug
                        .record_import_generation(&type_def.name, import_gen);
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
                    } else {
                        writeln!(output, "  {} = {}", type_def.name, type_str)?;
                    }
                    if idx < module.types.len() - 1 {
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
                    // Same-package reference - check if it's in the symbol table
                    if let Some(symbol) = self.symbol_table.get(name) {
                        let (current_group, current_version) =
                            Self::parse_module_name(&module.name);

                        // Check if it's in the same group/version
                        // With the unified module approach (one module per version),
                        // all types are in the same module but in different files
                        // So we need imports for any reference to another type
                        if symbol.group == current_group && symbol.version == current_version {
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

    /// Analyze dependencies for a specific type
    #[allow(dead_code)]
    fn analyze_type_dependencies(
        &self,
        ty: &Type,
        module: &amalgam_core::ir::Module,
        deps: &mut HashSet<String>,
    ) {
        match ty {
            Type::Reference {
                name,
                module: ref_module,
            } => {
                // Check if this is a reference to another type
                if ref_module.is_none() {
                    // Same-package reference - check if it's in the symbol table
                    if let Some(symbol) = self.symbol_table.get(name) {
                        let (current_group, current_version) =
                            Self::parse_module_name(&module.name);
                        // If it's same package/version, it will be extracted to a separate file
                        // so we ALWAYS need an import for it (unless it's a self-reference)
                        if symbol.group == current_group && symbol.version == current_version {
                            // Don't add import for self-reference
                            if let Some(current_type) = &self.current_type_name {
                                if name != current_type {
                                    deps.insert(name.clone());
                                }
                            } else {
                                deps.insert(name.clone());
                            }
                        }
                    }
                }
            }
            Type::Array(inner) => self.analyze_type_dependencies(inner, module, deps),
            Type::Optional(inner) => self.analyze_type_dependencies(inner, module, deps),
            Type::Map { value, .. } => self.analyze_type_dependencies(value, module, deps),
            Type::Record { fields, .. } => {
                for field in fields.values() {
                    self.analyze_type_dependencies(&field.ty, module, deps);
                }
            }
            Type::Union { types, .. } => {
                for union_ty in types {
                    self.analyze_type_dependencies(union_ty, module, deps);
                }
            }
            _ => {}
        }
    }
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
        let mut codegen = NickelCodegen::new();
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
        let mut codegen = NickelCodegen::new();
        let module = create_test_module();
        let array_type = Type::Array(Box::new(Type::String));
        assert_eq!(
            codegen.type_to_nickel(&array_type, &module, 0).unwrap(),
            "Array String"
        );
    }

    #[test]
    fn test_optional_generation() {
        let mut codegen = NickelCodegen::new();
        let module = create_test_module();
        let optional_type = Type::Optional(Box::new(Type::String));
        assert_eq!(
            codegen.type_to_nickel(&optional_type, &module, 0).unwrap(),
            "String | Null"
        );
    }

    #[test]
    fn test_doc_formatting() {
        let codegen = NickelCodegen::new();

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

/// Sanitize a string to be a valid Nickel variable name
/// Converts dots, slashes, and other special characters to underscores
fn sanitize_import_variable_name(name: &str) -> String {
    name.replace(['-', '.', '/', ':', '\\'], "_")
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
