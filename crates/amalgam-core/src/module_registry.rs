//! Module registry for tracking module locations and resolving imports
//!
//! This module provides a registry that maps module names to their actual filesystem
//! locations, enabling correct import path resolution across different package structures.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use petgraph::algo::{is_cyclic_directed, kosaraju_scc, toposort};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;
use serde::{Deserialize, Serialize};

use crate::error::CoreError;
use crate::ir::{Module, IR};

/// Semantic classification of module layout patterns
/// Note: These are NOT mutually exclusive - a package can have complex
/// combinations of namespace partitioning AND version directories!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleLayout {
    /// Mixed structure with both versioned and non-versioned paths at root
    /// K8s pattern: some dirs are versions (v1, v2), others are namespaces (resource)
    /// Structure: package/{version|namespace}/type.ncl
    MixedRoot,

    /// API groups with their own versions (full K8s pattern)
    /// Structure: package/apigroup/version/type.ncl
    /// Example: k8s_io/apps/v1/Deployment.ncl, k8s_io/core/v1/Pod.ncl
    ApiGroupVersioned,

    /// Namespace directories with versions inside
    /// Structure: package/namespace/version/type.ncl
    NamespacedVersioned,

    /// Namespace directories without versions (CrossPlane pattern)
    /// Structure: package/namespace/subnamespace/type.ncl
    NamespacedFlat,

    /// Single flat directory with all types
    /// Structure: package/type.ncl
    Flat,

    /// Auto-detected from filesystem structure
    /// Will be resolved to one of the above based on discovery
    AutoDetect,
}

/// Information about a module's location in the filesystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// The module's name (e.g., "k8s.io.v1")
    pub name: String,

    /// Source domain (e.g., "k8s.io", "github.com/crossplane", "local://")
    /// This is the canonical source of the module
    pub domain: String,

    /// Logical namespace within the domain (e.g., "api.core", "apiextensions")
    /// This represents the API grouping
    pub namespace: String,

    /// The API group (e.g., "k8s.io") - DEPRECATED: Use domain + namespace
    pub group: String,

    /// The version (e.g., "v1")
    pub version: String,

    /// The module's layout classification
    pub layout: ModuleLayout,

    /// The normalized filesystem path (e.g., "k8s_io/v1")
    pub path: PathBuf,

    /// The package root directory (e.g., "k8s_io" or "crossplane/apiextensions.crossplane.io/crossplane")
    pub package_root: PathBuf,

    /// Set of type names in this module with their correct casing
    /// e.g., "ObjectMeta", "CELDeviceSelector", "Pod"
    #[serde(default)]
    pub type_names: HashSet<String>,
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

impl Default for ModuleDependencyGraph {
    fn default() -> Self {
        Self::new()
    }
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
            (self.module_indices.get(from), self.module_indices.get(to))
        {
            self.graph.add_edge(from_idx, to_idx, dep_type);
        }
    }

    pub fn topological_sort(&self) -> Result<Vec<String>, CoreError> {
        if is_cyclic_directed(&self.graph) {
            return Err(CoreError::CircularDependency(
                "Circular dependency detected in modules".to_string(),
            ));
        }

        toposort(&self.graph, None)
            .map(|indices| {
                indices
                    .into_iter()
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

        // Add edges for dependencies by analyzing type references
        self.analyze_dependencies(&mut graph);

        self.dependency_graph = Some(graph);
    }

    /// Analyze module dependencies based on type references
    fn analyze_dependencies(&self, graph: &mut ModuleDependencyGraph) {
        // For each module, look for type references to other modules
        for module_name in self.modules.keys() {
            // Check all type definitions in this module
            // Note: We'd need access to the actual Module/IR here to inspect types
            // For now, we can detect cross-module references based on naming patterns

            // Common pattern: types referencing other modules will have qualified names
            // e.g., "io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta"
            for other_module in self.modules.keys() {
                if module_name != other_module {
                    // Check if this module might reference the other module
                    // This is a simplified check - in practice we'd analyze the actual Type definitions
                    if self.might_reference(module_name, other_module) {
                        graph.add_dependency(
                            module_name,
                            other_module,
                            DependencyType::TypeReference,
                        );
                    }
                }
            }
        }
    }

    /// Check if one module might reference another based on naming patterns
    fn might_reference(&self, from_module: &str, to_module: &str) -> bool {
        // Common cross-references in K8s:
        // - Most modules reference meta.v1 for ObjectMeta
        // - Apps modules reference core.v1 for PodSpec
        // - Many modules reference each other for shared types

        // If from_module is an API group and to_module is meta.v1, likely a reference
        if to_module.contains("meta.v1") || to_module.contains("apimachinery") {
            return true;
        }

        // Apps references core
        if from_module.contains("apps") && to_module.contains("core") {
            return true;
        }

        // Higher-level APIs often reference lower-level ones
        if from_module.contains("batch") && to_module.contains("core") {
            return true;
        }

        false
    }

    /// Register a module in the registry
    pub fn register_module(&mut self, module: &Module) {
        let (group, version) = Self::parse_module_name(&module.name);
        let (domain, namespace) = Self::extract_domain_namespace(&group);
        let layout = Self::detect_layout(&group);
        let (package_root, module_path) = Self::calculate_paths(&group, &version);

        // Build the type names set from the module's types
        let mut type_names = HashSet::new();
        for typ in &module.types {
            // Store each type name exactly as it appears in the schema
            type_names.insert(typ.name.clone());
        }

        let info = ModuleInfo {
            name: module.name.clone(),
            domain: domain.clone(),
            namespace: namespace.clone(),
            group: group.clone(),
            version: version.clone(),
            layout,
            path: module_path,
            package_root,
            type_names,
        };

        self.modules.insert(module.name.clone(), info);
    }

    /// Get module info by name
    pub fn get(&self, module_name: &str) -> Option<&ModuleInfo> {
        self.modules.get(module_name)
    }

    /// Calculate the filesystem depth of a module based on how map_module_to_file_path
    /// would lay out the file. This is critical for generating correct relative import paths.
    ///
    /// The depth is the number of directory levels from the package root to the file.
    /// For example, api/core/v1.ncl has depth 2 (api, core directories).
    fn calculate_module_filesystem_depth(module_name: &str) -> usize {
        match module_name {
            // Core k8s.io modules: api/core/{version}.ncl = 2 directory levels
            name if name.starts_with("k8s.io.") => 2,

            // Apimachinery runtime/util/api types
            "apimachinery.pkg.runtime" => 1, // apimachinery.pkg/runtime.ncl
            "apimachinery.pkg.util.intstr" => 2, // apimachinery.pkg/util/intstr.ncl
            "apimachinery.pkg.api.resource" => 2, // apimachinery.pkg/api/resource.ncl

            // APIExtensions server: apiextensions-apiserver.pkg.apis/{group}/{version}.ncl = 2 levels
            name if name.starts_with("apiextensions-apiserver.pkg.apis.") => 2,

            // Kube aggregator: kube-aggregator.pkg.apis/{group}/{version}.ncl = 2 levels
            name if name.starts_with("kube-aggregator.pkg.apis.") => 2,

            // Version module: version.ncl = 0 levels (at root)
            "k8s.io.version" => 0,

            // Apimachinery meta: apimachinery.pkg.apis/meta/{version}/mod.ncl = 3 levels
            name if name.starts_with("apimachinery.pkg.apis.meta.") => 3,

            // Apimachinery runtime versioned: apimachinery.pkg.apis/runtime/{version}/mod.ncl = 3 levels
            name if name.starts_with("apimachinery.pkg.apis.runtime.") => 3,

            // io.k8s.* patterns that use dots-to-slashes fallback: count directory levels
            // The last dot-part is the filename (version), so depth = parts - 1
            // e.g., io.k8s.kube-aggregator.pkg.apis.apiregistration.v1 has 7 parts = 6 directory levels
            name if name.starts_with("io.k8s.") => name.split('.').count() - 1,

            // Default CRD/package pattern: domain_name/version = 1 level
            // e.g., example.io.v1 -> example_io/v1.ncl (1 directory level)
            // e.g., apiextensions.crossplane.io.v1 -> apiextensions_crossplane_io/v1.ncl
            _ => 1,
        }
    }

    /// Generate the appropriate number of parent directory traversals for a given depth
    fn generate_parent_path(depth: usize) -> String {
        if depth == 0 {
            ".".to_string()
        } else {
            vec![".."; depth].join("/")
        }
    }

    /// Calculate the import path from one module to another
    pub fn calculate_import_path(
        &self,
        from_module: &str,
        to_module: &str,
        to_type: &str,
    ) -> Option<String> {
        let from_info = self.get(from_module)?;
        let to_info = self.get(to_module)?;

        // Verify the type exists in the target module with its proper casing
        if !to_info.type_names.contains(to_type) {
            return None; // Type not found in registry - return None instead of panicking
        }

        // Use the type name exactly as provided (it must already be properly cased)
        let type_name = to_type;

        // Case 1: Same module - use relative import
        if from_module == to_module {
            return Some(format!("./{}.ncl", type_name));
        }

        // Calculate the filesystem depth of the FROM module to generate correct relative paths
        let from_depth = Self::calculate_module_filesystem_depth(from_module);
        let parent_path = Self::generate_parent_path(from_depth);

        // Special handling for k8s.io consolidated modules
        // k8s.io uses consolidated module files (v1.ncl) instead of individual type files
        if to_info.domain == "k8s.io"
            || to_info.domain.starts_with("io.k8s.")
            || to_module.starts_with("io.k8s.")
        {
            // Check if we're importing from a non-k8s package (like Crossplane CRDs)
            // In that case, we need to include k8s_io/ in the path
            let is_cross_package = !from_info.domain.starts_with("k8s.io")
                && !from_info.domain.starts_with("io.k8s.")
                && !from_module.starts_with("k8s.io")
                && !from_module.starts_with("io.k8s.")
                && !from_module.starts_with("apimachinery.");
            let cross_pkg_prefix = if is_cross_package { "k8s_io/" } else { "" };

            // Map to consolidated module based on the module structure
            let type_lower = to_type.to_lowercase();

            // Check if this is an apimachinery type
            if type_lower == "objectmeta"
                || type_lower == "labelselector"
                || type_lower == "listmeta"
                || type_lower == "time"
                || type_lower == "condition"
                || type_lower == "managedfieldsentry"
                || type_lower == "microtime"
                || type_lower == "deletionoptions"
                || type_lower == "ownerreference"
                || type_lower == "status"
                || type_lower == "statusdetails"
                || type_lower == "statuscause"
            {
                // These are in apimachinery.pkg.apis/meta/v1/mod.ncl (consolidated module)
                return Some(format!(
                    "{}/{}apimachinery.pkg.apis/meta/{}/mod.ncl",
                    parent_path, cross_pkg_prefix, to_info.version
                ));
            } else if type_lower == "typedlocalobjectreference" {
                // Core API types are in api/core/v1.ncl (consolidated file, not directory)
                return Some(format!(
                    "{}/{}api/core/{}.ncl",
                    parent_path, cross_pkg_prefix, to_info.version
                ));
            } else if type_lower == "intorstring" || type_lower == "rawextension" {
                // These are in the root v0/mod.ncl (unversioned types)
                return Some(format!("{}/{}v0/mod.ncl", parent_path, cross_pkg_prefix));
            } else {
                // Regular API types are in consolidated version files
                // Parse the module name to get the API group structure
                if to_module == "k8s.io.v1" {
                    // Core API group - types are in api/core/v1.ncl (consolidated file)
                    return Some(format!(
                        "{}/{}api/core/{}.ncl",
                        parent_path, cross_pkg_prefix, to_info.version
                    ));
                } else if to_module.starts_with("k8s.io.") && to_module != "k8s.io.v1" {
                    // Other k8s.io API groups - extract the API group name
                    let parts: Vec<&str> = to_module.split('.').collect();
                    if parts.len() >= 3 {
                        let api_group = parts[2]; // e.g., "apps", "batch", "autoscaling", "networking"
                        return Some(format!(
                            "{}/{}api/{}/{}.ncl",
                            parent_path, cross_pkg_prefix, api_group, to_info.version
                        ));
                    }
                } else if to_module.starts_with("io.k8s.api.core") {
                    // Legacy core API group pattern - types are in api/core/v1.ncl
                    return Some(format!(
                        "{}/{}api/core/{}.ncl",
                        parent_path, cross_pkg_prefix, to_info.version
                    ));
                } else if to_module.starts_with("io.k8s.api.") {
                    // Legacy k8s.io API groups - extract the API group name
                    let parts: Vec<&str> = to_module.split('.').collect();
                    if parts.len() >= 4 {
                        let api_group = parts[3]; // e.g., "apps", "batch", "autoscaling"
                        return Some(format!(
                            "{}/{}api/{}/{}.ncl",
                            parent_path, cross_pkg_prefix, api_group, to_info.version
                        ));
                    }
                } else if to_module.starts_with("io.k8s.apimachinery.pkg.apis.meta") {
                    // apimachinery types
                    return Some(format!(
                        "{}/{}apimachinery.pkg.apis/meta/{}.ncl",
                        parent_path, cross_pkg_prefix, to_info.version
                    ));
                } else if to_module.starts_with("io.k8s.kube-aggregator") {
                    // kube-aggregator types - go to flattened path
                    return Some(format!(
                        "{}/{}kube-aggregator.pkg.apis/apiregistration/{}.ncl",
                        parent_path, cross_pkg_prefix, to_info.version
                    ));
                } else if to_module.starts_with("io.k8s.apiextensions-apiserver") {
                    // apiextensions types - go to flattened path
                    return Some(format!(
                        "{}/{}apiextensions-apiserver.pkg.apis/apiextensions/{}.ncl",
                        parent_path, cross_pkg_prefix, to_info.version
                    ));
                }
            }
        }

        // Case 2: Same API group, different version (for non-k8s packages)
        // e.g., from io.k8s.api.apps.v1 to io.k8s.api.apps.v1beta1
        if from_info.domain == to_info.domain && from_info.namespace == to_info.namespace {
            // They're in the same API group directory, just different versions
            return Some(format!("../{}/{}.ncl", to_info.version, type_name));
        }

        // Case 3: Different API groups or packages - need full relative path
        // Calculate from the actual module paths, not just package roots
        let from_depth = from_info.path.components().count();
        let to_path = to_info.path.clone();

        // Go up from current module location to package root
        let mut path_parts: Vec<&str> = vec![".."; from_depth];

        // Go down to target module location
        for component in to_path.components() {
            if let Some(s) = component.as_os_str().to_str() {
                path_parts.push(s);
            }
        }

        let relative_path = path_parts.join("/");
        Some(format!("{}/{}.ncl", relative_path, type_name))
    }

    /// Parse module name into group and version
    /// Handles patterns like:
    /// - io.k8s.api.apps.v1 -> (io.k8s.api.apps, v1)
    /// - io.k8s.api.core.v1 -> (io.k8s.api.core, v1)
    /// - apiextensions.crossplane.io.crossplane -> (apiextensions.crossplane.io, crossplane)
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

    /// Calculate the filesystem paths for a module based on its layout
    fn calculate_paths(group: &str, version: &str) -> (PathBuf, PathBuf) {
        let _layout = Self::detect_layout(group);
        let (_domain, _namespace) = Self::extract_domain_namespace(group);

        // Handle K8s API groups properly
        if group.starts_with("io.k8s.api.") {
            // Extract the API group (e.g., "apps", "batch", "core")
            let api_group = group.strip_prefix("io.k8s.api.").unwrap_or("").to_string();

            // K8s should use ApiGroupVersioned structure: k8s_io/{api_group}/{version}/
            let root = PathBuf::from("k8s_io");
            let mut module_path = root.clone();

            if !api_group.is_empty() && api_group != "core" {
                // Non-core API groups get their own subdirectory
                module_path.push(&api_group);
            }
            module_path.push(version);

            // Package root is still k8s_io, but module path includes the API group
            return (root, module_path);
        }

        // Handle other apimachinery packages
        if group.starts_with("io.k8s.apimachinery.") {
            // For apimachinery types, extract the sub-package
            let sub_package = group
                .strip_prefix("io.k8s.apimachinery.")
                .unwrap_or("")
                .replace('.', "/");

            let root = PathBuf::from("k8s_io");
            let mut module_path = root.clone();
            module_path.push("apimachinery");
            if !sub_package.is_empty() {
                module_path.push(sub_package);
            }
            module_path.push(version);

            return (root, module_path);
        }

        // CrossPlane handling
        if group.contains("crossplane.io") {
            // CrossPlane uses namespace without version dirs
            // Structure: crossplane/{domain}/ (no redundant crossplane subdirectory)
            let mut root = PathBuf::from("crossplane");
            root.push(group);
            return (root.clone(), root);
        }

        // Default K8s handling for backward compatibility (for now)
        if group == "k8s.io" {
            let root = PathBuf::from("k8s_io");
            let mut path = root.clone();
            path.push(version);
            return (root, path);
        }

        // Generic fallback - simple versioned structure
        let root = PathBuf::from(group.replace('.', "_"));
        let mut path = root.clone();
        path.push(version);
        (root, path)
    }

    /// Extract domain and namespace from a group
    fn extract_domain_namespace(group: &str) -> (String, String) {
        if group.is_empty() {
            return ("local://".to_string(), "core".to_string());
        }

        // Check for well-known domain patterns
        let parts: Vec<&str> = group.split('.').collect();

        // Special case for k8s.io - it's just the domain with implicit core namespace
        if group == "k8s.io" {
            return ("k8s.io".to_string(), "core".to_string());
        }

        // Check if this looks like a domain with namespace prefix
        // Pattern: namespace.domain.tld or namespace.subdomain.domain.tld
        if parts.len() >= 2 {
            // Look for common TLDs
            let tld = parts[parts.len() - 1];
            if matches!(tld, "io" | "com" | "org" | "net" | "dev" | "app") {
                // Check if we have at least domain.tld
                if parts.len() >= 2 {
                    // Determine where the domain starts
                    // For patterns like apiextensions.crossplane.io, we want:
                    // domain: crossplane.io, namespace: apiextensions
                    let domain_parts = if parts.len() >= 3
                        && (parts[parts.len() - 2] == "crossplane"
                            || parts[parts.len() - 2] == "kubernetes"
                            || parts[parts.len() - 2] == "istio"
                            || parts[parts.len() - 2] == "linkerd")
                    {
                        // Known projects with namespace.project.io pattern
                        2
                    } else {
                        // Default: assume domain.tld
                        2
                    };

                    let domain = parts[parts.len() - domain_parts..].join(".");
                    let namespace = if parts.len() > domain_parts {
                        parts[0..parts.len() - domain_parts].join(".")
                    } else {
                        "default".to_string()
                    };

                    return (domain, namespace);
                }
            }
        }

        // Fallback: treat the whole thing as a local package
        (format!("local://{}", group), "default".to_string())
    }

    /// Detect the module layout pattern based on domain and structure
    fn detect_layout(group: &str) -> ModuleLayout {
        // TODO: This should use filesystem discovery once integrated
        // For now, use heuristics based on known patterns

        let (domain, namespace) = Self::extract_domain_namespace(group);

        // Detect based on known patterns
        match domain.as_str() {
            "k8s.io" => {
                // K8s uses complex structure:
                // - Some paths are API groups with versions (apps/v1, batch/v1)
                // - Some are just versions at root (v1 for core)
                // - Some are special non-versioned (resource)
                // For now, assume MixedRoot but ideally should be ApiGroupVersioned
                ModuleLayout::MixedRoot
            }
            d if d.ends_with(".io") && namespace != "default" && namespace != "core" => {
                // Projects with namespace prefixes typically use namespace partitioning
                // but we don't know if they have versions without filesystem discovery
                ModuleLayout::NamespacedFlat
            }
            d if d.starts_with("local://") => ModuleLayout::Flat,
            _ => ModuleLayout::MixedRoot, // Default assumption
        }
    }

    /// Check if an import is required between two modules
    pub fn requires_import(&self, from_module: &str, to_module: &str) -> bool {
        from_module != to_module
    }

    /// Get all registered modules
    pub fn modules(&self) -> impl Iterator<Item = &ModuleInfo> {
        self.modules.values()
    }

    /// Get the dependency graph (building it if needed)
    pub fn get_dependency_graph(&mut self) -> &ModuleDependencyGraph {
        if self.dependency_graph.is_none() {
            self.build_dependency_graph();
        }
        self.dependency_graph.as_ref().unwrap()
    }

    /// Get all modules in topological order (dependencies first)
    pub fn get_modules_in_order(&mut self) -> Result<Vec<String>, CoreError> {
        let graph = self.get_dependency_graph();
        graph.topological_sort()
    }

    /// Check for circular dependencies
    pub fn check_for_cycles(&mut self) -> Vec<Vec<String>> {
        let graph = self.get_dependency_graph();
        graph.detect_cycles()
    }

    /// Get all modules that depend on a given module
    pub fn get_dependents(&self, module_name: &str) -> Vec<String> {
        if let Some(graph) = &self.dependency_graph {
            if let Some(&node_idx) = graph.module_indices.get(module_name) {
                let dependents: Vec<String> = graph
                    .graph
                    .neighbors_directed(node_idx, Direction::Incoming)
                    .map(|idx| graph.graph[idx].name.clone())
                    .collect();
                return dependents;
            }
        }
        Vec::new()
    }

    /// Get all modules that a given module depends on
    pub fn get_dependencies(&self, module_name: &str) -> Vec<String> {
        if let Some(graph) = &self.dependency_graph {
            if let Some(&node_idx) = graph.module_indices.get(module_name) {
                let dependencies: Vec<String> = graph
                    .graph
                    .neighbors_directed(node_idx, Direction::Outgoing)
                    .map(|idx| graph.graph[idx].name.clone())
                    .collect();
                return dependencies;
            }
        }
        Vec::new()
    }

    /// Find the module that contains a specific type name
    pub fn find_module_for_type(&self, type_name: &str) -> Option<&ModuleInfo> {
        self.modules
            .values()
            .find(|module| module.type_names.contains(type_name))
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
            (
                "apiextensions.crossplane.io.v1",
                ("apiextensions.crossplane.io", "v1"),
            ),
            ("k8s.io.resource", ("k8s.io", "resource")),
        ];

        for (input, (expected_group, expected_version)) in cases {
            let (group, version) = ModuleRegistry::parse_module_name(input);
            assert_eq!(group, expected_group, "Failed for {}", input);
            assert_eq!(version, expected_version, "Failed for {}", input);
        }
    }

    #[test]
    fn test_extract_domain_namespace() {
        let cases = vec![
            ("k8s.io", ("k8s.io", "core")),
            (
                "apiextensions.crossplane.io",
                ("crossplane.io", "apiextensions"),
            ),
            ("pkg.crossplane.io", ("crossplane.io", "pkg")),
            ("example.com", ("example.com", "default")),
            ("api.example.com", ("example.com", "api")),
            ("", ("local://", "core")),
            ("mypackage", ("local://mypackage", "default")),
        ];

        for (input, (expected_domain, expected_namespace)) in cases {
            let (domain, namespace) = ModuleRegistry::extract_domain_namespace(input);
            assert_eq!(domain, expected_domain, "Failed domain for {}", input);
            assert_eq!(
                namespace, expected_namespace,
                "Failed namespace for {}",
                input
            );
        }
    }

    #[test]
    fn test_detect_layout() {
        let cases = vec![
            ("k8s.io", ModuleLayout::MixedRoot),
            ("apiextensions.crossplane.io", ModuleLayout::NamespacedFlat),
            ("pkg.crossplane.io", ModuleLayout::NamespacedFlat),
            ("example.com", ModuleLayout::MixedRoot),
            ("", ModuleLayout::Flat),
            ("mypackage", ModuleLayout::Flat),
        ];

        for (input, expected_layout) in cases {
            let layout = ModuleRegistry::detect_layout(input);
            assert_eq!(layout, expected_layout, "Failed layout for {}", input);
        }
    }

    #[test]
    fn test_calculate_paths() {
        let cases = vec![
            (
                "k8s.io",
                "v1",
                (PathBuf::from("k8s_io"), PathBuf::from("k8s_io/v1")),
            ),
            (
                "example.io",
                "v1",
                (PathBuf::from("example_io"), PathBuf::from("example_io/v1")),
            ),
            (
                "apiextensions.crossplane.io",
                "v1",
                (
                    PathBuf::from("crossplane/apiextensions.crossplane.io"),
                    PathBuf::from("crossplane/apiextensions.crossplane.io"),
                ),
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
        let mut k8s_v1_types = HashSet::new();
        k8s_v1_types.insert("Pod".to_string());
        k8s_v1_types.insert("ObjectMeta".to_string());

        registry.modules.insert(
            "k8s.io.v1".to_string(),
            ModuleInfo {
                name: "k8s.io.v1".to_string(),
                domain: "k8s.io".to_string(),
                namespace: "core".to_string(),
                group: "k8s.io".to_string(),
                version: "v1".to_string(),
                layout: ModuleLayout::MixedRoot,
                path: PathBuf::from("k8s_io/v1"),
                package_root: PathBuf::from("k8s_io"),
                type_names: k8s_v1_types,
            },
        );

        let mut k8s_v1alpha3_types = HashSet::new();
        k8s_v1alpha3_types.insert("ObjectMeta".to_string());

        registry.modules.insert(
            "k8s.io.v1alpha3".to_string(),
            ModuleInfo {
                name: "k8s.io.v1alpha3".to_string(),
                domain: "k8s.io".to_string(),
                namespace: "core".to_string(),
                group: "k8s.io".to_string(),
                version: "v1alpha3".to_string(),
                layout: ModuleLayout::MixedRoot,
                path: PathBuf::from("k8s_io/v1alpha3"),
                package_root: PathBuf::from("k8s_io"),
                type_names: k8s_v1alpha3_types,
            },
        );

        let mut example_types = HashSet::new();
        example_types.insert("ObjectMeta".to_string());

        registry.modules.insert(
            "example.io.v1".to_string(),
            ModuleInfo {
                name: "example.io.v1".to_string(),
                domain: "example.io".to_string(),
                namespace: "default".to_string(),
                group: "example.io".to_string(),
                version: "v1".to_string(),
                layout: ModuleLayout::MixedRoot,
                path: PathBuf::from("example_io/v1"),
                package_root: PathBuf::from("example_io"),
                type_names: example_types,
            },
        );

        // Test same module - type name must be exact
        assert_eq!(
            registry.calculate_import_path("k8s.io.v1", "k8s.io.v1", "Pod"),
            Some("./Pod.ncl".to_string())
        );

        // Test same package, different version - k8s.io uses consolidated modules
        // k8s.io.v1alpha3 -> api/core/v1alpha3.ncl (2 directory levels: api/core)
        // ObjectMeta is in apimachinery.pkg.apis/meta/v1/mod.ncl
        // So path is: ../../apimachinery.pkg.apis/meta/v1/mod.ncl
        assert_eq!(
            registry.calculate_import_path("k8s.io.v1alpha3", "k8s.io.v1", "ObjectMeta"),
            Some("../../apimachinery.pkg.apis/meta/v1/mod.ncl".to_string())
        );

        // Test different packages - cross-package imports include k8s_io/ prefix
        // example.io.v1 -> example_io/v1.ncl (1 directory level: example_io)
        // ObjectMeta is in k8s_io/apimachinery.pkg.apis/meta/v1/mod.ncl
        // So path is: ../k8s_io/apimachinery.pkg.apis/meta/v1/mod.ncl
        assert_eq!(
            registry.calculate_import_path("example.io.v1", "k8s.io.v1", "ObjectMeta"),
            Some("../k8s_io/apimachinery.pkg.apis/meta/v1/mod.ncl".to_string())
        );
    }

    #[test]
    fn test_deeply_nested_module_depth_calculation() {
        // Test that io.k8s.* modules have correct depth calculation
        // The depth is the number of directory levels, which is module parts minus 1
        // (since the last part is the version/filename, not a directory)
        assert_eq!(
            ModuleRegistry::calculate_module_filesystem_depth(
                "io.k8s.kube-aggregator.pkg.apis.apiregistration.v1"
            ),
            6 // io/k8s/kube-aggregator/pkg/apis/apiregistration/ = 6 directory levels
        );

        assert_eq!(
            ModuleRegistry::calculate_module_filesystem_depth(
                "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1"
            ),
            6 // io/k8s/apiextensions-apiserver/pkg/apis/apiextensions/ = 6 directory levels
        );

        // k8s.io.v1 -> api/core/v1.ncl = 2 directory levels (api, core)
        assert_eq!(
            ModuleRegistry::calculate_module_filesystem_depth("k8s.io.v1"),
            2
        );

        // Crossplane CRDs -> domain_name/v1.ncl = 1 directory level
        assert_eq!(
            ModuleRegistry::calculate_module_filesystem_depth(
                "apiextensions.crossplane.io.v1"
            ),
            1
        );

        // apimachinery.pkg.apis.meta.v1 -> apimachinery.pkg.apis/meta/v1/mod.ncl = 3 levels
        assert_eq!(
            ModuleRegistry::calculate_module_filesystem_depth(
                "apimachinery.pkg.apis.meta.v1"
            ),
            3
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
        assert_eq!(
            reimported_data.dependency_edges.len(),
            debug_data.dependency_edges.len()
        );
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
            domain: "test.com".to_string(),
            namespace: "default".to_string(),
            group: "test".to_string(),
            version: "v1".to_string(),
            layout: ModuleLayout::MixedRoot,
            path: PathBuf::from("test/v1"),
            package_root: PathBuf::from("test"),
            type_names: HashSet::new(),
        };

        let module2 = ModuleInfo {
            name: "module2".to_string(),
            domain: "test.com".to_string(),
            namespace: "default".to_string(),
            group: "test".to_string(),
            version: "v2".to_string(),
            layout: ModuleLayout::MixedRoot,
            path: PathBuf::from("test/v2"),
            package_root: PathBuf::from("test"),
            type_names: HashSet::new(),
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
            domain: "test.com".to_string(),
            namespace: "default".to_string(),
            group: "test".to_string(),
            version: "v1".to_string(),
            layout: ModuleLayout::MixedRoot,
            path: PathBuf::from("test/v1"),
            package_root: PathBuf::from("test"),
            type_names: HashSet::new(),
        };

        let module2 = ModuleInfo {
            name: "module2".to_string(),
            domain: "test.com".to_string(),
            namespace: "default".to_string(),
            group: "test".to_string(),
            version: "v2".to_string(),
            layout: ModuleLayout::MixedRoot,
            path: PathBuf::from("test/v2"),
            package_root: PathBuf::from("test"),
            type_names: HashSet::new(),
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
