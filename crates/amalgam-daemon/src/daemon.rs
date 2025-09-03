//! Enhanced production-ready daemon implementation

use crate::health::{start_health_server, DaemonMetrics, HealthState};
use crate::watcher::{EnhancedWatcher, FileChange, FileChangeKind, WatcherConfig};
use amalgam_codegen::{nickel::NickelCodegen, Codegen};
use amalgam_core::{compilation_unit::CompilationUnit, module_registry::ModuleRegistry};
use amalgam_parser::{crd::CRDParser, openapi::OpenAPIParser, Parser};
use anyhow::{Context, Result};
use futures::StreamExt;
use lru::LruCache;
use serde::{Deserialize, Serialize};
use signal_hook::consts::signal::{SIGHUP, SIGINT, SIGTERM};
use signal_hook_tokio::Signals;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};

/// Daemon configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    /// Paths to watch for changes
    pub watch_paths: Vec<PathBuf>,
    /// Output directory for generated files
    pub output_dir: PathBuf,
    /// Health check server port
    pub health_port: u16,
    /// Enable Kubernetes CRD watching
    pub enable_k8s: bool,
    /// K8s namespace to watch (None = all namespaces)
    pub k8s_namespace: Option<String>,
    /// File extensions to watch
    pub file_extensions: Vec<String>,
    /// Debounce duration in milliseconds
    pub debounce_ms: u64,
    /// Cache size limit
    pub cache_size: usize,
    /// Enable incremental compilation
    pub incremental: bool,
    /// Log level
    pub log_level: String,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            watch_paths: vec![PathBuf::from(".")],
            output_dir: PathBuf::from("./generated"),
            health_port: 8080,
            enable_k8s: false,
            k8s_namespace: None,
            file_extensions: vec!["yaml".to_string(), "yml".to_string(), "json".to_string()],
            debounce_ms: 500,
            cache_size: 1000,
            incremental: true,
            log_level: "info".to_string(),
        }
    }
}

/// Cache entry for compiled files
#[derive(Debug, Clone)]
struct CacheEntry {
    last_modified: SystemTime,
    _last_compiled: SystemTime,
    _checksum: String,
    _dependencies: Vec<PathBuf>,
}

/// Enhanced production daemon
pub struct ProductionDaemon {
    config: Arc<DaemonConfig>,
    cache: Arc<Mutex<LruCache<PathBuf, CacheEntry>>>,
    metrics: Arc<DaemonMetrics>,
    is_ready: Arc<RwLock<bool>>,
    last_compilation: Arc<RwLock<Option<String>>>,
    _compilation_unit: Arc<RwLock<CompilationUnit>>,
    shutdown_tx: tokio::sync::broadcast::Sender<()>,
}

impl ProductionDaemon {
    /// Create a new production daemon
    pub fn new(config: DaemonConfig) -> Result<Self> {
        let cache_size =
            NonZeroUsize::new(config.cache_size).unwrap_or(NonZeroUsize::new(1000).unwrap());

        let (shutdown_tx, _) = tokio::sync::broadcast::channel(1);

        Ok(Self {
            config: Arc::new(config),
            cache: Arc::new(Mutex::new(LruCache::new(cache_size))),
            metrics: Arc::new(DaemonMetrics::new()?),
            is_ready: Arc::new(RwLock::new(false)),
            last_compilation: Arc::new(RwLock::new(None)),
            _compilation_unit: Arc::new(RwLock::new(CompilationUnit::new(Arc::new(
                ModuleRegistry::new(),
            )))),
            shutdown_tx,
        })
    }

    /// Run the daemon
    pub async fn run(self: Arc<Self>) -> Result<()> {
        info!("Starting Amalgam production daemon");
        info!("Configuration: {:?}", self.config);

        // Create output directory
        std::fs::create_dir_all(&self.config.output_dir).with_context(|| {
            format!(
                "Failed to create output directory: {:?}",
                self.config.output_dir
            )
        })?;

        // Start health server
        let health_handle = self.start_health_server();

        // Start signal handler
        let signal_handle = self.clone().start_signal_handler();

        // Start file watcher
        let watcher_handle = self.clone().start_file_watcher();

        // Start K8s watcher if enabled
        let k8s_handle = if self.config.enable_k8s {
            Some(self.clone().start_k8s_watcher())
        } else {
            None
        };

        // Mark as ready
        *self.is_ready.write().await = true;
        info!("Daemon is ready and watching for changes");

        // Wait for shutdown or error
        tokio::select! {
            result = health_handle => {
                error!("Health server stopped: {:?}", result);
            }
            result = signal_handle => {
                info!("Signal handler stopped: {:?}", result);
            }
            result = watcher_handle => {
                error!("File watcher stopped: {:?}", result);
            }
            result = async {
                if let Some(handle) = k8s_handle {
                    match handle.await {
                        Ok(res) => res,
                        Err(e) => Err(anyhow::anyhow!("Join error: {}", e)),
                    }
                } else {
                    // Just wait forever if no K8s handle
                    std::future::pending::<Result<()>>().await
                }
            } => {
                error!("K8s watcher stopped: {:?}", result);
            }
        }

        info!("Daemon shutting down");
        Ok(())
    }

    /// Start the health check server
    fn start_health_server(self: &Arc<Self>) -> JoinHandle<Result<()>> {
        let state = HealthState {
            start_time: Instant::now(),
            is_ready: self.is_ready.clone(),
            metrics: self.metrics.clone(),
            last_compilation: self.last_compilation.clone(),
        };

        let port = self.config.health_port;

        tokio::spawn(async move { start_health_server(port, state).await })
    }

    /// Start signal handler for graceful shutdown
    fn start_signal_handler(self: Arc<Self>) -> JoinHandle<Result<()>> {
        tokio::spawn(async move {
            let mut signals = Signals::new([SIGHUP, SIGINT, SIGTERM])?;

            while let Some(signal) = signals.next().await {
                match signal {
                    SIGHUP => {
                        info!("Received SIGHUP, reloading configuration");
                        // TODO: Implement configuration reload
                    }
                    SIGINT | SIGTERM => {
                        info!("Received shutdown signal");
                        let _ = self.shutdown_tx.send(());
                        break;
                    }
                    _ => {}
                }
            }

            Ok(())
        })
    }

    /// Start file watcher
    fn start_file_watcher(self: Arc<Self>) -> JoinHandle<Result<()>> {
        let config = WatcherConfig {
            debounce_duration: Duration::from_millis(self.config.debounce_ms),
            extensions: self.config.file_extensions.clone(),
            ..Default::default()
        };

        let daemon = self.clone();
        let mut shutdown_rx = self.shutdown_tx.subscribe();

        tokio::spawn(async move {
            let mut watcher = EnhancedWatcher::new(config)?;

            // Add watch paths
            for path in &daemon.config.watch_paths {
                watcher.watch(path)?;
            }

            // Update metrics
            daemon
                .metrics
                .set_files_watched(daemon.config.watch_paths.len());

            loop {
                tokio::select! {
                    change = watcher.next_change() => {
                        if let Some(change) = change {
                            if let Err(e) = daemon.handle_file_change(change).await {
                                error!("Error handling file change: {}", e);
                            }
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        info!("File watcher shutting down");
                        break;
                    }
                }
            }

            Ok(())
        })
    }

    /// Start Kubernetes CRD watcher
    #[cfg(feature = "kubernetes")]
    fn start_k8s_watcher(self: Arc<Self>) -> JoinHandle<Result<()>> {
        use crate::k8s::EnhancedK8sWatcher;

        let daemon = self.clone();
        let mut shutdown_rx = self.shutdown_tx.subscribe();

        tokio::spawn(async move {
            let watcher = EnhancedK8sWatcher::new(
                daemon.config.output_dir.clone(),
                daemon.config.k8s_namespace.clone(),
            )
            .await?;

            tokio::select! {
                result = watcher.watch_crds() => {
                    if let Err(e) = result {
                        error!("K8s watcher error: {}", e);
                    }
                }
                _ = shutdown_rx.recv() => {
                    info!("K8s watcher shutting down");
                }
            }

            Ok(())
        })
    }

    #[cfg(not(feature = "kubernetes"))]
    fn start_k8s_watcher(self: Arc<Self>) -> JoinHandle<Result<()>> {
        tokio::spawn(async move {
            warn!("Kubernetes support not enabled");
            Ok(())
        })
    }

    /// Handle a file change event
    async fn handle_file_change(&self, change: FileChange) -> Result<()> {
        match change.kind {
            FileChangeKind::Created | FileChangeKind::Modified => {
                self.compile_file(&change.path).await?;
            }
            FileChangeKind::Removed => {
                self.handle_file_removal(&change.path).await?;
            }
            FileChangeKind::Renamed { from, to } => {
                self.handle_file_removal(&from).await?;
                self.compile_file(&to).await?;
            }
        }

        Ok(())
    }

    /// Compile a file
    async fn compile_file(&self, path: &Path) -> Result<()> {
        let start = Instant::now();
        info!("Compiling: {:?}", path);

        // Check cache if incremental compilation is enabled
        if self.config.incremental {
            let mut cache = self.cache.lock().await;
            if let Some(entry) = cache.get(path) {
                let metadata = std::fs::metadata(path)?;
                let modified = metadata.modified()?;

                if modified <= entry.last_modified {
                    debug!("File unchanged, skipping: {:?}", path);
                    return Ok(());
                }
            }
        }

        // Read file content
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {:?}", path))?;

        // Parse based on file extension
        let ir = match path.extension().and_then(|s| s.to_str()) {
            Some("yaml") | Some("yml") => {
                // Try as CRD first
                if let Ok(crd) = serde_yaml::from_str::<amalgam_parser::crd::CRD>(&content) {
                    let parser = CRDParser::new();
                    parser.parse(crd)?
                } else {
                    // Try as OpenAPI
                    let spec = serde_yaml::from_str(&content)?;
                    let parser = OpenAPIParser::new();
                    parser.parse(spec)?
                }
            }
            Some("json") => {
                // Try as OpenAPI
                let spec = serde_json::from_str(&content)?;
                let parser = OpenAPIParser::new();
                parser.parse(spec)?
            }
            _ => {
                warn!("Unsupported file type: {:?}", path);
                return Ok(());
            }
        };

        // Generate Nickel code
        let mut codegen = NickelCodegen::from_ir(&ir);
        let generated = codegen.generate(&ir)?;

        // Write output
        let output_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let output_path = self.config.output_dir.join(format!("{}.ncl", output_name));

        std::fs::write(&output_path, generated)
            .with_context(|| format!("Failed to write output: {:?}", output_path))?;

        // Update cache
        let metadata = std::fs::metadata(path)?;
        let cache_entry = CacheEntry {
            last_modified: metadata.modified()?,
            _last_compiled: SystemTime::now(),
            _checksum: format!("{:?}", metadata.len()),
            _dependencies: vec![], // TODO: Track dependencies
        };

        let mut cache = self.cache.lock().await;
        cache.put(path.to_path_buf(), cache_entry);

        // Update metrics
        let duration = start.elapsed();
        self.metrics.record_compilation(duration);
        self.metrics.set_cache_size(cache.len());

        // Update last compilation
        *self.last_compilation.write().await = Some(path.display().to_string());

        info!("Compiled {:?} in {:?}", path, duration);
        Ok(())
    }

    /// Handle file removal
    async fn handle_file_removal(&self, path: &Path) -> Result<()> {
        info!("File removed: {:?}", path);

        // Remove from cache
        let mut cache = self.cache.lock().await;
        cache.pop(path);

        // Remove generated file
        let output_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let output_path = self.config.output_dir.join(format!("{}.ncl", output_name));

        if output_path.exists() {
            std::fs::remove_file(&output_path)
                .with_context(|| format!("Failed to remove output: {:?}", output_path))?;
            info!("Removed generated file: {:?}", output_path);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_daemon_creation() {
        let config = DaemonConfig::default();
        let daemon = ProductionDaemon::new(config).unwrap();
        assert!(!*daemon.is_ready.read().await);
    }

    #[tokio::test]
    async fn test_cache_operations() {
        let temp_dir = TempDir::new().unwrap();
        let config = DaemonConfig {
            output_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        let daemon = Arc::new(ProductionDaemon::new(config).unwrap());

        // Create a valid OpenAPI test file
        let test_file = temp_dir.path().join("test.json");
        let openapi_spec = r#"{
            "openapi": "3.0.0",
            "info": {
                "title": "Test API",
                "version": "1.0.0"
            },
            "paths": {}
        }"#;
        std::fs::write(&test_file, openapi_spec).unwrap();

        // Compile it
        daemon.compile_file(&test_file).await.unwrap();

        // Check cache
        let cache = daemon.cache.lock().await;
        assert!(cache.contains(&test_file));
    }
}
