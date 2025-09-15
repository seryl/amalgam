use crate::pipeline::{
    InputSource, OutputTarget, PipelineBuilder, PipelineDiagnostics, PipelineError, UnifiedPipeline,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmalgamManifest {
    pub metadata: ManifestMetadata,
    pub pipeline: PipelineConfig,
    pub stages: Vec<StageConfig>,
    pub dependencies: Option<Vec<DependencyConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub output_dir: PathBuf,
    #[serde(default)]
    pub export_diagnostics: bool,
    #[serde(default = "default_error_recovery")]
    pub error_recovery: String,
    #[serde(default)]
    pub optimization: OptimizationConfig,
    #[serde(default)]
    pub validation: ValidationConfig,
    #[serde(default)]
    pub diagnostics: DiagnosticsConfig,
    #[serde(default)]
    pub conditions: HashMap<String, ConditionConfig>,
    #[serde(default)]
    pub error_handling: ErrorHandlingConfig,
    #[serde(default)]
    pub performance: PerformanceConfig,
    #[serde(default)]
    pub extensions: Vec<ExtensionConfig>,
}

fn default_error_recovery() -> String {
    "fail-fast".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizationConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub deduplicate_modules: bool,
    #[serde(default)]
    pub consolidate_imports: bool,
    #[serde(default)]
    pub eliminate_unused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidationConfig {
    #[serde(default)]
    pub validate_syntax: bool,
    #[serde(default)]
    pub validate_types: bool,
    #[serde(default)]
    pub validate_contracts: bool,
    #[serde(default)]
    pub validate_imports: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiagnosticsConfig {
    #[serde(default)]
    pub export_dag: bool,
    #[serde(default)]
    pub export_symbol_table: bool,
    #[serde(default)]
    pub export_timing: bool,
    #[serde(default)]
    pub export_memory_usage: bool,
    pub diagnostics_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionConfig {
    pub condition: String,
    pub default: Option<bool>,
    pub fallback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ErrorHandlingConfig {
    #[serde(default = "default_on_stage_failure")]
    pub on_stage_failure: String,
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    #[serde(default = "default_retry_strategy")]
    pub retry_strategy: String,
    #[serde(default)]
    pub recovery_rules: Vec<RecoveryRule>,
}

fn default_on_stage_failure() -> String {
    "fail".to_string()
}

fn default_max_retries() -> u32 {
    0
}

fn default_retry_strategy() -> String {
    "linear".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryRule {
    pub error_pattern: String,
    pub suggestion: String,
    pub auto_fix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceConfig {
    #[serde(default)]
    pub parallel_stages: bool,
    #[serde(default = "default_max_parallel_stages")]
    pub max_parallel_stages: u32,
    #[serde(default)]
    pub use_streaming: bool,
    #[serde(default = "default_chunk_size")]
    pub chunk_size: u32,
    #[serde(default)]
    pub cache_intermediate_results: bool,
    pub cache_ttl: Option<String>,
}

fn default_max_parallel_stages() -> u32 {
    2
}

fn default_chunk_size() -> u32 {
    100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub extension_type: String,
    pub plugin_path: Option<PathBuf>,
    #[serde(default)]
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageConfig {
    pub name: String,
    pub description: Option<String>,
    pub input: InputConfig,
    pub processing: ProcessingConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    #[serde(rename = "type")]
    pub input_type: String,
    pub crd_paths: Option<Vec<String>>,
    pub include_patterns: Option<Vec<String>>,
    pub go_module: Option<String>,
    pub type_patterns: Option<Vec<String>>,
    pub spec_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingConfig {
    #[serde(default = "default_import_strategy")]
    pub import_strategy: String,
    #[serde(default = "default_layout")]
    pub layout: String,
    #[serde(default = "default_symbol_resolution")]
    pub symbol_resolution: String,
    #[serde(default)]
    pub special_cases: Vec<SpecialCaseConfig>,
}

fn default_import_strategy() -> String {
    "hierarchical".to_string()
}

fn default_layout() -> String {
    "single-file".to_string()
}

fn default_symbol_resolution() -> String {
    "eager".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialCaseConfig {
    pub pattern: String,
    pub action: String,
    #[serde(default)]
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    #[serde(rename = "type")]
    pub output_type: String,
    pub target_path: PathBuf,
    #[serde(default)]
    pub include_contracts: bool,
    pub package_name: Option<String>,
    #[serde(default)]
    pub include_json_tags: bool,
    pub format: Option<FormatConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatConfig {
    pub indent: Option<u32>,
    pub max_line_length: Option<u32>,
    #[serde(default)]
    pub trailing_commas: bool,
    #[serde(default)]
    pub use_inline_contracts: bool,
    #[serde(default)]
    pub separate_contract_files: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyConfig {
    pub stage: String,
    pub depends_on: Vec<String>,
    pub import_symbols: Vec<String>,
}

impl AmalgamManifest {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, PipelineError> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| PipelineError::ConfigError(format!("Failed to read manifest: {}", e)))?;

        toml::from_str(&content)
            .map_err(|e| PipelineError::ConfigError(format!("Failed to parse manifest: {}", e)))
    }

    pub fn parse(content: &str) -> Result<Self, PipelineError> {
        toml::from_str(content)
            .map_err(|e| PipelineError::ConfigError(format!("Failed to parse manifest: {}", e)))
    }
}

impl FromStr for AmalgamManifest {
    type Err = PipelineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl AmalgamManifest {
    pub fn execute(&self) -> Result<PipelineDiagnostics, PipelineError> {
        let mut pipelines = Vec::new();

        for stage in &self.stages {
            let pipeline = self.build_pipeline_for_stage(stage)?;
            pipelines.push(pipeline);
        }

        let mut combined_diagnostics = PipelineDiagnostics {
            execution_id: uuid::Uuid::now_v7().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            duration_ms: 0,
            stages: vec![],
            dependency_graph: None,
            symbol_table: None,
            memory_usage: crate::pipeline::MemoryUsage::default(),
            performance_metrics: crate::pipeline::PerformanceMetrics::default(),
            errors: vec![],
            warnings: vec![],
        };

        for _pipeline in pipelines {
            // For now, just create placeholder diagnostics
            // In the full implementation, this would call pipeline.execute()
            let stage_diagnostics = crate::pipeline::StageDiagnostics {
                stage_name: "placeholder".to_string(),
                stage_type: "placeholder".to_string(),
                duration_ms: 100,
                input_size: 0,
                output_size: 0,
                modules_processed: 0,
                types_generated: 0,
                imports_resolved: 0,
                errors: vec![],
                warnings: vec![],
                metadata: HashMap::new(),
            };

            combined_diagnostics.stages.push(stage_diagnostics);
            combined_diagnostics.duration_ms += 100;
        }

        if self.pipeline.export_diagnostics {
            if let Some(diagnostics_path) = &self.pipeline.diagnostics.diagnostics_path {
                self.export_diagnostics(&combined_diagnostics, diagnostics_path)?;
            }
        }

        Ok(combined_diagnostics)
    }

    fn build_pipeline_for_stage(
        &self,
        stage: &StageConfig,
    ) -> Result<UnifiedPipeline, PipelineError> {
        let input_source = self.build_input_source(&stage.input)?;
        let output_target = self.build_output_target(&stage.output)?;

        let pipeline = PipelineBuilder::with_input(input_source)
            .output(output_target)
            .build();

        Ok(pipeline)
    }

    fn build_input_source(&self, input: &InputConfig) -> Result<InputSource, PipelineError> {
        match input.input_type.as_str() {
            "k8s-crd" => {
                let urls = input
                    .crd_paths
                    .as_ref()
                    .ok_or_else(|| {
                        PipelineError::ConfigError("k8s-crd input requires crd_paths".to_string())
                    })?
                    .clone();

                Ok(InputSource::CRDs {
                    urls,
                    domain: "k8s.io".to_string(),
                    versions: vec!["v1".to_string()],
                    auth: None,
                })
            }
            "go-types" => {
                let go_module = input
                    .go_module
                    .as_ref()
                    .ok_or_else(|| {
                        PipelineError::ConfigError("go-types input requires go_module".to_string())
                    })?
                    .clone();

                Ok(InputSource::GoTypes {
                    package: go_module,
                    types: input.type_patterns.clone().unwrap_or_default(),
                    version: None,
                    module_path: None,
                })
            }
            "openapi" => {
                let spec_path = input.spec_path.as_ref().ok_or_else(|| {
                    PipelineError::ConfigError("openapi input requires spec_path".to_string())
                })?;

                Ok(InputSource::OpenAPI {
                    url: format!("file://{}", spec_path.display()),
                    version: "v1".to_string(),
                    domain: None,
                    auth: None,
                })
            }
            _ => Err(PipelineError::ConfigError(format!(
                "Unknown input type: {}",
                input.input_type
            ))),
        }
    }

    fn build_output_target(&self, output: &OutputConfig) -> Result<OutputTarget, PipelineError> {
        match output.output_type.as_str() {
            "nickel" => Ok(OutputTarget::NickelPackage {
                contracts: output.include_contracts,
                validation: true,
                rich_exports: true,
                usage_patterns: true,
                package_metadata: crate::pipeline::PackageMetadata::default(),
                formatting: crate::pipeline::NickelFormatting::default(),
            }),
            "go" => Ok(OutputTarget::Go {
                package_name: output
                    .package_name
                    .clone()
                    .unwrap_or_else(|| "generated".to_string()),
                imports: vec![],
                tags: vec![],
                generate_json_tags: output.include_json_tags,
            }),
            _ => Err(PipelineError::ConfigError(format!(
                "Unknown output type: {}",
                output.output_type
            ))),
        }
    }

    fn export_diagnostics(
        &self,
        diagnostics: &PipelineDiagnostics,
        diagnostics_path: &Path,
    ) -> Result<(), PipelineError> {
        let diagnostics_json = serde_json::to_string_pretty(diagnostics).map_err(|e| {
            PipelineError::ConfigError(format!("Failed to serialize diagnostics: {}", e))
        })?;

        std::fs::write(diagnostics_path, diagnostics_json).map_err(|e| {
            PipelineError::ConfigError(format!("Failed to write diagnostics: {}", e))
        })?;

        Ok(())
    }

    pub fn validate(&self) -> Result<Vec<String>, PipelineError> {
        let mut warnings = Vec::new();

        let stage_names: std::collections::HashSet<_> =
            self.stages.iter().map(|s| s.name.as_str()).collect();

        if let Some(ref dependencies) = self.dependencies {
            for dep in dependencies {
                if !stage_names.contains(dep.stage.as_str()) {
                    warnings.push(format!(
                        "Dependency references unknown stage: {}",
                        dep.stage
                    ));
                }

                for depends_on in &dep.depends_on {
                    if !stage_names.contains(depends_on.as_str()) {
                        warnings.push(format!(
                            "Stage {} depends on unknown stage: {}",
                            dep.stage, depends_on
                        ));
                    }
                }
            }
        }

        for stage in &self.stages {
            match stage.input.input_type.as_str() {
                "k8s-crd" => {
                    if stage.input.crd_paths.is_none() {
                        warnings.push(format!(
                            "Stage {} uses k8s-crd input but has no crd_paths",
                            stage.name
                        ));
                    }
                }
                "go-types" => {
                    if stage.input.go_module.is_none() {
                        warnings.push(format!(
                            "Stage {} uses go-types input but has no go_module",
                            stage.name
                        ));
                    }
                }
                "openapi" => {
                    if stage.input.spec_path.is_none() {
                        warnings.push(format!(
                            "Stage {} uses openapi input but has no spec_path",
                            stage.name
                        ));
                    }
                }
                _ => {
                    warnings.push(format!(
                        "Stage {} uses unknown input type: {}",
                        stage.name, stage.input.input_type
                    ));
                }
            }
        }

        Ok(warnings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_parsing() {
        let manifest_toml = r#"
[metadata]
name = "test-manifest"
version = "0.1.0"
description = "Test manifest for unified pipeline"

[pipeline]
output_dir = "examples/pkgs/test"
export_diagnostics = true
error_recovery = "best-effort"

[[stages]]
name = "core-types"
description = "Import core Kubernetes types"

[stages.input]
type = "k8s-crd"
crd_paths = ["k8s.io/api/core/v1"]
include_patterns = ["Pod", "Service"]

[stages.processing]
import_strategy = "hierarchical"
layout = "single-file"
symbol_resolution = "eager"

[stages.output]
type = "nickel"
target_path = "core/types.ncl"
include_contracts = true
        "#;

        let manifest = AmalgamManifest::from_str(manifest_toml).unwrap();
        assert_eq!(manifest.metadata.name, "test-manifest");
        assert_eq!(manifest.stages.len(), 1);
        assert_eq!(manifest.stages[0].name, "core-types");
    }

    #[test]
    fn test_manifest_validation() {
        let manifest_toml = r#"
[metadata]
name = "test-manifest"
version = "0.1.0"

[pipeline]
output_dir = "examples/pkgs/test"

[[stages]]
name = "stage1"

[stages.input]
type = "k8s-crd"
crd_paths = ["k8s.io/api/core/v1"]

[stages.processing]

[stages.output]
type = "nickel"
target_path = "output.ncl"

[[dependencies]]
stage = "stage1"
depends_on = ["nonexistent-stage"]
import_symbols = ["all"]
        "#;

        let manifest = AmalgamManifest::from_str(manifest_toml).unwrap();
        let warnings = manifest.validate().unwrap();
        assert!(!warnings.is_empty());
        assert!(warnings.iter().any(|w| w.contains("nonexistent-stage")));
    }
}
