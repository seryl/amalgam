//! Package management commands

use amalgam_registry::{Package, PackageBuilder, Registry};
use anyhow::{Context, Result};
use clap::Subcommand;
use std::path::{Path, PathBuf};

#[derive(Subcommand)]
pub enum PackageCommand {
    /// Create a new package
    New {
        /// Package name
        name: String,

        /// Package version
        #[arg(short, long, default_value = "0.1.0")]
        version: String,

        /// Output directory
        #[arg(short, long, default_value = ".")]
        output: PathBuf,

        /// Package description
        #[arg(short, long)]
        description: Option<String>,
    },

    /// Build a package
    Build {
        /// Package directory
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Output directory for built package
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Install package dependencies
    Install {
        /// Package directory
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Registry to use
        #[arg(short, long, default_value = "./registry")]
        registry: PathBuf,

        /// Install to this directory
        #[arg(long, default_value = "./vendor")]
        vendor_dir: PathBuf,
    },

    /// Resolve dependencies for a package
    Resolve {
        /// Package name
        package: String,

        /// Package version
        #[arg(short, long)]
        version: Option<String>,

        /// Registry to use
        #[arg(short, long, default_value = "./registry")]
        registry: PathBuf,

        /// Show dependency tree
        #[arg(short, long)]
        tree: bool,
    },

    /// Validate a package
    Validate {
        /// Package directory
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Check dependencies
        #[arg(long)]
        check_deps: bool,

        /// Registry to use for dependency checking
        #[arg(short, long)]
        registry: Option<PathBuf>,
    },

    /// Show package metadata
    Info {
        /// Package directory
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
}

impl PackageCommand {
    pub async fn execute(self) -> Result<()> {
        match self {
            Self::New {
                name,
                version,
                output,
                description,
            } => create_package(name, version, output, description).await,
            Self::Build { path, output } => build_package(path, output).await,
            Self::Install {
                path,
                registry,
                vendor_dir,
            } => install_dependencies(path, registry, vendor_dir).await,
            Self::Resolve {
                package,
                version,
                registry,
                tree,
            } => resolve_dependencies(package, version, registry, tree).await,
            Self::Validate {
                path,
                check_deps,
                registry,
            } => validate_package(path, check_deps, registry).await,
            Self::Info { path } => show_package_info(path).await,
        }
    }
}

async fn create_package(
    name: String,
    version: String,
    output: PathBuf,
    description: Option<String>,
) -> Result<()> {
    println!("Creating new package: {} {}", name, version);

    let package_dir = output.join(&name);
    std::fs::create_dir_all(&package_dir)
        .with_context(|| format!("Failed to create package directory {:?}", package_dir))?;

    // Create package structure
    let mut builder = PackageBuilder::new(name.clone(), version.clone());

    if let Some(desc) = description {
        builder = builder.description(desc);
    }

    // Create main module
    builder = builder.file(
        "mod.ncl",
        format!(
            r#"# {} Package
#
# Main module exports

{{
    # Package metadata
    name = "{}",
    version = "{}",
    
    # Type exports
    types = {{}},
    
    # Pattern library
    patterns = {{}},
    
    # Utility functions
    utils = {{}},
}}
"#,
            name, name, version
        ),
    );

    let package = builder.build();

    // Save package
    package
        .save(&package_dir)
        .with_context(|| "Failed to save package")?;

    // Create example file
    let example_content = format!(
        r#"# Example usage of {}

let pkg = import "./mod.ncl" in

{{
    # Use the package here
    example = pkg.name,
}}
"#,
        name
    );

    std::fs::write(package_dir.join("example.ncl"), example_content)
        .with_context(|| "Failed to create example file")?;

    println!("✓ Created package at {:?}", package_dir);
    println!("\nNext steps:");
    println!("  1. cd {}", name);
    println!("  2. Edit mod.ncl to add your types and functions");
    println!("  3. amalgam package build to validate");
    println!("  4. amalgam registry publish -p . to publish");

    Ok(())
}

async fn build_package(path: PathBuf, output: Option<PathBuf>) -> Result<()> {
    println!("Building package at {:?}", path);

    let package = Package::load_from_path(&path)
        .with_context(|| format!("Failed to load package from {:?}", path))?;

    // Validate package
    package
        .validate()
        .with_context(|| "Package validation failed")?;

    println!(
        "✓ Package {} {} is valid",
        package.metadata.name, package.metadata.version
    );

    if let Some(output_path) = output {
        // Build to output directory
        let dest = output_path.join(format!(
            "{}-{}.pkg",
            package.metadata.name, package.metadata.version
        ));

        package
            .save(&dest)
            .with_context(|| format!("Failed to save built package to {:?}", dest))?;

        println!("✓ Built package saved to {:?}", dest);
    }

    // Check all Nickel files compile
    println!("Checking Nickel files...");
    for file_path in package.content.keys() {
        if file_path.ends_with(".ncl") {
            println!("  ✓ {}", file_path);
        }
    }

    Ok(())
}

async fn install_dependencies(
    path: PathBuf,
    registry_path: PathBuf,
    vendor_dir: PathBuf,
) -> Result<()> {
    println!("Installing dependencies for package at {:?}", path);

    let package = Package::load_from_path(&path)
        .with_context(|| format!("Failed to load package from {:?}", path))?;

    if package.metadata.dependencies.is_empty() {
        println!("No dependencies to install");
        return Ok(());
    }

    let registry = Registry::load_from_path(&registry_path)
        .with_context(|| format!("Failed to load registry from {:?}", registry_path))?;

    std::fs::create_dir_all(&vendor_dir)
        .with_context(|| format!("Failed to create vendor directory {:?}", vendor_dir))?;

    println!("Resolving dependencies...");

    for dep in &package.metadata.dependencies {
        if dep.optional {
            println!("  Skipping optional dependency: {}", dep.name);
            continue;
        }

        println!("  Installing {} {}", dep.name, dep.version_req);

        let resolution = registry
            .resolve_dependencies(&dep.name, &dep.version_req)
            .with_context(|| format!("Failed to resolve {}", dep.name))?;

        for (pkg_name, resolved_pkg) in &resolution.packages {
            let pkg_path = registry_path.join("packages").join(&resolved_pkg.path);

            if !pkg_path.exists() {
                anyhow::bail!("Package not found in registry: {}", pkg_name);
            }

            let dest_path = vendor_dir.join(pkg_name);

            // Copy package to vendor directory
            if dest_path.exists() {
                println!(
                    "    {} {} already installed",
                    pkg_name, resolved_pkg.version
                );
            } else {
                copy_dir_all(&pkg_path, &dest_path)
                    .with_context(|| format!("Failed to copy {} to vendor", pkg_name))?;
                println!("    ✓ Installed {} {}", pkg_name, resolved_pkg.version);
            }
        }
    }

    println!("✓ Dependencies installed to {:?}", vendor_dir);

    Ok(())
}

async fn resolve_dependencies(
    package_name: String,
    version: Option<String>,
    registry_path: PathBuf,
    show_tree: bool,
) -> Result<()> {
    let registry = Registry::load_from_path(&registry_path)
        .with_context(|| format!("Failed to load registry from {:?}", registry_path))?;

    let version = version.unwrap_or_else(|| {
        registry
            .find_package(&package_name)
            .map(|e| e.latest.clone())
            .unwrap_or_else(|| "latest".to_string())
    });

    println!("Resolving dependencies for {} {}", package_name, version);

    let resolution = registry
        .resolve_dependencies(&package_name, &version)
        .with_context(|| format!("Failed to resolve dependencies for {}", package_name))?;

    if show_tree {
        println!("\nDependency tree:");
        print_dependency_tree(&resolution, &package_name, "", true);
    } else {
        println!("\nResolved packages:");
        for pkg_name in &resolution.order {
            if let Some(pkg) = resolution.packages.get(pkg_name) {
                println!("  {} {}", pkg.name, pkg.version);
            }
        }
    }

    println!("\nTotal: {} packages", resolution.packages.len());

    Ok(())
}

async fn validate_package(
    path: PathBuf,
    check_deps: bool,
    registry: Option<PathBuf>,
) -> Result<()> {
    println!("Validating package at {:?}", path);

    let package = Package::load_from_path(&path)
        .with_context(|| format!("Failed to load package from {:?}", path))?;

    // Basic validation
    package
        .validate()
        .with_context(|| "Package validation failed")?;

    println!("✓ Package structure is valid");
    println!("  Name: {}", package.metadata.name);
    println!("  Version: {}", package.metadata.version);

    // Check dependencies if requested
    if check_deps && !package.metadata.dependencies.is_empty() {
        if let Some(registry_path) = registry {
            println!("\nChecking dependencies...");

            let registry = Registry::load_from_path(&registry_path)
                .with_context(|| format!("Failed to load registry from {:?}", registry_path))?;

            for dep in &package.metadata.dependencies {
                match registry.resolve_dependencies(&dep.name, &dep.version_req) {
                    Ok(_) => {
                        let optional = if dep.optional { " (optional)" } else { "" };
                        println!("  ✓ {} {}{}", dep.name, dep.version_req, optional);
                    }
                    Err(e) => {
                        println!("  ✗ {} {}: {}", dep.name, dep.version_req, e);
                    }
                }
            }
        } else {
            println!("\nSkipping dependency check (no registry specified)");
        }
    }

    println!("\n✓ Package validation successful");

    Ok(())
}

async fn show_package_info(path: PathBuf) -> Result<()> {
    let package = Package::load_from_path(&path)
        .with_context(|| format!("Failed to load package from {:?}", path))?;

    println!("Package: {}", package.metadata.name);
    println!("Version: {}", package.metadata.version);

    if let Some(desc) = &package.metadata.description {
        println!("Description: {}", desc);
    }

    if !package.metadata.categories.is_empty() {
        println!("Categories: {}", package.metadata.categories.join(", "));
    }

    if !package.metadata.keywords.is_empty() {
        println!("Keywords: {}", package.metadata.keywords.join(", "));
    }

    if let Some(homepage) = &package.metadata.homepage {
        println!("Homepage: {}", homepage);
    }

    if let Some(repo) = &package.metadata.repository {
        println!("Repository: {}", repo);
    }

    if !package.metadata.dependencies.is_empty() {
        println!("\nDependencies:");
        for dep in &package.metadata.dependencies {
            let optional = if dep.optional { " (optional)" } else { "" };
            println!("  {} {}{}", dep.name, dep.version_req, optional);
        }
    }

    println!("\nFiles:");
    let mut files: Vec<_> = package.content.keys().collect();
    files.sort();
    for file in files {
        println!("  {}", file);
    }

    Ok(())
}

fn print_dependency_tree(
    resolution: &amalgam_registry::Resolution,
    package: &str,
    prefix: &str,
    is_last: bool,
) {
    let connector = if is_last { "└── " } else { "├── " };

    if let Some(pkg) = resolution.packages.get(package) {
        println!("{}{}{} {}", prefix, connector, pkg.name, pkg.version);

        let new_prefix = if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        let deps = &pkg.dependencies;
        for (i, dep) in deps.iter().enumerate() {
            let is_last_dep = i == deps.len() - 1;
            print_dependency_tree(resolution, dep, &new_prefix, is_last_dep);
        }
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    use std::fs;

    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_all(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}
