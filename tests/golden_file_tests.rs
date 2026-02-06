//! Golden file tests for ascfix.
//!
//! These tests compare actual output against expected output files.
//! Test fixtures are in tests/golden/{input,expected}/ directories.

use std::fs;
use std::path::Path;

#[test]
fn golden_file_simple_box() {
    let input = fs::read_to_string("tests/golden/input/simple_box.txt")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/golden/expected/simple_box.txt")
        .expect("Failed to read expected fixture");

    // Process the input
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, &input);

    // Compare
    assert_eq!(
        result.trim(),
        expected.trim(),
        "Output does not match expected for simple_box"
    );
}

#[test]
fn golden_file_box_with_arrow() {
    let input = fs::read_to_string("tests/golden/input/box_with_arrow.txt")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/golden/expected/box_with_arrow.txt")
        .expect("Failed to read expected fixture");

    // Process the input
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, &input);

    // Compare
    assert_eq!(
        result.trim(),
        expected.trim(),
        "Output does not match expected for box_with_arrow"
    );
}

#[test]
fn golden_file_markdown_with_diagram() {
    let input = fs::read_to_string("tests/golden/input/markdown_with_diagram.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/golden/expected/markdown_with_diagram.md")
        .expect("Failed to read expected fixture");

    // Process the input
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, &input);

    // Compare
    assert_eq!(
        result.trim(),
        expected.trim(),
        "Output does not match expected for markdown_with_diagram"
    );
}

#[test]
fn all_golden_files_have_expected_output() {
    let input_dir = Path::new("tests/golden/input");
    let expected_dir = Path::new("tests/golden/expected");

    let mut input_files = Vec::new();
    let mut expected_files = Vec::new();

    // Collect all input files
    if let Ok(entries) = fs::read_dir(input_dir) {
        for entry in entries.flatten() {
            if entry.path().is_file() {
                if let Some(name) = entry.path().file_name().and_then(|n| n.to_str()) {
                    input_files.push(name.to_string());
                }
            }
        }
    }

    // Collect all expected files
    if let Ok(entries) = fs::read_dir(expected_dir) {
        for entry in entries.flatten() {
            if entry.path().is_file() {
                if let Some(name) = entry.path().file_name().and_then(|n| n.to_str()) {
                    expected_files.push(name.to_string());
                }
            }
        }
    }

    // Verify every input has a corresponding expected file
    for input_file in &input_files {
        assert!(
            expected_files.contains(input_file),
            "Input file '{input_file}' has no corresponding expected file"
        );
    }
}

#[test]
fn golden_files_directory_exists() {
    let input_dir = Path::new("tests/golden/input");
    let expected_dir = Path::new("tests/golden/expected");

    assert!(
        input_dir.exists(),
        "Input directory tests/golden/input does not exist"
    );
    assert!(
        expected_dir.exists(),
        "Expected directory tests/golden/expected does not exist"
    );
}
