//! Manifest-based package generation for CI/CD workflows

use amalgam_core::compilation_unit::CompilationUnit;
use amalgam_core::special_cases::{SpecialCasePipeline, WithSpecialCases};

// Define DetectedSource locally to avoid import issues
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DetectedSource {
    OpenAPI {
        url: String,
        domain: Option<String>,
        version: Option<String>,
    },
    CRDs {
        urls: Vec<String>,
        domain: Option<String>,
        versions: Vec<String>,
    },
    GoSource {
        path: String,
        domain: Option<String>,
        version: Option<String>,
    },
    Unknown {
        source: String,
    },
    MultiDomainCRDs {
        domains_to_urls: std::collections::HashMap<String, Vec<String>>,
        source_url: String,
    },
}

impl DetectedSource {
    pub fn domain(&self) -> Option<&str> {
        match self {
            DetectedSource::OpenAPI { domain, .. } => domain.as_deref(),
            DetectedSource::CRDs { domain, .. } => domain.as_deref(),
            DetectedSource::GoSource { domain, .. } => domain.as_deref(),
            DetectedSource::Unknown { .. } => None,
            DetectedSource::MultiDomainCRDs { .. } => None, // Multi-domain doesn't have a single domain
        }
    }
}

// Enhanced source detection with actual content parsing
async fn simple_detect_source(source: &str) -> Result<DetectedSource> {
    info!("Detecting source type for: {}", source);

    // Handle GitHub directory URLs specially
    if source.contains("github.com") && source.contains("/tree/") {
        return detect_github_directory(source).await;
    }

    // Fetch content from URL or file
    let content = fetch_content(source).await?;

    // Try to detect based on content
    if let Some(detected) = detect_openapi(&content, source) {
        info!(
            "Detected OpenAPI/Swagger source with domain: {:?}",
            detected.domain()
        );
        return Ok(detected);
    }

    if let Some(detected) = detect_crd(&content, source) {
        info!(
            "Detected Kubernetes CRD source with domain: {:?}",
            detected.domain()
        );
        return Ok(detected);
    }

    if source.ends_with(".go") {
        if let Some(detected) = detect_go_source(&content, source) {
            info!("Detected Go source with domain: {:?}", detected.domain());
            return Ok(detected);
        }
    }

    // Unknown source type
    warn!("Could not detect source type for: {}", source);
    Ok(DetectedSource::Unknown {
        source: source.to_string(),
    })
}

/// Detect sources from a GitHub directory listing
/// Detect GitHub directory and return all domains found (for expansion)
async fn detect_github_directory_for_expansion(
    url: &str,
) -> Result<std::collections::HashMap<String, Vec<String>>> {
    info!("Detecting all domains in GitHub directory: {}", url);

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

    // Group CRD files by domain
    let mut domains_to_urls: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for content_item in contents.iter() {
        if content_item.name.ends_with(".yaml") || content_item.name.ends_with(".yml") {
            // Try to extract domain from filename first (e.g., "apiextensions.crossplane.io_compositions.yaml")
            let domain = if let Some(underscore_pos) = content_item.name.find('_') {
                let potential_domain = &content_item.name[..underscore_pos];
                if potential_domain.contains('.') {
                    Some(potential_domain.to_string())
                } else {
                    // Fetch content to get actual domain
                    if let Ok(crd_content) = fetch_content(&content_item.download_url).await {
                        extract_crd_domain(&crd_content)
                    } else {
                        None
                    }
                }
            } else {
                // Fetch content to get actual domain
                if let Ok(crd_content) = fetch_content(&content_item.download_url).await {
                    extract_crd_domain(&crd_content)
                } else {
                    None
                }
            };

            if let Some(domain) = domain {
                domains_to_urls
                    .entry(domain)
                    .or_default()
                    .push(content_item.download_url.clone());
            }
        }
    }

    info!(
        "Found {} domains in GitHub directory",
        domains_to_urls.len()
    );
    for (domain, urls) in &domains_to_urls {
        info!("  - {}: {} CRD files", domain, urls.len());
    }

    Ok(domains_to_urls)
}

async fn detect_github_directory(url: &str) -> Result<DetectedSource> {
    info!("Detecting GitHub directory: {}", url);

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

    // Group CRD files by domain
    let mut domains_to_urls: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for content_item in contents.iter() {
        if content_item.name.ends_with(".yaml") || content_item.name.ends_with(".yml") {
            // Try to extract domain from filename first (e.g., "apiextensions.crossplane.io_compositions.yaml")
            let domain = if let Some(underscore_pos) = content_item.name.find('_') {
                let potential_domain = &content_item.name[..underscore_pos];
                if potential_domain.contains('.') {
                    Some(potential_domain.to_string())
                } else {
                    // Fetch content to get actual domain
                    if let Ok(crd_content) = fetch_content(&content_item.download_url).await {
                        extract_crd_domain(&crd_content)
                    } else {
                        None
                    }
                }
            } else {
                // Fetch content to get actual domain
                if let Ok(crd_content) = fetch_content(&content_item.download_url).await {
                    extract_crd_domain(&crd_content)
                } else {
                    None
                }
            };

            if let Some(domain) = domain {
                domains_to_urls
                    .entry(domain)
                    .or_default()
                    .push(content_item.download_url.clone());
            }
        }
    }

    if domains_to_urls.is_empty() {
        warn!("No CRD files found in GitHub directory: {}", url);
        return Ok(DetectedSource::Unknown {
            source: url.to_string(),
        });
    }

    // If we have multiple domains, we'll still return the first one here
    // but we'll handle expansion in a separate function
    let (first_domain, first_urls) = domains_to_urls
        .iter()
        .next()
        .map(|(d, u)| (d.clone(), u.clone()))
        .ok_or_else(|| anyhow::anyhow!("No domains found"))?;

    info!(
        "Detected GitHub directory with CRDs for {} domains (returning first: {})",
        domains_to_urls.len(),
        first_domain
    );

    // Store all domains in a special marker that we'll expand later
    if domains_to_urls.len() > 1 {
        // For multi-domain sources, we'll handle this specially
        Ok(DetectedSource::MultiDomainCRDs {
            domains_to_urls,
            source_url: url.to_string(),
        })
    } else {
        Ok(DetectedSource::CRDs {
            urls: first_urls,
            domain: Some(first_domain),
            versions: vec!["v1".to_string()], // Default version
        })
    }
}

#[derive(Debug, serde::Deserialize)]
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

/// Fetch content from URL or file
async fn fetch_content(source: &str) -> Result<String> {
    if source.starts_with("http://") || source.starts_with("https://") {
        info!("Fetching content from URL: {}", source);
        let response = reqwest::get(source)
            .await
            .with_context(|| format!("Failed to fetch URL: {}", source))?;

        response
            .text()
            .await
            .with_context(|| format!("Failed to read response from: {}", source))
    } else if source.starts_with("file://") {
        let path = source.strip_prefix("file://").unwrap();
        std::fs::read_to_string(path).with_context(|| format!("Failed to read file: {}", path))
    } else {
        std::fs::read_to_string(source).with_context(|| format!("Failed to read file: {}", source))
    }
}

/// Detect OpenAPI/Swagger and extract domain
fn detect_openapi(content: &str, source: &str) -> Option<DetectedSource> {
    if !content.contains("\"swagger\"") && !content.contains("\"openapi\"") {
        return None;
    }

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
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
        if let Some(definitions) = json.get("definitions").and_then(|d| d.as_object()) {
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

    if parts.len() >= 3 && parts[0] == "io" && parts[1] == "k8s" {
        return Some("k8s.io".to_string());
    }

    if parts.len() >= 3 && parts[0].len() >= 2 && parts[1].len() >= 2 {
        let domain = format!("{}.{}", parts[0], parts[1]);
        if parts[0] == "com" || parts[0] == "org" || parts[0] == "io" || parts[0] == "net" {
            return Some(format!("{}.{}", parts[1], parts[0]));
        }
        return Some(domain);
    }

    None
}

/// Extract version from OpenAPI spec
fn extract_openapi_version(content: &str) -> Option<String> {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
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
    if !content.contains("kind: CustomResourceDefinition")
        && !content.contains("kind: \"CustomResourceDefinition\"")
    {
        return None;
    }

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
    if let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(content) {
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
            if let Some(start) = line.find("+groupName=") {
                let value_start = start + "+groupName=".len();
                let value = &line[value_start..];
                let domain = value.split_whitespace().next()?;
                return Some(domain.to_string());
            }
        }
        if line.contains("+kubebuilder:rbac:groups=") {
            if let Some(start) = line.find("groups=") {
                let value_start = start + "groups=".len();
                let value = &line[value_start..];
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

use amalgam_core::module_registry::ModuleRegistry;
use amalgam_parser::Parser as SchemaParser;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{info, warn};

/// Main manifest configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    /// Global configuration
    pub config: ManifestConfig,

    /// List of packages to generate
    pub packages: Vec<PackageDefinition>,
}

/// Global configuration for manifest
#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestConfig {
    /// Base output directory for all packages
    pub output_base: PathBuf,

    /// Enable package mode by default
    #[serde(default = "default_true")]
    pub package_mode: bool,

    /// Base package ID for dependencies (e.g., "github:seryl/nickel-pkgs")
    pub base_package_id: String,

    /// Local package path prefix for development (e.g., "examples/pkgs")
    /// When set, generates Path dependencies instead of Index dependencies
    #[serde(default)]
    pub local_package_prefix: Option<String>,
}

/// Simplified package source definition
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum PackageSource {
    /// Single source URL/path
    Single(String),
    /// Multiple sources that should be merged into one package
    Multiple(Vec<String>),
}

/// Definition of a package to generate - NEW SIMPLIFIED VERSION
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PackageDefinition {
    /// Source(s) to fetch types from - URL(s) or path(s)
    pub source: PackageSource,

    /// Optional domain override (usually inferred from source)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Optional name override (usually inferred from domain)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional description for documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether this package is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
}

/// Legacy package definition for backwards compatibility
#[derive(Debug, Deserialize, Serialize)]
pub struct LegacyPackageDefinition {
    /// Package name
    pub name: String,

    /// Type of source (k8s-core, url, crd, openapi)
    #[serde(rename = "type")]
    pub source_type: SourceType,

    /// Version (for k8s-core and package versioning)
    pub version: Option<String>,

    /// URL (for url type)
    pub url: Option<String>,

    /// Git ref (tag, branch, or commit) for URL sources
    pub git_ref: Option<String>,

    /// File path (for crd/openapi types)
    pub file: Option<PathBuf>,

    /// Output directory name
    pub output: String,

    /// Package description
    pub description: String,

    /// Keywords for package discovery
    pub keywords: Vec<String>,

    /// Dependencies on other packages with version constraints
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,

    /// Whether this package is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Directory structure type for import path calculation
    #[serde(default)]
    pub directory_structure: Option<DirectoryStructure>,
}

/// Dependency specification with version constraints
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum DependencySpec {
    /// Simple string version (for backwards compatibility)
    Simple(String),
    /// Full specification with version constraints
    Full {
        version: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        min_version: Option<String>,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum SourceType {
    K8sCore,
    Url,
    Crd,
    OpenApi,
}

impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceType::K8sCore => write!(f, "k8s-core"),
            SourceType::Url => write!(f, "url"),
            SourceType::Crd => write!(f, "crd"),
            SourceType::OpenApi => write!(f, "openapi"),
        }
    }
}

/// Directory structure for generated packages
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum DirectoryStructure {
    /// Uses version subdirectories: pkgs/package/version/file.ncl
    #[default]
    Versioned,
    /// Uses nested API groups without version subdirs: pkgs/package/api.group/subdir/file.ncl
    Nested,
}

fn default_true() -> bool {
    true
}

impl PackageDefinition {
    /// Convert to a normalized internal representation
    pub async fn normalize(&self) -> Result<NormalizedPackage> {
        // Get all source URLs
        let sources = match &self.source {
            PackageSource::Single(s) => vec![s.clone()],
            PackageSource::Multiple(s) => s.clone(),
        };

        // Detect source types and extract metadata
        let mut detected_sources = Vec::new();
        for source in &sources {
            let detected = simple_detect_source(source).await?;
            detected_sources.push(detected);
        }

        // Extract domain (should be consistent across all sources)
        let domain = self
            .domain
            .clone()
            .or_else(|| {
                detected_sources
                    .iter()
                    .find_map(|s| s.domain().map(|d| d.to_string()))
            })
            .unwrap_or_else(|| "local".to_string());

        // Generate package name from domain
        let name = self
            .name
            .clone()
            .unwrap_or_else(|| domain.replace('.', "_"));

        Ok(NormalizedPackage {
            name,
            domain,
            sources: detected_sources,
            description: self.description.clone(),
            enabled: self.enabled,
        })
    }
}

/// Normalized package with all inferred information
#[derive(Debug)]
pub struct NormalizedPackage {
    pub name: String,
    pub domain: String,
    pub sources: Vec<DetectedSource>,
    pub description: Option<String>,
    pub enabled: bool,
}

impl NormalizedPackage {
    /// Get the output path for this package using universal algorithm
    pub fn output_path(&self, base: &Path) -> PathBuf {
        // Universal algorithm: domain with dots replaced by underscores
        let domain_path = self.domain.replace('.', "_");
        base.join(domain_path)
    }
}

impl Manifest {
    /// Load manifest from file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read manifest file: {}", path.display()))?;

        toml::from_str(&content)
            .with_context(|| format!("Failed to parse manifest file: {}", path.display()))
    }

    /// Expand packages with multi-domain sources into separate packages
    async fn expand_multi_domain_packages(&self) -> Result<Vec<PackageDefinition>> {
        let mut expanded = Vec::new();

        for package in &self.packages {
            // First detect the source to see if it's multi-domain
            let sources = match &package.source {
                PackageSource::Single(s) => vec![s.clone()],
                PackageSource::Multiple(s) => s.clone(),
            };

            let mut is_multi_domain = false;
            let mut domains_to_urls = std::collections::HashMap::new();

            for source in &sources {
                if source.contains("github.com") && source.contains("/tree/") {
                    // Detect GitHub directory to check for multiple domains
                    if let Ok(detected) = detect_github_directory_for_expansion(source).await {
                        if detected.len() > 1 {
                            is_multi_domain = true;
                            for (domain, urls) in detected {
                                domains_to_urls
                                    .entry(domain)
                                    .or_insert_with(Vec::new)
                                    .extend(urls);
                            }
                        }
                    }
                }
            }

            if is_multi_domain {
                info!(
                    "Expanding multi-domain package with {} domains",
                    domains_to_urls.len()
                );
                // Create separate packages for each domain
                for (domain, urls) in domains_to_urls {
                    let mut new_package = package.clone();
                    new_package.name = Some(domain.replace('.', "_"));
                    new_package.domain = Some(domain.clone());
                    new_package.source = PackageSource::Multiple(urls);
                    new_package.description = Some(format!(
                        "{} CRDs for domain {}",
                        package.description.as_deref().unwrap_or("Generated"),
                        domain
                    ));
                    expanded.push(new_package);
                }
            } else {
                // Keep the original package
                expanded.push(package.clone());
            }
        }

        Ok(expanded)
    }

    /// Generate all packages defined in the manifest
    pub async fn generate_all(&self) -> Result<GenerationReport> {
        let mut report = GenerationReport::default();

        // Create output base directory
        fs::create_dir_all(&self.config.output_base).with_context(|| {
            format!(
                "Failed to create output directory: {}",
                self.config.output_base.display()
            )
        })?;

        // First, perform smart cleanup of removed packages
        self.cleanup_removed_packages(&mut report)?;

        // Read manifest content for fingerprinting
        let manifest_content =
            std::fs::read_to_string(".amalgam-manifest.toml").unwrap_or_else(|_| String::new());

        // Expand packages with multi-domain sources
        let expanded_packages = self.expand_multi_domain_packages().await?;

        for package in &expanded_packages {
            // Normalize package to get all inferred information
            let normalized = match package.normalize().await {
                Ok(n) => n,
                Err(e) => {
                    warn!("Failed to normalize package: {}", e);
                    report.failed.push(("unknown".to_string(), e.to_string()));
                    continue;
                }
            };

            if !normalized.enabled {
                info!("Skipping disabled package: {}", normalized.name);
                report.skipped.push(normalized.name.clone());
                continue;
            }

            info!(
                "Generating package: {} (domain: {})",
                normalized.name, normalized.domain
            );

            match self
                .generate_normalized_package(&normalized, &manifest_content)
                .await
            {
                Ok(output_path) => {
                    info!(
                        "✓ Successfully generated {} at {:?}",
                        normalized.name, output_path
                    );
                    report.successful.push(normalized.name.clone());
                }
                Err(e) => {
                    warn!("✗ Failed to generate {}: {}", normalized.name, e);
                    report.failed.push((normalized.name.clone(), e.to_string()));
                }
            }
        }

        Ok(report)
    }

    /// Generate a normalized package
    async fn generate_normalized_package(
        &self,
        normalized: &NormalizedPackage,
        _manifest_content: &str,
    ) -> Result<PathBuf> {
        let output_path = normalized.output_path(&self.config.output_base);

        // Process based on detected source types
        for source in &normalized.sources {
            match source {
                DetectedSource::OpenAPI { url, .. } => {
                    info!("Processing OpenAPI source: {}", url);
                    self.generate_from_openapi_url(url, &output_path).await?;
                }
                DetectedSource::CRDs { urls, .. } => {
                    info!("Processing {} CRD sources", urls.len());
                    // For multiple CRDs, we need to collect them all first
                    // and organize by group/version
                    self.generate_from_multiple_crds(urls, &output_path).await?;
                }
                DetectedSource::GoSource { path, .. } => {
                    info!("Processing Go source: {}", path);
                    // TODO: Implement Go source processing
                    anyhow::bail!("Go source processing not yet implemented");
                }
                DetectedSource::Unknown { source } => {
                    warn!("Unknown source type: {}", source);
                    anyhow::bail!("Unable to determine source type for: {}", source);
                }
                DetectedSource::MultiDomainCRDs { .. } => {
                    // This should have been expanded earlier
                    warn!("MultiDomainCRDs should have been expanded");
                    anyhow::bail!("MultiDomainCRDs should have been expanded at package level");
                }
            }
        }

        // Generate package manifest if needed
        if self.config.package_mode {
            self.generate_normalized_package_manifest(normalized, &output_path)?;
        }

        Ok(output_path)
    }

    /// Clean up packages that were removed from manifest
    fn cleanup_removed_packages(&self, _report: &mut GenerationReport) -> Result<()> {
        // This would clean up packages that exist on disk but are no longer in manifest
        // For now, we'll skip this functionality
        Ok(())
    }

    /// Write generated files to disk from concatenated output
    fn write_generated_files(&self, generated: &str, output: &Path) -> Result<()> {
        // Parse the generated string and split into individual files
        // The generated string contains module boundaries that we need to parse

        // First, write the consolidated file for debugging
        let output_file = output.join("generated.ncl");
        fs::write(&output_file, generated)?;
        info!(
            "Generated consolidated Nickel code at: {}",
            output_file.display()
        );

        // Split the generated content by module markers
        let modules = self.split_modules_by_marker(generated)?;

        // Write each module to its own file
        for (module_name, module_content) in modules {
            if let Some(file_path) = self.map_module_to_file_path(&module_name, output) {
                // Create directory if it doesn't exist
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                // Write the module content
                fs::write(&file_path, module_content)?;
                info!(
                    "Generated module {} at: {}",
                    module_name,
                    file_path.display()
                );
            } else {
                warn!("Could not map module {} to file path", module_name);
            }
        }

        Ok(())
    }

    /// Split generated content by module markers
    fn split_modules_by_marker(&self, generated: &str) -> Result<Vec<(String, String)>> {
        let mut modules = Vec::new();
        let lines: Vec<&str> = generated.lines().collect();
        let mut current_module: Option<String> = None;
        let mut current_content = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];

            // Check if this is a module marker line
            if line.starts_with("# Module: ") {
                // Save the previous module if we have one
                if let Some(module_name) = current_module.take() {
                    let content = current_content.join("\n");
                    if !content.trim().is_empty() {
                        modules.push((module_name, content));
                    }
                    current_content.clear();
                }

                // Extract the new module name
                let module_name = line
                    .strip_prefix("# Module: ")
                    .unwrap_or("")
                    .trim()
                    .to_string();
                current_module = Some(module_name);

                // Include the module marker in the content
                current_content.push(line);
            } else {
                // Add line to current module content
                current_content.push(line);
            }

            i += 1;
        }

        // Don't forget the last module
        if let Some(module_name) = current_module {
            let content = current_content.join("\n");
            if !content.trim().is_empty() {
                modules.push((module_name, content));
            }
        }

        Ok(modules)
    }

    /// Map module name to appropriate file path within the package structure
    fn map_module_to_file_path(&self, module_name: &str, output: &Path) -> Option<PathBuf> {
        // Handle different module name patterns
        match module_name {
            // Core k8s.io modules with API groups
            name if name.starts_with("k8s.io.") => {
                let suffix = name.strip_prefix("k8s.io.").unwrap();
                let parts: Vec<&str> = suffix.split('.').collect();

                if parts.len() == 1 && parts[0].starts_with('v') {
                    // Simple version like k8s.io.v1 -> api/core/v1.ncl
                    Some(
                        output
                            .join("api")
                            .join("core")
                            .join(format!("{}.ncl", parts[0])),
                    )
                } else if parts.len() >= 2 {
                    // API group with version like k8s.io.authentication.v1 -> api/authentication/v1.ncl
                    let api_group = parts[0];
                    let version = parts[parts.len() - 1];
                    Some(
                        output
                            .join("api")
                            .join(api_group)
                            .join(format!("{}.ncl", version)),
                    )
                } else {
                    // Fallback for unexpected patterns
                    Some(output.join(format!("{}.ncl", suffix)))
                }
            }

            // Apimachinery meta types with version (e.g., apimachinery.pkg.apis.meta.v1)
            name if name.starts_with("apimachinery.pkg.apis.meta.") => {
                let version = name.strip_prefix("apimachinery.pkg.apis.meta.").unwrap();
                Some(
                    output
                        .join("apimachinery.pkg.apis")
                        .join("meta")
                        .join(version)
                        .join("mod.ncl"),
                )
            }

            // Apimachinery runtime types with version
            name if name.starts_with("apimachinery.pkg.apis.runtime.") => {
                let version = name.strip_prefix("apimachinery.pkg.apis.runtime.").unwrap();
                Some(
                    output
                        .join("apimachinery.pkg.apis")
                        .join("runtime")
                        .join(version)
                        .join("mod.ncl"),
                )
            }

            // Legacy apimachinery runtime types (backward compatibility)
            "apimachinery.pkg.runtime" => Some(output.join("apimachinery.pkg").join("runtime.ncl")),

            // Apimachinery utility types
            "apimachinery.pkg.util.intstr" => Some(
                output
                    .join("apimachinery.pkg")
                    .join("util")
                    .join("intstr.ncl"),
            ),

            // Apimachinery API resource types
            "apimachinery.pkg.api.resource" => Some(
                output
                    .join("apimachinery.pkg")
                    .join("api")
                    .join("resource.ncl"),
            ),

            // APIExtensions server types
            name if name.starts_with("apiextensions-apiserver.pkg.apis.") => {
                let suffix = name
                    .strip_prefix("apiextensions-apiserver.pkg.apis.")
                    .unwrap();
                let parts: Vec<&str> = suffix.split('.').collect();
                if parts.len() >= 2 {
                    let group = parts[0];
                    let version = parts[1];
                    Some(
                        output
                            .join("apiextensions-apiserver.pkg.apis")
                            .join(group)
                            .join(format!("{}.ncl", version)),
                    )
                } else {
                    None
                }
            }

            // Kube aggregator types
            name if name.starts_with("kube-aggregator.pkg.apis.") => {
                let suffix = name.strip_prefix("kube-aggregator.pkg.apis.").unwrap();
                let parts: Vec<&str> = suffix.split('.').collect();
                if parts.len() >= 2 {
                    let group = parts[0];
                    let version = parts[1];
                    Some(
                        output
                            .join("kube-aggregator.pkg.apis")
                            .join(group)
                            .join(format!("{}.ncl", version)),
                    )
                } else {
                    None
                }
            }

            // Version module (special case)
            "k8s.io.version" => Some(output.join("version.ncl")),

            // Fallback: create a direct mapping for unrecognized modules
            _ => {
                warn!("Unrecognized module pattern: {}", module_name);
                // Convert dots to directory separators and add .ncl extension
                let path_parts: Vec<&str> = module_name.split('.').collect();
                if !path_parts.is_empty() {
                    let mut path = output.to_path_buf();
                    for part in &path_parts[..path_parts.len() - 1] {
                        path = path.join(part);
                    }
                    path = path.join(format!("{}.ncl", path_parts.last().unwrap()));
                    Some(path)
                } else {
                    None
                }
            }
        }
    }

    /// Generate mod.ncl files for package hierarchy
    fn generate_mod_ncl_hierarchy(&self, output: &Path) -> Result<()> {
        // Special handling for k8s_io package to ensure proper exports
        if output.file_name() == Some(std::ffi::OsStr::new("k8s_io")) {
            self.generate_k8s_root_mod(output)?;
        } else {
            // For other packages, generate root mod.ncl with version exports
            self.generate_package_root_mod(output)?;
        }

        Ok(())
    }

    /// Generate the root mod.ncl for non-k8s packages with version exports
    fn generate_package_root_mod(&self, output: &Path) -> Result<()> {
        let mut exports = Vec::new();

        // Add comment header
        let package_name = output
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("package");
        exports.push(format!("# {} Package Module", package_name));
        exports.push("# Auto-generated - do not edit manually".to_string());
        exports.push(String::new());
        exports.push("{".to_string());

        // Find all version directories in the package
        let mut versions = Vec::new();
        if let Ok(entries) = fs::read_dir(output) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        // Check if it's a version directory (starts with 'v')
                        if name.starts_with('v') && name.len() > 1 {
                            // Check if it has a mod.ncl file
                            if path.join("mod.ncl").exists() {
                                versions.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }

        // Sort versions for consistent output
        versions.sort();

        // Generate exports for each version
        for version in &versions {
            exports.push(format!("  {} = import \"./{}/mod.ncl\",", version, version));
        }

        // Add version shortcuts for common patterns
        if exports.len() > 4 {
            // Has actual version exports
            exports.push(String::new());
            exports.push("  # Version shortcuts for convenience".to_string());

            // Find the latest stable version (v1 preferred over v1beta1, v1alpha1)
            let stable_versions = ["v1", "v2", "v3"];
            for v in &stable_versions {
                if versions.contains(&v.to_string()) {
                    exports.push(format!("  latest = import \"./{}/mod.ncl\",", v));
                    break;
                }
            }
        }

        exports.push("}".to_string());

        // Write the mod.ncl file
        let mod_content = exports.join("\n");
        let mod_file = output.join("mod.ncl");
        fs::write(&mod_file, mod_content)?;
        info!("Generated package root mod.ncl with version exports");

        Ok(())
    }

    /// Generate the root mod.ncl for k8s_io package with proper exports
    fn generate_k8s_root_mod(&self, output: &Path) -> Result<()> {
        let mut exports = vec![
            "# Kubernetes API Package Module".to_string(),
            "# Auto-generated - do not edit manually".to_string(),
            "".to_string(),
            "{".to_string(),
        ];

        // Export main API structure
        if output.join("api/mod.ncl").exists() {
            exports.push("  api = import \"./api/mod.ncl\",".to_string());
        }

        // Export apimachinery structures
        if output.join("apimachinery/mod.ncl").exists() {
            exports.push("  apimachinery = import \"./apimachinery/mod.ncl\",".to_string());
        }
        if output.join("apimachinery.pkg/mod.ncl").exists() {
            exports.push(
                "  \"apimachinery.pkg\" = import \"./apimachinery.pkg/mod.ncl\",".to_string(),
            );
        }
        if output.join("apimachinery.pkg.apis/mod.ncl").exists() {
            exports.push(
                "  \"apimachinery.pkg.apis\" = import \"./apimachinery.pkg.apis/mod.ncl\","
                    .to_string(),
            );
        }

        // Export apiextensions
        if output
            .join("apiextensions-apiserver.pkg.apis/mod.ncl")
            .exists()
        {
            exports.push("  apiextensions_apiserver = import \"./apiextensions-apiserver.pkg.apis/mod.ncl\",".to_string());
        }

        // Export kube-aggregator
        if output.join("kube-aggregator.pkg.apis/mod.ncl").exists() {
            exports.push(
                "  kube_aggregator = import \"./kube-aggregator.pkg.apis/mod.ncl\",".to_string(),
            );
        }

        // Create version shortcuts for convenience
        // These map to the most common location for each version
        exports.push("".to_string());
        exports.push("  # Version shortcuts for convenience".to_string());

        // v0 - Unversioned runtime types (create if needed)
        self.ensure_v0_types(output)?;
        exports.push("  v0 = import \"./v0.ncl\",".to_string());

        // Dynamically find all versions across all API groups
        let mut version_map: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        // Walk through api directory to find all version files
        if let Ok(api_dir) = fs::read_dir(output.join("api")) {
            for group_entry in api_dir.flatten() {
                let group_path = group_entry.path();
                if group_path.is_dir() {
                    // Look for version files in this group
                    if let Ok(entries) = fs::read_dir(&group_path) {
                        for entry in entries.flatten() {
                            if let Some(filename) = entry.file_name().to_str() {
                                if filename.ends_with(".ncl") && filename.starts_with("v") {
                                    let version = filename.trim_end_matches(".ncl");
                                    let relative_path = format!(
                                        "api/{}/{}",
                                        group_path.file_name().unwrap().to_str().unwrap(),
                                        filename
                                    );
                                    version_map
                                        .entry(version.to_string())
                                        .or_default()
                                        .push(relative_path);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Sort versions in a logical order: v1, v2, v1alpha1, v1alpha2, v1beta1, etc.
        let mut versions: Vec<String> = version_map.keys().cloned().collect();
        versions.sort_by(|a, b| {
            // Custom sort to put stable versions first, then alpha, then beta
            let a_stable = !a.contains("alpha") && !a.contains("beta");
            let b_stable = !b.contains("alpha") && !b.contains("beta");

            match (a_stable, b_stable) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.cmp(b),
            }
        });

        // Export each version, using the first location found (prefer core for v1)
        for version in &versions {
            if let Some(locations) = version_map.get(version) {
                let location = if version == "v1" {
                    // Prefer core/v1 for the main v1 export
                    locations
                        .iter()
                        .find(|l| l.contains("core/v1"))
                        .or_else(|| locations.first())
                        .unwrap()
                } else {
                    // Use first location for other versions
                    &locations[0]
                };
                exports.push(format!("  {} = import \"./{}\",", version, location));
            }
        }

        // Close the export record
        exports.push("}".to_string());

        let mod_content = exports.join("\n");
        let mod_file = output.join("mod.ncl");
        fs::write(&mod_file, mod_content)?;
        info!("Generated k8s_io root mod.ncl with proper exports");

        Ok(())
    }

    /// Ensure v0.ncl exists with unversioned runtime types
    fn ensure_v0_types(&self, output: &Path) -> Result<()> {
        let v0_path = output.join("v0.ncl");

        // Check if we need to create v0.ncl
        if !v0_path.exists() {
            let v0_content = r#"# Unversioned runtime types for Kubernetes
# These types are used across multiple API versions

{
  # IntOrString is a type that can hold either an integer or a string
  # In Nickel, we represent this as a String type with a contract
  IntOrString = String,
  
  # RawExtension is used to hold arbitrary JSON data
  # It can contain any valid JSON/Nickel value
  RawExtension = {
    ..
  },
  
  # Type contracts for validation
  contracts = {
    # Contract for IntOrString - accepts both numbers (as strings) and strings
    intOrString = fun value =>
      std.is_string value,
    
    # Contract for RawExtension - accepts any record
    rawExtension = fun value =>
      std.is_record value || value == null,
  },
}"#;

            fs::write(&v0_path, v0_content)?;
            info!("Created v0.ncl with unversioned runtime types");
        }

        Ok(())
    }

    /// Generate package manifest for normalized package
    fn generate_normalized_package_manifest(
        &self,
        normalized: &NormalizedPackage,
        output_path: &Path,
    ) -> Result<()> {
        // Generate Nickel-pkg.ncl using domain and name
        let manifest_content = format!(
            r#"{{
  name = "{}",
  version = "0.1.0",
  description = {},
}}"#,
            normalized.name,
            normalized
                .description
                .as_ref()
                .map(|d| format!("\"{}\"", d))
                .unwrap_or_else(|| "null".to_string()),
        );

        let manifest_path = output_path.join("Nickel-pkg.ncl");
        fs::write(&manifest_path, manifest_content)?;
        info!("Generated package manifest at {:?}", manifest_path);

        Ok(())
    }

    /// Generate from OpenAPI URL
    async fn generate_from_openapi_url(&self, url: &str, output: &Path) -> Result<()> {
        use amalgam_codegen::nickel::NickelCodegen;
        use amalgam_parser::openapi::OpenAPIParser;
        use amalgam_parser::swagger::parse_swagger_json;

        info!("Fetching API spec from: {}", url);

        // Fetch the spec
        let response = reqwest::get(url).await?;
        let content = response.text().await?;

        // Detect whether it's Swagger 2.0 or OpenAPI 3.0
        let ir = if content.contains("\"swagger\"") && content.contains("\"2.") {
            info!("Detected Swagger 2.0 specification");
            // Use the Swagger parser
            parse_swagger_json(&content)
                .with_context(|| format!("Failed to parse Swagger 2.0 spec from {}", url))?
        } else {
            info!("Detected OpenAPI 3.0 specification");
            // Use the OpenAPI 3.0 parser
            let spec: openapiv3::OpenAPI = serde_json::from_str(&content)
                .with_context(|| format!("Failed to parse OpenAPI 3.0 spec from {}", url))?;

            let parser = OpenAPIParser::new();
            parser
                .parse(spec)
                .with_context(|| format!("Failed to parse OpenAPI to IR from {}", url))?
        };

        // Apply special cases to the IR modules
        let special_cases = SpecialCasePipeline::new();
        let mut processed_modules = ir.modules.clone();
        for module in &mut processed_modules {
            module.apply_special_cases(&special_cases);

            // Normalize module names for k8s types
            // Convert io.k8s.api.<group>.<version> -> k8s.io.<group>.<version>
            // Convert io.k8s.api.core.v1 -> k8s.io.v1 (core is special - no group in output)
            // Convert io.k8s.apimachinery.pkg.apis.meta.v1 -> k8s.io.v1 (meta goes to core)
            info!("Checking module for normalization: {}", module.name);
            if module.name.starts_with("io.k8s.") {
                info!("Module {} starts with io.k8s.", module.name);
                if module.name.starts_with("io.k8s.api.") {
                    // Extract API group and version
                    let api_part = module.name.strip_prefix("io.k8s.api.").unwrap_or("");
                    let parts: Vec<&str> = api_part.split('.').collect();

                    if parts.len() >= 2 {
                        let api_group = parts[0];
                        let version = parts[parts.len() - 1];

                        if version.starts_with('v') {
                            // Special case: core API group doesn't appear in module name
                            if api_group == "core" {
                                info!(
                                    "Normalizing core module: {} -> k8s.io.{}",
                                    module.name, version
                                );
                                module.name = format!("k8s.io.{}", version);
                            } else {
                                // Keep API group in module name
                                info!(
                                    "Normalizing API group module: {} -> k8s.io.{}.{}",
                                    module.name, api_group, version
                                );
                                module.name = format!("k8s.io.{}.{}", api_group, version);
                            }
                        }
                    }
                } else if module.name.starts_with("io.k8s.apimachinery") {
                    // Handle apimachinery types - preserve in apimachinery.pkg.apis structure
                    let parts: Vec<&str> = module.name.split('.').collect();
                    // Look for version after "meta" or at the end
                    if module.name.contains("meta") {
                        if let Some(version) = parts.iter().find(|&&p| p.starts_with('v')) {
                            // Meta types go to apimachinery.pkg.apis.meta.v1
                            module.name = format!("apimachinery.pkg.apis.meta.{}", version);
                        }
                    } else if module.name.contains("runtime") || module.name.contains("util") {
                        // Runtime/util types stay in apimachinery
                        module.name = "apimachinery.pkg.apis.runtime.v0".to_string();
                    } else if let Some(version) = parts.iter().find(|&&p| p.starts_with('v')) {
                        // Other versioned apimachinery types
                        module.name = format!("apimachinery.pkg.apis.meta.{}", version);
                    }
                }
            } else if module.name.starts_with("apimachinery.") {
                // Handle apimachinery modules without the io.k8s prefix
                let parts: Vec<&str> = module.name.split('.').collect();
                if module.name.contains("meta") {
                    if let Some(version) = parts.iter().find(|&&p| p.starts_with('v')) {
                        module.name = format!("apimachinery.pkg.apis.meta.{}", version);
                    }
                } else if let Some(version) = parts.iter().find(|&&p| p.starts_with('v')) {
                    module.name = format!("apimachinery.pkg.apis.meta.{}", version);
                }
            } else if module.name.starts_with("api.") {
                // Handle k8s API modules without the io.k8s prefix
                // Format is: api.<group>.<version>
                let api_part = module.name.strip_prefix("api.").unwrap_or("");
                let parts: Vec<&str> = api_part.split('.').collect();

                if parts.len() >= 2 {
                    let api_group = parts[0];
                    let version = parts[parts.len() - 1];

                    if version.starts_with('v') {
                        // Special case: core API group doesn't appear in module name
                        if api_group == "core" {
                            info!(
                                "Normalizing core module: {} -> k8s.io.{}",
                                module.name, version
                            );
                            module.name = format!("k8s.io.{}", version);
                        } else {
                            // Keep API group in module name
                            info!(
                                "Normalizing API group module: {} -> k8s.io.{}.{}",
                                module.name, api_group, version
                            );
                            module.name = format!("k8s.io.{}.{}", api_group, version);
                        }
                    }
                }
            }
        }

        // Debug: Check module names after normalization
        info!("Processed modules after normalization:");
        for module in &processed_modules {
            info!(
                "  Module: {} with {} types",
                module.name,
                module.types.len()
            );
        }

        // Phase 1: Complete analysis using CompilationUnit
        let registry = Arc::new(ModuleRegistry::new());
        let mut compilation_unit = CompilationUnit::new(registry.clone());
        compilation_unit.analyze_modules(processed_modules)?;

        // Check for circular dependencies
        if compilation_unit.has_circular_dependencies() {
            warn!("Circular dependencies detected in module graph");
        }

        // Phase 2: Generate with full knowledge of dependencies
        let mut codegen = NickelCodegen::new(registry);
        codegen.set_special_cases(special_cases.clone());
        let generated = codegen.generate_with_compilation_unit(&compilation_unit)?;

        // Create output directory
        fs::create_dir_all(output)?;

        // Split generated output into files
        self.write_generated_files(&generated, output)?;

        // Generate mod.ncl files
        self.generate_mod_ncl_hierarchy(output)?;

        Ok(())
    }

    /// Generate from multiple CRD URLs
    async fn generate_from_multiple_crds(&self, urls: &[String], output: &Path) -> Result<()> {
        use amalgam_parser::crd::{CRDParser, CRD};
        use amalgam_parser::package::NamespacedPackage;

        use std::collections::HashMap;

        info!("Processing {} CRD files", urls.len());

        // Organize CRDs by group
        let mut packages_by_group: HashMap<String, NamespacedPackage> = HashMap::new();

        for url in urls {
            info!("Fetching CRD from: {}", url);

            // Fetch the CRD
            let response = reqwest::get(url).await?;
            let content = response.text().await?;

            // Parse YAML to CRD
            let crd: CRD = serde_yaml::from_str(&content)
                .with_context(|| format!("Failed to parse CRD from {}", url))?;

            let group = crd.spec.group.clone();

            // Get or create package for this group
            let package = packages_by_group
                .entry(group.clone())
                .or_insert_with(|| NamespacedPackage::new(group.clone()));

            // Parse CRD to IR
            let parser = CRDParser::new();
            let temp_ir = parser.parse(crd.clone())?;

            // Add types from the parsed IR to the package
            for module in &temp_ir.modules {
                for type_def in &module.types {
                    // Extract version from module name
                    // CRD parser creates names like: Kind.version.group (e.g., Composition.v1.apiextensions.crossplane.io)
                    let parts: Vec<&str> = module.name.split('.').collect();
                    let version = if parts.len() >= 2 {
                        // The version is the second part (after the Kind)
                        parts[1]
                    } else {
                        "v1"
                    };

                    package.add_type(
                        group.clone(),
                        version.to_string(),
                        type_def.name.clone(),
                        type_def.clone(),
                    );
                }
            }
        }

        // Generate files for each group
        for (group, package) in &packages_by_group {
            // In the universal system, the output directory IS the package directory
            // We don't need to add the group as a subdirectory

            // Get all versions for this group
            let versions = package.versions(group);

            for version in &versions {
                let version_dir = output.join(version);
                fs::create_dir_all(&version_dir)?;

                // Get types for this version and generate files
                let files = package.generate_version_files(group, version);

                for (filename, content) in files {
                    let file_path = version_dir.join(&filename);
                    fs::write(&file_path, content)?;
                    info!("Generated {}", file_path.display());
                }

                // Generate version-level mod.ncl
                if let Some(version_mod) = package.generate_version_module(group, version) {
                    fs::write(version_dir.join("mod.ncl"), version_mod)?;
                }
            }
        }

        // Generate mod.ncl hierarchy
        self.generate_mod_ncl_hierarchy(output)?;

        Ok(())
    }
}

/// Report of package generation results
#[derive(Debug, Default)]
pub struct GenerationReport {
    pub successful: Vec<String>,
    pub failed: Vec<(String, String)>,
    pub skipped: Vec<String>,
}

impl GenerationReport {
    /// Print a summary of generation results
    pub fn print_summary(&self) {
        if !self.successful.is_empty() {
            println!(
                "✅ Successfully generated {} packages:",
                self.successful.len()
            );
            for name in &self.successful {
                println!("  - {}", name);
            }
        }

        if !self.failed.is_empty() {
            println!("\n✗ Failed to generate {} packages:", self.failed.len());
            for (name, error) in &self.failed {
                println!("  - {}: {}", name, error);
            }
        }

        if !self.skipped.is_empty() {
            println!("\n⊘ Skipped {} disabled packages:", self.skipped.len());
            for name in &self.skipped {
                println!("  - {}", name);
            }
        }

        let total = self.successful.len() + self.failed.len() + self.skipped.len();
        println!("\nTotal: {} packages processed", total);
    }
}
