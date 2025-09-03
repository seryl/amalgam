//! Source auto-detection and domain inference
//!
//! This module provides intelligent detection of source types and
//! automatic extraction of domains from source content.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Detected source type with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetectedSource {
    /// OpenAPI/Swagger specification
    OpenAPI {
        url: String,
        /// Inferred domain from definitions
        domain: Option<String>,
        /// Detected version from spec
        version: Option<String>,
    },
    /// Kubernetes CRD(s)
    CRDs {
        urls: Vec<String>,
        /// Domain from spec.group
        domain: Option<String>,
        /// Versions found in CRDs
        versions: Vec<String>,
    },
    /// Go source code
    GoSource {
        path: String,
        /// Domain from +groupName annotation
        domain: Option<String>,
        /// Package version
        version: Option<String>,
    },
    /// Unknown source type
    Unknown { source: String },
}

impl DetectedSource {
    /// Get the domain from the detected source
    pub fn domain(&self) -> Option<&str> {
        match self {
            DetectedSource::OpenAPI { domain, .. } => domain.as_deref(),
            DetectedSource::CRDs { domain, .. } => domain.as_deref(),
            DetectedSource::GoSource { domain, .. } => domain.as_deref(),
            DetectedSource::Unknown { .. } => None,
        }
    }

    /// Get the inferred package name (domain with dots replaced)
    pub fn package_name(&self) -> String {
        self.domain()
            .map(|d| d.replace('.', "_"))
            .unwrap_or_else(|| "unknown".to_string())
    }
}

/// Detect source type and extract metadata
pub async fn detect_source(source: &str) -> Result<DetectedSource> {
    debug!("Detecting source type for: {}", source);

    // Fetch or read the content
    let content = fetch_content(source)
        .await
        .with_context(|| format!("Failed to fetch source: {}", source))?;

    // Try to detect based on content
    if let Some(detected) = detect_openapi(&content, source) {
        info!("Detected OpenAPI/Swagger source");
        return Ok(detected);
    }

    if let Some(detected) = detect_crd(&content, source) {
        info!("Detected Kubernetes CRD source");
        return Ok(detected);
    }

    if source.ends_with(".go") {
        if let Some(detected) = detect_go_source(&content, source) {
            info!("Detected Go source");
            return Ok(detected);
        }
    }

    // Unknown source type
    Ok(DetectedSource::Unknown {
        source: source.to_string(),
    })
}

/// Fetch content from URL or file
async fn fetch_content(source: &str) -> Result<String> {
    if source.starts_with("http://") || source.starts_with("https://") {
        // Fetch from URL
        let response = reqwest::get(source)
            .await
            .with_context(|| format!("Failed to fetch URL: {}", source))?;

        response
            .text()
            .await
            .with_context(|| format!("Failed to read response from: {}", source))
    } else if source.starts_with("file://") {
        // Local file with file:// prefix
        let path = source.strip_prefix("file://").unwrap();
        std::fs::read_to_string(path).with_context(|| format!("Failed to read file: {}", path))
    } else {
        // Assume local file path
        std::fs::read_to_string(source).with_context(|| format!("Failed to read file: {}", source))
    }
}

/// Detect OpenAPI/Swagger and extract domain
fn detect_openapi(content: &str, source: &str) -> Option<DetectedSource> {
    // Check for OpenAPI/Swagger markers
    if !content.contains("\"swagger\"") && !content.contains("\"openapi\"") {
        return None;
    }

    // Try to parse as JSON and extract domain from definitions
    let domain = extract_openapi_domain(content);
    let version = extract_openapi_version(content);

    Some(DetectedSource::OpenAPI {
        url: source.to_string(),
        domain,
        version,
    })
}

/// Extract domain from OpenAPI definitions
fn extract_openapi_domain(content: &str) -> Option<String> {
    // Parse JSON and look for definitions like "io.k8s.api.core.v1.Pod"
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
        if let Some(definitions) = json.get("definitions").and_then(|d| d.as_object()) {
            // Look for the first definition with a domain pattern
            for key in definitions.keys() {
                if let Some(domain) = extract_domain_from_definition(key) {
                    return Some(domain);
                }
            }
        }
    }
    None
}

/// Extract domain from a definition key like "io.k8s.api.core.v1.Pod"
fn extract_domain_from_definition(key: &str) -> Option<String> {
    let parts: Vec<&str> = key.split('.').collect();

    // Look for patterns like io.k8s.api.* or io.k8s.apimachinery.*
    if parts.len() >= 3 && parts[0] == "io" && parts[1] == "k8s" {
        // Kubernetes types - normalize to k8s.io
        return Some("k8s.io".to_string());
    }

    // Look for other domain patterns (e.g., com.example.api.v1.Type)
    if parts.len() >= 3 {
        // Check if first parts look like a domain
        if parts[0].len() >= 2 && parts[1].len() >= 2 {
            // Take first two parts as domain (e.g., "example.com")
            let domain = format!("{}.{}", parts[0], parts[1]);
            // Reverse if it looks like a reversed domain
            if parts[0] == "com" || parts[0] == "org" || parts[0] == "io" || parts[0] == "net" {
                return Some(format!("{}.{}", parts[1], parts[0]));
            }
            return Some(domain);
        }
    }

    None
}

/// Extract version from OpenAPI spec
fn extract_openapi_version(content: &str) -> Option<String> {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
        // Try to get version from info.version
        if let Some(version) = json
            .get("info")
            .and_then(|i| i.get("version"))
            .and_then(|v| v.as_str())
        {
            return Some(version.to_string());
        }
    }
    None
}

/// Detect Kubernetes CRD and extract domain
fn detect_crd(content: &str, source: &str) -> Option<DetectedSource> {
    // Check for CRD markers
    if !content.contains("kind: CustomResourceDefinition")
        && !content.contains("kind: \"CustomResourceDefinition\"")
    {
        return None;
    }

    // Extract domain from spec.group
    let domain = extract_crd_domain(content);
    let versions = extract_crd_versions(content);

    Some(DetectedSource::CRDs {
        urls: vec![source.to_string()],
        domain,
        versions,
    })
}

/// Extract domain from CRD spec.group
fn extract_crd_domain(content: &str) -> Option<String> {
    // Try to parse as YAML
    if let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(content) {
        // Handle both single CRD and List of CRDs
        let crds = if yaml
            .get("kind")
            .and_then(|k| k.as_str())
            .map(|k| k == "CustomResourceDefinition")
            .unwrap_or(false)
        {
            vec![&yaml]
        } else if yaml.get("items").is_some() {
            yaml.get("items")
                .and_then(|i| i.as_sequence())
                .map(|items| items.iter().collect())
                .unwrap_or_default()
        } else {
            vec![]
        };

        // Get group from first CRD
        for crd in crds {
            if let Some(group) = crd
                .get("spec")
                .and_then(|s| s.get("group"))
                .and_then(|g| g.as_str())
            {
                return Some(group.to_string());
            }
        }
    }
    None
}

/// Extract versions from CRD
fn extract_crd_versions(content: &str) -> Vec<String> {
    let mut versions = Vec::new();

    if let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(content) {
        // Handle both single CRD and List
        let crds = if yaml
            .get("kind")
            .and_then(|k| k.as_str())
            .map(|k| k == "CustomResourceDefinition")
            .unwrap_or(false)
        {
            vec![&yaml]
        } else if yaml.get("items").is_some() {
            yaml.get("items")
                .and_then(|i| i.as_sequence())
                .map(|items| items.iter().collect())
                .unwrap_or_default()
        } else {
            vec![]
        };

        for crd in crds {
            if let Some(crd_versions) = crd
                .get("spec")
                .and_then(|s| s.get("versions"))
                .and_then(|v| v.as_sequence())
            {
                for version in crd_versions {
                    if let Some(name) = version.get("name").and_then(|n| n.as_str()) {
                        if !versions.contains(&name.to_string()) {
                            versions.push(name.to_string());
                        }
                    }
                }
            }
        }
    }

    versions
}

/// Detect Go source and extract domain
fn detect_go_source(content: &str, source: &str) -> Option<DetectedSource> {
    // Look for +groupName annotation
    let domain = extract_go_domain(content);
    let version = extract_go_version(content);

    Some(DetectedSource::GoSource {
        path: source.to_string(),
        domain,
        version,
    })
}

/// Extract domain from Go +groupName annotation
fn extract_go_domain(content: &str) -> Option<String> {
    for line in content.lines() {
        if line.contains("+groupName=") {
            // Extract value after +groupName=
            if let Some(start) = line.find("+groupName=") {
                let value_start = start + "+groupName=".len();
                let value = &line[value_start..];
                // Take until whitespace or end of line
                let domain = value.split_whitespace().next()?;
                return Some(domain.to_string());
            }
        }
        // Also check for +kubebuilder:rbac annotations that might contain group info
        if line.contains("+kubebuilder:rbac:groups=") {
            if let Some(start) = line.find("groups=") {
                let value_start = start + "groups=".len();
                let value = &line[value_start..];
                // Take until comma or end
                let domain = value.split(',').next()?.trim();
                if !domain.is_empty() {
                    return Some(domain.to_string());
                }
            }
        }
    }
    None
}

/// Extract version from Go package name
fn extract_go_version(content: &str) -> Option<String> {
    for line in content.lines() {
        if line.starts_with("package ") {
            let package_name = line.strip_prefix("package ")?.trim();
            // Check if package name is a version (v1, v1beta1, etc.)
            if package_name.starts_with('v')
                && package_name.len() > 1
                && package_name.chars().nth(1).unwrap().is_ascii_digit()
            {
                return Some(package_name.to_string());
            }
        }
    }
    None
}

/// Detect sources from a GitHub directory listing
pub async fn detect_github_directory(url: &str) -> Result<Vec<String>> {
    // Convert GitHub web URL to API URL
    let api_url = convert_github_url_to_api(url)?;

    // Fetch directory contents
    let client = reqwest::Client::new();
    let response = client
        .get(&api_url)
        .header("User-Agent", "amalgam")
        .send()
        .await?;

    let contents: Vec<GitHubContent> = response.json().await?;

    // Filter for YAML files that might be CRDs
    let crd_urls: Vec<String> = contents
        .iter()
        .filter(|c| c.name.ends_with(".yaml") || c.name.ends_with(".yml"))
        .map(|c| c.download_url.clone())
        .collect();

    Ok(crd_urls)
}

#[derive(Debug, Deserialize)]
struct GitHubContent {
    name: String,
    download_url: String,
}

/// Convert GitHub web URL to API URL
fn convert_github_url_to_api(url: &str) -> Result<String> {
    // Convert https://github.com/owner/repo/tree/branch/path
    // to https://api.github.com/repos/owner/repo/contents/path?ref=branch

    if !url.contains("github.com") {
        return Ok(url.to_string());
    }

    let parts: Vec<&str> = url.split('/').collect();
    if parts.len() < 7 {
        return Ok(url.to_string());
    }

    let owner = parts[3];
    let repo = parts[4];
    let branch = parts[6];
    let path = parts[7..].join("/");

    Ok(format!(
        "https://api.github.com/repos/{}/{}/contents/{}?ref={}",
        owner, repo, path, branch
    ))
}
