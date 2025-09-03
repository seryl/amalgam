//! Content fingerprinting for intelligent change detection
//!
//! This module provides universal change detection across all source types
//! by creating content-based fingerprints that capture everything affecting
//! code generation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::time::SystemTime;

/// Universal content fingerprint for change detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentFingerprint {
    /// Hash of all content that affects code generation
    pub content_hash: String,
    /// Source-specific metadata hash (URLs, versions, etc.)  
    pub metadata_hash: String,
    /// Combined hash for quick comparison
    pub combined_hash: String,
    /// Hash of all generated output files
    pub output_hash: Option<String>,
    /// Manifest hash (for tracking manifest changes)
    pub manifest_hash: Option<String>,
    /// When this fingerprint was created
    pub created_at: DateTime<Utc>,
    /// Source type and location information
    pub source_info: SourceInfo,
    /// Version of amalgam that created this fingerprint
    pub amalgam_version: String,
    /// List of generated files with their individual hashes
    pub generated_files: Option<BTreeMap<String, String>>,
}

/// Source-specific information for different ingest types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceInfo {
    /// Git repository source
    GitRepo {
        url: String,
        commit: String,
        paths: Vec<String>,
        /// ETags or last-modified headers if available
        http_metadata: Option<BTreeMap<String, String>>,
    },
    /// Kubernetes cluster API
    K8sCluster {
        version: String,
        server_version: String,
        /// Hash of all CRD resource versions
        api_resources_hash: String,
    },
    /// Collection of URLs (like GitHub file listings)
    UrlCollection {
        base_url: String,
        urls: Vec<String>,
        etags: Vec<Option<String>>,
        last_modified: Vec<Option<DateTime<Utc>>>,
    },
    /// Local files
    LocalFiles {
        paths: Vec<String>,
        mtimes: Vec<SystemTime>,
        file_sizes: Vec<u64>,
    },
    /// Kubernetes core types from OpenAPI
    K8sCore {
        version: String,
        openapi_hash: String,
        spec_url: String,
    },
}

/// Builder for creating content fingerprints
pub struct FingerprintBuilder {
    content_parts: Vec<Vec<u8>>,
    metadata_parts: Vec<String>,
    source_info: Option<SourceInfo>,
    output_files: Option<BTreeMap<String, Vec<u8>>>,
    manifest_content: Option<String>,
}

impl FingerprintBuilder {
    /// Create a new fingerprint builder
    pub fn new() -> Self {
        Self {
            content_parts: Vec::new(),
            metadata_parts: Vec::new(),
            source_info: None,
            output_files: None,
            manifest_content: None,
        }
    }

    /// Add content that affects code generation (CRD YAML, OpenAPI spec, etc.)
    pub fn add_content(&mut self, content: &[u8]) -> &mut Self {
        self.content_parts.push(content.to_vec());
        self
    }

    /// Add content from string
    pub fn add_content_str(&mut self, content: &str) -> &mut Self {
        self.add_content(content.as_bytes())
    }

    /// Add metadata that could affect generation (versions, URLs, etc.)
    pub fn add_metadata(&mut self, key: &str, value: &str) -> &mut Self {
        self.metadata_parts.push(format!("{}={}", key, value));
        self
    }

    /// Set source information
    pub fn with_source_info(&mut self, source_info: SourceInfo) -> &mut Self {
        self.source_info = Some(source_info);
        self
    }

    /// Add generated output files for content tracking
    pub fn add_output_file(&mut self, relative_path: &str, content: &[u8]) -> &mut Self {
        if self.output_files.is_none() {
            self.output_files = Some(BTreeMap::new());
        }
        if let Some(ref mut files) = self.output_files {
            files.insert(relative_path.to_string(), content.to_vec());
        }
        self
    }

    /// Add output files from a directory
    pub fn add_output_directory(
        &mut self,
        output_dir: &std::path::Path,
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        if !output_dir.exists() {
            return Ok(self);
        }

        for entry in walkdir::WalkDir::new(output_dir) {
            let entry = entry?;
            let path = entry.path();

            // Skip directories and non-nickel files
            if path.is_file() && path.extension().map(|e| e == "ncl").unwrap_or(false) {
                let content = std::fs::read(path)?;
                let relative_path = path
                    .strip_prefix(output_dir)
                    .map_err(|_| "Failed to get relative path")?
                    .to_string_lossy()
                    .to_string();

                self.add_output_file(&relative_path, &content);
            }
        }

        Ok(self)
    }

    /// Set manifest content for tracking manifest changes
    pub fn with_manifest_content(&mut self, manifest_content: &str) -> &mut Self {
        self.manifest_content = Some(manifest_content.to_string());
        self
    }

    /// Build the final fingerprint
    pub fn build(&self) -> ContentFingerprint {
        let content_hash = self.hash_content();
        let metadata_hash = self.hash_metadata();
        let output_hash = self.hash_output();
        let manifest_hash = self.hash_manifest();
        let combined_hash = self.hash_combined(
            &content_hash,
            &metadata_hash,
            output_hash.as_deref(),
            manifest_hash.as_deref(),
        );

        ContentFingerprint {
            content_hash,
            metadata_hash,
            combined_hash,
            output_hash,
            manifest_hash,
            created_at: Utc::now(),
            source_info: self
                .source_info
                .clone()
                .unwrap_or_else(|| SourceInfo::LocalFiles {
                    paths: vec!["unknown".to_string()],
                    mtimes: vec![SystemTime::now()],
                    file_sizes: vec![0],
                }),
            amalgam_version: env!("CARGO_PKG_VERSION").to_string(),
            generated_files: self.hash_individual_files(),
        }
    }

    fn hash_content(&self) -> String {
        let mut hasher = Sha256::new();

        // Sort content for deterministic hashing
        let mut sorted_content = self.content_parts.clone();
        sorted_content.sort();

        for content in &sorted_content {
            hasher.update(content);
        }

        format!("{:x}", hasher.finalize())
    }

    fn hash_metadata(&self) -> String {
        let mut hasher = Sha256::new();

        // Sort metadata for deterministic hashing
        let mut sorted_metadata = self.metadata_parts.clone();
        sorted_metadata.sort();

        for metadata in &sorted_metadata {
            hasher.update(metadata.as_bytes());
        }

        format!("{:x}", hasher.finalize())
    }

    fn hash_output(&self) -> Option<String> {
        if let Some(ref output_files) = self.output_files {
            let mut hasher = Sha256::new();

            // Hash files in deterministic order
            for (path, content) in output_files {
                hasher.update(path.as_bytes());
                hasher.update(content);
            }

            Some(format!("{:x}", hasher.finalize()))
        } else {
            None
        }
    }

    fn hash_manifest(&self) -> Option<String> {
        if let Some(ref manifest_content) = self.manifest_content {
            let mut hasher = Sha256::new();
            hasher.update(manifest_content.as_bytes());
            Some(format!("{:x}", hasher.finalize()))
        } else {
            None
        }
    }

    fn hash_individual_files(&self) -> Option<BTreeMap<String, String>> {
        if let Some(ref output_files) = self.output_files {
            let mut file_hashes = BTreeMap::new();

            for (path, content) in output_files {
                let mut hasher = Sha256::new();
                hasher.update(content);
                file_hashes.insert(path.clone(), format!("{:x}", hasher.finalize()));
            }

            Some(file_hashes)
        } else {
            None
        }
    }

    fn hash_combined(
        &self,
        content_hash: &str,
        metadata_hash: &str,
        output_hash: Option<&str>,
        manifest_hash: Option<&str>,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content_hash.as_bytes());
        hasher.update(metadata_hash.as_bytes());

        if let Some(output) = output_hash {
            hasher.update(output.as_bytes());
        }

        if let Some(manifest) = manifest_hash {
            hasher.update(manifest.as_bytes());
        }

        format!("{:x}", hasher.finalize())
    }
}

impl Default for FingerprintBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ContentFingerprint {
    /// Check if this fingerprint represents the same content as another
    pub fn content_matches(&self, other: &ContentFingerprint) -> bool {
        self.combined_hash == other.combined_hash
    }

    /// Check if only metadata changed (requiring regeneration with new timestamps)
    pub fn metadata_changed(&self, other: &ContentFingerprint) -> bool {
        self.content_hash == other.content_hash
            && self.output_hash == other.output_hash
            && (self.metadata_hash != other.metadata_hash
                || self.manifest_hash != other.manifest_hash)
    }

    /// Check if content changed (requiring full regeneration)
    pub fn content_changed(&self, other: &ContentFingerprint) -> bool {
        self.content_hash != other.content_hash
    }

    /// Check if output files changed (someone manually edited generated files)
    pub fn output_changed(&self, other: &ContentFingerprint) -> bool {
        self.output_hash != other.output_hash
    }

    /// Check if manifest changed (packages added/removed/modified)
    pub fn manifest_changed(&self, other: &ContentFingerprint) -> bool {
        self.manifest_hash != other.manifest_hash
    }

    /// Get list of files that changed between fingerprints
    pub fn changed_files(&self, other: &ContentFingerprint) -> Vec<String> {
        let mut changed = Vec::new();

        let current_files = self.generated_files.as_ref();
        let other_files = other.generated_files.as_ref();

        match (current_files, other_files) {
            (Some(current), Some(other)) => {
                // Check for modified or removed files
                for (path, current_hash) in current {
                    if let Some(other_hash) = other.get(path) {
                        if current_hash != other_hash {
                            changed.push(path.clone());
                        }
                    } else {
                        changed.push(path.clone()); // New file
                    }
                }

                // Check for removed files
                for path in other.keys() {
                    if !current.contains_key(path) {
                        changed.push(path.clone());
                    }
                }
            }
            _ => {
                // One or both don't have file tracking, assume all changed
            }
        }

        changed
    }

    /// Get a short hash for display purposes
    pub fn short_hash(&self) -> String {
        self.combined_hash.chars().take(12).collect()
    }

    /// Save fingerprint to a file
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Load fingerprint from a file
    pub fn load_from_file(
        path: &std::path::Path,
    ) -> Result<ContentFingerprint, Box<dyn std::error::Error>> {
        if !path.exists() {
            return Err("Fingerprint file does not exist".into());
        }
        let content = std::fs::read_to_string(path)?;
        let fingerprint = serde_json::from_str(&content)?;
        Ok(fingerprint)
    }

    /// Create a fingerprint file path for a package
    pub fn fingerprint_path(output_dir: &std::path::Path) -> std::path::PathBuf {
        output_dir.join(".amalgam-fingerprint.json")
    }
}

/// Trait for source types to implement fingerprinting
pub trait Fingerprintable {
    /// Create a content fingerprint for this source
    fn create_fingerprint(&self) -> Result<ContentFingerprint, Box<dyn std::error::Error>>;

    /// Check if content has changed since the last fingerprint
    fn has_changed(
        &self,
        last_fingerprint: &ContentFingerprint,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let current = self.create_fingerprint()?;
        Ok(current.content_changed(last_fingerprint) || current.metadata_changed(last_fingerprint))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprint_builder() {
        let mut builder = FingerprintBuilder::new();
        builder
            .add_content_str("test content")
            .add_metadata("version", "1.0.0")
            .add_metadata("source", "test");

        let fingerprint = builder.build();

        assert!(!fingerprint.content_hash.is_empty());
        assert!(!fingerprint.metadata_hash.is_empty());
        assert!(!fingerprint.combined_hash.is_empty());
        assert_eq!(fingerprint.short_hash().len(), 12);
    }

    #[test]
    fn test_fingerprint_comparison() {
        let mut builder1 = FingerprintBuilder::new();
        builder1.add_content_str("same content");
        let fp1 = builder1.build();

        let mut builder2 = FingerprintBuilder::new();
        builder2.add_content_str("same content");
        let fp2 = builder2.build();

        assert!(fp1.content_matches(&fp2));
    }

    #[test]
    fn test_content_vs_metadata_changes() {
        let mut builder1 = FingerprintBuilder::new();
        builder1
            .add_content_str("content")
            .add_metadata("version", "1.0.0");
        let fp1 = builder1.build();

        // Same content, different metadata
        let mut builder2 = FingerprintBuilder::new();
        builder2
            .add_content_str("content")
            .add_metadata("version", "1.0.1");
        let fp2 = builder2.build();

        assert!(fp1.metadata_changed(&fp2));
        assert!(!fp1.content_changed(&fp2));

        // Different content
        let mut builder3 = FingerprintBuilder::new();
        builder3
            .add_content_str("different content")
            .add_metadata("version", "1.0.0");
        let fp3 = builder3.build();

        assert!(fp1.content_changed(&fp3));
        assert!(!fp1.metadata_changed(&fp3));
    }
}
