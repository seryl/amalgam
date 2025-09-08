use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::error::CoreError;
use crate::ir::Module;
use crate::module_registry::ModuleRegistry;
use crate::types::Type;
use petgraph::graph::{DiGraph, NodeIndex};

/// Represents a complete compilation unit with full analysis before code generation
/// This implements the two-phase compilation pattern from compiler engineering
#[derive(Debug, Clone)]
pub struct CompilationUnit {
    /// All modules in the compilation unit
    pub modules: HashMap<String, ModuleAnalysis>,
    /// Global symbol table mapping type references to their definitions
    pub global_symbol_table: HashMap<String, TypeLocation>,
    /// Dependency graph tracking module relationships
    pub dependency_graph: DiGraph<String, ()>,
    /// Module registry for path resolution
    pub module_registry: Arc<ModuleRegistry>,
}

/// Analysis results for a single module
#[derive(Debug, Clone)]
pub struct ModuleAnalysis {
    /// Module identifier (e.g., "k8s_io.api.core.v1")
    pub id: String,
    /// The IR module
    pub module: Module,
    /// External type references this module needs
    pub external_refs: HashSet<String>,
    /// Types this module provides
    pub provided_types: HashSet<String>,
    /// Required imports (module_id -> types needed from that module)
    pub required_imports: HashMap<String, HashSet<String>>,
}

/// Location of a type definition
#[derive(Debug, Clone)]
pub struct TypeLocation {
    /// Module containing the type
    pub module_id: String,
    /// The type name
    pub type_name: String,
    /// Full canonical reference
    pub canonical_ref: String,
}

impl CompilationUnit {
    pub fn new(module_registry: Arc<ModuleRegistry>) -> Self {
        Self {
            modules: HashMap::new(),
            global_symbol_table: HashMap::new(),
            dependency_graph: DiGraph::new(),
            module_registry,
        }
    }

    /// Phase 1: Analyze all modules and build complete symbol table
    pub fn analyze_modules(&mut self, modules: Vec<Module>) -> Result<(), CoreError> {
        // First pass: Register all modules and their types
        for module in modules {
            self.register_module(module)?;
        }

        // Second pass: Resolve all external references
        let module_ids: Vec<String> = self.modules.keys().cloned().collect();
        for module_id in module_ids {
            self.resolve_module_references(&module_id)?;
        }

        // Third pass: Build dependency graph
        self.build_dependency_graph()?;

        // Fourth pass: Calculate import requirements
        self.calculate_import_requirements()?;

        Ok(())
    }

    /// Register a module and its types in the global symbol table
    fn register_module(&mut self, module: Module) -> Result<(), CoreError> {
        let module_id = module.name.clone();
        let mut provided_types = HashSet::new();

        // Register all types provided by this module
        for type_def in &module.types {
            let canonical_ref = format!("{}.{}", &module_id, &type_def.name);
            provided_types.insert(type_def.name.clone());
            
            self.global_symbol_table.insert(
                canonical_ref.clone(),
                TypeLocation {
                    module_id: module_id.clone(),
                    type_name: type_def.name.clone(),
                    canonical_ref,
                },
            );
        }

        let analysis = ModuleAnalysis {
            id: module_id.clone(),
            module,
            external_refs: HashSet::new(),
            provided_types,
            required_imports: HashMap::new(),
        };

        self.modules.insert(module_id, analysis);
        Ok(())
    }

    /// Resolve external references for a module
    fn resolve_module_references(&mut self, module_id: &str) -> Result<(), CoreError> {
        let module = self.modules.get(module_id)
            .ok_or_else(|| CoreError::ModuleNotFound(module_id.to_string()))?
            .module.clone();

        let mut external_refs = HashSet::new();

        // Walk through all types and collect external references
        for type_def in &module.types {
            self.collect_type_references(&type_def.ty, &module_id, &mut external_refs)?;
        }

        // Update the module analysis with external references
        if let Some(analysis) = self.modules.get_mut(module_id) {
            analysis.external_refs = external_refs;
        }

        Ok(())
    }

    /// Recursively collect type references from a type definition
    fn collect_type_references(
        &self,
        ty: &Type,
        current_module: &str,
        refs: &mut HashSet<String>,
    ) -> Result<(), CoreError> {
        match ty {
            Type::Reference { name, module } => {
                // Check if this is an external reference
                if let Some(ref_module) = module {
                    if ref_module != current_module {
                        refs.insert(format!("{}.{}", ref_module, name));
                    }
                } else {
                    // Try to resolve the reference
                    let canonical_ref = self.resolve_type_reference(name, current_module)?;
                    if !canonical_ref.starts_with(current_module) {
                        refs.insert(canonical_ref);
                    }
                }
            }
            Type::Array(inner) => {
                self.collect_type_references(inner, current_module, refs)?;
            }
            Type::Optional(inner) => {
                self.collect_type_references(inner, current_module, refs)?;
            }
            Type::Union { types, .. } => {
                for ty in types {
                    self.collect_type_references(ty, current_module, refs)?;
                }
            }
            Type::Record { fields, .. } => {
                for field in fields.values() {
                    self.collect_type_references(&field.ty, current_module, refs)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Resolve a type reference to its canonical form
    fn resolve_type_reference(&self, name: &str, current_module: &str) -> Result<String, CoreError> {
        // First check if it's in the current module
        let local_ref = format!("{}.{}", current_module, name);
        if self.global_symbol_table.contains_key(&local_ref) {
            return Ok(local_ref);
        }

        // Try to find it in the global symbol table
        for (canonical_ref, location) in &self.global_symbol_table {
            if location.type_name == name {
                return Ok(canonical_ref.clone());
            }
        }

        // If not found, return as-is (might be a built-in type)
        Ok(name.to_string())
    }

    /// Build the dependency graph from module references
    fn build_dependency_graph(&mut self) -> Result<(), CoreError> {
        let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

        // Add all modules as nodes
        for module_id in self.modules.keys() {
            let idx = self.dependency_graph.add_node(module_id.clone());
            node_map.insert(module_id.clone(), idx);
        }

        // Add edges for dependencies
        for (module_id, analysis) in &self.modules {
            let from_idx = node_map[module_id];
            
            for external_ref in &analysis.external_refs {
                // Extract module from reference
                if let Some(location) = self.global_symbol_table.get(external_ref) {
                    if let Some(to_idx) = node_map.get(&location.module_id) {
                        self.dependency_graph.add_edge(from_idx, *to_idx, ());
                    }
                }
            }
        }

        Ok(())
    }

    /// Calculate import requirements for each module
    fn calculate_import_requirements(&mut self) -> Result<(), CoreError> {
        for module_id in self.modules.keys().cloned().collect::<Vec<_>>() {
            let external_refs = self.modules[&module_id].external_refs.clone();
            let mut required_imports: HashMap<String, HashSet<String>> = HashMap::new();

            for external_ref in external_refs {
                if let Some(location) = self.global_symbol_table.get(&external_ref) {
                    required_imports
                        .entry(location.module_id.clone())
                        .or_default()
                        .insert(location.type_name.clone());
                }
            }

            if let Some(analysis) = self.modules.get_mut(&module_id) {
                analysis.required_imports = required_imports;
            }
        }

        Ok(())
    }

    /// Get the import requirements for a specific module
    pub fn get_module_imports(&self, module_id: &str) -> Option<&HashMap<String, HashSet<String>>> {
        self.modules.get(module_id).map(|a| &a.required_imports)
    }

    /// Check if there are circular dependencies
    pub fn has_circular_dependencies(&self) -> bool {
        // Use petgraph's cycle detection
        petgraph::algo::is_cyclic_directed(&self.dependency_graph)
    }

    /// Get modules in topological order (dependencies first)
    pub fn get_modules_in_order(&self) -> Result<Vec<String>, CoreError> {
        use petgraph::algo::toposort;
        
        match toposort(&self.dependency_graph, None) {
            Ok(sorted) => {
                Ok(sorted.into_iter()
                    .map(|idx| self.dependency_graph[idx].clone())
                    .collect())
            }
            Err(_) => Err(CoreError::CircularDependency(
                "Circular dependency detected in module graph".to_string()
            ))
        }
    }
}