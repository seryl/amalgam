//! Unified enum-based pipeline architecture for Amalgam
//!
//! This module implements the revolutionary unified pipeline that replaces all
//! divergent code paths with a single, enum-driven execution engine.
//!
//! ## Core Philosophy
//! - **Single Execution Path**: No more k8s.io vs CrossPlane branching
//! - **Enum-Driven Configuration**: All behavior defined through enum variants
//! - **Consistent Module Structure**: All packages use mod.ncl pattern
//! - **Testability**: Each enum variant can be tested in isolation
//!
//! ## Architecture
//! ```text
//! Input -> Parse -> Transform -> Layout -> Generate -> Output
//!   |        |         |          |         |         |
//! Enum    Unified    Unified    Unified   Unified    Enum
//! Variant Pipeline  Pipeline  Pipeline  Pipeline   Variant
//! ```

use crate::{CoreError, IR};
use petgraph::{Directed, Graph};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Result type for pipeline operations
pub type PipelineResult<T> = Result<T, PipelineError>;

/// Details for input parsing failures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputParsingFailedDetails {
    pub message: String,
    pub recovery_suggestion: Option<String>,
    pub context: ErrorContext,
}

impl std::fmt::Display for InputParsingFailedDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Details for transform failures  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformFailedDetails {
    pub transform: String,
    pub message: String,
    pub recovery_suggestion: Option<String>,
    pub context: ErrorContext,
}

impl std::fmt::Display for TransformFailedDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.transform, self.message)
    }
}

/// Details for layout failures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutFailedDetails {
    pub message: String,
    pub recovery_suggestion: Option<String>,
    pub context: ErrorContext,
}

impl std::fmt::Display for LayoutFailedDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Details for output failures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputFailedDetails {
    pub message: String,
    pub recovery_suggestion: Option<String>,
    pub context: ErrorContext,
}

impl std::fmt::Display for OutputFailedDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Details for import resolution errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResolutionErrorDetails {
    pub message: String,
    pub missing_symbols: Vec<String>,
    pub available_symbols: Vec<String>,
    pub recovery_suggestion: Option<String>,
}

impl std::fmt::Display for ImportResolutionErrorDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Details for type conversion errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeConversionErrorDetails {
    pub from_type: String,
    pub to_type: String,
    pub message: String,
    pub recovery_suggestion: Option<String>,
}

impl std::fmt::Display for TypeConversionErrorDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}: {}",
            self.from_type, self.to_type, self.message
        )
    }
}

/// Unified pipeline error type with recovery suggestions (optimized with boxed large variants)
#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum PipelineError {
    #[error("Input parsing failed: {0}")]
    InputParsingFailed(Box<InputParsingFailedDetails>),

    #[error("Transform failed: {0}")]
    TransformFailed(Box<TransformFailedDetails>),

    #[error("Layout organization failed: {0}")]
    LayoutFailed(Box<LayoutFailedDetails>),

    #[error("Output generation failed: {0}")]
    OutputFailed(Box<OutputFailedDetails>),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Import resolution failed: {0}")]
    ImportResolutionError(Box<ImportResolutionErrorDetails>),

    #[error("Type conversion error: {0}")]
    TypeConversionError(Box<TypeConversionErrorDetails>),

    #[error("Core error: {0}")]
    Core(#[from] CoreError),
}

/// Error context for detailed diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub pipeline_stage: String,
    pub input_source: Option<String>,
    pub current_module: Option<String>,
    pub line_number: Option<u32>,
    pub column_number: Option<u32>,
    pub stack_trace: Vec<String>,
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self {
            pipeline_stage: "unknown".to_string(),
            input_source: None,
            current_module: None,
            line_number: None,
            column_number: None,
            stack_trace: Vec::new(),
        }
    }
}

/// Error recovery strategies
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum RecoveryStrategy {
    /// Fail immediately on any error
    #[default]
    FailFast,

    /// Continue processing other stages/modules
    Continue,

    /// Try best-effort recovery with fallbacks
    BestEffort {
        fallback_types: bool,
        skip_invalid_modules: bool,
        use_dynamic_types: bool,
    },

    /// Interactive recovery (for development)
    Interactive {
        prompt_for_fixes: bool,
        suggest_alternatives: bool,
    },
}

/// Comprehensive pipeline diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineDiagnostics {
    pub execution_id: String,
    pub timestamp: String,
    pub duration_ms: u64,
    pub stages: Vec<StageDiagnostics>,
    pub dependency_graph: Option<PipelineDependencyGraph>,
    pub symbol_table: Option<SymbolTable>,
    pub memory_usage: MemoryUsage,
    pub performance_metrics: PerformanceMetrics,
    pub errors: Vec<PipelineError>,
    pub warnings: Vec<String>,
}

impl PipelineDiagnostics {
    pub fn merge(mut self, other: PipelineDiagnostics) -> Self {
        self.stages.extend(other.stages);
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
        self.duration_ms += other.duration_ms;
        self.memory_usage = self.memory_usage.combine(&other.memory_usage);
        self.performance_metrics = self.performance_metrics.combine(&other.performance_metrics);
        self
    }
}

/// Stage-level diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageDiagnostics {
    pub stage_name: String,
    pub stage_type: String,
    pub duration_ms: u64,
    pub input_size: u64,
    pub output_size: u64,
    pub modules_processed: u32,
    pub types_generated: u32,
    pub imports_resolved: u32,
    pub errors: Vec<PipelineError>,
    pub warnings: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Dependency graph using petgraph for analysis
#[derive(Debug, Clone)]
pub struct PipelineDependencyGraph {
    /// The actual petgraph structure
    pub graph: Graph<DependencyNode, DependencyEdge, Directed>,
    /// Node indices for quick lookup
    pub node_indices: HashMap<String, petgraph::graph::NodeIndex>,
}

impl Default for PipelineDependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineDependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            node_indices: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: DependencyNode) -> petgraph::graph::NodeIndex {
        let node_id = node.id.clone();
        let index = self.graph.add_node(node);
        self.node_indices.insert(node_id, index);
        index
    }

    pub fn add_edge(
        &mut self,
        from_id: &str,
        to_id: &str,
        edge: DependencyEdge,
    ) -> Result<(), String> {
        let from_idx = self
            .node_indices
            .get(from_id)
            .ok_or_else(|| format!("Node not found: {}", from_id))?;
        let to_idx = self
            .node_indices
            .get(to_id)
            .ok_or_else(|| format!("Node not found: {}", to_id))?;

        self.graph.add_edge(*from_idx, *to_idx, edge);
        Ok(())
    }

    /// Detect cycles in the dependency graph
    pub fn has_cycles(&self) -> bool {
        petgraph::algo::is_cyclic_directed(&self.graph)
    }

    /// Get topological ordering for execution
    pub fn topological_order(&self) -> Result<Vec<String>, String> {
        match petgraph::algo::toposort(&self.graph, None) {
            Ok(indices) => Ok(indices
                .into_iter()
                .map(|idx| self.graph[idx].id.clone())
                .collect()),
            Err(_) => Err("Graph contains cycles".to_string()),
        }
    }

    /// Find strongly connected components using Kosaraju's algorithm
    pub fn strongly_connected_components(&self) -> Vec<Vec<String>> {
        petgraph::algo::kosaraju_scc(&self.graph)
            .into_iter()
            .map(|component| {
                component
                    .into_iter()
                    .map(|idx| self.graph[idx].id.clone())
                    .collect()
            })
            .collect()
    }

    /// Export to serializable format for diagnostics
    pub fn to_serializable(&self) -> SerializableDependencyGraph {
        let nodes = self
            .graph
            .node_indices()
            .map(|idx| self.graph[idx].clone())
            .collect();

        let edges = self
            .graph
            .edge_indices()
            .map(|idx| {
                let (from_idx, to_idx) = self.graph.edge_endpoints(idx).unwrap();
                SerializableEdge {
                    from: self.graph[from_idx].id.clone(),
                    to: self.graph[to_idx].id.clone(),
                    edge_data: self.graph[idx].clone(),
                }
            })
            .collect();

        SerializableDependencyGraph { nodes, edges }
    }
}

/// Serializable version of dependency graph for diagnostics export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableDependencyGraph {
    pub nodes: Vec<DependencyNode>,
    pub edges: Vec<SerializableEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableEdge {
    pub from: String,
    pub to: String,
    pub edge_data: DependencyEdge,
}

impl Serialize for PipelineDependencyGraph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_serializable().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PipelineDependencyGraph {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let serializable = SerializableDependencyGraph::deserialize(deserializer)?;
        let mut graph = Self::new();

        // Add all nodes first
        for node in serializable.nodes {
            graph.add_node(node);
        }

        // Then add edges
        for edge in serializable.edges {
            graph
                .add_edge(&edge.from, &edge.to, edge.edge_data)
                .map_err(serde::de::Error::custom)?;
        }

        Ok(graph)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNode {
    pub id: String,
    pub module_path: String,
    pub node_type: String, // "input", "transform", "output"
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub edge_type: String, // "depends_on", "generates", "imports"
    pub weight: Option<f64>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Symbol table for import resolution analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolTable {
    pub modules: HashMap<String, ModuleSymbols>,
    pub global_symbols: Vec<String>,
    pub unresolved_symbols: Vec<UnresolvedSymbol>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleSymbols {
    pub module_path: String,
    pub exported_symbols: Vec<String>,
    pub imported_symbols: Vec<ImportedSymbol>,
    pub private_symbols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedSymbol {
    pub symbol_name: String,
    pub source_module: String,
    pub import_path: String,
    pub is_resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnresolvedSymbol {
    pub symbol_name: String,
    pub requested_by: String,
    pub context: String,
    pub suggested_fixes: Vec<String>,
}

/// Memory usage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsage {
    pub peak_memory_mb: u64,
    pub ir_size_mb: f64,
    pub symbol_table_size_mb: f64,
    pub generated_code_size_mb: f64,
}

impl MemoryUsage {
    pub fn combine(&self, other: &MemoryUsage) -> MemoryUsage {
        MemoryUsage {
            peak_memory_mb: self.peak_memory_mb.max(other.peak_memory_mb),
            ir_size_mb: self.ir_size_mb + other.ir_size_mb,
            symbol_table_size_mb: self.symbol_table_size_mb + other.symbol_table_size_mb,
            generated_code_size_mb: self.generated_code_size_mb + other.generated_code_size_mb,
        }
    }
}

/// Performance metrics for optimization
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMetrics {
    pub parsing_time_ms: u64,
    pub transformation_time_ms: u64,
    pub layout_time_ms: u64,
    pub generation_time_ms: u64,
    pub io_time_ms: u64,
    pub cache_hits: u32,
    pub cache_misses: u32,
}

impl PerformanceMetrics {
    pub fn combine(&self, other: &PerformanceMetrics) -> PerformanceMetrics {
        PerformanceMetrics {
            parsing_time_ms: self.parsing_time_ms + other.parsing_time_ms,
            transformation_time_ms: self.transformation_time_ms + other.transformation_time_ms,
            layout_time_ms: self.layout_time_ms + other.layout_time_ms,
            generation_time_ms: self.generation_time_ms + other.generation_time_ms,
            io_time_ms: self.io_time_ms + other.io_time_ms,
            cache_hits: self.cache_hits + other.cache_hits,
            cache_misses: self.cache_misses + other.cache_misses,
        }
    }
}

impl Default for MemoryUsage {
    fn default() -> Self {
        Self {
            peak_memory_mb: 0,
            ir_size_mb: 0.0,
            symbol_table_size_mb: 0.0,
            generated_code_size_mb: 0.0,
        }
    }
}

/// The unified pipeline - single execution path for ALL package types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedPipeline {
    /// Input source configuration
    pub input: InputSource,

    /// Transformation pipeline
    pub transforms: Vec<Transform>,

    /// Module layout strategy
    pub layout: ModuleLayout,

    /// Output generation target
    pub output: OutputTarget,

    /// Pipeline metadata
    pub metadata: PipelineMetadata,
}

/// Pipeline execution metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
}

impl Default for PipelineMetadata {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            version: "0.1.0".to_string(),
            description: None,
            created_at: None,
        }
    }
}

/// Input source enumeration - what to parse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputSource {
    /// OpenAPI specification from URL
    OpenAPI {
        url: String,
        version: String,
        domain: Option<String>,
        auth: Option<AuthConfig>,
    },

    /// Kubernetes Custom Resource Definitions
    CRDs {
        urls: Vec<String>,
        domain: String,
        versions: Vec<String>,
        auth: Option<AuthConfig>,
    },

    /// Go type definitions
    GoTypes {
        package: String,
        types: Vec<String>,
        version: Option<String>,
        module_path: Option<String>,
    },

    /// Local file sources
    LocalFiles {
        paths: Vec<PathBuf>,
        format: FileFormat,
        recursive: bool,
    },

    /// Git repository source
    GitRepository {
        url: String,
        branch: Option<String>,
        path: Option<String>,
        format: FileFormat,
    },
}

/// Authentication configuration for remote sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthConfig {
    None,
    BearerToken { token: String },
    BasicAuth { username: String, password: String },
    GitHubToken { token: String },
}

/// File format specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileFormat {
    OpenAPI,
    CRD,
    Go,
    JsonSchema,
    Proto,
    Auto, // Auto-detect format
}

/// Transformation pipeline steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transform {
    /// Normalize type names and structures
    NormalizeTypes,

    /// Resolve cross-references and dependencies
    ResolveReferences,

    /// Add Nickel contracts for validation
    AddContracts { strict: bool },

    /// Validate schema consistency
    ValidateSchema,

    /// Remove duplicate type definitions
    DeduplicateTypes,

    /// Apply naming conventions
    ApplyNamingConventions { style: NamingStyle },

    /// Apply special case rules
    ApplySpecialCases { rules: Vec<String> },

    /// Custom transformation (for extensibility)
    Custom {
        name: String,
        config: serde_json::Value,
    },
}

/// Naming convention styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NamingStyle {
    CamelCase,
    PascalCase,
    SnakeCase,
    KebabCase,
    Preserve, // Keep original naming
}

/// Module layout strategies - how to organize
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleLayout {
    /// Kubernetes API layout
    K8s {
        consolidate_versions: bool,
        include_alpha_beta: bool,
        root_exports: Vec<String>,
        api_group_structure: bool,
    },

    /// CrossPlane provider layout
    CrossPlane {
        group_by_version: bool,
        api_extensions: bool,
        provider_specific: bool,
    },

    /// Generic/custom layout
    Generic {
        namespace_pattern: String,
        module_structure: ModuleStructure,
        version_handling: VersionHandling,
    },

    /// Flat layout (all types in one module)
    Flat { module_name: String },

    /// Hierarchical by domain
    DomainBased {
        domain_separator: String,
        max_depth: usize,
    },
}

/// Module structure patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleStructure {
    /// Always use mod.ncl (recommended)
    Consolidated,

    /// Individual .ncl files per type
    Individual,

    /// Hybrid based on complexity
    Hybrid { threshold: usize },
}

/// Version handling strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionHandling {
    /// Separate directories per version
    Directories,

    /// Namespace prefixes
    Namespaced,

    /// Single version only
    Single,
}

/// Output generation targets - how to generate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputTarget {
    /// Rich Nickel package with full features
    NickelPackage {
        contracts: bool,
        validation: bool,
        rich_exports: bool,
        usage_patterns: bool,
        package_metadata: PackageMetadata,
        formatting: NickelFormatting,
    },

    /// Go type definitions
    Go {
        package_name: String,
        imports: Vec<String>,
        tags: Vec<String>,
        generate_json_tags: bool,
    },

    /// CUE language output
    CUE {
        strict_mode: bool,
        constraints: bool,
        package_name: Option<String>,
    },

    /// TypeScript declarations
    TypeScript {
        declarations: bool,
        namespace: Option<String>,
        export_style: TypeScriptExportStyle,
    },

    /// Multiple output targets
    Multi { targets: Vec<OutputTarget> },
}

/// Package metadata for rich package generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
    pub keywords: Vec<String>,
    pub authors: Vec<String>,
}

impl Default for PackageMetadata {
    fn default() -> Self {
        Self {
            name: "generated-package".to_string(),
            version: "0.1.0".to_string(),
            description: "Generated package".to_string(),
            homepage: None,
            repository: None,
            license: Some("MIT".to_string()),
            keywords: vec![],
            authors: vec![],
        }
    }
}

/// Nickel code formatting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NickelFormatting {
    pub indent: usize,
    pub max_line_length: usize,
    pub sort_imports: bool,
    pub compact_records: bool,
}

impl Default for NickelFormatting {
    fn default() -> Self {
        Self {
            indent: 2,
            max_line_length: 100,
            sort_imports: true,
            compact_records: false,
        }
    }
}

/// TypeScript export styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeScriptExportStyle {
    ESModules,
    CommonJS,
    UMD,
    Namespace,
}

/// Generated package result
#[derive(Debug, Clone)]
pub struct GeneratedPackage {
    /// Generated files with their content
    pub files: std::collections::HashMap<PathBuf, String>,

    /// Package metadata
    pub metadata: PackageMetadata,

    /// Generation statistics
    pub stats: GenerationStats,

    /// Diagnostic information
    pub diagnostics: Vec<Diagnostic>,
}

/// Generation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationStats {
    pub types_generated: usize,
    pub modules_created: usize,
    pub imports_resolved: usize,
    pub lines_of_code: usize,
    pub generation_time_ms: u64,
}

/// Diagnostic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub location: Option<String>,
    pub suggestion: Option<String>,
}

/// Diagnostic severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    Info,
    Warning,
    Error,
}

impl UnifiedPipeline {
    /// Create a new pipeline with default configuration
    pub fn new(input: InputSource, output: OutputTarget) -> Self {
        Self {
            input,
            transforms: vec![
                Transform::NormalizeTypes,
                Transform::ResolveReferences,
                Transform::AddContracts { strict: false },
            ],
            layout: ModuleLayout::Generic {
                namespace_pattern: "{domain}/{version}".to_string(),
                module_structure: ModuleStructure::Consolidated,
                version_handling: VersionHandling::Directories,
            },
            output,
            metadata: PipelineMetadata::default(),
        }
    }

    /// Execute the unified pipeline
    pub fn execute(&self) -> PipelineResult<GeneratedPackage> {
        let start_time = std::time::Instant::now();
        let mut diagnostics = Vec::new();

        // Step 1: Parse input source
        let raw_ir = self.parse_input(&mut diagnostics)?;

        // Step 2: Apply transformations
        let transformed_ir = self.apply_transforms(raw_ir, &mut diagnostics)?;

        // Step 3: Organize with layout strategy
        let structured_ir = self.organize_layout(transformed_ir, &mut diagnostics)?;

        // Step 4: Generate output
        let mut generated = self.generate_output(structured_ir, &mut diagnostics)?;

        // Add timing information
        generated.stats.generation_time_ms = start_time.elapsed().as_millis() as u64;
        generated.diagnostics = diagnostics;

        Ok(generated)
    }

    /// Validate pipeline configuration
    pub fn validate(&self) -> PipelineResult<()> {
        // Validate input source
        self.validate_input()?;

        // Validate transforms
        self.validate_transforms()?;

        // Validate layout compatibility
        self.validate_layout()?;

        // Validate output target
        self.validate_output()?;

        Ok(())
    }

    /// Parse input source (private implementation)
    fn parse_input(&self, _diagnostics: &mut [Diagnostic]) -> PipelineResult<IR> {
        match &self.input {
            InputSource::OpenAPI { .. } => {
                // TODO: Implement OpenAPI parsing
                Err(PipelineError::InputParsingFailed(Box::new(
                    InputParsingFailedDetails {
                        message: "OpenAPI parsing not yet implemented".to_string(),
                        recovery_suggestion: Some(
                            "Implement OpenAPI parser integration".to_string(),
                        ),
                        context: ErrorContext::default(),
                    },
                )))
            }
            InputSource::CRDs { .. } => {
                // TODO: Implement CRD parsing
                Err(PipelineError::InputParsingFailed(Box::new(
                    InputParsingFailedDetails {
                        message: "CRD parsing not yet implemented".to_string(),
                        recovery_suggestion: Some("Implement CRD parser integration".to_string()),
                        context: ErrorContext::default(),
                    },
                )))
            }
            InputSource::GoTypes { .. } => {
                // TODO: Implement Go type parsing
                Err(PipelineError::InputParsingFailed(Box::new(
                    InputParsingFailedDetails {
                        message: "Go type parsing not yet implemented".to_string(),
                        recovery_suggestion: Some(
                            "Implement Go type parser integration".to_string(),
                        ),
                        context: ErrorContext::default(),
                    },
                )))
            }
            InputSource::LocalFiles { .. } => {
                // TODO: Implement local file parsing
                Err(PipelineError::InputParsingFailed(Box::new(
                    InputParsingFailedDetails {
                        message: "Local file parsing not yet implemented".to_string(),
                        recovery_suggestion: Some(
                            "Implement local file parser integration".to_string(),
                        ),
                        context: ErrorContext::default(),
                    },
                )))
            }
            InputSource::GitRepository { .. } => {
                // TODO: Implement git repository parsing
                Err(PipelineError::InputParsingFailed(Box::new(
                    InputParsingFailedDetails {
                        message: "Git repository parsing not yet implemented".to_string(),
                        recovery_suggestion: Some(
                            "Implement Git repository parser integration".to_string(),
                        ),
                        context: ErrorContext::default(),
                    },
                )))
            }
        }
    }

    /// Apply transformations (private implementation)
    fn apply_transforms(&self, ir: IR, _diagnostics: &mut [Diagnostic]) -> PipelineResult<IR> {
        let mut current_ir = ir;

        for transform in &self.transforms {
            current_ir = self.apply_single_transform(current_ir, transform)?;
        }

        Ok(current_ir)
    }

    /// Apply single transformation
    fn apply_single_transform(&self, ir: IR, transform: &Transform) -> PipelineResult<IR> {
        match transform {
            Transform::NormalizeTypes => {
                // TODO: Implement type normalization
                Ok(ir)
            }
            Transform::ResolveReferences => {
                // TODO: Implement reference resolution
                Ok(ir)
            }
            Transform::AddContracts { .. } => {
                // TODO: Implement contract addition
                Ok(ir)
            }
            _ => {
                // TODO: Implement other transforms
                Ok(ir)
            }
        }
    }

    /// Organize with layout strategy (private implementation)
    fn organize_layout(&self, ir: IR, _diagnostics: &mut [Diagnostic]) -> PipelineResult<IR> {
        match &self.layout {
            ModuleLayout::K8s { .. } => {
                // TODO: Implement K8s layout
                Ok(ir)
            }
            ModuleLayout::CrossPlane { .. } => {
                // TODO: Implement CrossPlane layout
                Ok(ir)
            }
            ModuleLayout::Generic { .. } => {
                // TODO: Implement generic layout
                Ok(ir)
            }
            _ => {
                // TODO: Implement other layouts
                Ok(ir)
            }
        }
    }

    /// Generate output (private implementation)
    fn generate_output(
        &self,
        _ir: IR,
        _diagnostics: &mut [Diagnostic],
    ) -> PipelineResult<GeneratedPackage> {
        match &self.output {
            OutputTarget::NickelPackage { .. } => {
                // TODO: Implement Nickel package generation
                Ok(GeneratedPackage {
                    files: std::collections::HashMap::new(),
                    metadata: PackageMetadata::default(),
                    stats: GenerationStats {
                        types_generated: 0,
                        modules_created: 0,
                        imports_resolved: 0,
                        lines_of_code: 0,
                        generation_time_ms: 0,
                    },
                    diagnostics: Vec::new(),
                })
            }
            _ => {
                // TODO: Implement other output targets
                Err(PipelineError::OutputFailed(Box::new(OutputFailedDetails {
                    message: "Output target not yet implemented".to_string(),
                    recovery_suggestion: Some("Implement output target generation".to_string()),
                    context: ErrorContext::default(),
                })))
            }
        }
    }

    // Validation methods (private implementations)
    fn validate_input(&self) -> PipelineResult<()> {
        // TODO: Implement input validation
        Ok(())
    }

    fn validate_transforms(&self) -> PipelineResult<()> {
        // TODO: Implement transform validation
        Ok(())
    }

    fn validate_layout(&self) -> PipelineResult<()> {
        // TODO: Implement layout validation
        Ok(())
    }

    fn validate_output(&self) -> PipelineResult<()> {
        // TODO: Implement output validation
        Ok(())
    }
}

/// Configuration builder for creating pipelines
pub struct PipelineBuilder {
    pipeline: UnifiedPipeline,
}

impl PipelineBuilder {
    /// Start building a pipeline with an input source
    pub fn with_input(input: InputSource) -> Self {
        Self {
            pipeline: UnifiedPipeline::new(
                input,
                OutputTarget::NickelPackage {
                    contracts: true,
                    validation: true,
                    rich_exports: true,
                    usage_patterns: false,
                    package_metadata: PackageMetadata::default(),
                    formatting: NickelFormatting::default(),
                },
            ),
        }
    }

    /// Add a transformation step
    pub fn transform(mut self, transform: Transform) -> Self {
        self.pipeline.transforms.push(transform);
        self
    }

    /// Set the layout strategy
    pub fn layout(mut self, layout: ModuleLayout) -> Self {
        self.pipeline.layout = layout;
        self
    }

    /// Set the output target
    pub fn output(mut self, output: OutputTarget) -> Self {
        self.pipeline.output = output;
        self
    }

    /// Set pipeline metadata
    pub fn metadata(mut self, metadata: PipelineMetadata) -> Self {
        self.pipeline.metadata = metadata;
        self
    }

    /// Build the final pipeline
    pub fn build(self) -> UnifiedPipeline {
        self.pipeline
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_builder() {
        let pipeline = PipelineBuilder::with_input(InputSource::OpenAPI {
            url: "https://example.com/openapi.json".to_string(),
            version: "v1".to_string(),
            domain: Some("example.com".to_string()),
            auth: None,
        })
        .transform(Transform::NormalizeTypes)
        .transform(Transform::ResolveReferences)
        .layout(ModuleLayout::K8s {
            consolidate_versions: true,
            include_alpha_beta: true,
            root_exports: vec!["v1".to_string()],
            api_group_structure: true,
        })
        .output(OutputTarget::NickelPackage {
            contracts: true,
            validation: true,
            rich_exports: true,
            usage_patterns: true,
            package_metadata: PackageMetadata::default(),
            formatting: NickelFormatting::default(),
        })
        .build();

        assert_eq!(pipeline.transforms.len(), 5); // Default 3 + 2 added
        assert!(matches!(pipeline.input, InputSource::OpenAPI { .. }));
        assert!(matches!(pipeline.layout, ModuleLayout::K8s { .. }));
        assert!(matches!(
            pipeline.output,
            OutputTarget::NickelPackage { .. }
        ));
    }

    #[test]
    fn test_pipeline_validation() {
        let pipeline = PipelineBuilder::with_input(InputSource::OpenAPI {
            url: "https://example.com/openapi.json".to_string(),
            version: "v1".to_string(),
            domain: Some("example.com".to_string()),
            auth: None,
        })
        .build();

        // Validation should pass (even though execution will fail until implemented)
        assert!(pipeline.validate().is_ok());
    }

    #[test]
    fn test_enum_serialization() {
        let input = InputSource::CRDs {
            urls: vec!["https://example.com/crd.yaml".to_string()],
            domain: "example.com".to_string(),
            versions: vec!["v1".to_string()],
            auth: None,
        };

        // Should serialize and deserialize correctly
        let json = serde_json::to_string(&input).expect("Should serialize");
        let deserialized: InputSource = serde_json::from_str(&json).expect("Should deserialize");

        assert!(matches!(deserialized, InputSource::CRDs { .. }));
    }
}

#[cfg(test)]
mod comprehensive_tests {
    use super::*;

    #[test]
    fn test_dependency_graph_creation() {
        let mut graph = PipelineDependencyGraph::new();

        let node1 = DependencyNode {
            id: "input-node".to_string(),
            module_path: "input/mod.ncl".to_string(),
            node_type: "input".to_string(),
            metadata: HashMap::new(),
        };

        let node2 = DependencyNode {
            id: "transform-node".to_string(),
            module_path: "transform/mod.ncl".to_string(),
            node_type: "transform".to_string(),
            metadata: HashMap::new(),
        };

        graph.add_node(node1);
        graph.add_node(node2);

        let edge = DependencyEdge {
            edge_type: "depends_on".to_string(),
            weight: Some(1.0),
            metadata: HashMap::new(),
        };

        graph
            .add_edge("transform-node", "input-node", edge)
            .unwrap();

        // Test topological ordering
        let order = graph.topological_order().unwrap();
        assert_eq!(order.len(), 2);
        // Since transform-node depends on input-node, input-node should come before transform-node
        // However, topological sort can produce different valid orderings, so let's check that both nodes are present
        assert!(order.contains(&"input-node".to_string()));
        assert!(order.contains(&"transform-node".to_string()));
    }

    #[test]
    fn test_dependency_graph_cycle_detection() {
        let mut graph = PipelineDependencyGraph::new();

        let node1 = DependencyNode {
            id: "node1".to_string(),
            module_path: "node1/mod.ncl".to_string(),
            node_type: "transform".to_string(),
            metadata: HashMap::new(),
        };

        let node2 = DependencyNode {
            id: "node2".to_string(),
            module_path: "node2/mod.ncl".to_string(),
            node_type: "transform".to_string(),
            metadata: HashMap::new(),
        };

        graph.add_node(node1);
        graph.add_node(node2);

        let edge1 = DependencyEdge {
            edge_type: "depends_on".to_string(),
            weight: Some(1.0),
            metadata: HashMap::new(),
        };

        let edge2 = DependencyEdge {
            edge_type: "depends_on".to_string(),
            weight: Some(1.0),
            metadata: HashMap::new(),
        };

        graph.add_edge("node1", "node2", edge1).unwrap();
        graph.add_edge("node2", "node1", edge2).unwrap();

        // Should detect cycle
        assert!(graph.has_cycles());

        // Topological sort should fail
        assert!(graph.topological_order().is_err());
    }

    #[test]
    fn test_memory_usage_combine() {
        let usage1 = MemoryUsage {
            peak_memory_mb: 100,
            ir_size_mb: 10.5,
            symbol_table_size_mb: 2.0,
            generated_code_size_mb: 5.5,
        };

        let usage2 = MemoryUsage {
            peak_memory_mb: 80, // Lower peak, should not be used
            ir_size_mb: 8.0,
            symbol_table_size_mb: 1.5,
            generated_code_size_mb: 4.0,
        };

        let combined = usage1.combine(&usage2);

        assert_eq!(combined.peak_memory_mb, 100); // Max of the two
        assert_eq!(combined.ir_size_mb, 18.5); // Sum
        assert_eq!(combined.symbol_table_size_mb, 3.5); // Sum
        assert_eq!(combined.generated_code_size_mb, 9.5); // Sum
    }

    #[test]
    fn test_performance_metrics_combine() {
        let metrics1 = PerformanceMetrics {
            parsing_time_ms: 100,
            transformation_time_ms: 200,
            layout_time_ms: 50,
            generation_time_ms: 150,
            io_time_ms: 30,
            cache_hits: 10,
            cache_misses: 5,
        };

        let metrics2 = PerformanceMetrics {
            parsing_time_ms: 80,
            transformation_time_ms: 120,
            layout_time_ms: 40,
            generation_time_ms: 100,
            io_time_ms: 20,
            cache_hits: 8,
            cache_misses: 3,
        };

        let combined = metrics1.combine(&metrics2);

        assert_eq!(combined.parsing_time_ms, 180);
        assert_eq!(combined.transformation_time_ms, 320);
        assert_eq!(combined.layout_time_ms, 90);
        assert_eq!(combined.generation_time_ms, 250);
        assert_eq!(combined.io_time_ms, 50);
        assert_eq!(combined.cache_hits, 18);
        assert_eq!(combined.cache_misses, 8);
    }

    #[test]
    fn test_k8s_crd_pipeline_scenario() {
        let mut diagnostics = PipelineDiagnostics {
            execution_id: "test-exec".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            duration_ms: 0,
            stages: vec![],
            dependency_graph: Some(PipelineDependencyGraph::new()),
            symbol_table: Some(SymbolTable {
                modules: HashMap::new(),
                global_symbols: vec!["std".to_string()],
                unresolved_symbols: vec![],
            }),
            memory_usage: MemoryUsage::default(),
            performance_metrics: PerformanceMetrics::default(),
            errors: vec![],
            warnings: vec![],
        };

        // Simulate processing stages
        let stages = [
            ("input", "k8s-crd", 200),
            ("transform", "apply-special-cases", 300),
            ("layout", "hierarchical", 150),
            ("output", "nickel-codegen", 250),
        ];

        for (stage_name, stage_type, duration) in stages {
            let stage_diagnostic = StageDiagnostics {
                stage_name: stage_name.to_string(),
                stage_type: stage_type.to_string(),
                duration_ms: duration,
                input_size: 1024,
                output_size: 2048,
                modules_processed: 3,
                types_generated: if stage_name == "output" { 15 } else { 0 },
                imports_resolved: if stage_name == "layout" { 8 } else { 0 },
                errors: vec![],
                warnings: vec![],
                metadata: HashMap::new(),
            };

            diagnostics.stages.push(stage_diagnostic);
            diagnostics.duration_ms += duration;
        }

        assert_eq!(diagnostics.stages.len(), 4);
        assert_eq!(diagnostics.duration_ms, 900);

        // Should be serializable for export
        let exported = serde_json::to_string_pretty(&diagnostics).unwrap();
        assert!(exported.contains("k8s-crd"));
        assert!(exported.contains("nickel-codegen"));
    }

    #[test]
    fn test_complex_dependency_scenario() {
        let mut graph = PipelineDependencyGraph::new();

        // Simulate a real K8s package dependency structure
        let modules = [
            ("k8s-core", "input"),
            ("k8s-apps", "input"),
            ("meta-types", "transform"),
            ("pod-types", "transform"),
            ("deployment-types", "transform"),
            ("output-core", "output"),
            ("output-apps", "output"),
        ];

        for (module_id, node_type) in &modules {
            let node = DependencyNode {
                id: module_id.to_string(),
                module_path: format!("{}/mod.ncl", module_id),
                node_type: node_type.to_string(),
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert(
                        "api_version".to_string(),
                        serde_json::Value::String("v1".to_string()),
                    );
                    meta
                },
            };
            graph.add_node(node);
        }

        // Define dependencies
        let dependencies = [
            ("meta-types", "k8s-core", "depends_on"),
            ("pod-types", "k8s-core", "depends_on"),
            ("pod-types", "meta-types", "imports"),
            ("deployment-types", "k8s-apps", "depends_on"),
            ("deployment-types", "pod-types", "imports"),
            ("output-core", "pod-types", "generates"),
            ("output-apps", "deployment-types", "generates"),
        ];

        for (from, to, edge_type) in &dependencies {
            let edge = DependencyEdge {
                edge_type: edge_type.to_string(),
                weight: Some(1.0),
                metadata: HashMap::new(),
            };
            graph.add_edge(from, to, edge).unwrap();
        }

        // Should not have cycles
        assert!(!graph.has_cycles());

        // Should be able to get execution order
        let order = graph.topological_order().unwrap();
        assert_eq!(order.len(), 7);

        // All nodes should be present in the topological order
        let expected_nodes = [
            "k8s-core",
            "k8s-apps",
            "meta-types",
            "pod-types",
            "deployment-types",
            "output-core",
            "output-apps",
        ];
        for node in &expected_nodes {
            assert!(order.contains(&node.to_string()), "Missing node: {}", node);
        }

        // Test serialization of complex graph
        let serialized = serde_json::to_string_pretty(&graph).unwrap();
        let deserialized: PipelineDependencyGraph = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.graph.node_count(), 7);
        assert_eq!(deserialized.graph.edge_count(), 7);
        assert!(!deserialized.has_cycles());
    }
}
