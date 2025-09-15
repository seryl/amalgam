//! Test that version changes in manifests trigger fingerprint differences

use amalgam_core::fingerprint::Fingerprintable;
use amalgam_parser::incremental::{K8sCoreSource, UrlSource};

#[test]
fn test_k8s_version_change_triggers_fingerprint_difference(
) -> Result<(), Box<dyn std::error::Error>> {
    // Create two K8s sources with different versions but same content
    let source_v1_31 = K8sCoreSource {
        version: "v1.31.0".to_string(),
        openapi_spec: "same_content".to_string(),
        spec_url: "https://dl.k8s.io/v1.31.0/api/openapi-spec/swagger.json".to_string(),
    };

    let source_v1_33 = K8sCoreSource {
        version: "v1.33.4".to_string(),
        openapi_spec: "same_content".to_string(),
        spec_url: "https://dl.k8s.io/v1.33.4/api/openapi-spec/swagger.json".to_string(),
    };

    // Create fingerprints
    let fingerprint_v1_31 = source_v1_31.create_fingerprint()?;
    let fingerprint_v1_33 = source_v1_33.create_fingerprint()?;

    // The fingerprints should be different even though content is the same
    // because version is included in metadata
    assert_ne!(
        fingerprint_v1_31.metadata_hash, fingerprint_v1_33.metadata_hash,
        "Different K8s versions should produce different metadata hashes"
    );

    // Test that has_changed detects the version change
    let changed = source_v1_33.has_changed(&fingerprint_v1_31)?;
    assert!(
        changed,
        "K8s version change from v1.31.0 to v1.33.4 should be detected"
    );
    Ok(())
}

#[test]
fn test_url_git_ref_change_triggers_fingerprint_difference(
) -> Result<(), Box<dyn std::error::Error>> {
    // Create two URL sources with different git refs
    let source_v1 = UrlSource {
        base_url: "https://github.com/crossplane/crossplane/tree/v1.17.2/cluster/crds".to_string(),
        urls: vec![
            "https://github.com/crossplane/crossplane/tree/v1.17.2/cluster/crds".to_string(),
        ],
        contents: vec!["same_content".to_string()],
    };

    let source_v2 = UrlSource {
        base_url: "https://github.com/crossplane/crossplane/tree/v2.0.2/cluster/crds".to_string(),
        urls: vec!["https://github.com/crossplane/crossplane/tree/v2.0.2/cluster/crds".to_string()],
        contents: vec!["same_content".to_string()],
    };

    // Create fingerprints
    let fingerprint_v1 = source_v1.create_fingerprint()?;
    let fingerprint_v2 = source_v2.create_fingerprint()?;

    // The fingerprints should be different because base_url is different
    assert_ne!(
        fingerprint_v1.metadata_hash, fingerprint_v2.metadata_hash,
        "Different URL versions should produce different metadata hashes"
    );

    // Test that has_changed detects the URL change
    let changed = source_v2.has_changed(&fingerprint_v1)?;
    assert!(
        changed,
        "URL change from v1.17.2 to v2.0.2 should be detected"
    );
    Ok(())
}

#[test]
fn test_same_version_no_change() -> Result<(), Box<dyn std::error::Error>> {
    // Create two identical K8s sources
    let source1 = K8sCoreSource {
        version: "v1.33.4".to_string(),
        openapi_spec: "same_content".to_string(),
        spec_url: "https://dl.k8s.io/v1.33.4/api/openapi-spec/swagger.json".to_string(),
    };

    let source2 = K8sCoreSource {
        version: "v1.33.4".to_string(),
        openapi_spec: "same_content".to_string(),
        spec_url: "https://dl.k8s.io/v1.33.4/api/openapi-spec/swagger.json".to_string(),
    };

    // Create fingerprints
    let fingerprint1 = source1.create_fingerprint()?;
    let fingerprint2 = source2.create_fingerprint()?;

    // The fingerprints should be identical
    assert_eq!(
        fingerprint1.content_hash, fingerprint2.content_hash,
        "Identical sources should produce identical content hashes"
    );
    assert_eq!(
        fingerprint1.metadata_hash, fingerprint2.metadata_hash,
        "Identical sources should produce identical metadata hashes"
    );

    // Test that has_changed returns false
    let changed = source2.has_changed(&fingerprint1)?;
    assert!(
        !changed,
        "Identical sources should not be detected as changed"
    );
    Ok(())
}

#[test]
fn test_metadata_only_change_detected() -> Result<(), Box<dyn std::error::Error>> {
    // Test that even if content is the same, metadata changes are detected
    let source_old = K8sCoreSource {
        version: "v1.31.0".to_string(),
        openapi_spec: "identical_spec_content".to_string(),
        spec_url: "https://dl.k8s.io/v1.31.0/api/openapi-spec/swagger.json".to_string(),
    };

    let source_new = K8sCoreSource {
        version: "v1.33.4".to_string(),
        openapi_spec: "identical_spec_content".to_string(), // Same content!
        spec_url: "https://dl.k8s.io/v1.33.4/api/openapi-spec/swagger.json".to_string(),
    };

    let fingerprint_old = source_old.create_fingerprint()?;

    // Check if change is detected
    let changed = source_new.has_changed(&fingerprint_old)?;

    assert!(
        changed,
        "Version metadata change should trigger regeneration even with identical content"
    );
    Ok(())
}
