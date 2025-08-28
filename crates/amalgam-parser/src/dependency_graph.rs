//! Dependency graph for managing type dependencies

use crate::imports::TypeReference;
use std::collections::{HashMap, HashSet, VecDeque};

/// Represents a node in the dependency graph
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeNode {
    pub group: String,
    pub version: String,
    pub kind: String,
}

impl From<TypeReference> for TypeNode {
    fn from(type_ref: TypeReference) -> Self {
        Self {
            group: type_ref.group,
            version: type_ref.version,
            kind: type_ref.kind,
        }
    }
}

impl TypeNode {
    pub fn new(group: String, version: String, kind: String) -> Self {
        Self {
            group,
            version,
            kind,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{}/{}/{}", self.group, self.version, self.kind)
    }
}

/// Dependency graph for tracking type dependencies
pub struct DependencyGraph {
    /// Adjacency list: node -> set of nodes it depends on
    dependencies: HashMap<TypeNode, HashSet<TypeNode>>,
    /// Reverse dependencies: node -> set of nodes that depend on it
    dependents: HashMap<TypeNode, HashSet<TypeNode>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: TypeNode) {
        self.dependencies.entry(node.clone()).or_default();
        self.dependents.entry(node).or_default();
    }

    /// Add a dependency edge: `from` depends on `to`
    pub fn add_dependency(&mut self, from: TypeNode, to: TypeNode) {
        self.dependencies
            .entry(from.clone())
            .or_default()
            .insert(to.clone());
        self.dependents.entry(to).or_default().insert(from);
    }

    /// Get all direct dependencies of a node
    pub fn dependencies_of(&self, node: &TypeNode) -> Option<&HashSet<TypeNode>> {
        self.dependencies.get(node)
    }

    /// Get all nodes that depend on the given node
    pub fn dependents_of(&self, node: &TypeNode) -> Option<&HashSet<TypeNode>> {
        self.dependents.get(node)
    }

    /// Perform topological sort on the graph
    /// Returns nodes in dependency order (dependencies come before dependents)
    pub fn topological_sort(&self) -> Result<Vec<TypeNode>, CycleError> {
        let mut result = Vec::new();
        let mut in_degree: HashMap<TypeNode, usize> = HashMap::new();
        let mut queue = VecDeque::new();

        // Calculate in-degrees
        for node in self.dependencies.keys() {
            in_degree.insert(node.clone(), 0);
        }

        for deps in self.dependencies.values() {
            for dep in deps {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }

        // Find nodes with no incoming edges
        for (node, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node.clone());
            }
        }

        // Process nodes
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());

            if let Some(deps) = self.dependencies.get(&node) {
                for dep in deps {
                    if let Some(degree) = in_degree.get_mut(dep) {
                        *degree = degree.saturating_sub(1);
                        if *degree == 0 {
                            queue.push_back(dep.clone());
                        }
                    }
                }
            }
        }

        // Check for cycles
        if result.len() != self.dependencies.len() {
            return Err(CycleError::new(self.find_cycle()));
        }

        // Reverse to get dependency order
        result.reverse();
        Ok(result)
    }

    /// Find a cycle in the graph (if any)
    fn find_cycle(&self) -> Vec<TypeNode> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for node in self.dependencies.keys() {
            if !visited.contains(node) {
                if let Some(cycle) =
                    self.find_cycle_dfs(node, &mut visited, &mut rec_stack, &mut path)
                {
                    return cycle;
                }
            }
        }

        Vec::new()
    }

    fn find_cycle_dfs(
        &self,
        node: &TypeNode,
        visited: &mut HashSet<TypeNode>,
        rec_stack: &mut HashSet<TypeNode>,
        path: &mut Vec<TypeNode>,
    ) -> Option<Vec<TypeNode>> {
        visited.insert(node.clone());
        rec_stack.insert(node.clone());
        path.push(node.clone());

        if let Some(deps) = self.dependencies.get(node) {
            for dep in deps {
                if !visited.contains(dep) {
                    if let Some(cycle) = self.find_cycle_dfs(dep, visited, rec_stack, path) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(dep) {
                    // Found a cycle
                    let cycle_start = path.iter().position(|n| n == dep).unwrap();
                    return Some(path[cycle_start..].to_vec());
                }
            }
        }

        rec_stack.remove(node);
        path.pop();
        None
    }

    /// Get all transitive dependencies of a node
    pub fn transitive_dependencies(&self, node: &TypeNode) -> HashSet<TypeNode> {
        let mut result = HashSet::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_back(node.clone());

        while let Some(current) = to_visit.pop_front() {
            if let Some(deps) = self.dependencies.get(&current) {
                for dep in deps {
                    if result.insert(dep.clone()) {
                        to_visit.push_back(dep.clone());
                    }
                }
            }
        }

        result
    }

    /// Check if there's a path from `from` to `to`
    pub fn has_path(&self, from: &TypeNode, to: &TypeNode) -> bool {
        let transitive = self.transitive_dependencies(from);
        transitive.contains(to)
    }
}

/// Error type for cycle detection
#[derive(Debug)]
pub struct CycleError {
    pub cycle: Vec<TypeNode>,
}

impl CycleError {
    pub fn new(cycle: Vec<TypeNode>) -> Self {
        Self { cycle }
    }
}

impl std::fmt::Display for CycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dependency cycle detected: ")?;
        for (i, node) in self.cycle.iter().enumerate() {
            if i > 0 {
                write!(f, " -> ")?;
            }
            write!(f, "{}", node.full_name())?;
        }
        Ok(())
    }
}

impl std::error::Error for CycleError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort() {
        let mut graph = DependencyGraph::new();

        let a = TypeNode::new("test".to_string(), "v1".to_string(), "A".to_string());
        let b = TypeNode::new("test".to_string(), "v1".to_string(), "B".to_string());
        let c = TypeNode::new("test".to_string(), "v1".to_string(), "C".to_string());

        graph.add_node(a.clone());
        graph.add_node(b.clone());
        graph.add_node(c.clone());

        // A depends on B, B depends on C
        graph.add_dependency(a.clone(), b.clone());
        graph.add_dependency(b.clone(), c.clone());

        let sorted = graph.topological_sort().unwrap();

        // C should come first (no dependencies), then B, then A
        assert_eq!(sorted[0], c);
        assert_eq!(sorted[1], b);
        assert_eq!(sorted[2], a);
    }

    #[test]
    fn test_cycle_detection() {
        let mut graph = DependencyGraph::new();

        let a = TypeNode::new("test".to_string(), "v1".to_string(), "A".to_string());
        let b = TypeNode::new("test".to_string(), "v1".to_string(), "B".to_string());
        let c = TypeNode::new("test".to_string(), "v1".to_string(), "C".to_string());

        graph.add_node(a.clone());
        graph.add_node(b.clone());
        graph.add_node(c.clone());

        // Create a cycle: A -> B -> C -> A
        graph.add_dependency(a.clone(), b.clone());
        graph.add_dependency(b.clone(), c.clone());
        graph.add_dependency(c.clone(), a.clone());

        let result = graph.topological_sort();
        assert!(result.is_err());

        if let Err(e) = result {
            assert!(!e.cycle.is_empty());
        }
    }
}
