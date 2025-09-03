//! Manifest-based package generation for CI/CD workflows

use amalgam_parser::package::NamespacedPackage;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
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

/// Definition of a package to generate
#[derive(Debug, Deserialize, Serialize)]
pub struct PackageDefinition {
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

fn default_true() -> bool {
    true
}

impl Manifest {
    /// Load manifest from file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read manifest file: {}", path.display()))?;

        toml::from_str(&content)
            .with_context(|| format!("Failed to parse manifest file: {}", path.display()))
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

        for package in &self.packages {
            if !package.enabled {
                info!("Skipping disabled package: {}", package.name);
                report.skipped.push(package.name.clone());
                continue;
            }

            info!("Generating package: {}", package.name);

            match self
                .generate_package_with_manifest(package, &manifest_content)
                .await
            {
                Ok(output_path) => {
                    info!(
                        "✓ Successfully generated {} at {:?}",
                        package.name, output_path
                    );
                    report.successful.push(package.name.clone());
                }
                Err(e) => {
                    warn!("✗ Failed to generate {}: {}", package.name, e);
                    report.failed.push((package.name.clone(), e.to_string()));
                }
            }
        }

        Ok(report)
    }

    /// Generate a single package with manifest content tracking
    async fn generate_package_with_manifest(
        &self,
        package: &PackageDefinition,
        manifest_content: &str,
    ) -> Result<PathBuf> {
        use amalgam_parser::incremental::{
            detect_change_type, save_fingerprint_with_output, ChangeType,
        };

        let output_path = self.config.output_base.join(&package.output);

        // Check if we need to regenerate using intelligent change detection
        let source = self.create_fingerprint_source(package).await?;
        let change_type = detect_change_type(&output_path, source.as_ref())
            .map_err(|e| anyhow::anyhow!("Failed to detect changes: {}", e))?;

        match change_type {
            ChangeType::NoChange => {
                info!("📦 {} - No changes detected, skipping", package.name);
                return Ok(output_path);
            }
            ChangeType::MetadataOnly => {
                info!(
                    "📦 {} - Only metadata changed, updating manifest",
                    package.name
                );
                // Update manifest with new timestamp but keep existing files
                if self.config.package_mode {
                    self.generate_package_manifest(package, &output_path)?;
                }
                // Save new fingerprint with updated metadata
                save_fingerprint_with_output(&output_path, source.as_ref(), Some(manifest_content))
                    .map_err(|e| anyhow::anyhow!("Failed to save fingerprint: {}", e))?;
                return Ok(output_path);
            }
            ChangeType::ContentChanged => {
                info!("📦 {} - Content changed, regenerating", package.name);
            }
            ChangeType::FirstGeneration => {
                info!("📦 {} - First generation", package.name);
            }
            ChangeType::OutputChanged => {
                info!(
                    "📦 {} - Output structure changed, regenerating",
                    package.name
                );
            }
            ChangeType::ManifestChanged => {
                info!("📦 {} - Manifest changed, regenerating", package.name);
            }
        }

        // Build the command based on source type
        let result = match package.source_type {
            SourceType::K8sCore => self.generate_k8s_core(package, &output_path).await,
            SourceType::Url => self.generate_from_url(package, &output_path).await,
            SourceType::Crd => self.generate_from_crd(package, &output_path).await,
            SourceType::OpenApi => self.generate_from_openapi(package, &output_path).await,
        };

        // Generate package manifest if successful
        if result.is_ok() && self.config.package_mode {
            self.generate_package_manifest(package, &output_path)?;
            // Save fingerprint with output content tracking after successful generation
            save_fingerprint_with_output(&output_path, source.as_ref(), Some(manifest_content))
                .map_err(|e| anyhow::anyhow!("Failed to save fingerprint: {}", e))?;
        }

        result
    }

    /// Create a fingerprint source for change detection
    async fn create_fingerprint_source(
        &self,
        package: &PackageDefinition,
    ) -> Result<Box<dyn amalgam_core::fingerprint::Fingerprintable>> {
        use amalgam_parser::incremental::*;

        match package.source_type {
            SourceType::K8sCore => {
                let manifest_version = self.get_k8s_version_from_manifest();
                let version = package
                    .version
                    .as_deref()
                    .or(manifest_version.as_deref())
                    .unwrap_or(env!("DEFAULT_K8S_VERSION"));
                // For k8s core, we would fetch the OpenAPI spec and hash it
                let spec_url = format!(
                    "https://dl.k8s.io/{}/api/openapi-spec/swagger.json",
                    version
                );
                let source = K8sCoreSource {
                    version: version.to_string(),
                    openapi_spec: "".to_string(), // Would be fetched in real implementation
                    spec_url,
                };
                Ok(Box::new(source))
            }
            SourceType::Url => {
                let url = package
                    .url
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("URL required for url type package"))?;

                // Include git ref and version in the fingerprint URL
                let fingerprint_url = if let Some(ref git_ref) = package.git_ref {
                    format!("{}@{}", url, git_ref)
                } else if let Some(ref version) = package.version {
                    format!("{}@{}", url, version)
                } else {
                    url.clone()
                };

                // For URL sources, we would fetch all the URLs and hash their content
                let source = UrlSource {
                    base_url: fingerprint_url.clone(),
                    urls: vec![fingerprint_url], // Simplified - would list all files
                    contents: vec!["".to_string()], // Would be actual content
                };
                Ok(Box::new(source))
            }
            SourceType::Crd | SourceType::OpenApi => {
                // For file-based sources
                let file = package.file.as_ref().ok_or_else(|| {
                    anyhow::anyhow!(
                        "File path required for {:?} type package",
                        package.source_type
                    )
                })?;

                let content = if std::path::Path::new(file).exists() {
                    std::fs::read_to_string(file).unwrap_or_default()
                } else {
                    String::new()
                };

                let source = LocalFilesSource {
                    paths: vec![file.to_string_lossy().to_string()],
                    contents: vec![content],
                };
                Ok(Box::new(source))
            }
        }
    }

    async fn generate_k8s_core(
        &self,
        package: &PackageDefinition,
        output: &Path,
    ) -> Result<PathBuf> {
        use crate::handle_k8s_core_import;

        let manifest_version = self.get_k8s_version_from_manifest();
        let version = package
            .version
            .as_deref()
            .or(manifest_version.as_deref())
            .unwrap_or(env!("DEFAULT_K8S_VERSION"));

        info!("Fetching Kubernetes {} core types...", version);
        handle_k8s_core_import(version, output, true).await?;

        Ok(output.to_path_buf())
    }

    async fn generate_from_url(
        &self,
        package: &PackageDefinition,
        output: &Path,
    ) -> Result<PathBuf> {
        let url = package
            .url
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("URL required for url type package"))?;

        // Build URL with git ref if specified
        let fetch_url = if let Some(ref git_ref) = package.git_ref {
            // Replace /tree/main or /tree/master with the specified ref
            if url.contains("/tree/") {
                let parts: Vec<&str> = url.split("/tree/").collect();
                if parts.len() == 2 {
                    let base = parts[0];
                    let path_parts: Vec<&str> = parts[1].split('/').collect();
                    if path_parts.len() > 1 {
                        // Reconstruct with new ref
                        format!("{}/tree/{}/{}", base, git_ref, path_parts[1..].join("/"))
                    } else {
                        format!("{}/tree/{}", base, git_ref)
                    }
                } else {
                    url.clone()
                }
            } else {
                // Append ref if no /tree/ found
                format!("{}/tree/{}", url.trim_end_matches('/'), git_ref)
            }
        } else {
            url.clone()
        };

        info!("Fetching CRDs from URL: {}", fetch_url);
        if package.git_ref.is_some() {
            info!("Using git ref: {}", package.git_ref.as_ref().unwrap());
        }

        // Use the existing URL import functionality
        use amalgam_parser::crd::CRDParser;
        use amalgam_parser::fetch::CRDFetcher;
        use amalgam_parser::package::NamespacedPackage;
        use amalgam_parser::Parser as SchemaParser;

        let fetcher = CRDFetcher::new()?;
        let crds = fetcher.fetch_from_url(&fetch_url).await?;
        fetcher.finish();

        info!("Found {} CRDs", crds.len());

        // Use unified pipeline with NamespacedPackage
        let mut packages_by_group: std::collections::HashMap<String, NamespacedPackage> =
            std::collections::HashMap::new();

        for crd in crds {
            let group = crd.spec.group.clone();

            // Get or create package for this group
            let package = packages_by_group
                .entry(group.clone())
                .or_insert_with(|| NamespacedPackage::new(group.clone()));

            // Parse CRD to get types
            let parser = CRDParser::new();
            let temp_ir = parser.parse(crd.clone())?;

            // Add types from the parsed IR to the package
            for module in &temp_ir.modules {
                for type_def in &module.types {
                    // Extract version from module name
                    let parts: Vec<&str> = module.name.split('.').collect();
                    let version = if parts.len() > 2 {
                        parts[parts.len() - 2]
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

        // Create output directory structure
        fs::create_dir_all(output)?;

        // Generate files for each group using unified pipeline
        let mut all_groups = Vec::new();
        for (group, package) in packages_by_group {
            all_groups.push(group.clone());
            let group_dir = output.join(&group);
            fs::create_dir_all(&group_dir)?;

            // Get all versions for this group
            let versions = package.versions(&group);

            // Generate version directories and files
            let mut version_modules = Vec::new();
            for version in versions {
                let version_dir = group_dir.join(&version);
                fs::create_dir_all(&version_dir)?;

                // Generate all files for this version using unified pipeline
                let version_files = package.generate_version_files(&group, &version);

                // Write all generated files
                for (filename, content) in version_files {
                    fs::write(version_dir.join(&filename), content)?;
                }

                version_modules.push(format!("  {} = import \"./{}/mod.ncl\",", version, version));
            }

            // Write group module
            if !version_modules.is_empty() {
                let group_mod = format!(
                    "# Module: {}\n# Generated with unified pipeline\n\n{{\n{}\n}}\n",
                    group,
                    version_modules.join("\n")
                );
                fs::write(group_dir.join("mod.ncl"), group_mod)?;
            }
        }

        // Write main module file
        let group_imports: Vec<String> = all_groups
            .iter()
            .map(|g| {
                let sanitized = g.replace(['.', '-'], "_");
                format!("  {} = import \"./{}/mod.ncl\",", sanitized, g)
            })
            .collect();

        let main_module = format!(
            "# Package: {}\n# Generated with unified pipeline from manifest\n\n{{\n{}\n}}\n",
            package.name,
            group_imports.join("\n")
        );
        fs::write(output.join("mod.ncl"), main_module)?;

        Ok(output.to_path_buf())
    }

    async fn generate_from_crd(
        &self,
        package: &PackageDefinition,
        output: &Path,
    ) -> Result<PathBuf> {
        let file = package
            .file
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("File path required for crd type package"))?;

        info!("Importing CRD from {:?}", file);

        // Read and parse the CRD file
        use amalgam_parser::crd::{CRDParser, CRD};
        use amalgam_parser::Parser as SchemaParser;

        let crd_content = fs::read_to_string(file)
            .with_context(|| format!("Failed to read CRD file: {:?}", file))?;

        let crd: CRD = serde_yaml::from_str(&crd_content)
            .with_context(|| format!("Failed to parse CRD YAML: {:?}", file))?;

        // Use unified pipeline with NamespacedPackage
        let mut package = NamespacedPackage::new(crd.spec.group.clone());

        // Parse CRD to get types
        let parser = CRDParser::new();
        let ir = parser.parse(crd.clone())?;

        // Add types from the parsed IR to the package
        for module in &ir.modules {
            for type_def in &module.types {
                // Extract version from module name
                let parts: Vec<&str> = module.name.split('.').collect();
                let version = if parts.len() > 2 {
                    parts[parts.len() - 2]
                } else {
                    "v1"
                };

                package.add_type(
                    crd.spec.group.clone(),
                    version.to_string(),
                    type_def.name.clone(),
                    type_def.clone(),
                );
            }
        }

        // Create output directory structure
        fs::create_dir_all(output)?;

        // Generate files using unified pipeline
        let group_dir = output.join(&crd.spec.group);
        fs::create_dir_all(&group_dir)?;

        for version in package.versions(&crd.spec.group) {
            let version_dir = group_dir.join(&version);
            fs::create_dir_all(&version_dir)?;

            // Generate all files for this version
            let files = package.generate_version_files(&crd.spec.group, &version);
            for (filename, content) in files {
                let file_path = version_dir.join(&filename);
                fs::write(&file_path, content)
                    .with_context(|| format!("Failed to write file: {:?}", file_path))?;
            }
        }

        // Write main module file for the group
        if let Some(group_module) = package.generate_group_module(&crd.spec.group) {
            fs::write(group_dir.join("mod.ncl"), group_module)?;
        }

        // Write main package module
        let main_module = package.generate_main_module();
        fs::write(output.join("mod.ncl"), main_module)?;

        Ok(output.to_path_buf())
    }

    async fn generate_from_openapi(
        &self,
        package: &PackageDefinition,
        output: &Path,
    ) -> Result<PathBuf> {
        let file = package
            .file
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("File path required for openapi type package"))?;

        info!("Importing OpenAPI spec from {:?}", file);

        // Read and parse the OpenAPI file
        use amalgam_parser::walkers::openapi::OpenAPIWalker;
        use amalgam_parser::walkers::SchemaWalker;
        use openapiv3::OpenAPI;

        let openapi_content = fs::read_to_string(file)
            .with_context(|| format!("Failed to read OpenAPI file: {:?}", file))?;

        // Try parsing as JSON first, then YAML
        let openapi: OpenAPI = serde_json::from_str(&openapi_content)
            .or_else(|_| serde_yaml::from_str(&openapi_content))
            .with_context(|| format!("Failed to parse OpenAPI spec: {:?}", file))?;

        // Use package name as base module, sanitizing it for filesystem use
        let base_module = package.name.replace([' ', '-'], "_").to_lowercase();

        // Use OpenAPI walker to generate IR
        let walker = OpenAPIWalker::new(base_module.clone());
        let ir = walker.walk(openapi)?;

        // Create NamespacedPackage and add types
        let mut ns_package = NamespacedPackage::new(base_module.clone());

        for module in &ir.modules {
            for type_def in &module.types {
                // For OpenAPI, we use a simpler structure: base_module/v1/types
                ns_package.add_type(
                    base_module.clone(),
                    "v1".to_string(), // OpenAPI doesn't have versions like CRDs
                    type_def.name.to_lowercase(),
                    type_def.clone(),
                );
            }
        }

        // Create output directory structure
        fs::create_dir_all(output)?;

        // Generate files using unified pipeline
        let group_dir = output.join(&base_module);
        fs::create_dir_all(&group_dir)?;

        let version_dir = group_dir.join("v1");
        fs::create_dir_all(&version_dir)?;

        // Generate all files
        let files = ns_package.generate_version_files(&base_module, "v1");
        for (filename, content) in files {
            let file_path = version_dir.join(&filename);
            fs::write(&file_path, content)
                .with_context(|| format!("Failed to write file: {:?}", file_path))?;
        }

        // Write module files
        if let Some(group_module) = ns_package.generate_group_module(&base_module) {
            fs::write(group_dir.join("mod.ncl"), group_module)?;
        }

        let main_module = ns_package.generate_main_module();
        fs::write(output.join("mod.ncl"), main_module)?;

        Ok(output.to_path_buf())
    }

    fn generate_package_manifest(&self, package: &PackageDefinition, output: &Path) -> Result<()> {
        use amalgam_codegen::nickel_manifest::{
            NickelDependency, NickelManifestConfig, NickelManifestGenerator,
        };
        use amalgam_codegen::package_mode::PackageMode;
        use amalgam_core::IR;
        use std::collections::{HashMap, HashSet};
        use std::path::PathBuf;

        // Use the current manifest file for type registry
        let manifest_path = PathBuf::from(".amalgam-manifest.toml");
        let manifest = if manifest_path.exists() {
            Some(&manifest_path)
        } else {
            None
        };
        let _package_mode = PackageMode::new_with_analyzer(manifest);

        // Build a map of package names to their outputs for dependency resolution
        let package_map: HashMap<String, String> = self
            .packages
            .iter()
            .map(|p| (p.output.clone(), p.name.clone()))
            .collect();

        // Scan generated files for dependencies
        let mut detected_deps = HashSet::new();
        if output.exists() {
            // Walk through all generated .ncl files and look for imports
            for entry in walkdir::WalkDir::new(output)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "ncl"))
            {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    // Look for imports - could be any package name from our manifest
                    for line in content.lines() {
                        // Check for imports of any known package
                        for pkg_output in package_map.keys() {
                            let import_pattern = format!("import \"{}\"", pkg_output);
                            if line.contains(&import_pattern) {
                                detected_deps.insert(pkg_output.clone());
                            }
                        }
                    }
                }
            }
        }

        // Fix version format - remove 'v' prefix for Nickel packages
        let version = package.version.as_deref().unwrap_or("0.1.0");
        let clean_version = version.strip_prefix('v').unwrap_or(version);

        // Create NickelManifestConfig based on package definition
        let config = NickelManifestConfig {
            name: package.name.clone(),
            version: clean_version.to_string(),
            minimal_nickel_version: "1.9.0".to_string(),
            description: package.description.clone(),
            authors: vec!["amalgam".to_string()],
            license: "Apache-2.0".to_string(),
            keywords: package.keywords.clone(),
            base_package_id: Some(self.config.base_package_id.clone()),
            local_dev_mode: self.config.local_package_prefix.is_some(),
            local_package_prefix: self.config.local_package_prefix.clone(),
        };

        // Create the generator
        let generator = NickelManifestGenerator::new(config);

        // Build IR from the package output directory
        // We need to scan the generated files and build an IR
        let mut ir = IR::new();

        // Scan the output directory to build modules
        if output.exists() {
            // Walk through all generated .ncl files to build the IR
            for entry in walkdir::WalkDir::new(output)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "ncl"))
            {
                // Extract module name from path
                if let Ok(rel_path) = entry.path().strip_prefix(output) {
                    if let Some(parent) = rel_path.parent() {
                        let module_name = parent
                            .to_str()
                            .unwrap_or("")
                            .replace(std::path::MAIN_SEPARATOR, ".");

                        if !module_name.is_empty() && module_name != "." {
                            // Create a basic module entry in the IR
                            let module = amalgam_core::ir::Module {
                                name: module_name,
                                imports: Vec::new(),
                                types: Vec::new(),
                                constants: Vec::new(),
                                metadata: Default::default(),
                            };
                            ir.add_module(module);
                        }
                    }
                }
            }
        }

        // Build dependencies map with the correct type
        let mut dependencies = HashMap::new();

        // Add detected dependencies
        for dep_output in &detected_deps {
            let dep_package = self.packages.iter().find(|p| &p.output == dep_output);

            if self.config.local_package_prefix.is_some() {
                // Use Path dependency for local development
                let path = PathBuf::from(format!("../{}", dep_output));
                dependencies.insert(dep_output.clone(), NickelDependency::Path { path });
            } else {
                // Use Index dependency for production
                let package_id = if let Some(dep_pkg) = dep_package {
                    format!(
                        "{}/{}",
                        self.config.base_package_id.trim_end_matches('/'),
                        dep_pkg.name
                    )
                } else {
                    format!(
                        "{}/{}",
                        self.config.base_package_id.trim_end_matches('/'),
                        dep_output
                    )
                };

                let version = if let Some(dep_pkg) = dep_package {
                    if let Some(ref constraint) = package.dependencies.get(dep_output.as_str()) {
                        match constraint {
                            DependencySpec::Simple(v) => v.clone(),
                            DependencySpec::Full { version, .. } => version.clone(),
                        }
                    } else if let Some(ref dep_version) = dep_pkg.version {
                        dep_version
                            .strip_prefix('v')
                            .unwrap_or(dep_version)
                            .to_string()
                    } else {
                        "*".to_string()
                    }
                } else {
                    "*".to_string()
                };

                dependencies.insert(
                    dep_output.clone(),
                    NickelDependency::Index {
                        package: package_id,
                        version,
                    },
                );
            }
        }

        // Add explicit dependencies not auto-detected
        for (dep_name, dep_spec) in &package.dependencies {
            if !detected_deps.contains(dep_name.as_str()) {
                let version = match dep_spec {
                    DependencySpec::Simple(v) => v.clone(),
                    DependencySpec::Full { version, .. } => version.clone(),
                };

                if self.config.local_package_prefix.is_some() {
                    let path = PathBuf::from(format!("../{}", dep_name));
                    dependencies.insert(dep_name.clone(), NickelDependency::Path { path });
                } else {
                    let dep_package = self
                        .packages
                        .iter()
                        .find(|p| p.output == *dep_name || p.name == *dep_name);

                    let package_id = if let Some(dep_pkg) = dep_package {
                        format!(
                            "{}/{}",
                            self.config.base_package_id.trim_end_matches('/'),
                            dep_pkg.name
                        )
                    } else {
                        format!(
                            "{}/{}",
                            self.config.base_package_id.trim_end_matches('/'),
                            dep_name
                        )
                    };

                    dependencies.insert(
                        dep_name.clone(),
                        NickelDependency::Index {
                            package: package_id,
                            version,
                        },
                    );
                }
            }
        }

        // Generate manifest using the unified IR pipeline
        let manifest_content = generator
            .generate_manifest(&ir, Some(dependencies))
            .with_context(|| "Failed to generate Nickel manifest")?;

        // Write manifest file
        let manifest_path = output.join("Nickel-pkg.ncl");
        fs::write(manifest_path, manifest_content)?;

        Ok(())
    }

    /// Clean up packages that are no longer defined in the manifest
    fn cleanup_removed_packages(&self, report: &mut GenerationReport) -> Result<()> {
        if !self.config.output_base.exists() {
            return Ok(());
        }

        // Get list of current package outputs from manifest
        let current_outputs: std::collections::HashSet<String> = self
            .packages
            .iter()
            .filter(|p| p.enabled)
            .map(|p| p.output.clone())
            .collect();

        // Scan output directory for existing packages
        let mut packages_to_remove = Vec::new();

        for entry in fs::read_dir(&self.config.output_base)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let dir_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();

                // Check if this directory has a fingerprint file (indicating it's a generated package)
                let fingerprint_path = path.join(".amalgam-fingerprint.json");
                if fingerprint_path.exists() && !current_outputs.contains(&dir_name) {
                    packages_to_remove.push((dir_name.clone(), path.clone()));
                }
            }
        }

        // Remove packages that are no longer in the manifest
        for (package_name, package_path) in packages_to_remove {
            info!(
                "🗑️  Removing package no longer in manifest: {}",
                package_name
            );

            match fs::remove_dir_all(&package_path) {
                Ok(()) => {
                    info!("✓ Successfully removed {}", package_name);
                    report.successful.push(format!("REMOVED: {}", package_name));
                }
                Err(e) => {
                    warn!("✗ Failed to remove {}: {}", package_name, e);
                    report
                        .failed
                        .push((format!("REMOVE: {}", package_name), e.to_string()));
                }
            }
        }

        Ok(())
    }

    /// Get the k8s version from the manifest (looks for k8s-core type packages)
    fn get_k8s_version_from_manifest(&self) -> Option<String> {
        self.packages
            .iter()
            .find(|p| p.source_type == SourceType::K8sCore)
            .and_then(|p| p.version.clone())
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
    /// Print a summary of the generation results
    pub fn print_summary(&self) {
        println!("\n=== Package Generation Summary ===");

        if !self.successful.is_empty() {
            println!(
                "\n✓ Successfully generated {} packages:",
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
