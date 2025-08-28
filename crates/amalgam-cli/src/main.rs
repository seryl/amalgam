use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use tracing::info;

use amalgam_codegen::{go::GoCodegen, nickel::NickelCodegen, Codegen};
use amalgam_parser::{
    crd::{CRDParser, CRD},
    openapi::OpenAPIParser,
    Parser as SchemaParser,
};

mod vendor;

#[derive(Parser)]
#[command(name = "amalgam")]
#[command(about = "Generate type-safe Nickel configurations from any schema source", long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Enable debug output
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
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

        /// Target language
        #[arg(short, long, default_value = "nickel")]
        target: String,
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
        Commands::Import { source } => handle_import(source).await,
        Commands::Generate {
            input,
            output,
            target,
        } => handle_generate(input, output, &target),
        Commands::Convert {
            input,
            from,
            output,
            to,
        } => handle_convert(input, &from, output, &to),
        Commands::Vendor { command } => {
            let project_root = std::env::current_dir()?;
            let manager = vendor::VendorManager::new(project_root);
            manager.execute(command).await
        }
    }
}

async fn handle_import(source: ImportSource) -> Result<()> {
    match source {
        ImportSource::Url {
            url,
            output,
            package,
        } => {
            info!("Fetching CRDs from URL: {}", url);

            // Determine package name
            let package_name = package.unwrap_or_else(|| {
                url.split('/')
                    .last()
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

            // Generate package structure
            let mut generator = amalgam_parser::package::PackageGenerator::new(
                package_name.clone(),
                output.clone(),
            );
            generator.add_crds(crds);

            let package_structure = generator.generate_package()?;

            // Create output directory structure
            fs::create_dir_all(&output)?;

            // Write main module file
            let main_module = package_structure.generate_main_module();
            fs::write(output.join("mod.ncl"), main_module)?;

            // Create group/version/kind structure
            for group in package_structure.groups() {
                let group_dir = output.join(&group);
                fs::create_dir_all(&group_dir)?;

                // Write group module
                if let Some(group_mod) = package_structure.generate_group_module(&group) {
                    fs::write(group_dir.join("mod.ncl"), group_mod)?;
                }

                // Create version directories
                for version in package_structure.versions(&group) {
                    let version_dir = group_dir.join(&version);
                    fs::create_dir_all(&version_dir)?;

                    // Write version module
                    if let Some(version_mod) =
                        package_structure.generate_version_module(&group, &version)
                    {
                        fs::write(version_dir.join("mod.ncl"), version_mod)?;
                    }

                    // Write individual kind files
                    for kind in package_structure.kinds(&group, &version) {
                        if let Some(kind_content) =
                            package_structure.generate_kind_file(&group, &version, &kind)
                        {
                            fs::write(version_dir.join(format!("{}.ncl", kind)), kind_content)?;
                        }
                    }
                }
            }

            info!("Generated package '{}' in {:?}", package_name, output);
            info!("Package structure:");
            for group in package_structure.groups() {
                info!("  {}/", group);
                for version in package_structure.versions(&group) {
                    let kinds = package_structure.kinds(&group, &version);
                    info!("    {}/: {} types", version, kinds.len());
                }
            }

            Ok(())
        }

        ImportSource::Crd { file, output } => {
            info!("Importing CRD from {:?}", file);

            let content = fs::read_to_string(&file)
                .with_context(|| format!("Failed to read CRD file: {:?}", file))?;

            let crd: CRD = if file.extension().map_or(false, |ext| ext == "json") {
                serde_json::from_str(&content)?
            } else {
                serde_yaml::from_str(&content)?
            };

            let parser = CRDParser::new();
            let mut ir = parser.parse(crd.clone())?;

            // Add imports for any k8s type references
            use amalgam_core::ir::Import;
            use amalgam_parser::imports::ImportResolver;

            // Analyze the IR for external references and add imports
            for module in &mut ir.modules {
                let mut import_resolver = ImportResolver::new();

                // Analyze all types in the module
                for type_def in &module.types {
                    import_resolver.analyze_type(&type_def.ty);
                }

                // Generate imports based on detected references
                for type_ref in import_resolver.references() {
                    // Get group and version from the CRD
                    let group = &crd.spec.group;
                    let version = crd
                        .spec
                        .versions
                        .first()
                        .map(|v| v.name.as_str())
                        .unwrap_or("v1");

                    // Convert TypeReference to Import
                    let import_path = type_ref.import_path(group, version);
                    let alias = Some(type_ref.module_alias());

                    tracing::debug!(
                        "Adding import for {:?} -> path: {}, alias: {:?}",
                        type_ref,
                        import_path,
                        alias
                    );

                    module.imports.push(Import {
                        path: import_path,
                        alias,
                        items: vec![], // Empty items means import the whole module
                    });
                }

                tracing::debug!(
                    "Module {} has {} imports",
                    module.name,
                    module.imports.len()
                );
            }

            // Generate Nickel code by default
            let mut codegen = NickelCodegen::new();
            let code = codegen.generate(&ir)?;

            if let Some(output_path) = output {
                fs::write(&output_path, code)
                    .with_context(|| format!("Failed to write output: {:?}", output_path))?;
                info!("Generated Nickel code written to {:?}", output_path);
            } else {
                println!("{}", code);
            }

            Ok(())
        }

        ImportSource::OpenApi { file, output } => {
            info!("Importing OpenAPI spec from {:?}", file);

            let content = fs::read_to_string(&file)
                .with_context(|| format!("Failed to read OpenAPI file: {:?}", file))?;

            let spec: openapiv3::OpenAPI = if file.extension().map_or(false, |ext| ext == "json") {
                serde_json::from_str(&content)?
            } else {
                serde_yaml::from_str(&content)?
            };

            let parser = OpenAPIParser::new();
            let mut ir = parser.parse(spec)?;

            // Add imports for any k8s type references
            use amalgam_core::ir::Import;
            use amalgam_parser::imports::ImportResolver;

            // Analyze the IR for external references and add imports
            for module in &mut ir.modules {
                let mut import_resolver = ImportResolver::new();

                // Analyze all types in the module
                for type_def in &module.types {
                    import_resolver.analyze_type(&type_def.ty);
                }

                // Generate imports based on detected references
                for type_ref in import_resolver.references() {
                    // For OpenAPI, use a default group/version or extract from the spec
                    let group = "api"; // Default group for OpenAPI specs
                    let version = "v1"; // Default version

                    // Convert TypeReference to Import
                    let import_path = type_ref.import_path(group, version);
                    let alias = Some(type_ref.module_alias());

                    tracing::debug!(
                        "Adding import for {:?} -> path: {}, alias: {:?}",
                        type_ref,
                        import_path,
                        alias
                    );

                    module.imports.push(Import {
                        path: import_path,
                        alias,
                        items: vec![], // Empty items means import the whole module
                    });
                }

                tracing::debug!(
                    "Module {} has {} imports",
                    module.name,
                    module.imports.len()
                );
            }

            // Generate Nickel code by default
            let mut codegen = NickelCodegen::new();
            let code = codegen.generate(&ir)?;

            if let Some(output_path) = output {
                fs::write(&output_path, code)
                    .with_context(|| format!("Failed to write output: {:?}", output_path))?;
                info!("Generated Nickel code written to {:?}", output_path);
            } else {
                println!("{}", code);
            }

            Ok(())
        }

        ImportSource::K8s { .. } => {
            anyhow::bail!("Kubernetes import not yet implemented. Build with --features kubernetes to enable.")
        }
    }
}

fn handle_generate(input: PathBuf, output: PathBuf, target: &str) -> Result<()> {
    info!("Generating {} code from {:?}", target, input);

    let ir_content = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read IR file: {:?}", input))?;

    let ir: amalgam_core::IR =
        serde_json::from_str(&ir_content).with_context(|| "Failed to parse IR JSON")?;

    let code = match target {
        "nickel" => {
            let mut codegen = NickelCodegen::new();
            codegen.generate(&ir)?
        }
        "go" => {
            let mut codegen = GoCodegen::new();
            codegen.generate(&ir)?
        }
        _ => {
            anyhow::bail!("Unsupported target language: {}", target);
        }
    };

    fs::write(&output, code).with_context(|| format!("Failed to write output: {:?}", output))?;

    info!("Generated code written to {:?}", output);
    Ok(())
}

fn handle_convert(input: PathBuf, from: &str, output: PathBuf, to: &str) -> Result<()> {
    info!("Converting from {} to {}", from, to);

    let content = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read input file: {:?}", input))?;

    // Parse input to IR
    let ir = match from {
        "crd" => {
            let crd: CRD = if input.extension().map_or(false, |ext| ext == "json") {
                serde_json::from_str(&content)?
            } else {
                serde_yaml::from_str(&content)?
            };
            CRDParser::new().parse(crd)?
        }
        "openapi" => {
            let spec: openapiv3::OpenAPI = if input.extension().map_or(false, |ext| ext == "json") {
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
            let mut codegen = NickelCodegen::new();
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
