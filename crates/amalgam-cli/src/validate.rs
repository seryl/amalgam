//! Validation module for Nickel packages and files

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::{error, info, warn};

/// Run validation on a Nickel package or file
pub fn run_validation(path: &Path) -> Result<()> {
    run_validation_with_package_path(path, None)
}

/// Run validation on a Nickel package or file with optional package path
pub fn run_validation_with_package_path(path: &Path, package_path: Option<&Path>) -> Result<()> {
    info!("Validating Nickel package/file at {:?}", path);
    if let Some(pkg_path) = package_path {
        info!("Using package path prefix: {:?}", pkg_path);
    }

    // First check if we have a local nickel binary available
    let nickel_binary = find_nickel_binary()?;

    if path.is_file() {
        validate_single_file_with_package_path(&nickel_binary, path, package_path)
    } else if path.is_dir() {
        validate_package_with_package_path(&nickel_binary, path, package_path)
    } else {
        anyhow::bail!("Path {} does not exist", path.display())
    }
}

/// Find the nickel binary to use
fn find_nickel_binary() -> Result<String> {
    // First, check if we have a local override in nickel/
    let local_nickel = Path::new("nickel/target/release/nickel");
    if local_nickel.exists() {
        info!("Using local Nickel binary from nickel/target/release/nickel");
        return Ok(local_nickel.canonicalize()?.display().to_string());
    }

    let local_nickel_debug = Path::new("nickel/target/debug/nickel");
    if local_nickel_debug.exists() {
        info!("Using local Nickel binary from nickel/target/debug/nickel");
        return Ok(local_nickel_debug.canonicalize()?.display().to_string());
    }

    // Check if nickel is in PATH
    if let Ok(output) = Command::new("which").arg("nickel").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            info!("Using system Nickel binary from {}", path);
            return Ok("nickel".to_string());
        }
    }

    anyhow::bail!(
        "Nickel binary not found. Please either:\n\
        1. Build nickel locally: cd nickel && cargo build --release\n\
        2. Install nickel: cargo install nickel-lang-cli\n\
        3. Use nix: nix-shell -p nickel"
    )
}

/// Validate a single Nickel file
fn validate_single_file(nickel_binary: &str, file: &Path) -> Result<()> {
    validate_single_file_with_package_path(nickel_binary, file, None)
}

/// Validate a single Nickel file with optional package path
fn validate_single_file_with_package_path(
    nickel_binary: &str,
    file: &Path,
    package_path: Option<&Path>,
) -> Result<()> {
    info!("Validating single file: {}", file.display());

    // Build the command with optional package path
    let mut cmd = Command::new(nickel_binary);

    // If package_path is provided, set NICKEL_IMPORT_PATH environment variable
    if let Some(pkg_path) = package_path {
        cmd.env("NICKEL_IMPORT_PATH", pkg_path.display().to_string());
    }

    // Use `nickel typecheck` to validate the file
    let output = cmd
        .arg("typecheck")
        .arg(file)
        .output()
        .context("Failed to run nickel typecheck")?;

    if output.status.success() {
        info!("✓ {} validates successfully", file.display());

        // Also try to parse it
        let parse_output = Command::new(nickel_binary)
            .arg("eval")
            .arg("--field")
            .arg("dummy") // Just parse, don't evaluate
            .arg(file)
            .env("NICKEL_IMPORT_RESOLUTION", "1")
            .output();

        if let Ok(parse_result) = parse_output {
            if !parse_result.status.success() {
                let stderr = String::from_utf8_lossy(&parse_result.stderr);
                if !stderr.contains("field `dummy` not found") {
                    warn!("Parse warnings:\n{}", stderr);
                }
            }
        }

        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("✗ {} validation failed:\n{}", file.display(), stderr);
        anyhow::bail!("Validation failed")
    }
}

/// Validate a Nickel package directory
fn validate_package(nickel_binary: &str, dir: &Path) -> Result<()> {
    validate_package_with_package_path(nickel_binary, dir, None)
}

/// Validate a Nickel package directory with optional package path
fn validate_package_with_package_path(
    nickel_binary: &str,
    dir: &Path,
    package_path: Option<&Path>,
) -> Result<()> {
    info!("Validating package directory: {}", dir.display());

    // Check for mod.ncl as the entry point
    let mod_file = dir.join("mod.ncl");
    if mod_file.exists() {
        info!("Found mod.ncl, validating as package");

        // Set up import resolution for the package
        std::env::set_current_dir(dir).context("Failed to change to package directory")?;

        // Validate the main module
        validate_single_file_with_package_path(nickel_binary, Path::new("mod.ncl"), package_path)?;

        // Also check for Nickel-pkg.ncl
        let pkg_manifest = Path::new("Nickel-pkg.ncl");
        if pkg_manifest.exists() {
            info!("Found Nickel-pkg.ncl, validating package manifest");

            // Use nickel package commands if available
            let pkg_check = Command::new(nickel_binary)
                .arg("package")
                .arg("lock")
                .arg("--dry-run")
                .output();

            match pkg_check {
                Ok(output) if output.status.success() => {
                    info!("✓ Package manifest validates successfully");
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if stderr.contains("unrecognized subcommand") {
                        warn!("Package commands not available in this Nickel version");
                        // Fall back to basic validation
                        validate_single_file_with_package_path(
                            nickel_binary,
                            pkg_manifest,
                            package_path,
                        )?;
                    } else {
                        warn!("Package manifest warnings:\n{}", stderr);
                    }
                }
                Err(e) => {
                    warn!("Could not check package manifest: {}", e);
                }
            }
        }

        info!("✓ Package validates successfully");
        Ok(())
    } else {
        // Validate all .ncl files in the directory
        info!("No mod.ncl found, validating individual files");

        let mut all_ok = true;
        let mut validated_count = 0;

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "ncl") {
                match validate_single_file(nickel_binary, &path) {
                    Ok(()) => validated_count += 1,
                    Err(e) => {
                        error!("Failed to validate {}: {}", path.display(), e);
                        all_ok = false;
                    }
                }
            }
        }

        if validated_count == 0 {
            warn!("No .ncl files found in directory");
        } else if all_ok {
            info!("✓ All {} files validated successfully", validated_count);
        } else {
            anyhow::bail!("Some files failed validation");
        }

        Ok(())
    }
}
