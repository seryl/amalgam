//! Dependency resolution using a DAG-based solver

use crate::index::{IndexEntry, PackageIndex, VersionEntry};
use crate::version::VersionConstraint;
use anyhow::{Context, Result};
use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tracing::{debug, info};

/// Resolved package dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    pub root: String,
    pub packages: HashMap<String, ResolvedPackage>,
    pub order: Vec<String>,
}

/// A resolved package with its exact version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedPackage {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub path: String,
}

/// Dependency resolver using SAT-style constraint solving
pub struct DependencyResolver<'a> {
    index: &'a PackageIndex,
    graph: DiGraph<String, ()>,
    nodes: HashMap<String, NodeIndex>,
    constraints: HashMap<String, VersionConstraint>,
    resolved: HashMap<String, String>, // package -> version
}

impl<'a> DependencyResolver<'a> {
    /// Create a new resolver with the package index
    pub fn new(index: &'a PackageIndex) -> Self {
        Self {
            index,
            graph: DiGraph::new(),
            nodes: HashMap::new(),
            constraints: HashMap::new(),
            resolved: HashMap::new(),
        }
    }

    /// Resolve dependencies for a package
    pub fn resolve(&mut self, package_name: &str, version: &str) -> Result<Resolution> {
        info!("Resolving dependencies for {} {}", package_name, version);

        // Clear previous state
        self.graph.clear();
        self.nodes.clear();
        self.constraints.clear();
        self.resolved.clear();

        // Start resolution from root package
        self.resolve_package(package_name, version, None)?;

        // Check for cycles
        let sorted = toposort(&self.graph, None)
            .map_err(|_| anyhow::anyhow!("Dependency cycle detected"))?;

        // Build resolution result
        let mut packages = HashMap::new();
        let mut order = Vec::new();

        for node_idx in sorted.iter().rev() {
            let package_id = &self.graph[*node_idx];
            let (name, version) = package_id
                .split_once('@')
                .ok_or_else(|| anyhow::anyhow!("Invalid package ID: {}", package_id))?;

            let entry = self
                .index
                .find_package(name)
                .ok_or_else(|| anyhow::anyhow!("Package not found: {}", name))?;

            let version_entry = entry
                .versions
                .iter()
                .find(|v| v.version == version)
                .ok_or_else(|| anyhow::anyhow!("Version not found: {} {}", name, version))?;

            let dependencies: Vec<String> = version_entry
                .dependencies
                .iter()
                .filter(|d| !d.optional)
                .map(|d| d.name.clone())
                .collect();

            packages.insert(
                name.to_string(),
                ResolvedPackage {
                    name: name.to_string(),
                    version: version.to_string(),
                    dependencies,
                    path: version_entry.path.clone(),
                },
            );

            order.push(name.to_string());
        }

        Ok(Resolution {
            root: package_name.to_string(),
            packages,
            order,
        })
    }

    /// Recursively resolve a package and its dependencies
    fn resolve_package(
        &mut self,
        name: &str,
        version_req: &str,
        parent: Option<NodeIndex>,
    ) -> Result<NodeIndex> {
        debug!("Resolving {} {}", name, version_req);

        // Check if already resolved
        if let Some(resolved_version) = self.resolved.get(name) {
            // Verify version compatibility
            let constraint = VersionConstraint::parse(version_req)?;
            if !constraint.matches(resolved_version) {
                anyhow::bail!(
                    "Version conflict: {} requires {}, but {} is already resolved",
                    name,
                    version_req,
                    resolved_version
                );
            }

            let package_id = format!("{}@{}", name, resolved_version);
            return Ok(self.nodes[&package_id]);
        }

        // Find matching version
        let entry = self
            .index
            .find_package(name)
            .ok_or_else(|| anyhow::anyhow!("Package not found: {}", name))?;

        let version = self.find_best_version(entry, version_req)?;
        let package_id = format!("{}@{}", name, version.version);

        // Add to graph
        let node = self.graph.add_node(package_id.clone());
        self.nodes.insert(package_id.clone(), node);

        // Add edge from parent if exists
        if let Some(parent_node) = parent {
            self.graph.add_edge(parent_node, node, ());
        }

        // Record resolution
        self.resolved
            .insert(name.to_string(), version.version.clone());

        // Resolve dependencies
        for dep in &version.dependencies {
            if !dep.optional {
                self.resolve_package(&dep.name, &dep.version_req, Some(node))
                    .with_context(|| {
                        format!("Failed to resolve dependency {} for {}", dep.name, name)
                    })?;
            }
        }

        Ok(node)
    }

    /// Find the best matching version for a package
    fn find_best_version<'b>(
        &self,
        entry: &'b IndexEntry,
        version_req: &str,
    ) -> Result<&'b VersionEntry> {
        let constraint = if version_req == "*" || version_req.is_empty() {
            // Use latest version if no constraint specified
            VersionConstraint::Any
        } else {
            VersionConstraint::parse(version_req)?
        };

        // Find all matching versions (excluding yanked)
        let mut matching: Vec<_> = entry
            .versions
            .iter()
            .filter(|v| !v.yanked && constraint.matches(&v.version))
            .collect();

        if matching.is_empty() {
            anyhow::bail!(
                "No matching version found for {} with constraint {}",
                entry.name,
                version_req
            );
        }

        // Sort by version (highest first)
        matching.sort_by(|a, b| {
            semver::Version::parse(&b.version)
                .unwrap()
                .cmp(&semver::Version::parse(&a.version).unwrap())
        });

        Ok(matching[0])
    }
}

/// Batch resolver for multiple packages
pub struct BatchResolver<'a> {
    index: &'a PackageIndex,
}

impl<'a> BatchResolver<'a> {
    pub fn new(index: &'a PackageIndex) -> Self {
        Self { index }
    }

    /// Resolve dependencies for multiple root packages
    pub fn resolve_all(&self, packages: Vec<(&str, &str)>) -> Result<HashMap<String, Resolution>> {
        let mut results = HashMap::new();

        for (name, version) in packages {
            let mut resolver = DependencyResolver::new(self.index);
            let resolution = resolver.resolve(name, version)?;
            results.insert(name.to_string(), resolution);
        }

        Ok(results)
    }

    /// Check for conflicts between multiple resolutions
    pub fn check_conflicts(
        &self,
        resolutions: &HashMap<String, Resolution>,
    ) -> Vec<ConflictReport> {
        let mut conflicts = Vec::new();
        let mut version_map: HashMap<String, HashSet<String>> = HashMap::new();

        // Collect all resolved versions
        for resolution in resolutions.values() {
            for package in resolution.packages.values() {
                version_map
                    .entry(package.name.clone())
                    .or_default()
                    .insert(package.version.clone());
            }
        }

        // Check for packages with multiple versions
        for (name, versions) in version_map {
            if versions.len() > 1 {
                conflicts.push(ConflictReport {
                    package: name,
                    versions: versions.into_iter().collect(),
                });
            }
        }

        conflicts
    }
}

/// Conflict report for dependency resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictReport {
    pub package: String,
    pub versions: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package::PackageBuilder;

    fn create_test_index() -> PackageIndex {
        let mut index = PackageIndex::new();

        // Add test packages
        let pkg_a = PackageBuilder::new("pkg-a", "1.0.0")
            .dependency("pkg-b", "^1.0")
            .file("mod.ncl", "{}")
            .build();
        index.add_package(pkg_a).unwrap();

        let pkg_b = PackageBuilder::new("pkg-b", "1.0.0")
            .file("mod.ncl", "{}")
            .build();
        index.add_package(pkg_b).unwrap();

        let pkg_b_2 = PackageBuilder::new("pkg-b", "2.0.0")
            .file("mod.ncl", "{}")
            .build();
        index.add_package(pkg_b_2).unwrap();

        index
    }

    #[test]
    fn test_simple_resolution() {
        let index = create_test_index();
        let mut resolver = DependencyResolver::new(&index);

        let resolution = resolver.resolve("pkg-a", "1.0.0").unwrap();

        assert_eq!(resolution.packages.len(), 2);
        assert!(resolution.packages.contains_key("pkg-a"));
        assert!(resolution.packages.contains_key("pkg-b"));
    }

    #[test]
    fn test_version_selection() {
        let index = create_test_index();
        let mut resolver = DependencyResolver::new(&index);

        // Should select pkg-b 1.0.0 due to constraint
        let resolution = resolver.resolve("pkg-a", "1.0.0").unwrap();
        let pkg_b = &resolution.packages["pkg-b"];
        assert_eq!(pkg_b.version, "1.0.0");
    }
}
