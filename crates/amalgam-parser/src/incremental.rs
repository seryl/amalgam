//! Incremental update support for all parser types
//!
//! This module implements fingerprinting for different source types
//! to enable intelligent change detection and incremental updates.

use amalgam_core::fingerprint::{
    ContentFingerprint, FingerprintBuilder, Fingerprintable, SourceInfo,
};
use std::path::Path;

/// URL-based source fingerprinting (GitHub, GitLab, etc.)
pub struct UrlSource {
    pub base_url: String,
    pub urls: Vec<String>,
    pub contents: Vec<String>,
}

impl Fingerprintable for UrlSource {
    fn create_fingerprint(&self) -> Result<ContentFingerprint, Box<dyn std::error::Error>> {
        let mut builder = FingerprintBuilder::new();

        // Add all content that affects generation
        for content in &self.contents {
            builder.add_content_str(content);
        }

        // Add metadata that could change
        builder.add_metadata("base_url", &self.base_url);
        builder.add_metadata("url_count", &self.urls.len().to_string());

        // TODO: Add ETags and Last-Modified headers when available
        let source_info = SourceInfo::UrlCollection {
            base_url: self.base_url.clone(),
            urls: self.urls.clone(),
            etags: vec![None; self.urls.len()], // Will be populated from HTTP headers
            last_modified: vec![None; self.urls.len()],
        };

        builder.with_source_info(source_info);

        Ok(builder.build())
    }
}

/// Kubernetes cluster source fingerprinting
pub struct K8sClusterSource {
    pub server_version: String,
    pub api_version: String,
    pub crd_specs: Vec<String>,
}

impl Fingerprintable for K8sClusterSource {
    fn create_fingerprint(&self) -> Result<ContentFingerprint, Box<dyn std::error::Error>> {
        let mut builder = FingerprintBuilder::new();

        // Hash all CRD specifications
        for spec in &self.crd_specs {
            builder.add_content_str(spec);
        }

        // Add server metadata
        builder.add_metadata("server_version", &self.server_version);
        builder.add_metadata("api_version", &self.api_version);
        builder.add_metadata("crd_count", &self.crd_specs.len().to_string());

        // Create API resources hash
        let mut api_hasher = sha2::Sha256::new();
        use sha2::Digest;
        for spec in &self.crd_specs {
            api_hasher.update(spec.as_bytes());
        }
        let api_resources_hash = format!("{:x}", api_hasher.finalize());

        let source_info = SourceInfo::K8sCluster {
            version: self.api_version.clone(),
            server_version: self.server_version.clone(),
            api_resources_hash,
        };

        builder.with_source_info(source_info);
        Ok(builder.build())
    }
}

/// Kubernetes core types (OpenAPI) fingerprinting
pub struct K8sCoreSource {
    pub version: String,
    pub openapi_spec: String,
    pub spec_url: String,
}

impl Fingerprintable for K8sCoreSource {
    fn create_fingerprint(&self) -> Result<ContentFingerprint, Box<dyn std::error::Error>> {
        let mut builder = FingerprintBuilder::new();

        // The OpenAPI spec content is what matters for generation
        builder.add_content_str(&self.openapi_spec);

        // Version and URL are metadata
        builder.add_metadata("k8s_version", &self.version);
        builder.add_metadata("spec_url", &self.spec_url);

        // Hash just the OpenAPI spec for the fingerprint
        let mut hasher = sha2::Sha256::new();
        use sha2::Digest;
        hasher.update(self.openapi_spec.as_bytes());
        let openapi_hash = format!("{:x}", hasher.finalize());

        let source_info = SourceInfo::K8sCore {
            version: self.version.clone(),
            openapi_hash,
            spec_url: self.spec_url.clone(),
        };

        builder.with_source_info(source_info);
        Ok(builder.build())
    }
}

/// Local files fingerprinting
pub struct LocalFilesSource {
    pub paths: Vec<String>,
    pub contents: Vec<String>,
}

impl Fingerprintable for LocalFilesSource {
    fn create_fingerprint(&self) -> Result<ContentFingerprint, Box<dyn std::error::Error>> {
        let mut builder = FingerprintBuilder::new();

        // Add all file contents
        for content in &self.contents {
            builder.add_content_str(content);
        }

        // Add metadata
        builder.add_metadata("file_count", &self.paths.len().to_string());
        for path in &self.paths {
            builder.add_metadata("file_path", path);
        }

        // Get file metadata
        let mut mtimes = Vec::new();
        let mut file_sizes = Vec::new();

        for path in &self.paths {
            if let Ok(metadata) = std::fs::metadata(path) {
                mtimes.push(
                    metadata
                        .modified()
                        .unwrap_or(std::time::SystemTime::UNIX_EPOCH),
                );
                file_sizes.push(metadata.len());
            } else {
                mtimes.push(std::time::SystemTime::UNIX_EPOCH);
                file_sizes.push(0);
            }
        }

        let source_info = SourceInfo::LocalFiles {
            paths: self.paths.clone(),
            mtimes,
            file_sizes,
        };

        builder.with_source_info(source_info);
        Ok(builder.build())
    }
}

/// Git repository fingerprinting
pub struct GitRepoSource {
    pub url: String,
    pub commit: String,
    pub paths: Vec<String>,
    pub contents: Vec<String>,
}

impl Fingerprintable for GitRepoSource {
    fn create_fingerprint(&self) -> Result<ContentFingerprint, Box<dyn std::error::Error>> {
        let mut builder = FingerprintBuilder::new();

        // Add all file contents from the repo
        for content in &self.contents {
            builder.add_content_str(content);
        }

        // Add Git metadata
        builder.add_metadata("git_url", &self.url);
        builder.add_metadata("git_commit", &self.commit);
        builder.add_metadata("path_count", &self.paths.len().to_string());

        let source_info = SourceInfo::GitRepo {
            url: self.url.clone(),
            commit: self.commit.clone(),
            paths: self.paths.clone(),
            http_metadata: None, // Could add ETags from GitHub API
        };

        builder.with_source_info(source_info);
        Ok(builder.build())
    }
}

/// High-level function to check if a package needs regeneration
pub fn needs_regeneration(
    output_dir: &Path,
    source: &dyn Fingerprintable,
) -> Result<bool, Box<dyn std::error::Error>> {
    let fingerprint_path = ContentFingerprint::fingerprint_path(output_dir);

    // If no previous fingerprint exists, we need to generate
    if !fingerprint_path.exists() {
        return Ok(true);
    }

    let last_fingerprint = ContentFingerprint::load_from_file(&fingerprint_path)?;
    source.has_changed(&last_fingerprint)
}

/// Save fingerprint after successful generation
pub fn save_fingerprint(
    output_dir: &Path,
    source: &dyn Fingerprintable,
) -> Result<(), Box<dyn std::error::Error>> {
    let fingerprint = source.create_fingerprint()?;
    let fingerprint_path = ContentFingerprint::fingerprint_path(output_dir);

    // Ensure directory exists
    if let Some(parent) = fingerprint_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    fingerprint.save_to_file(&fingerprint_path)?;
    Ok(())
}

/// Save fingerprint with output content tracking after successful generation
pub fn save_fingerprint_with_output(
    output_dir: &Path,
    source: &dyn Fingerprintable,
    manifest_content: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    use amalgam_core::fingerprint::FingerprintBuilder;

    // Create a new fingerprint that includes output content
    let source_fingerprint = source.create_fingerprint()?;

    let mut builder = FingerprintBuilder::new();

    // Copy source fingerprint data
    builder.with_source_info(source_fingerprint.source_info.clone());
    builder.add_content(source_fingerprint.content_hash.as_bytes());

    // Add output directory content
    if output_dir.exists() {
        builder.add_output_directory(output_dir)?;
    }

    // Add manifest content if provided
    if let Some(manifest) = manifest_content {
        builder.with_manifest_content(manifest);
    }

    let enhanced_fingerprint = builder.build();
    let fingerprint_path = ContentFingerprint::fingerprint_path(output_dir);

    // Ensure directory exists
    if let Some(parent) = fingerprint_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    enhanced_fingerprint.save_to_file(&fingerprint_path)?;
    Ok(())
}

/// Check what type of change occurred (for different update strategies)
#[derive(Debug, Clone)]
pub enum ChangeType {
    /// No changes detected
    NoChange,
    /// Only metadata changed (version, timestamps) - might update with same content
    MetadataOnly,
    /// Source content changed - full regeneration required
    ContentChanged,
    /// Generated output files were manually modified
    OutputChanged,
    /// Manifest changed (packages added/removed/modified)
    ManifestChanged,
    /// No previous fingerprint - first generation
    FirstGeneration,
}

pub fn detect_change_type(
    output_dir: &Path,
    source: &dyn Fingerprintable,
) -> Result<ChangeType, Box<dyn std::error::Error>> {
    let fingerprint_path = ContentFingerprint::fingerprint_path(output_dir);

    if !fingerprint_path.exists() {
        return Ok(ChangeType::FirstGeneration);
    }

    let last_fingerprint = ContentFingerprint::load_from_file(&fingerprint_path)?;
    let current_fingerprint = source.create_fingerprint()?;

    // Check different types of changes in priority order
    if current_fingerprint.content_matches(&last_fingerprint) {
        Ok(ChangeType::NoChange)
    } else if current_fingerprint.manifest_changed(&last_fingerprint) {
        Ok(ChangeType::ManifestChanged)
    } else if current_fingerprint.content_changed(&last_fingerprint) {
        Ok(ChangeType::ContentChanged)
    } else if current_fingerprint.output_changed(&last_fingerprint) {
        Ok(ChangeType::OutputChanged)
    } else if current_fingerprint.metadata_changed(&last_fingerprint) {
        Ok(ChangeType::MetadataOnly)
    } else {
        // Shouldn't happen, but default to content changed to be safe
        Ok(ChangeType::ContentChanged)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_url_source_fingerprinting() {
        let source = UrlSource {
            base_url: "https://example.com".to_string(),
            urls: vec!["file1.yaml".to_string(), "file2.yaml".to_string()],
            contents: vec!["content1".to_string(), "content2".to_string()],
        };

        let fingerprint = source.create_fingerprint().unwrap();
        assert!(!fingerprint.content_hash.is_empty());
        assert!(!fingerprint.combined_hash.is_empty());

        // Same content should produce same fingerprint
        let source2 = UrlSource {
            base_url: "https://example.com".to_string(),
            urls: vec!["file1.yaml".to_string(), "file2.yaml".to_string()],
            contents: vec!["content1".to_string(), "content2".to_string()],
        };
        let fingerprint2 = source2.create_fingerprint().unwrap();
        assert!(fingerprint.content_matches(&fingerprint2));
    }

    #[test]
    fn test_needs_regeneration() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path();

        let source = UrlSource {
            base_url: "https://example.com".to_string(),
            urls: vec!["file1.yaml".to_string()],
            contents: vec!["content1".to_string()],
        };

        // First time should need regeneration
        assert!(needs_regeneration(output_dir, &source).unwrap());

        // Save fingerprint
        save_fingerprint(output_dir, &source).unwrap();

        // Second time should not need regeneration
        assert!(!needs_regeneration(output_dir, &source).unwrap());

        // Changed content should need regeneration
        let changed_source = UrlSource {
            base_url: "https://example.com".to_string(),
            urls: vec!["file1.yaml".to_string()],
            contents: vec!["different_content".to_string()],
        };
        assert!(needs_regeneration(output_dir, &changed_source).unwrap());
    }

    #[test]
    fn test_change_type_detection() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path();

        let source = UrlSource {
            base_url: "https://example.com".to_string(),
            urls: vec!["file1.yaml".to_string()],
            contents: vec!["content1".to_string()],
        };

        // First generation
        match detect_change_type(output_dir, &source).unwrap() {
            ChangeType::FirstGeneration => {}
            other => panic!("Expected FirstGeneration, got {:?}", other),
        }

        // Save fingerprint
        save_fingerprint(output_dir, &source).unwrap();

        // No change
        match detect_change_type(output_dir, &source).unwrap() {
            ChangeType::NoChange => {}
            other => panic!("Expected NoChange, got {:?}", other),
        }

        // Content change
        let changed_source = UrlSource {
            base_url: "https://example.com".to_string(),
            urls: vec!["file1.yaml".to_string()],
            contents: vec!["different_content".to_string()],
        };
        match detect_change_type(output_dir, &changed_source).unwrap() {
            ChangeType::ContentChanged => {}
            other => panic!("Expected ContentChanged, got {:?}", other),
        }
    }
}
