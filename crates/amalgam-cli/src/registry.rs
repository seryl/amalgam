//! Registry management commands

use amalgam_registry::{Package, PackageIndex, Registry};
use anyhow::{Context, Result};
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum RegistryCommand {
    /// Initialize a new package registry
    Init {
        /// Directory to initialize the registry in
        #[arg(short, long, default_value = "./registry")]
        path: PathBuf,
    },

    /// Add a package to the registry
    Add {
        /// Path to the package directory
        #[arg(short, long)]
        package: PathBuf,

        /// Registry directory
        #[arg(short, long, default_value = "./registry")]
        registry: PathBuf,
    },

    /// List packages in the registry
    List {
        /// Registry directory
        #[arg(short, long, default_value = "./registry")]
        registry: PathBuf,

        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Search for packages
    Search {
        /// Search query
        query: String,

        /// Registry directory
        #[arg(short, long, default_value = "./registry")]
        registry: PathBuf,
    },

    /// Show package information
    Info {
        /// Package name
        package: String,

        /// Registry directory
        #[arg(short, long, default_value = "./registry")]
        registry: PathBuf,
    },

    /// Publish a package to the registry
    Publish {
        /// Path to the package directory
        #[arg(short, long)]
        package: PathBuf,

        /// Registry directory
        #[arg(short, long, default_value = "./registry")]
        registry: PathBuf,

        /// Dry run - validate without publishing
        #[arg(long)]
        dry_run: bool,
    },
}

impl RegistryCommand {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::Init { path } => init_registry(path).await,
            Self::Add { package, registry } => add_package(package, registry).await,
            Self::List { registry, verbose } => list_packages(registry, verbose).await,
            Self::Search { query, registry } => search_packages(query, registry).await,
            Self::Info { package, registry } => show_package_info(package, registry).await,
            Self::Publish {
                package,
                registry,
                dry_run,
            } => publish_package(package, registry, dry_run).await,
        }
    }
}

async fn init_registry(path: PathBuf) -> Result<()> {
    println!("Initializing registry at {:?}", path);

    std::fs::create_dir_all(&path)
        .with_context(|| format!("Failed to create registry directory at {:?}", path))?;

    let index = PackageIndex::new();
    let index_path = path.join("index.json");
    index
        .save(&index_path)
        .with_context(|| "Failed to save initial index")?;

    // Create packages directory
    std::fs::create_dir_all(path.join("packages"))
        .with_context(|| "Failed to create packages directory")?;

    println!("✓ Registry initialized successfully");
    Ok(())
}

async fn add_package(package_path: PathBuf, registry_path: PathBuf) -> Result<()> {
    println!("Adding package from {:?} to registry", package_path);

    let package = Package::load_from_path(&package_path)
        .with_context(|| format!("Failed to load package from {:?}", package_path))?;

    // Validate package
    package
        .validate()
        .with_context(|| "Package validation failed")?;

    let mut registry = Registry::load_from_path(&registry_path)
        .with_context(|| format!("Failed to load registry from {:?}", registry_path))?;

    let package_name = package.metadata.name.clone();
    let package_version = package.metadata.version.clone();

    registry
        .add_package(package)
        .with_context(|| "Failed to add package to registry")?;

    let index_path = registry_path.join("index.json");
    registry
        .save(&index_path)
        .with_context(|| "Failed to save updated index")?;

    println!("✓ Added {} {} to registry", package_name, package_version);
    Ok(())
}

async fn list_packages(registry_path: PathBuf, verbose: bool) -> Result<()> {
    let registry = Registry::load_from_path(&registry_path)
        .with_context(|| format!("Failed to load registry from {:?}", registry_path))?;

    let names = registry.package_names();

    if names.is_empty() {
        println!("No packages in registry");
        return Ok(());
    }

    println!("Packages in registry:");
    for name in names {
        if let Some(entry) = registry.find_package(&name) {
            if verbose {
                println!("\n  {} ({})", entry.name, entry.latest);
                if let Some(desc) = &entry.description {
                    println!("    {}", desc);
                }
                println!(
                    "    Versions: {}",
                    entry
                        .versions
                        .iter()
                        .map(|v| v.version.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                println!("    Categories: {}", entry.categories.join(", "));
            } else {
                println!("  {} ({})", entry.name, entry.latest);
            }
        }
    }

    if !verbose {
        println!("\nUse --verbose for more details");
    }

    Ok(())
}

async fn search_packages(query: String, registry_path: PathBuf) -> Result<()> {
    let registry = Registry::load_from_path(&registry_path)
        .with_context(|| format!("Failed to load registry from {:?}", registry_path))?;

    let results = registry.search(&query);

    if results.is_empty() {
        println!("No packages found matching '{}'", query);
        return Ok(());
    }

    println!("Found {} packages matching '{}':", results.len(), query);
    for entry in results {
        println!("  {} ({})", entry.name, entry.latest);
        if let Some(desc) = &entry.description {
            println!("    {}", desc);
        }
    }

    Ok(())
}

async fn show_package_info(package_name: String, registry_path: PathBuf) -> Result<()> {
    let registry = Registry::load_from_path(&registry_path)
        .with_context(|| format!("Failed to load registry from {:?}", registry_path))?;

    let entry = registry
        .find_package(&package_name)
        .ok_or_else(|| anyhow::anyhow!("Package '{}' not found", package_name))?;

    println!("Package: {}", entry.name);
    println!("Latest: {}", entry.latest);

    if let Some(desc) = &entry.description {
        println!("Description: {}", desc);
    }

    if !entry.categories.is_empty() {
        println!("Categories: {}", entry.categories.join(", "));
    }

    if !entry.keywords.is_empty() {
        println!("Keywords: {}", entry.keywords.join(", "));
    }

    if let Some(homepage) = &entry.homepage {
        println!("Homepage: {}", homepage);
    }

    if let Some(repo) = &entry.repository {
        println!("Repository: {}", repo);
    }

    println!("\nVersions:");
    for version in &entry.versions {
        let status = if version.yanked { " (yanked)" } else { "" };
        println!(
            "  {} - published {}{}",
            version.version,
            version.published_at.format("%Y-%m-%d"),
            status
        );

        if !version.dependencies.is_empty() {
            println!("    Dependencies:");
            for dep in &version.dependencies {
                let optional = if dep.optional { " (optional)" } else { "" };
                println!("      {} {}{}", dep.name, dep.version_req, optional);
            }
        }
    }

    println!(
        "\nCreated: {}",
        entry.created_at.format("%Y-%m-%d %H:%M:%S")
    );
    println!("Updated: {}", entry.updated_at.format("%Y-%m-%d %H:%M:%S"));

    Ok(())
}

async fn publish_package(
    package_path: PathBuf,
    registry_path: PathBuf,
    dry_run: bool,
) -> Result<()> {
    println!("Publishing package from {:?}", package_path);

    let package = Package::load_from_path(&package_path)
        .with_context(|| format!("Failed to load package from {:?}", package_path))?;

    // Validate package
    package
        .validate()
        .with_context(|| "Package validation failed")?;

    println!(
        "Package: {} {}",
        package.metadata.name, package.metadata.version
    );

    if dry_run {
        println!("✓ Package validation successful (dry run - not published)");
        return Ok(());
    }

    // Copy package to registry
    let dest_path = registry_path
        .join("packages")
        .join(&package.metadata.name)
        .join(&package.metadata.version);

    package
        .save(&dest_path)
        .with_context(|| "Failed to save package to registry")?;

    // Update index
    let mut registry = Registry::load_from_path(&registry_path)
        .with_context(|| format!("Failed to load registry from {:?}", registry_path))?;

    registry
        .add_package(package.clone())
        .with_context(|| "Failed to add package to index")?;

    let index_path = registry_path.join("index.json");
    registry
        .save(&index_path)
        .with_context(|| "Failed to save updated index")?;

    println!(
        "✓ Published {} {} successfully",
        package.metadata.name, package.metadata.version
    );

    Ok(())
}
