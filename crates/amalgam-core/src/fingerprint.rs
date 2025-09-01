//! Content fingerprinting for intelligent change detection
//! 
//! This module provides universal change detection across all source types
//! by creating content-based fingerprints that capture everything affecting
//! code generation.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};

/// Universal content fingerprint for change detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentFingerprint {
    /// Hash of all content that affects code generation
    pub content_hash: String,
    /// Source-specific metadata hash (URLs, versions, etc.)  
    pub metadata_hash: String,
    /// Combined hash for quick comparison
    pub combined_hash: String,
    /// When this fingerprint was created
    pub created_at: DateTime<Utc>,
    /// Source type and location information
    pub source_info: SourceInfo,
    /// Version of amalgam that created this fingerprint
    pub amalgam_version: String,
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
}

impl FingerprintBuilder {
    /// Create a new fingerprint builder
    pub fn new() -> Self {
        Self {
            content_parts: Vec::new(),
            metadata_parts: Vec::new(),
            source_info: None,
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

    /// Build the final fingerprint
    pub fn build(&self) -> ContentFingerprint {
        let content_hash = self.hash_content();
        let metadata_hash = self.hash_metadata();
        let combined_hash = self.hash_combined(&content_hash, &metadata_hash);

        ContentFingerprint {
            content_hash,
            metadata_hash,
            combined_hash,
            created_at: Utc::now(),
            source_info: self.source_info.clone().unwrap_or_else(|| {
                SourceInfo::LocalFiles {
                    paths: vec!["unknown".to_string()],
                    mtimes: vec![SystemTime::now()],
                    file_sizes: vec![0],
                }
            }),
            amalgam_version: env!("CARGO_PKG_VERSION").to_string(),
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

    fn hash_combined(&self, content_hash: &str, metadata_hash: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content_hash.as_bytes());
        hasher.update(metadata_hash.as_bytes());
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
        self.content_hash == other.content_hash && self.metadata_hash != other.metadata_hash
    }

    /// Check if content changed (requiring full regeneration)
    pub fn content_changed(&self, other: &ContentFingerprint) -> bool {
        self.content_hash != other.content_hash
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
    pub fn load_from_file(path: &std::path::Path) -> Result<ContentFingerprint, Box<dyn std::error::Error>> {
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
    fn has_changed(&self, last_fingerprint: &ContentFingerprint) -> Result<bool, Box<dyn std::error::Error>> {
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