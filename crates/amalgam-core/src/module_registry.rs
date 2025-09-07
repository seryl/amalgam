//! Module registry for tracking module locations and resolving imports
//!
//! This module provides a registry that maps module names to their actual filesystem
//! locations, enabling correct import path resolution across different package structures.

use std::collections::HashMap;
use std::path::PathBuf;

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::{toposort, is_cyclic_directed, kosaraju_scc};
use serde::{Deserialize, Serialize};

use crate::ir::{Module, IR};
use crate::error::CoreError;

/// Information about a module's location in the filesystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// The module's name (e.g., "k8s.io.v1")
    pub name: String,
    /// The API group (e.g., "k8s.io")
    pub group: String,
    /// The version (e.g., "v1")
    pub version: String,
    /// The normalized filesystem path (e.g., "k8s_io/v1")
    pub path: PathBuf,
    /// The package root directory (e.g., "k8s_io" or "crossplane/apiextensions.crossplane.io/crossplane")
    pub package_root: PathBuf,
}

/// Types of dependencies between modules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyType {
    /// Direct import dependency
    Import,
    /// Type reference dependency
    TypeReference,
    /// Transitive dependency
    Transitive,
}

/// Module dependency graph using petgraph
#[derive(Debug)]
pub struct ModuleDependencyGraph {
    graph: DiGraph<ModuleInfo, DependencyType>,
    module_indices: HashMap<String, NodeIndex>,
}

impl ModuleDependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            module_indices: HashMap::new(),
        }
    }
    
    pub fn add_module(&mut self, module: ModuleInfo) -> NodeIndex {
        let name = module.name.clone();
        let idx = self.graph.add_node(module);
        self.module_indices.insert(name, idx);
        idx
    }
    
    pub fn add_dependency(&mut self, from: &str, to: &str, dep_type: DependencyType) {
        if let (Some(&from_idx), Some(&to_idx)) = 
            (self.module_indices.get(from), self.module_indices.get(to)) {
            self.graph.add_edge(from_idx, to_idx, dep_type);
        }
    }
    
    pub fn topological_sort(&self) -> Result<Vec<String>, CoreError> {
        if is_cyclic_directed(&self.graph) {
            return Err(CoreError::CircularDependency("Circular dependency detected in modules".to_string()));
        }
        
        toposort(&self.graph, None)
            .map(|indices| {
                indices.into_iter()
                    .map(|idx| self.graph[idx].name.clone())
                    .collect()
            })
            .map_err(|_| CoreError::CircularDependency("Failed to sort modules".to_string()))
    }
    
    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        let sccs = kosaraju_scc(&self.graph);
        
        sccs.into_iter()
            .filter(|scc| scc.len() > 1)
            .map(|scc| {
                scc.into_iter()
                    .map(|idx| self.graph[idx].name.clone())
                    .collect()
            })
            .collect()
    }
}

/// Registry for tracking all modules and their locations
#[derive(Debug, Default)]
pub struct ModuleRegistry {
    /// Map from module name to module info
    modules: HashMap<String, ModuleInfo>,
    /// Dependency graph (built lazily)
    dependency_graph: Option<ModuleDependencyGraph>,
}

impl ModuleRegistry {
    /// Create a new module registry
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            dependency_graph: None,
        }
    }

    /// Build a registry from an IR
    pub fn from_ir(ir: &IR) -> Self {
        let mut registry = Self::new();
        
        for module in &ir.modules {
            registry.register_module(module);
        }
        
        // Build the dependency graph after all modules are registered
        registry.build_dependency_graph();
        
        registry
    }
    
    /// Build the dependency graph from registered modules
    pub fn build_dependency_graph(&mut self) {
        let mut graph = ModuleDependencyGraph::new();
        
        // Add all modules as nodes
        for module_info in self.modules.values() {
            graph.add_module(module_info.clone());
        }
        
        // Add edges for dependencies
        // This will be populated when we analyze module imports
        // For now, we'll need to extract dependencies from the IR
        
        self.dependency_graph = Some(graph);
    }

    /// Register a module in the registry
    pub fn register_module(&mut self, module: &Module) {
        let (group, version) = Self::parse_module_name(&module.name);
        let (package_root, module_path) = Self::calculate_paths(&group, &version);
        
        let info = ModuleInfo {
            name: module.name.clone(),
            group: group.clone(),
            version: version.clone(),
            path: module_path,
            package_root,
        };
        
        self.modules.insert(module.name.clone(), info);
    }

    /// Get module info by name
    pub fn get(&self, module_name: &str) -> Option<&ModuleInfo> {
        self.modules.get(module_name)
    }

    /// Calculate the import path from one module to another
    pub fn calculate_import_path(&self, from_module: &str, to_module: &str, to_type: &str) -> Option<String> {
        let from_info = self.get(from_module)?;
        let to_info = self.get(to_module)?;
        
        // Normalize type name to lowercase
        let type_name = to_type.to_lowercase();
        
        // Case 1: Same module - use relative import
        if from_module == to_module {
            return Some(format!("./{}.ncl", type_name));
        }
        
        // Case 2: Same package, different version
        if from_info.group == to_info.group {
            return Some(format!("../{}/{}.ncl", to_info.version, type_name));
        }
        
        // Case 3: Different packages - calculate relative path
        let relative_path = self.calculate_relative_path(&from_info.package_root, &to_info.package_root);
        Some(format!("{}/{}/{}.ncl", relative_path, to_info.version, type_name))
    }

    /// Calculate relative path between two package roots
    fn calculate_relative_path(&self, from_root: &PathBuf, to_root: &PathBuf) -> String {
        // Calculate how many levels deep we are from the packages root
        // From a file in from_root/<version>/<file>.ncl, we need to go up through:
        // 1. The version directory
        // 2. All components of the package root
        
        let from_depth = from_root.components().count() + 1; // +1 for version directory
        
        let mut path_parts = vec![];
        
        // Go up to the packages root
        for _ in 0..from_depth {
            path_parts.push("..");
        }
        
        // Go down to the target package
        for component in to_root.components() {
            if let Some(s) = component.as_os_str().to_str() {
                path_parts.push(s);
            }
        }
        
        path_parts.join("/")
    }

    /// Parse module name into group and version
    fn parse_module_name(module_name: &str) -> (String, String) {
        // Split on dots and find where the version starts
        let parts: Vec<&str> = module_name.split('.').collect();
        
        // Find the version part (starts with 'v' followed by a digit or is a special version)
        let version_idx = parts.iter().rposition(|p| {
            p.starts_with('v') && p.len() > 1 && p.chars().nth(1).unwrap().is_ascii_digit()
                || *p == "v0"
                || *p == "crossplane"
                || *p == "resource"
        });
        
        match version_idx {
            Some(idx) => {
                let group = parts[..idx].join(".");
                let version = parts[idx].to_string();
                (group, version)
            }
            None => {
                // No clear version found, assume the whole thing is the group
                // and use a default version
                (module_name.to_string(), "v1".to_string())
            }
        }
    }

    /// Calculate the filesystem paths for a module
    fn calculate_paths(group: &str, version: &str) -> (PathBuf, PathBuf) {
        let package_root = match group {
            "k8s.io" => PathBuf::from("k8s_io"),
            "" => PathBuf::from("core"),
            // CrossPlane groups have nested directory structures
            g if g.contains("crossplane.io") => {
                let mut path = PathBuf::from("crossplane");
                path.push(g);
                path.push("crossplane");
                path
            }
            g if g.contains('.') => {
                // Convert dots to underscores for filesystem compatibility
                PathBuf::from(g.replace('.', "_"))
            }
            g => PathBuf::from(g),
        };
        
        let mut module_path = package_root.clone();
        module_path.push(version);
        
        (package_root, module_path)
    }

    /// Check if an import is required between two modules
    pub fn requires_import(&self, from_module: &str, to_module: &str) -> bool {
        from_module != to_module
    }

    /// Get all registered modules
    pub fn modules(&self) -> impl Iterator<Item = &ModuleInfo> {
        self.modules.values()
    }
    
    /// Process modules in dependency order using topological sort
    pub fn process_in_dependency_order<F>(&self, mut processor: F) -> Result<(), CoreError>
    where
        F: FnMut(&ModuleInfo) -> Result<(), CoreError>,
    {
        if let Some(ref graph) = self.dependency_graph {
            let sorted_names = graph.topological_sort()?;
            
            for name in sorted_names {
                if let Some(info) = self.get(&name) {
                    processor(info)?;
                } else {
                    return Err(CoreError::ModuleNotFound(name));
                }
            }
            
            Ok(())
        } else {
            // If no graph is built, process in registration order
            for module_info in self.modules.values() {
                processor(module_info)?;
            }
            Ok(())
        }
    }
    
    /// Detect circular dependencies in the module graph
    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        if let Some(ref graph) = self.dependency_graph {
            graph.detect_cycles()
        } else {
            Vec::new()
        }
    }
    
    /// Export registry data for debugging
    pub fn to_debug_data(&self) -> ModuleRegistryDebugData {
        ModuleRegistryDebugData {
            modules: self.modules.clone(),
            dependency_edges: self.extract_dependency_edges(),
            cycles: self.detect_cycles(),
        }
    }
    
    /// Import registry data from debug format
    pub fn from_debug_data(data: ModuleRegistryDebugData) -> Self {
        let mut registry = Self::new();
        registry.modules = data.modules;
        
        // Rebuild dependency graph from edges
        let mut graph = ModuleDependencyGraph::new();
        for module_info in registry.modules.values() {
            graph.add_module(module_info.clone());
        }
        
        for edge in data.dependency_edges {
            graph.add_dependency(&edge.from, &edge.to, edge.dep_type);
        }
        
        registry.dependency_graph = Some(graph);
        registry
    }
    
    /// Extract dependency edges for serialization
    fn extract_dependency_edges(&self) -> Vec<DependencyEdge> {
        let mut edges = Vec::new();
        
        if let Some(ref graph) = self.dependency_graph {
            for edge in graph.graph.edge_indices() {
                if let Some((source, target)) = graph.graph.edge_endpoints(edge) {
                    let from = &graph.graph[source].name;
                    let to = &graph.graph[target].name;
                    let dep_type = *graph.graph.edge_weight(edge).unwrap();
                    
                    edges.push(DependencyEdge {
                        from: from.clone(),
                        to: to.clone(),
                        dep_type,
                    });
                }
            }
        }
        
        edges
    }
}

/// Debug data structure for ModuleRegistry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleRegistryDebugData {
    /// All registered modules
    pub modules: HashMap<String, ModuleInfo>,
    /// Dependency edges between modules
    pub dependency_edges: Vec<DependencyEdge>,
    /// Detected dependency cycles
    pub cycles: Vec<Vec<String>>,
}

/// A single dependency edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
    pub dep_type: DependencyType,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::Module;

    #[test]
    fn test_parse_module_name() {
        let cases = vec![
            ("k8s.io.v1", ("k8s.io", "v1")),
            ("k8s.io.v1alpha3", ("k8s.io", "v1alpha3")),
            ("k8s.io.v0", ("k8s.io", "v0")),
            ("apiextensions.crossplane.io.v1", ("apiextensions.crossplane.io", "v1")),
            ("k8s.io.resource", ("k8s.io", "resource")),
        ];
        
        for (input, (expected_group, expected_version)) in cases {
            let (group, version) = ModuleRegistry::parse_module_name(input);
            assert_eq!(group, expected_group, "Failed for {}", input);
            assert_eq!(version, expected_version, "Failed for {}", input);
        }
    }

    #[test]
    fn test_calculate_paths() {
        let cases = vec![
            ("k8s.io", "v1", (PathBuf::from("k8s_io"), PathBuf::from("k8s_io/v1"))),
            ("example.io", "v1", (PathBuf::from("example_io"), PathBuf::from("example_io/v1"))),
            (
                "apiextensions.crossplane.io",
                "v1",
                (
                    PathBuf::from("crossplane/apiextensions.crossplane.io/crossplane"),
                    PathBuf::from("crossplane/apiextensions.crossplane.io/crossplane/v1"),
                )
            ),
        ];
        
        for (group, version, (expected_root, expected_path)) in cases {
            let (root, path) = ModuleRegistry::calculate_paths(group, version);
            assert_eq!(root, expected_root, "Failed root for {}", group);
            assert_eq!(path, expected_path, "Failed path for {}", group);
        }
    }

    #[test]
    fn test_import_path_calculation() {
        let mut registry = ModuleRegistry::new();
        
        // Register some test modules
        registry.modules.insert(
            "k8s.io.v1".to_string(),
            ModuleInfo {
                name: "k8s.io.v1".to_string(),
                group: "k8s.io".to_string(),
                version: "v1".to_string(),
                path: PathBuf::from("k8s_io/v1"),
                package_root: PathBuf::from("k8s_io"),
            },
        );
        
        registry.modules.insert(
            "k8s.io.v1alpha3".to_string(),
            ModuleInfo {
                name: "k8s.io.v1alpha3".to_string(),
                group: "k8s.io".to_string(),
                version: "v1alpha3".to_string(),
                path: PathBuf::from("k8s_io/v1alpha3"),
                package_root: PathBuf::from("k8s_io"),
            },
        );
        
        registry.modules.insert(
            "example.io.v1".to_string(),
            ModuleInfo {
                name: "example.io.v1".to_string(),
                group: "example.io".to_string(),
                version: "v1".to_string(),
                path: PathBuf::from("example_io/v1"),
                package_root: PathBuf::from("example_io"),
            },
        );
        
        // Test same module
        assert_eq!(
            registry.calculate_import_path("k8s.io.v1", "k8s.io.v1", "Pod"),
            Some("./pod.ncl".to_string())
        );
        
        // Test same package, different version
        assert_eq!(
            registry.calculate_import_path("k8s.io.v1alpha3", "k8s.io.v1", "ObjectMeta"),
            Some("../v1/objectmeta.ncl".to_string())
        );
        
        // Test different packages
        assert_eq!(
            registry.calculate_import_path("example.io.v1", "k8s.io.v1", "ObjectMeta"),
            Some("../../k8s_io/v1/objectmeta.ncl".to_string())
        );
    }
    
    #[test]
    fn test_debug_data_export_import() {
        let mut registry = ModuleRegistry::new();
        
        // Add some test modules
        let module1 = Module {
            name: "k8s.io.v1".to_string(),
            types: vec![],
            imports: vec![],
            constants: vec![],
            metadata: crate::ir::Metadata::default(),
        };
        let module2 = Module {
            name: "k8s.io.v1alpha3".to_string(),
            types: vec![],
            imports: vec![],
            constants: vec![],
            metadata: crate::ir::Metadata::default(),
        };
        
        registry.register_module(&module1);
        registry.register_module(&module2);
        
        // Build dependency graph and add a dependency
        registry.build_dependency_graph();
        if let Some(ref mut graph) = registry.dependency_graph {
            graph.add_dependency("k8s.io.v1", "k8s.io.v1alpha3", DependencyType::Import);
        }
        
        // Export to debug data
        let debug_data = registry.to_debug_data();
        
        // Verify exported data
        assert_eq!(debug_data.modules.len(), 2);
        assert!(debug_data.modules.contains_key("k8s.io.v1"));
        assert!(debug_data.modules.contains_key("k8s.io.v1alpha3"));
        assert_eq!(debug_data.dependency_edges.len(), 1);
        
        // Import from debug data
        let imported_registry = ModuleRegistry::from_debug_data(debug_data.clone());
        
        // Verify imported registry matches original
        assert_eq!(imported_registry.modules.len(), 2);
        assert!(imported_registry.get("k8s.io.v1").is_some());
        assert!(imported_registry.get("k8s.io.v1alpha3").is_some());
        
        // Export again and compare
        let reimported_data = imported_registry.to_debug_data();
        assert_eq!(reimported_data.modules.len(), debug_data.modules.len());
        assert_eq!(reimported_data.dependency_edges.len(), debug_data.dependency_edges.len());
    }
    
    #[test]
    fn test_debug_data_serialization() {
        let mut registry = ModuleRegistry::new();
        
        // Add a test module
        let module = Module {
            name: "test.module.v1".to_string(),
            types: vec![],
            imports: vec![],
            constants: vec![],
            metadata: crate::ir::Metadata::default(),
        };
        registry.register_module(&module);
        
        // Export debug data
        let debug_data = registry.to_debug_data();
        
        // Serialize to JSON
        let json = serde_json::to_string(&debug_data).expect("Should serialize");
        
        // Deserialize back
        let deserialized: ModuleRegistryDebugData = 
            serde_json::from_str(&json).expect("Should deserialize");
        
        // Verify
        assert_eq!(deserialized.modules.len(), 1);
        assert!(deserialized.modules.contains_key("test.module.v1"));
    }
    
    #[test]
    fn test_dependency_graph_operations() {
        let mut graph = ModuleDependencyGraph::new();
        
        // Add modules
        let module1 = ModuleInfo {
            name: "module1".to_string(),
            group: "test".to_string(),
            version: "v1".to_string(),
            path: PathBuf::from("test/v1"),
            package_root: PathBuf::from("test"),
        };
        
        let module2 = ModuleInfo {
            name: "module2".to_string(),
            group: "test".to_string(),
            version: "v2".to_string(),
            path: PathBuf::from("test/v2"),
            package_root: PathBuf::from("test"),
        };
        
        graph.add_module(module1);
        graph.add_module(module2);
        
        // Add dependency
        graph.add_dependency("module1", "module2", DependencyType::TypeReference);
        
        // Test topological sort
        let sorted = graph.topological_sort().expect("Should sort");
        assert_eq!(sorted.len(), 2);
        // Both modules should be in the sorted result
        assert!(sorted.contains(&"module1".to_string()));
        assert!(sorted.contains(&"module2".to_string()));
    }
    
    #[test]
    fn test_cycle_detection() {
        let mut graph = ModuleDependencyGraph::new();
        
        // Create modules
        let module1 = ModuleInfo {
            name: "module1".to_string(),
            group: "test".to_string(),
            version: "v1".to_string(),
            path: PathBuf::from("test/v1"),
            package_root: PathBuf::from("test"),
        };
        
        let module2 = ModuleInfo {
            name: "module2".to_string(),
            group: "test".to_string(),
            version: "v2".to_string(),
            path: PathBuf::from("test/v2"),
            package_root: PathBuf::from("test"),
        };
        
        graph.add_module(module1);
        graph.add_module(module2);
        
        // Create a cycle
        graph.add_dependency("module1", "module2", DependencyType::Import);
        graph.add_dependency("module2", "module1", DependencyType::Import);
        
        // Should detect the cycle
        let cycles = graph.detect_cycles();
        assert_eq!(cycles.len(), 1);
        assert_eq!(cycles[0].len(), 2);
        assert!(cycles[0].contains(&"module1".to_string()));
        assert!(cycles[0].contains(&"module2".to_string()));
        
        // Topological sort should fail
        assert!(graph.topological_sort().is_err());
    }
}