//! Snapshot tests for mod.ncl generation
//!
//! These tests capture the exact output format and will fail if the format changes.
//! Use `cargo insta review` to review and approve intentional changes.

use amalgam_core::ir::TypeDefinition;
use amalgam_parser::package::NamespacedPackage;
use insta::assert_snapshot;

#[test]
fn snapshot_main_module_single_group() {
    let mut pkg = NamespacedPackage::new("k8s_io".to_string());

    pkg.add_type(
        "k8s.io".to_string(),
        "v1".to_string(),
        "pod".to_string(),
        TypeDefinition {
            name: "Pod".to_string(),
            doc: Some("Pod is a collection of containers that can run on a host".to_string()),
            ..Default::default()
        },
    );

    let output = pkg.generate_main_module();
    assert_snapshot!("main_module_single_group", output);
}

#[test]
fn snapshot_main_module_multiple_groups() {
    let mut pkg = NamespacedPackage::new("crossplane".to_string());

    pkg.add_type(
        "apiextensions.crossplane.io".to_string(),
        "v1".to_string(),
        "composition".to_string(),
        TypeDefinition {
            name: "Composition".to_string(),
            ..Default::default()
        },
    );

    pkg.add_type(
        "pkg.crossplane.io".to_string(),
        "v1".to_string(),
        "provider".to_string(),
        TypeDefinition {
            name: "Provider".to_string(),
            ..Default::default()
        },
    );

    pkg.add_type(
        "ops.crossplane.io".to_string(),
        "v1alpha1".to_string(),
        "storeconfig".to_string(),
        TypeDefinition {
            name: "StoreConfig".to_string(),
            ..Default::default()
        },
    );

    let output = pkg.generate_main_module();
    assert_snapshot!("main_module_multiple_groups", output);
}

#[test]
fn snapshot_group_module_single_version() {
    let mut pkg = NamespacedPackage::new("test".to_string());

    pkg.add_type(
        "apiextensions.crossplane.io".to_string(),
        "v1".to_string(),
        "composition".to_string(),
        TypeDefinition {
            name: "Composition".to_string(),
            ..Default::default()
        },
    );

    let output = pkg
        .generate_group_module("apiextensions.crossplane.io")
        .expect("Should generate");
    assert_snapshot!("group_module_single_version", output);
}

#[test]
fn snapshot_group_module_multiple_versions() {
    let mut pkg = NamespacedPackage::new("test".to_string());

    // Add types to v1
    pkg.add_type(
        "apiextensions.crossplane.io".to_string(),
        "v1".to_string(),
        "composition".to_string(),
        TypeDefinition {
            name: "Composition".to_string(),
            ..Default::default()
        },
    );

    // Add types to v1beta1
    pkg.add_type(
        "apiextensions.crossplane.io".to_string(),
        "v1beta1".to_string(),
        "compositionrevision".to_string(),
        TypeDefinition {
            name: "CompositionRevision".to_string(),
            ..Default::default()
        },
    );

    // Add types to v1alpha1
    pkg.add_type(
        "apiextensions.crossplane.io".to_string(),
        "v1alpha1".to_string(),
        "usage".to_string(),
        TypeDefinition {
            name: "Usage".to_string(),
            ..Default::default()
        },
    );

    let output = pkg
        .generate_group_module("apiextensions.crossplane.io")
        .expect("Should generate");
    assert_snapshot!("group_module_multiple_versions", output);
}

#[test]
fn snapshot_version_module_with_full_docs() {
    let mut pkg = NamespacedPackage::new("test".to_string());

    pkg.add_type(
        "apiextensions.crossplane.io".to_string(),
        "v1".to_string(),
        "composition".to_string(),
        TypeDefinition {
            name: "Composition".to_string(),
            doc: Some("A Composition specifies how to compose resources into a higher level infrastructure unit".to_string()),
            ..Default::default()
        },
    );

    pkg.add_type(
        "apiextensions.crossplane.io".to_string(),
        "v1".to_string(),
        "compositionrevision".to_string(),
        TypeDefinition {
            name: "CompositionRevision".to_string(),
            doc: Some("A CompositionRevision represents a revision of a Composition".to_string()),
            ..Default::default()
        },
    );

    pkg.add_type(
        "apiextensions.crossplane.io".to_string(),
        "v1".to_string(),
        "compositeresourcedefinition".to_string(),
        TypeDefinition {
            name: "CompositeResourceDefinition".to_string(),
            doc: Some("A CompositeResourceDefinition defines a new kind of composite infrastructure resource".to_string()),
            ..Default::default()
        },
    );

    let output = pkg
        .generate_version_module("apiextensions.crossplane.io", "v1")
        .expect("Should generate");
    assert_snapshot!("version_module_with_docs", output);
}

#[test]
fn snapshot_version_module_without_docs() {
    let mut pkg = NamespacedPackage::new("test".to_string());

    pkg.add_type(
        "test.io".to_string(),
        "v1".to_string(),
        "resource".to_string(),
        TypeDefinition {
            name: "Resource".to_string(),
            doc: None, // No documentation
            ..Default::default()
        },
    );

    pkg.add_type(
        "test.io".to_string(),
        "v1".to_string(),
        "config".to_string(),
        TypeDefinition {
            name: "Config".to_string(),
            doc: None,
            ..Default::default()
        },
    );

    let output = pkg
        .generate_version_module("test.io", "v1")
        .expect("Should generate");
    assert_snapshot!("version_module_without_docs", output);
}

#[test]
fn snapshot_version_module_mixed_docs() {
    let mut pkg = NamespacedPackage::new("test".to_string());

    // With doc
    pkg.add_type(
        "test.io".to_string(),
        "v1".to_string(),
        "documented".to_string(),
        TypeDefinition {
            name: "Documented".to_string(),
            doc: Some("This type has documentation".to_string()),
            ..Default::default()
        },
    );

    // Without doc
    pkg.add_type(
        "test.io".to_string(),
        "v1".to_string(),
        "undocumented".to_string(),
        TypeDefinition {
            name: "Undocumented".to_string(),
            doc: None,
            ..Default::default()
        },
    );

    let output = pkg
        .generate_version_module("test.io", "v1")
        .expect("Should generate");
    assert_snapshot!("version_module_mixed_docs", output);
}

#[test]
fn snapshot_version_module_long_doc_truncation() {
    let mut pkg = NamespacedPackage::new("test".to_string());

    let long_doc = "This is an extremely long documentation string that exceeds the 80 character limit and should be truncated with ellipsis to keep the mod.ncl file readable";

    pkg.add_type(
        "test.io".to_string(),
        "v1".to_string(),
        "longdoc".to_string(),
        TypeDefinition {
            name: "LongDoc".to_string(),
            doc: Some(long_doc.to_string()),
            ..Default::default()
        },
    );

    let output = pkg
        .generate_version_module("test.io", "v1")
        .expect("Should generate");
    assert_snapshot!("version_module_long_doc", output);
}

#[test]
fn snapshot_version_module_special_characters_in_doc() {
    let mut pkg = NamespacedPackage::new("test".to_string());

    pkg.add_type(
        "test.io".to_string(),
        "v1".to_string(),
        "special".to_string(),
        TypeDefinition {
            name: "Special".to_string(),
            doc: Some("Type with \"quotes\" and 'apostrophes' and $special characters".to_string()),
            ..Default::default()
        },
    );

    let output = pkg
        .generate_version_module("test.io", "v1")
        .expect("Should generate");
    assert_snapshot!("version_module_special_chars", output);
}

#[test]
fn snapshot_type_sorting_alphabetical() {
    let mut pkg = NamespacedPackage::new("test".to_string());

    // Add in non-alphabetical order
    pkg.add_type(
        "test.io".to_string(),
        "v1".to_string(),
        "zebra".to_string(),
        TypeDefinition {
            name: "Zebra".to_string(),
            ..Default::default()
        },
    );

    pkg.add_type(
        "test.io".to_string(),
        "v1".to_string(),
        "apple".to_string(),
        TypeDefinition {
            name: "Apple".to_string(),
            ..Default::default()
        },
    );

    pkg.add_type(
        "test.io".to_string(),
        "v1".to_string(),
        "mango".to_string(),
        TypeDefinition {
            name: "Mango".to_string(),
            ..Default::default()
        },
    );

    let output = pkg
        .generate_version_module("test.io", "v1")
        .expect("Should generate");

    // Should be sorted: Apple, Mango, Zebra
    assert_snapshot!("type_sorting", output);
}
