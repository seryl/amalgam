//! Runtime daemon for watching and regenerating types

use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// Cache entry for tracking file modifications and compilation state
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub last_modified: SystemTime,
    pub last_compiled: Option<SystemTime>,
    pub checksum: String,
}

/// Configuration for daemon behavior
#[derive(Debug, Clone)]
pub struct DaemonConfig {
    pub poll_interval_ms: u64,
    pub enable_incremental: bool,
    pub cache_size_limit: usize,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            poll_interval_ms: 1000, // Poll every second
            enable_incremental: true,
            cache_size_limit: 1000,
        }
    }
}

pub struct Daemon {
    watch_paths: Vec<PathBuf>,
    output_dir: PathBuf,
    config: DaemonConfig,
    cache: Arc<RwLock<HashMap<PathBuf, CacheEntry>>>,
}

impl Daemon {
    pub fn new(output_dir: PathBuf) -> Self {
        Self {
            watch_paths: Vec::new(),
            output_dir,
            config: DaemonConfig::default(),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn with_config(output_dir: PathBuf, config: DaemonConfig) -> Self {
        Self {
            watch_paths: Vec::new(),
            output_dir,
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_watch_path(&mut self, path: PathBuf) {
        self.watch_paths.push(path);
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting amalgam daemon");
        info!("Watching paths: {:?}", self.watch_paths);
        info!("Output directory: {:?}", self.output_dir);
        info!("Poll interval: {}ms", self.config.poll_interval_ms);

        if self.watch_paths.is_empty() {
            warn!("No watch paths configured");
            return Ok(());
        }

        // Initial scan and compilation
        self.scan_and_compile().await?;

        // Start polling loop for file changes
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(
            self.config.poll_interval_ms,
        ));

        loop {
            interval.tick().await;

            if let Err(e) = self.scan_and_compile().await {
                error!("Error during scan and compile: {}", e);
                // Continue running despite errors
            }
        }
    }

    /// Scan all watch paths for changes and compile if needed
    async fn scan_and_compile(&self) -> Result<()> {
        let mut files_changed = Vec::new();

        // Scan all watch paths for file changes
        for watch_path in &self.watch_paths {
            if let Ok(entries) = std::fs::read_dir(watch_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file()
                        && self.should_watch_file(path.as_path())
                        && self.file_needs_compilation(&path).await?
                    {
                        files_changed.push(path);
                    }
                }
            }
        }

        // Compile changed files
        if !files_changed.is_empty() {
            info!("Found {} changed files", files_changed.len());
            for file_path in files_changed {
                if let Err(e) = self.compile_file(&file_path).await {
                    error!("Failed to compile {}: {}", file_path.display(), e);
                }
            }
        }

        Ok(())
    }

    /// Check if a file should be watched based on its extension
    fn should_watch_file(&self, path: &std::path::Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| matches!(ext, "yaml" | "yml" | "json"))
            .unwrap_or(false)
    }

    /// Check if a file needs compilation based on modification time
    async fn file_needs_compilation(&self, path: &PathBuf) -> Result<bool> {
        let metadata = std::fs::metadata(path)?;
        let modified = metadata.modified()?;

        let cache = self.cache.read().await;

        match cache.get(path) {
            Some(entry) => {
                // File needs compilation if it's been modified since last compilation
                Ok(entry.last_compiled.is_none()
                    || modified > entry.last_modified
                    || !self.config.enable_incremental)
            }
            None => Ok(true), // New file, needs compilation
        }
    }

    /// Compile a single file using amalgam parser and codegen
    async fn compile_file(&self, path: &PathBuf) -> Result<()> {
        use amalgam_codegen::{nickel::NickelCodegen, Codegen};
        use amalgam_parser::{crd::CRDParser, Parser};

        info!("Compiling: {}", path.display());

        // Read and parse the file
        let content = std::fs::read_to_string(path)?;

        // For now, assume YAML files are CRDs
        if path.extension().and_then(|s| s.to_str()) == Some("yaml")
            || path.extension().and_then(|s| s.to_str()) == Some("yml")
        {
            // Parse as CRD
            let crd: amalgam_parser::crd::CRD = serde_yaml::from_str(&content)?;
            let parser = CRDParser::new();
            let ir = parser.parse(crd)?;

            // Generate Nickel code
            let mut codegen = NickelCodegen::from_ir(&ir);
            let generated = codegen.generate(&ir)?;

            // Write output
            let output_filename = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            let output_path = self.output_dir.join(format!("{}.ncl", output_filename));

            std::fs::create_dir_all(&self.output_dir)?;
            std::fs::write(&output_path, generated)?;

            info!("Generated: {}", output_path.display());
        }

        // Update cache
        self.update_cache(path.clone()).await?;

        Ok(())
    }

    /// Update the cache entry for a file
    async fn update_cache(&self, path: PathBuf) -> Result<()> {
        let metadata = std::fs::metadata(&path)?;
        let modified = metadata.modified()?;

        // Simple checksum based on modification time and size
        let checksum = format!("{:?}:{}", modified, metadata.len());

        let mut cache = self.cache.write().await;
        cache.insert(
            path,
            CacheEntry {
                last_modified: modified,
                last_compiled: Some(SystemTime::now()),
                checksum,
            },
        );

        // Limit cache size
        if cache.len() > self.config.cache_size_limit {
            // Remove oldest entries (simple strategy)
            let mut entries: Vec<_> = cache.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            entries.sort_by_key(|(_, entry)| entry.last_compiled.unwrap_or(SystemTime::UNIX_EPOCH));

            let to_remove = cache.len() - self.config.cache_size_limit / 2;
            let paths_to_remove: Vec<_> = entries
                .into_iter()
                .take(to_remove)
                .map(|(path, _)| path)
                .collect();

            for path in paths_to_remove {
                cache.remove(&path);
            }
        }

        Ok(())
    }
}

#[cfg(feature = "kubernetes")]
pub mod k8s {
    use super::*;
    use amalgam_codegen::{nickel::NickelCodegen, Codegen};
    use amalgam_parser::{crd::CRDParser, Parser};
    use futures::TryStreamExt;
    use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
    use kube::{
        runtime::{watcher, WatchStreamExt},
        Api, Client,
    };
    use serde_json;

    pub struct K8sWatcher {
        client: Client,
        output_dir: PathBuf,
    }

    impl K8sWatcher {
        pub async fn new(output_dir: PathBuf) -> Result<Self> {
            let client = Client::try_default().await?;
            Ok(Self { client, output_dir })
        }

        pub async fn watch_crds(&self) -> Result<()> {
            info!("Starting K8s CRD watcher");

            let crds: Api<CustomResourceDefinition> = Api::all(self.client.clone());
            let watcher_config = watcher::Config::default();

            // Watch for CRD changes
            let stream = watcher(crds, watcher_config).applied_objects();
            tokio::pin!(stream);

            while let Some(crd) = stream.try_next().await? {
                info!(
                    "CRD changed: {}",
                    crd.metadata.name.as_deref().unwrap_or("unknown")
                );

                if let Err(e) = self.process_crd(crd).await {
                    error!("Failed to process CRD: {}", e);
                }
            }

            Ok(())
        }

        async fn process_crd(&self, k8s_crd: CustomResourceDefinition) -> Result<()> {
            // Convert k8s CRD to amalgam CRD format
            let amalgam_crd = self.convert_k8s_crd_to_amalgam(k8s_crd)?;

            // Parse and generate types
            let parser = CRDParser::new();
            let ir = parser.parse(amalgam_crd)?;

            let mut codegen = NickelCodegen::from_ir(&ir);
            let generated = codegen.generate(&ir)?;

            // Write to output directory
            let crd_name = ir
                .modules
                .first()
                .map(|m| m.name.clone())
                .unwrap_or_else(|| "unknown".to_string());

            let output_path = self
                .output_dir
                .join(format!("{}.ncl", crd_name.to_lowercase()));

            std::fs::create_dir_all(&self.output_dir)?;
            std::fs::write(&output_path, generated)?;

            info!("Generated CRD types: {}", output_path.display());
            Ok(())
        }

        fn convert_k8s_crd_to_amalgam(
            &self,
            k8s_crd: CustomResourceDefinition,
        ) -> Result<amalgam_parser::crd::CRD> {
            // Convert k8s CRD to amalgam's CRD format
            // This is a simplified conversion - a full implementation would handle all fields

            let spec = k8s_crd.spec;
            let metadata = amalgam_parser::crd::CRDMetadata {
                name: k8s_crd.metadata.name.unwrap_or_default(),
            };

            let versions = spec
                .versions
                .into_iter()
                .map(|v| {
                    amalgam_parser::crd::CRDVersion {
                        name: v.name,
                        served: v.served,
                        storage: v.storage,
                        schema: v.schema.and_then(|s| {
                            // Convert k8s JSONSchemaProps to serde_json::Value
                            // This is a simplified conversion - a full implementation would handle all fields
                            s.open_api_v3_schema.and_then(|schema| {
                                serde_json::to_value(&schema).ok().map(|v| {
                                    amalgam_parser::crd::CRDSchema {
                                        openapi_v3_schema: v,
                                    }
                                })
                            })
                        }),
                    }
                })
                .collect();

            let names = amalgam_parser::crd::CRDNames {
                plural: spec.names.plural,
                singular: spec.names.singular.unwrap_or_default(),
                kind: spec.names.kind,
            };

            Ok(amalgam_parser::crd::CRD {
                api_version: "apiextensions.k8s.io/v1".to_string(),
                kind: "CustomResourceDefinition".to_string(),
                metadata,
                spec: amalgam_parser::crd::CRDSpec {
                    group: spec.group,
                    versions,
                    names,
                },
            })
        }
    }
}
