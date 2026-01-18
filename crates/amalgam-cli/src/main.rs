use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

use amalgam_codegen::{go::GoCodegen, nickel::NickelCodegen, rust::RustCodegen, Codegen};
use amalgam_parser::{
    crd::{CRDParser, CRD},
    openapi::OpenAPIParser,
    walkers::SchemaWalker,
    Parser as SchemaParser,
};
use daemon::DaemonCommand;
use package::PackageCommand;
use registry::RegistryCommand;

mod daemon;
mod manifest;
mod package;
mod registry;
mod rich_package;
mod validate;
mod vendor;

#[derive(Parser)]
#[command(name = "amalgam")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Generate type-safe Nickel configurations from any schema source", long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Enable debug output
    #[arg(short, long)]
    debug: bool,

    /// Enable import debugging (shows detailed import resolution)
    #[arg(long = "debug-imports")]
    debug_imports: bool,

    /// Export debug information to a JSON file
    #[arg(long = "debug-export")]
    debug_export: Option<PathBuf>,

    /// Path to the amalgam manifest file
    #[arg(short, long, default_value = ".amalgam-manifest.toml", global = true)]
    manifest: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Import types from various sources
    Import {
        #[command(subcommand)]
        source: ImportSource,
    },

    /// Generate code from IR
    Generate {
        /// Input IR file (JSON format)
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Target language (nickel, go, rust)
        #[arg(short, long, default_value = "nickel")]
        target: String,
    },

    /// Generate Rust types from CRDs or OpenAPI specs
    GenerateRust {
        /// Input file (CRD YAML/JSON or OpenAPI spec)
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path for generated Rust code
        #[arg(short, long)]
        output: PathBuf,

        /// Generate Merge trait implementations
        #[arg(long, default_value = "true")]
        merge: bool,

        /// Generate Validate trait implementations
        #[arg(long, default_value = "true")]
        validate: bool,

        /// Generate builder methods (with_*)
        #[arg(long, default_value = "true")]
        builders: bool,

        /// Runtime crate name for imports (default: amalgam_runtime)
        #[arg(long, default_value = "amalgam_runtime")]
        runtime_crate: String,
    },

    /// Convert from one format to another
    Convert {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,

        /// Input format (crd, openapi, go)
        #[arg(short = 'f', long)]
        from: String,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Output format (nickel, go, ir)
        #[arg(short, long)]
        to: String,
    },

    /// Vendor package management
    Vendor {
        #[command(subcommand)]
        command: vendor::VendorCommand,
    },

    /// Validate a Nickel package
    Validate {
        /// Path to the Nickel package or file to validate
        #[arg(short, long)]
        path: PathBuf,

        /// Package path prefix for dependency resolution (e.g., examples/pkgs)
        #[arg(long)]
        package_path: Option<PathBuf>,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Generate packages from a manifest file
    GenerateFromManifest {
        /// Only generate specific packages (by name)
        #[arg(short, long)]
        packages: Vec<String>,

        /// Dry run - show what would be generated without doing it
        #[arg(long)]
        dry_run: bool,

        /// Enable debug output (writes generated.ncl debug file)
        #[arg(long)]
        debug: bool,
    },

    /// Execute a unified pipeline from configuration
    Pipeline {
        /// Path to the pipeline configuration file
        #[arg(short, long)]
        config: PathBuf,

        /// Export diagnostics to a JSON file
        #[arg(long)]
        export_diagnostics: Option<PathBuf>,

        /// Error recovery strategy (fail-fast, continue, best-effort, interactive)
        #[arg(long, default_value = "fail-fast")]
        error_recovery: String,

        /// Dry run - show what would be executed without doing it
        #[arg(long)]
        dry_run: bool,
    },

    /// Package registry management
    Registry {
        #[command(subcommand)]
        command: RegistryCommand,
    },

    /// Package management operations
    Package {
        #[command(subcommand)]
        command: PackageCommand,
    },

    /// Runtime daemon for watching and regenerating types
    Daemon {
        #[command(subcommand)]
        command: DaemonCommand,
    },

    /// Generate a rich Nickel package with patterns and examples
    RichPackage {
        /// Input IR file (JSON format)
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory for the package
        #[arg(short, long)]
        output: PathBuf,

        /// Package name
        #[arg(short, long)]
        name: String,

        /// Package version
        #[arg(long, default_value = "0.1.0")]
        version: String,

        /// Package type (k8s, crossplane-aws, crossplane-gcp, crossplane-azure, custom)
        #[arg(long, default_value = "custom")]
        package_type: String,
    },
}

#[derive(Subcommand)]
enum ImportSource {
    /// Import from Kubernetes CRD
    Crd {
        /// CRD file path (YAML or JSON)
        #[arg(short, long)]
        file: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Generate as submittable package (with package imports)
        #[arg(long)]
        package_mode: bool,
    },

    /// Import CRDs from URL (GitHub repo, directory, or direct file)
    Url {
        /// URL to fetch CRDs from
        #[arg(short, long)]
        url: String,

        /// Output directory for package
        #[arg(short, long)]
        output: PathBuf,

        /// Package name (defaults to last part of URL)
        #[arg(short, long)]
        package: Option<String>,

        /// Generate Nickel package manifest (experimental)
        #[arg(long)]
        nickel_package: bool,

        /// Base directory for package resolution (defaults to current directory)
        #[arg(long, env = "AMALGAM_PACKAGE_BASE")]
        package_base: Option<PathBuf>,
    },

    /// Import from OpenAPI specification
    OpenApi {
        /// OpenAPI spec file path (YAML or JSON)
        #[arg(short, long)]
        file: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Import core Kubernetes types from upstream OpenAPI
    K8sCore {
        /// Kubernetes version (e.g., "v1.33.4", "master")
        #[arg(short, long, default_value = env!("DEFAULT_K8S_VERSION"))]
        version: String,

        /// Output directory for generated types
        #[arg(short, long, default_value = "k8s_io")]
        output: PathBuf,

        /// Specific types to import (if empty, imports common types)
        #[arg(short, long)]
        types: Vec<String>,

        /// Generate Nickel package manifest (experimental)
        #[arg(long)]
        nickel_package: bool,

        /// Base directory for package resolution (defaults to current directory)
        #[arg(long, env = "AMALGAM_PACKAGE_BASE")]
        package_base: Option<PathBuf>,
    },

    /// Import from Kubernetes cluster (not implemented)
    K8s {
        /// Kubernetes context to use
        #[arg(short, long)]
        context: Option<String>,

        /// CRD group to import
        #[arg(short, long)]
        group: Option<String>,

        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    let level = if cli.debug {
        tracing::Level::TRACE
    } else if cli.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(cli.debug) // Show target module in debug mode
        .init();

    match cli.command {
        Some(Commands::Import { source }) => handle_import(source).await,
        Some(Commands::Generate {
            input,
            output,
            target,
        }) => handle_generate(input, output, &target),
        Some(Commands::GenerateRust {
            input,
            output,
            merge,
            validate,
            builders,
            runtime_crate,
        }) => handle_generate_rust(input, output, merge, validate, builders, runtime_crate),
        Some(Commands::Convert {
            input,
            from,
            output,
            to,
        }) => handle_convert(input, &from, output, &to),
        Some(Commands::Vendor { command }) => {
            let project_root = std::env::current_dir()?;
            let manager = vendor::VendorManager::new(project_root);
            manager.execute(command).await
        }
        Some(Commands::Validate {
            path,
            package_path,
            verbose: _,
        }) => validate::run_validation_with_package_path(&path, package_path.as_deref()),
        Some(Commands::GenerateFromManifest {
            packages,
            dry_run,
            debug,
        }) => handle_manifest_generation(cli.manifest, packages, dry_run, debug).await,
        Some(Commands::Pipeline {
            config,
            export_diagnostics,
            error_recovery,
            dry_run,
        }) => handle_pipeline_execution(config, export_diagnostics, &error_recovery, dry_run).await,
        Some(Commands::Registry { command }) => command.execute().await,
        Some(Commands::Package { command }) => command.execute().await,
        Some(Commands::Daemon { command }) => command.execute().await,
        Some(Commands::RichPackage {
            input,
            output,
            name,
            version,
            package_type,
        }) => {
            // Use defaults for patterns, examples, and lsp_friendly
            handle_rich_package_generation(RichPackageGenConfig {
                input,
                output,
                name,
                version,
                package_type,
                patterns: true,
                examples: true,
                lsp_friendly: true,
            })
            .await
        }
        None => {
            // No command provided, show help
            use clap::CommandFactory;
            Cli::command().print_help()?;
            Ok(())
        }
    }
}

async fn handle_import(source: ImportSource) -> Result<()> {
    match source {
        ImportSource::Url {
            url,
            output,
            package,
            nickel_package,
            package_base: _,
        } => {
            info!("Fetching CRDs from URL: {}", url);

            // Determine package name
            let package_name = package.unwrap_or_else(|| {
                url.split('/')
                    .next_back()
                    .unwrap_or("generated")
                    .trim_end_matches(".yaml")
                    .trim_end_matches(".yml")
                    .to_string()
            });

            // Fetch CRDs
            let fetcher = amalgam_parser::fetch::CRDFetcher::new()?;
            let crds = fetcher.fetch_from_url(&url).await?;
            fetcher.finish(); // Clear progress bars when done

            info!("Found {} CRDs", crds.len());

            // Use unified pipeline with NamespacedPackage
            // Parse all CRDs and organize by group
            let mut packages_by_group: std::collections::HashMap<
                String,
                amalgam_parser::package::NamespacedPackage,
            > = std::collections::HashMap::new();

            for crd in crds {
                let group = crd.spec.group.clone();

                // Get or create package for this group
                let package = packages_by_group.entry(group.clone()).or_insert_with(|| {
                    amalgam_parser::package::NamespacedPackage::new(group.clone())
                });

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
            fs::create_dir_all(&output)?;

            // Generate files for each group using unified pipeline
            let mut all_groups = Vec::new();
            for (group, package) in &packages_by_group {
                all_groups.push(group.clone());
                let group_dir = output.join(group);
                fs::create_dir_all(&group_dir)?;

                // Get all versions for this group
                let versions = package.versions(group);

                // Generate version directories and files
                let mut version_modules = Vec::new();
                for version in versions {
                    let version_dir = group_dir.join(&version);
                    fs::create_dir_all(&version_dir)?;

                    // Generate all files for this version using unified pipeline
                    let version_files = package.generate_version_files(group, &version);

                    // Write all generated files
                    for (filename, content) in version_files {
                        fs::write(version_dir.join(&filename), content)?;
                    }

                    version_modules
                        .push(format!("  {} = import \"./{}/mod.ncl\",", version, version));
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
                "# Package: {}\n# Generated with unified pipeline\n\n{{\n{}\n}}\n",
                package_name,
                group_imports.join("\n")
            );
            fs::write(output.join("mod.ncl"), main_module)?;

            // Always generate Nickel package manifest - it's core to Nickel packages
            {
                info!("Generating Nickel package manifest");
                // Use the unified pipeline manifest generator instead of hardcoded string
                use amalgam_codegen::nickel_manifest::{
                    NickelManifestConfig, NickelManifestGenerator,
                };
                use amalgam_core::IR;

                // Build IR from all the packages
                let mut ir = IR::new();
                for (group, package) in &packages_by_group {
                    if let Some(types_in_group) = package.types.get(group) {
                        for (version, version_types) in types_in_group {
                            for type_def in version_types.values() {
                                let module = amalgam_core::ir::Module {
                                    name: format!("{}.{}", group, version),
                                    imports: Vec::new(),
                                    types: vec![type_def.clone()],
                                    constants: Vec::new(),
                                    metadata: Default::default(),
                                };
                                ir.add_module(module);
                            }
                        }
                    }
                }

                let manifest_config = NickelManifestConfig {
                    name: package_name.clone(),
                    version: "0.1.0".to_string(),
                    minimal_nickel_version: "1.9.0".to_string(),
                    description: format!(
                        "Type definitions for {} generated by Amalgam",
                        package_name
                    ),
                    authors: vec!["amalgam".to_string()],
                    license: "Apache-2.0".to_string(),
                    keywords: {
                        let mut keywords = vec!["kubernetes".to_string(), "types".to_string()];
                        // Add groups as keywords
                        for group in all_groups.iter() {
                            keywords.push(group.replace('.', "-"));
                        }
                        keywords
                    },
                    base_package_id: None,
                    local_dev_mode: true, // Use Path dependencies for development
                    local_package_prefix: Some("../".to_string()),
                };

                // Scan generated files for dependencies (like k8s_io imports)
                let mut detected_deps = std::collections::HashMap::new();
                if output.join("k8s_io").exists() || output.exists() {
                    use walkdir::WalkDir;
                    for entry in WalkDir::new(&output)
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().extension().is_some_and(|ext| ext == "ncl"))
                    {
                        if let Ok(content) = std::fs::read_to_string(entry.path()) {
                            // Look for k8s_io imports
                            if content.contains("import \"../../../k8s_io/")
                                || content.contains("import \"../../k8s_io/")
                            {
                                let path = std::path::PathBuf::from("../k8s_io");
                                detected_deps.insert(
                                    "k8s_io".to_string(),
                                    amalgam_codegen::nickel_manifest::NickelDependency::Path {
                                        path,
                                    },
                                );
                            }
                        }
                    }
                }

                let generator = NickelManifestGenerator::new(manifest_config);
                let manifest_content = generator
                    .generate_manifest(&ir, Some(detected_deps))
                    .expect("Failed to generate Nickel manifest");

                fs::write(output.join("Nickel-pkg.ncl"), manifest_content)?;
                info!("✓ Generated Nickel-pkg.ncl");
            }

            info!(
                "Generated package '{}' in {:?} using unified pipeline",
                package_name, output
            );
            info!("Package structure:");
            for group in &all_groups {
                info!("  {}/", group);
            }
            if nickel_package {
                info!("  Nickel-pkg.ncl (package manifest)");
            }

            Ok(())
        }

        ImportSource::Crd {
            file,
            output,
            package_mode,
        } => {
            info!("Importing CRD from {:?}", file);

            let content = fs::read_to_string(&file)
                .with_context(|| format!("Failed to read CRD file: {:?}", file))?;

            let crd: CRD = if file.extension().is_some_and(|ext| ext == "json") {
                serde_json::from_str(&content)?
            } else {
                serde_yaml::from_str(&content)?
            };

            // Use the unified pipeline through NamespacedPackage
            let mut package =
                amalgam_parser::package::NamespacedPackage::new(crd.spec.group.clone());

            // Parse CRD to get type definition
            let parser = CRDParser::new();
            let temp_ir = parser.parse(crd.clone())?;

            // Add types from the parsed IR to the package
            for module in &temp_ir.modules {
                for type_def in &module.types {
                    // Extract version from module name
                    // Module name format is: kind.version.group (e.g., "CompositeResourceDefinition.v1.apiextensions.crossplane.io")
                    let parts: Vec<&str> = module.name.split('.').collect();
                    let version = if parts.len() >= 2 {
                        // Version is at index 1 (second part)
                        parts[1]
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

            // Generate using unified pipeline
            let version = crd
                .spec
                .versions
                .first()
                .map(|v| v.name.clone())
                .unwrap_or_else(|| "v1".to_string());

            let files = package.generate_version_files(&crd.spec.group, &version);

            // For single file output, just get the first generated file
            let code = files
                .values()
                .next()
                .cloned()
                .unwrap_or_else(|| "# No types generated\n".to_string());

            // Apply package mode transformation if requested
            let final_code = if package_mode {
                // Transform relative imports to package imports
                // This is a post-processing step on the generated code
                transform_imports_to_package_mode(&code, &crd.spec.group)
            } else {
                code.clone()
            };

            if let Some(output_path) = output {
                fs::write(&output_path, &final_code)
                    .with_context(|| format!("Failed to write output: {:?}", output_path))?;
                info!("Generated Nickel code written to {:?}", output_path);
            } else {
                println!("{}", final_code);
            }

            Ok(())
        }

        ImportSource::OpenApi { file, output } => {
            info!("Importing OpenAPI spec from {:?}", file);

            let content = fs::read_to_string(&file)
                .with_context(|| format!("Failed to read OpenAPI file: {:?}", file))?;

            let spec: openapiv3::OpenAPI = if file.extension().is_some_and(|ext| ext == "json") {
                serde_json::from_str(&content)?
            } else {
                serde_yaml::from_str(&content)?
            };

            // Use the unified pipeline through NamespacedPackage
            // Extract namespace from filename or use default
            let namespace = file
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("openapi")
                .to_string();

            let mut package = amalgam_parser::package::NamespacedPackage::new(namespace.clone());

            // Parse using walker pattern
            let walker = amalgam_parser::walkers::openapi::OpenAPIWalker::new(&namespace);
            let ir = walker.walk(spec)?;

            // Add all types to the package from the generated IR
            for module in &ir.modules {
                for type_def in &module.types {
                    // Extract version from module name if present
                    let parts: Vec<&str> = module.name.split('.').collect();
                    let version = if parts.len() > 1 {
                        parts.last().unwrap().to_string()
                    } else {
                        "v1".to_string() // Default version
                    };

                    package.add_type(
                        namespace.clone(),
                        version.clone(),
                        type_def.name.clone(),
                        type_def.clone(),
                    );
                }
            }

            // Generate files using the unified pipeline
            let files = package.generate_version_files(&namespace, "v1");
            let code = files.values().next().unwrap_or(&String::new()).clone();

            if let Some(output_path) = output {
                fs::write(&output_path, &code)
                    .with_context(|| format!("Failed to write output: {:?}", output_path))?;
                info!("Generated Nickel code written to {:?}", output_path);
            } else {
                println!("{}", code);
            }

            Ok(())
        }

        ImportSource::K8sCore {
            version,
            output,
            types: _,
            nickel_package,
            package_base: _,
        } => {
            handle_k8s_core_import(&version, &output, nickel_package).await?;
            Ok(())
        }

        ImportSource::K8s { .. } => {
            anyhow::bail!("Kubernetes import not yet implemented. Build with --features kubernetes to enable.")
        }
    }
}

// Moved to lib.rs to avoid duplication
use amalgam::handle_k8s_core_import;
use amalgam_core::manifest::AmalgamManifest;
use amalgam_core::pipeline::{PipelineDiagnostics, RecoveryStrategy, UnifiedPipeline};

async fn handle_manifest_generation(
    manifest_path: PathBuf,
    packages: Vec<String>,
    dry_run: bool,
    debug: bool,
) -> Result<()> {
    use crate::manifest::Manifest;

    info!("Loading manifest from {:?}", manifest_path);
    let mut manifest = Manifest::from_file(&manifest_path)?;

    // Override debug flag from CLI if specified
    if debug {
        manifest.config.debug = true;
    }

    // Filter packages if specific ones were requested
    if !packages.is_empty() {
        manifest.packages.retain(|p| {
            if let Some(ref name) = p.name {
                packages.contains(name)
            } else {
                // If no name, use the inferred package name from domain
                false
            }
        });
        if manifest.packages.is_empty() {
            anyhow::bail!("No matching packages found for: {:?}", packages);
        }
    }

    if dry_run {
        info!("Dry run mode - showing what would be generated:");
        for package in &manifest.packages {
            if package.enabled {
                // Normalize the package to get inferred information
                match package.normalize().await {
                    Ok(normalized) => {
                        let output_path = normalized.output_path(&manifest.config.output_base);
                        info!(
                            "  - {} -> {} (domain: {})",
                            normalized.name,
                            output_path.display(),
                            normalized.domain
                        );
                    }
                    Err(e) => {
                        let display_name = package.name.as_deref().unwrap_or("unnamed");
                        warn!("  - {} -> Failed to normalize: {}", display_name, e);
                    }
                }
            }
        }
        return Ok(());
    }

    // Generate all packages
    let report = manifest.generate_all().await?;
    report.print_summary();

    if !report.failed.is_empty() {
        anyhow::bail!("Some packages failed to generate");
    }

    Ok(())
}

fn handle_generate(input: PathBuf, output: PathBuf, target: &str) -> Result<()> {
    info!("Generating {} code from {:?}", target, input);

    let ir_content = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read IR file: {:?}", input))?;

    let ir: amalgam_core::IR =
        serde_json::from_str(&ir_content).with_context(|| "Failed to parse IR JSON")?;

    let code = match target {
        "nickel" => {
            let mut codegen = NickelCodegen::from_ir(&ir);
            codegen.generate(&ir)?
        }
        "go" => {
            let mut codegen = GoCodegen::new();
            codegen.generate(&ir)?
        }
        "rust" => {
            let mut codegen = RustCodegen::new();
            codegen.generate(&ir)?
        }
        _ => {
            anyhow::bail!("Unsupported target language: {}. Supported: nickel, go, rust", target);
        }
    };

    fs::write(&output, code).with_context(|| format!("Failed to write output: {:?}", output))?;

    info!("Generated code written to {:?}", output);
    Ok(())
}

/// Handle the generate-rust command for generating Rust types from CRDs or OpenAPI specs
fn handle_generate_rust(
    input: PathBuf,
    output: PathBuf,
    generate_merge: bool,
    generate_validate: bool,
    generate_builders: bool,
    runtime_crate: String,
) -> Result<()> {
    use amalgam_codegen::rust::RustCodegenConfig;

    info!("Generating Rust types from {:?}", input);

    let content = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read input file: {:?}", input))?;

    // Detect input type and parse to IR
    let ir = if content.contains("kind: CustomResourceDefinition")
        || content.contains("kind: \"CustomResourceDefinition\"")
    {
        // CRD input
        info!("Detected CRD input");
        let crd: CRD = if input.extension().is_some_and(|ext| ext == "json") {
            serde_json::from_str(&content)?
        } else {
            serde_yaml::from_str(&content)?
        };
        CRDParser::new().parse(crd)?
    } else if content.contains("\"openapi\"") || content.contains("\"swagger\"") {
        // OpenAPI input
        info!("Detected OpenAPI input");
        let spec: openapiv3::OpenAPI = if input.extension().is_some_and(|ext| ext == "json") {
            serde_json::from_str(&content)?
        } else {
            serde_yaml::from_str(&content)?
        };
        OpenAPIParser::new().parse(spec)?
    } else if content.trim_start().starts_with('{') {
        // Assume it's IR JSON
        info!("Detected IR JSON input");
        serde_json::from_str(&content)?
    } else {
        anyhow::bail!("Could not detect input type. Expected CRD YAML, OpenAPI spec, or IR JSON.");
    };

    // Configure Rust codegen
    let config = RustCodegenConfig {
        generate_merge,
        generate_validate,
        generate_builders,
        generate_default: true,
        include_docs: true,
        box_recursive_types: true,
        runtime_crate,
    };

    let mut codegen = RustCodegen::new().with_config(config);
    let code = codegen.generate(&ir)?;

    // Write output
    fs::write(&output, &code)
        .with_context(|| format!("Failed to write output: {:?}", output))?;

    info!("Generated Rust types written to {:?}", output);
    info!(
        "Options: merge={}, validate={}, builders={}",
        generate_merge, generate_validate, generate_builders
    );

    Ok(())
}

fn handle_convert(input: PathBuf, from: &str, output: PathBuf, to: &str) -> Result<()> {
    info!("Converting from {} to {}", from, to);

    let content = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read input file: {:?}", input))?;

    // Parse input to IR
    let ir = match from {
        "crd" => {
            let crd: CRD = if input.extension().is_some_and(|ext| ext == "json") {
                serde_json::from_str(&content)?
            } else {
                serde_yaml::from_str(&content)?
            };
            CRDParser::new().parse(crd)?
        }
        "openapi" => {
            let spec: openapiv3::OpenAPI = if input.extension().is_some_and(|ext| ext == "json") {
                serde_json::from_str(&content)?
            } else {
                serde_yaml::from_str(&content)?
            };
            OpenAPIParser::new().parse(spec)?
        }
        _ => {
            anyhow::bail!("Unsupported input format: {}", from);
        }
    };

    // Generate output
    let output_content = match to {
        "nickel" => {
            let mut codegen = NickelCodegen::from_ir(&ir);
            codegen.generate(&ir)?
        }
        "go" => {
            let mut codegen = GoCodegen::new();
            codegen.generate(&ir)?
        }
        "ir" => serde_json::to_string_pretty(&ir)?,
        _ => {
            anyhow::bail!("Unsupported output format: {}", to);
        }
    };

    fs::write(&output, output_content)
        .with_context(|| format!("Failed to write output: {:?}", output))?;

    info!("Conversion complete. Output written to {:?}", output);
    Ok(())
}

/// Transform relative imports in generated code to package imports
/// This is used when --package-mode is enabled
fn transform_imports_to_package_mode(code: &str, group: &str) -> String {
    // Determine the base package ID based on the group
    let package_id = if group.starts_with("k8s.io") || group.contains("k8s.io") {
        "github:seryl/nickel-pkgs/k8s-io"
    } else if group.contains("crossplane") {
        "github:seryl/nickel-pkgs/crossplane"
    } else {
        // For unknown groups, keep relative imports
        return code.to_string();
    };

    // Transform import statements from relative to package imports
    let mut result = String::new();
    for line in code.lines() {
        if line.contains("import") && line.contains("../") {
            // Extract the module path from the import
            if let Some(start) = line.find('"') {
                if let Some(end) = line.rfind('"') {
                    let import_path = &line[start + 1..end];
                    // Count the number of ../ to determine depth
                    let depth = import_path.matches("../").count();

                    // Extract the module name (last part of the path)
                    let module_parts: Vec<&str> = import_path.split('/').collect();
                    let module_name = module_parts
                        .last()
                        .and_then(|s| s.strip_suffix(".ncl"))
                        .unwrap_or("");

                    // Construct package import
                    if depth >= 2 && module_name != "mod" {
                        // This looks like a cross-version import
                        let new_line = format!(
                            "{}import \"{}#/{}\".{}",
                            &line[..start],
                            package_id,
                            module_name,
                            &line[end + 1..]
                        );
                        result.push_str(&new_line);
                        result.push('\n');
                        continue;
                    }
                }
            }
        }
        result.push_str(line);
        result.push('\n');
    }

    // Remove trailing newline if original didn't have one
    if !code.ends_with('\n') && result.ends_with('\n') {
        result.pop();
    }

    result
}

async fn handle_pipeline_execution(
    config_path: PathBuf,
    export_diagnostics: Option<PathBuf>,
    error_recovery: &str,
    dry_run: bool,
) -> Result<()> {
    info!("Loading pipeline configuration from {:?}", config_path);

    // Load the configuration
    let config_content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read pipeline config: {:?}", config_path))?;

    let manifest: AmalgamManifest = toml::from_str(&config_content)
        .with_context(|| "Failed to parse pipeline configuration")?;

    // Parse error recovery strategy
    let recovery_strategy = match error_recovery {
        "continue" => RecoveryStrategy::Continue,
        "best-effort" => RecoveryStrategy::BestEffort {
            fallback_types: true,
            skip_invalid_modules: true,
            use_dynamic_types: false,
        },
        "interactive" => RecoveryStrategy::Interactive {
            prompt_for_fixes: true,
            suggest_alternatives: true,
        },
        _ => RecoveryStrategy::FailFast,
    };

    if dry_run {
        info!("Dry run mode - showing pipeline execution plan:");
        info!("  Pipeline: {}", manifest.metadata.name);
        info!("  Version: {}", manifest.metadata.version);
        info!("  Stages: {}", manifest.stages.len());
        for (i, stage) in manifest.stages.iter().enumerate() {
            info!("    Stage {}: {}", i + 1, stage.name);
            if let Some(desc) = &stage.description {
                info!("      Description: {}", desc);
            }
        }
        return Ok(());
    }

    // Execute each stage
    let _all_diagnostics: Vec<amalgam_core::pipeline::PipelineDiagnostics> = Vec::new();

    for (stage_idx, stage) in manifest.stages.iter().enumerate() {
        info!(
            "Executing stage {}/{}: {}",
            stage_idx + 1,
            manifest.stages.len(),
            stage.name
        );

        // Convert manifest config to pipeline types
        use amalgam_core::pipeline::{
            FileFormat, InputSource, ModuleLayout, ModuleStructure, OutputTarget, Transform,
            VersionHandling,
        };

        // Convert InputConfig to InputSource
        let input_source = match stage.input.input_type.as_str() {
            "openapi" => InputSource::OpenAPI {
                url: stage
                    .input
                    .spec_path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "https://example.com/openapi.yaml".to_string()),
                version: "v1".to_string(),
                domain: None,
                auth: None,
            },
            "crds" | "k8s-core" | "crossplane" => InputSource::CRDs {
                urls: stage
                    .input
                    .crd_paths
                    .clone()
                    .unwrap_or_else(|| vec!["https://example.com/crds".to_string()]),
                domain: "k8s.io".to_string(),
                versions: vec!["v1".to_string()],
                auth: None,
            },
            "go" => InputSource::GoTypes {
                package: stage
                    .input
                    .go_module
                    .clone()
                    .unwrap_or_else(|| "github.com/example/pkg".to_string()),
                types: vec![],
                version: None,
                module_path: None,
            },
            "file" => InputSource::LocalFiles {
                paths: vec![stage
                    .input
                    .spec_path
                    .clone()
                    .unwrap_or_else(|| PathBuf::from("./input.yaml"))],
                format: FileFormat::Auto,
                recursive: false,
            },
            _ => {
                warn!(
                    "Unknown input type: {}, defaulting to File",
                    stage.input.input_type
                );
                InputSource::LocalFiles {
                    paths: vec![PathBuf::from("./input.yaml")],
                    format: FileFormat::Auto,
                    recursive: false,
                }
            }
        };

        // Convert OutputConfig to OutputTarget
        use amalgam_core::pipeline::{NickelFormatting, PackageMetadata};

        let output_target = match stage.output.output_type.as_str() {
            "nickel" | "nickel-package" => OutputTarget::NickelPackage {
                contracts: true,
                validation: true,
                rich_exports: true,
                usage_patterns: false,
                package_metadata: PackageMetadata {
                    name: stage
                        .output
                        .package_name
                        .clone()
                        .unwrap_or_else(|| "generated".to_string()),
                    version: "0.1.0".to_string(),
                    description: "Generated by Amalgam".to_string(),
                    homepage: None,
                    repository: None,
                    license: Some("Apache-2.0".to_string()),
                    keywords: vec![],
                    authors: vec!["amalgam".to_string()],
                },
                formatting: NickelFormatting {
                    indent: 2,
                    max_line_length: 100,
                    sort_imports: true,
                    compact_records: false,
                },
            },
            "go" => OutputTarget::Go {
                package_name: stage
                    .output
                    .package_name
                    .clone()
                    .unwrap_or_else(|| "generated".to_string()),
                imports: vec![],
                tags: vec![],
                generate_json_tags: true,
            },
            "cue" => OutputTarget::CUE {
                package_name: Some(
                    stage
                        .output
                        .package_name
                        .clone()
                        .unwrap_or_else(|| "generated".to_string()),
                ),
                strict_mode: true,
                constraints: true,
            },
            _ => {
                warn!(
                    "Unknown output type: {}, defaulting to NickelPackage",
                    stage.output.output_type
                );
                OutputTarget::NickelPackage {
                    contracts: true,
                    validation: false,
                    rich_exports: false,
                    usage_patterns: false,
                    package_metadata: PackageMetadata {
                        name: "generated".to_string(),
                        version: "0.1.0".to_string(),
                        description: "Generated by Amalgam".to_string(),
                        homepage: None,
                        repository: None,
                        license: Some("Apache-2.0".to_string()),
                        keywords: vec![],
                        authors: vec!["amalgam".to_string()],
                    },
                    formatting: NickelFormatting {
                        indent: 2,
                        max_line_length: 100,
                        sort_imports: true,
                        compact_records: false,
                    },
                }
            }
        };

        // Build pipeline from converted types
        let mut pipeline = UnifiedPipeline::new(input_source, output_target);

        // Set default transforms based on processing config
        let mut transforms = vec![Transform::NormalizeTypes, Transform::ResolveReferences];

        // Add special cases if configured
        if !stage.processing.special_cases.is_empty() {
            transforms.push(Transform::ApplySpecialCases { rules: vec![] });
        }

        pipeline.transforms = transforms;

        // Set module layout
        pipeline.layout = match stage.processing.layout.as_str() {
            "flat" => ModuleLayout::Flat {
                module_name: "types".to_string(),
            },
            "k8s" => ModuleLayout::K8s {
                consolidate_versions: true,
                include_alpha_beta: false,
                root_exports: vec![],
                api_group_structure: true,
            },
            "crossplane" => ModuleLayout::CrossPlane {
                group_by_version: true,
                api_extensions: false,
                provider_specific: false,
            },
            _ => ModuleLayout::Generic {
                namespace_pattern: "{domain}/{version}".to_string(),
                module_structure: ModuleStructure::Consolidated,
                version_handling: VersionHandling::Directories,
            },
        };

        // Execute the pipeline
        match pipeline.execute() {
            Ok(_result) => {
                info!("  ✓ Stage completed successfully");
                // Diagnostics are Vec<Diagnostic>, not PipelineDiagnostics
                // We'd need to convert them if we want to store them
            }
            Err(e) => {
                warn!("  ✗ Stage failed: {}", e);

                // Handle error based on recovery strategy
                match recovery_strategy {
                    RecoveryStrategy::FailFast => {
                        return Err(e.into());
                    }
                    RecoveryStrategy::Continue => {
                        // Log and continue to next stage
                        warn!("Continuing to next stage despite error");
                    }
                    RecoveryStrategy::BestEffort { .. } => {
                        // Try to recover if possible
                        // Recovery suggestion is in the error variant fields, not a method
                        warn!("Best effort recovery - continuing despite error");
                    }
                    RecoveryStrategy::Interactive { .. } => {
                        // In a real implementation, would prompt user
                        warn!("Interactive mode not fully implemented, continuing...");
                    }
                }
            }
        }
    }

    // Export diagnostics if requested
    if let Some(export_path) = export_diagnostics {
        use amalgam_core::pipeline::{MemoryUsage, PerformanceMetrics};

        let combined_diagnostics = PipelineDiagnostics {
            execution_id: uuid::Uuid::now_v7().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            duration_ms: 0, // Would be calculated from actual execution time
            stages: vec![], // Would collect from actual stage executions
            dependency_graph: None,
            symbol_table: None,
            memory_usage: MemoryUsage {
                peak_memory_mb: 0,
                ir_size_mb: 0.0,
                symbol_table_size_mb: 0.0,
                generated_code_size_mb: 0.0,
            },
            performance_metrics: PerformanceMetrics {
                parsing_time_ms: 0,
                transformation_time_ms: 0,
                layout_time_ms: 0,
                generation_time_ms: 0,
                io_time_ms: 0,
                cache_hits: 0,
                cache_misses: 0,
            },
            errors: vec![],
            warnings: vec![],
        };

        let diagnostics_json = serde_json::to_string_pretty(&combined_diagnostics)?;
        fs::write(&export_path, diagnostics_json)
            .with_context(|| format!("Failed to write diagnostics: {:?}", export_path))?;
        info!("Diagnostics exported to {:?}", export_path);
    }

    info!("Pipeline execution complete");
    Ok(())
}

/// Configuration for rich package generation
struct RichPackageGenConfig {
    input: PathBuf,
    output: PathBuf,
    name: String,
    version: String,
    package_type: String,
    patterns: bool,
    examples: bool,
    lsp_friendly: bool,
}

async fn handle_rich_package_generation(config: RichPackageGenConfig) -> Result<()> {
    let RichPackageGenConfig {
        input,
        output,
        name,
        version,
        package_type,
        patterns,
        examples,
        lsp_friendly,
    } = config;
    use amalgam_codegen::nickel_rich::RichPackageConfig;

    info!("Generating rich Nickel package: {}", name);

    // Create config based on package type
    let config = match package_type.as_str() {
        "k8s" => rich_package::default_k8s_config(),
        "crossplane-aws" => rich_package::default_crossplane_config("aws"),
        "crossplane-gcp" => rich_package::default_crossplane_config("gcp"),
        "crossplane-azure" => rich_package::default_crossplane_config("azure"),
        _ => RichPackageConfig {
            name: name.clone(),
            version,
            description: format!("Rich Nickel package for {}", name),
            generate_patterns: patterns,
            include_examples: examples,
            lsp_friendly,
            promoted_types: vec![],
            api_groups: vec![],
        },
    };

    // Generate the rich package
    rich_package::generate_rich_package(&input, &output, config).await?;

    info!("✓ Rich package generated successfully at {:?}", output);
    Ok(())
}
