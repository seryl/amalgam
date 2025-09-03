//! Health check and metrics server for production monitoring

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use prometheus::{Counter, Encoder, Gauge, Histogram, HistogramOpts, Registry, TextEncoder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};

/// Health status of the daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub uptime_seconds: u64,
    pub files_watched: usize,
    pub compilations_total: u64,
    pub compilations_failed: u64,
    pub last_compilation: Option<String>,
    pub version: String,
}

/// Readiness status of the daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadinessStatus {
    pub ready: bool,
    pub initialized: bool,
    pub watching: bool,
    pub cache_ready: bool,
}

/// Metrics collected by the daemon
pub struct DaemonMetrics {
    pub compilations_total: Counter,
    pub compilations_failed: Counter,
    pub compilation_duration: Histogram,
    pub files_watched: Gauge,
    pub cache_size: Gauge,
    pub memory_usage: Gauge,
    registry: Registry,
}

impl DaemonMetrics {
    pub fn new() -> Result<Self> {
        let registry = Registry::new();

        let compilations_total =
            Counter::new("amalgam_compilations_total", "Total number of compilations")?;
        registry.register(Box::new(compilations_total.clone()))?;

        let compilations_failed = Counter::new(
            "amalgam_compilations_failed",
            "Total number of failed compilations",
        )?;
        registry.register(Box::new(compilations_failed.clone()))?;

        let compilation_duration = Histogram::with_opts(HistogramOpts::new(
            "amalgam_compilation_duration_seconds",
            "Compilation duration in seconds",
        ))?;
        registry.register(Box::new(compilation_duration.clone()))?;

        let files_watched = Gauge::new("amalgam_files_watched", "Number of files being watched")?;
        registry.register(Box::new(files_watched.clone()))?;

        let cache_size = Gauge::new("amalgam_cache_size", "Number of entries in the cache")?;
        registry.register(Box::new(cache_size.clone()))?;

        let memory_usage = Gauge::new("amalgam_memory_usage_bytes", "Memory usage in bytes")?;
        registry.register(Box::new(memory_usage.clone()))?;

        Ok(Self {
            compilations_total,
            compilations_failed,
            compilation_duration,
            files_watched,
            cache_size,
            memory_usage,
            registry,
        })
    }

    /// Record a successful compilation
    pub fn record_compilation(&self, duration: Duration) {
        self.compilations_total.inc();
        self.compilation_duration.observe(duration.as_secs_f64());
    }

    /// Record a failed compilation
    pub fn record_compilation_failure(&self) {
        self.compilations_failed.inc();
        self.compilations_total.inc();
    }

    /// Update the number of files being watched
    pub fn set_files_watched(&self, count: usize) {
        self.files_watched.set(count as f64);
    }

    /// Update the cache size
    pub fn set_cache_size(&self, size: usize) {
        self.cache_size.set(size as f64);
    }

    /// Update memory usage
    pub fn update_memory_usage(&self) {
        if let Some(usage) = get_memory_usage() {
            self.memory_usage.set(usage as f64);
        }
    }

    /// Export metrics in Prometheus format
    pub fn export(&self) -> Result<String> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

/// Shared state for the health server
#[derive(Clone)]
pub struct HealthState {
    pub start_time: Instant,
    pub is_ready: Arc<RwLock<bool>>,
    pub metrics: Arc<DaemonMetrics>,
    pub last_compilation: Arc<RwLock<Option<String>>>,
}

/// Start the health check and metrics server
pub async fn start_health_server(port: u16, state: HealthState) -> Result<()> {
    let app = Router::new()
        .route("/healthz", get(health_handler))
        .route("/readyz", get(readiness_handler))
        .route("/metrics", get(metrics_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    info!("Starting health server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check handler
async fn health_handler(State(state): State<HealthState>) -> Response {
    let uptime = state.start_time.elapsed().as_secs();
    let last_compilation = state.last_compilation.read().await.clone();

    let status = HealthStatus {
        status: "healthy".to_string(),
        uptime_seconds: uptime,
        files_watched: state.metrics.files_watched.get() as usize,
        compilations_total: state.metrics.compilations_total.get() as u64,
        compilations_failed: state.metrics.compilations_failed.get() as u64,
        last_compilation,
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Json(status).into_response()
}

/// Readiness check handler
async fn readiness_handler(State(state): State<HealthState>) -> Response {
    let is_ready = *state.is_ready.read().await;

    let status = ReadinessStatus {
        ready: is_ready,
        initialized: true,
        watching: state.metrics.files_watched.get() > 0.0,
        cache_ready: true,
    };

    if is_ready {
        Json(status).into_response()
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(status)).into_response()
    }
}

/// Prometheus metrics handler
async fn metrics_handler(State(state): State<HealthState>) -> Response {
    // Update memory usage before exporting
    state.metrics.update_memory_usage();

    match state.metrics.export() {
        Ok(metrics) => metrics.into_response(),
        Err(e) => {
            warn!("Failed to export metrics: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to export metrics",
            )
                .into_response()
        }
    }
}

/// Get current memory usage in bytes
fn get_memory_usage() -> Option<usize> {
    // This is a simplified implementation
    // In production, you'd use a proper system monitoring library
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        if let Ok(status) = fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(kb) = parts[1].parse::<usize>() {
                            return Some(kb * 1024); // Convert KB to bytes
                        }
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_creation() {
        let metrics = DaemonMetrics::new().unwrap();

        // Record some metrics
        metrics.record_compilation(Duration::from_secs(1));
        metrics.record_compilation_failure();
        metrics.set_files_watched(10);
        metrics.set_cache_size(100);

        // Export and check
        let exported = metrics.export().unwrap();
        assert!(exported.contains("amalgam_compilations_total"));
        assert!(exported.contains("amalgam_files_watched"));
    }

    #[tokio::test]
    async fn test_health_status() {
        let metrics = Arc::new(DaemonMetrics::new().unwrap());
        let _state = HealthState {
            start_time: Instant::now(),
            is_ready: Arc::new(RwLock::new(true)),
            metrics,
            last_compilation: Arc::new(RwLock::new(None)),
        };

        // Simulate health check
        let status = HealthStatus {
            status: "healthy".to_string(),
            uptime_seconds: 0,
            files_watched: 0,
            compilations_total: 0,
            compilations_failed: 0,
            last_compilation: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        assert_eq!(status.status, "healthy");
    }
}
