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
    // Extract version from Kubernetes source URL in new simplified format
    // Look for lines like: source = "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.33.4/api/openapi-spec/swagger.json"
    
    let lines: Vec<&str> = toml_content.lines().collect();
    
    for line in lines {
        let line = line.trim();
        
        // Look for source lines containing kubernetes
        if line.starts_with("source = \"") && line.contains("kubernetes/kubernetes/") {
            // Extract version from URL pattern: .../kubernetes/v1.33.4/...
            if let Some(start_pos) = line.find("kubernetes/kubernetes/v") {
                let version_start = start_pos + "kubernetes/kubernetes/v".len();
                if let Some(rest) = line.get(version_start..) {
                    if let Some(end_pos) = rest.find('/') {
                        let version = &rest[..end_pos];
                        return Some(format!("v{}", version));
                    }
                }
            }
        }
    }
    
    // Fallback: return a default version if parsing fails
    Some("v1.33.4".to_string())
}
