//! Documentation validation tests
//!
//! These tests verify that important topics are documented in the right places.

use std::fs;

#[test]
fn readme_mentions_known_limitations() {
    let readme = fs::read_to_string("README.md").expect("README.md should exist");

    assert!(
        readme.contains("## Known Limitations") || readme.contains("# Known Limitations"),
        "README.md should have a 'Known Limitations' section"
    );

    assert!(
        readme.contains("idempotence") || readme.contains("Idempotence"),
        "README.md should mention idempotence limitations"
    );
}

#[test]
fn library_usage_documents_limitations() {
    let lib = fs::read_to_string("LIBRARY_USAGE.md").expect("LIBRARY_USAGE.md should exist");

    assert!(
        lib.contains("## Known Limitations") || lib.contains("# Known Limitations"),
        "LIBRARY_USAGE.md should have a 'Known Limitations' section"
    );

    assert!(
        lib.contains("idempotence") || lib.contains("Idempotence"),
        "LIBRARY_USAGE.md should document idempotence constraints"
    );

    assert!(
        lib.contains("nested") || lib.contains("Nested"),
        "LIBRARY_USAGE.md should mention nested diagram limitations"
    );
}

#[test]
fn architecture_explains_limitations() {
    let arch = fs::read_to_string("ARCHITECTURE.md").expect("ARCHITECTURE.md should exist");

    assert!(
        arch.contains("idempotence") || arch.contains("Idempotence"),
        "ARCHITECTURE.md should explain idempotence limitations technically"
    );

    assert!(
        arch.contains("detection") || arch.contains("Detection"),
        "ARCHITECTURE.md should explain detection phase issues"
    );
}

#[test]
fn cross_references_are_valid() {
    let readme = fs::read_to_string("README.md").expect("README.md should exist");

    // If README mentions LIBRARY_USAGE.md, verify the reference
    if readme.contains("LIBRARY_USAGE.md") {
        assert!(
            std::path::Path::new("LIBRARY_USAGE.md").exists(),
            "Cross-referenced LIBRARY_USAGE.md should exist"
        );
    }

    // If README mentions ARCHITECTURE.md, verify the reference
    if readme.contains("ARCHITECTURE.md") {
        assert!(
            std::path::Path::new("ARCHITECTURE.md").exists(),
            "Cross-referenced ARCHITECTURE.md should exist"
        );
    }
}

#[test]
fn idempotence_tests_exist() {
    // Verify we have tests demonstrating the idempotence behavior
    let test_files = vec!["tests/idempotence_tests.rs", "tests/integration_tests.rs"];

    let mut found_idempotence_test = false;
    for test_file in test_files {
        if let Ok(content) = fs::read_to_string(test_file) {
            if content.contains("idempotence") || content.contains("idempotent") {
                found_idempotence_test = true;
                break;
            }
        }
    }

    assert!(
        found_idempotence_test,
        "Should have tests demonstrating idempotence behavior"
    );
}
