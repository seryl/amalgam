//! Daemon management commands

use amalgam_daemon::daemon::{DaemonConfig, ProductionDaemon};
use anyhow::{Context, Result};
use clap::Subcommand;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::signal;
use tracing::{error, info};

#[derive(Subcommand)]
pub enum DaemonCommand {
    /// Start the daemon
    Start {
        /// Configuration file path
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Paths to watch
        #[arg(short, long)]
        watch: Vec<PathBuf>,

        /// Output directory
        #[arg(short, long, default_value = "./generated")]
        output: PathBuf,

        /// Health check port
        #[arg(long, default_value = "8080")]
        health_port: u16,

        /// Enable Kubernetes CRD watching
        #[arg(long)]
        k8s: bool,

        /// Kubernetes namespace to watch
        #[arg(long)]
        k8s_namespace: Option<String>,

        /// Enable incremental compilation
        #[arg(long, default_value = "true")]
        incremental: bool,

        /// Log level (trace, debug, info, warn, error)
        #[arg(long, default_value = "info")]
        log_level: String,
    },

    /// Check daemon status
    Status {
        /// Health check port
        #[arg(long, default_value = "8080")]
        port: u16,
    },

    /// Reload daemon configuration
    Reload {
        /// Health check port
        #[arg(long, default_value = "8080")]
        port: u16,
    },

    /// Stop the daemon gracefully
    Stop {
        /// Health check port
        #[arg(long, default_value = "8080")]
        port: u16,
    },
}

impl DaemonCommand {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::Start {
                config,
                watch,
                output,
                health_port,
                k8s,
                k8s_namespace,
                incremental,
                log_level,
            } => {
                start_daemon(DaemonStartConfig {
                    config_path: config,
                    watch_paths: watch,
                    output_dir: output,
                    health_port,
                    enable_k8s: k8s,
                    k8s_namespace,
                    incremental,
                    log_level,
                })
                .await
            }
            Self::Status { port } => check_status(port).await,
            Self::Reload { port } => reload_daemon(port).await,
            Self::Stop { port } => stop_daemon(port).await,
        }
    }
}

/// Configuration for starting the daemon
struct DaemonStartConfig {
    config_path: Option<PathBuf>,
    watch_paths: Vec<PathBuf>,
    output_dir: PathBuf,
    health_port: u16,
    enable_k8s: bool,
    k8s_namespace: Option<String>,
    incremental: bool,
    log_level: String,
}

async fn start_daemon(config: DaemonStartConfig) -> Result<()> {
    let DaemonStartConfig {
        config_path,
        watch_paths,
        output_dir,
        health_port,
        enable_k8s,
        k8s_namespace,
        incremental,
        log_level,
    } = config;
    info!("Starting Amalgam daemon");

    // Load or create configuration
    let config = if let Some(path) = config_path {
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {:?}", path))?;
        toml::from_str(&content).with_context(|| "Failed to parse config file")?
    } else {
        // Create config from CLI arguments
        let watch_paths = if watch_paths.is_empty() {
            vec![PathBuf::from(".")]
        } else {
            watch_paths
        };

        DaemonConfig {
            watch_paths,
            output_dir,
            health_port,
            enable_k8s,
            k8s_namespace,
            incremental,
            log_level,
            ..Default::default()
        }
    };

    // Create and run daemon
    let daemon = Arc::new(ProductionDaemon::new(config)?);

    // Set up signal handler for graceful shutdown
    let _daemon_clone = daemon.clone();
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl-C");
        info!("Received shutdown signal");
        // The daemon will handle shutdown through its signal handler
    });

    // Run the daemon
    if let Err(e) = daemon.run().await {
        error!("Daemon error: {}", e);
        return Err(e);
    }

    info!("Daemon stopped");
    Ok(())
}

async fn check_status(port: u16) -> Result<()> {
    let url = format!("http://localhost:{}/healthz", port);

    info!("Checking daemon status at {}", url);

    let response = reqwest::get(&url)
        .await
        .with_context(|| format!("Failed to connect to daemon at {}", url))?;

    if response.status().is_success() {
        let status: serde_json::Value = response.json().await?;
        println!("Daemon Status:");
        println!("{}", serde_json::to_string_pretty(&status)?);
    } else {
        println!("Daemon is not responding (status: {})", response.status());
    }

    Ok(())
}

async fn reload_daemon(port: u16) -> Result<()> {
    info!("Sending reload signal to daemon on port {}", port);

    // In a real implementation, this would send a reload command
    // For now, we'll just check if the daemon is running
    let url = format!("http://localhost:{}/healthz", port);
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        println!("Daemon is running. Reload functionality not yet implemented.");
        println!("You can send SIGHUP to the daemon process to reload configuration.");
    } else {
        println!("Daemon is not responding");
    }

    Ok(())
}

async fn stop_daemon(port: u16) -> Result<()> {
    info!("Sending stop signal to daemon on port {}", port);

    // In a real implementation, this would send a shutdown command
    // For now, we'll just check if the daemon is running
    let url = format!("http://localhost:{}/healthz", port);
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        println!("Daemon is running. Stop functionality not yet implemented.");
        println!("You can send SIGTERM to the daemon process for graceful shutdown.");
    } else {
        println!("Daemon is not responding");
    }

    Ok(())
}

/// Create a default daemon configuration file
#[allow(dead_code)]
pub fn create_default_config() -> String {
    r#"# Amalgam Daemon Configuration

# Paths to watch for changes
watch_paths = ["."]

# Output directory for generated files
output_dir = "./generated"

# Health check server port
health_port = 8080

# Enable Kubernetes CRD watching
enable_k8s = false

# Kubernetes namespace to watch (null = all namespaces)
# k8s_namespace = "default"

# File extensions to watch
file_extensions = ["yaml", "yml", "json"]

# Debounce duration in milliseconds
debounce_ms = 500

# Cache size limit
cache_size = 1000

# Enable incremental compilation
incremental = true

# Log level (trace, debug, info, warn, error)
log_level = "info"
"#
    .to_string()
}
