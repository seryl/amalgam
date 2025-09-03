use std::fs;
use std::path::Path;

fn main() {
    // Read the k8s version from the manifest at build time
    let manifest_path = Path::new("../../.amalgam-manifest.toml");

    let k8s_version = if manifest_path.exists() {
        let content =
            fs::read_to_string(manifest_path).expect("Failed to read .amalgam-manifest.toml");

        // Parse TOML to extract k8s version, no fallback - if we can't parse it, fail the build
        extract_k8s_version(&content)
            .expect("Could not find k8s-core package version in .amalgam-manifest.toml")
    } else {
        panic!("Missing .amalgam-manifest.toml file - required for build-time version extraction");
    };

    // Make the version available as an environment variable at compile time
    println!("cargo:rustc-env=DEFAULT_K8S_VERSION={}", k8s_version);

    // Tell cargo to re-run if the manifest changes
    println!("cargo:rerun-if-changed=../../.amalgam-manifest.toml");
}

fn extract_k8s_version(toml_content: &str) -> Option<String> {
    // Simple TOML parsing to find k8s-core package version
    let lines: Vec<&str> = toml_content.lines().collect();
    let mut in_k8s_package = false;

    for i in 0..lines.len() {
        let line = lines[i].trim();

        // Check if we're starting a new [[packages]] section
        if line.starts_with("[[packages]]") {
            in_k8s_package = false;
        }

        // Check if this package is type = "k8s-core"
        if line.starts_with("type = \"k8s-core\"") {
            in_k8s_package = true;
        }

        // If we're in a k8s-core package and find a version, extract it
        if in_k8s_package && line.starts_with("version = \"") {
            if let Some(start) = line.find("\"") {
                if let Some(end) = line[start + 1..].find("\"") {
                    let version = &line[start + 1..start + 1 + end];
                    return Some(version.to_string());
                }
            }
        }
    }
    None
}
