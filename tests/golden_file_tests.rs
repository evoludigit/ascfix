//! Golden file tests for ascfix.
//!
//! These tests compare actual output against expected output files.
//! Test fixtures are in tests/data/unit/{input,expected}/ directories.

use std::fs;
use std::path::Path;

#[test]
fn golden_file_simple_box() {
    let input = fs::read_to_string("tests/data/unit/input/simple_box.txt")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/data/unit/expected/simple_box.txt")
        .expect("Failed to read expected fixture");

    // Process the input
    let config = ascfix::config::Config::default();
    let result =
        ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, &input, false, &config);

    // Compare
    assert_eq!(
        result.trim(),
        expected.trim(),
        "Output does not match expected for simple_box"
    );
}

#[test]
fn golden_file_box_with_arrow() {
    let input = fs::read_to_string("tests/data/unit/input/box_with_arrow.txt")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/data/unit/expected/box_with_arrow.txt")
        .expect("Failed to read expected fixture");

    // Process the input
    let config = ascfix::config::Config::default();
    let result =
        ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, &input, false, &config);

    // Compare
    assert_eq!(
        result.trim(),
        expected.trim(),
        "Output does not match expected for box_with_arrow"
    );
}

#[test]
fn golden_file_markdown_with_diagram() {
    let input = fs::read_to_string("tests/data/unit/input/markdown_with_diagram.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/data/unit/expected/markdown_with_diagram.md")
        .expect("Failed to read expected fixture");

    // Process the input
    let config = ascfix::config::Config::default();
    let result =
        ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, &input, false, &config);

    // Compare
    assert_eq!(
        result.trim(),
        expected.trim(),
        "Output does not match expected for markdown_with_diagram"
    );
}

#[test]
fn golden_file_ci_pipeline() {
    let input = fs::read_to_string("tests/data/unit/input/ci_pipeline.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/data/unit/expected/ci_pipeline.md")
        .expect("Failed to read expected fixture");

    // Process the input
    let config = ascfix::config::Config::default();
    let result =
        ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, &input, false, &config);

    // Compare
    assert_eq!(
        result.trim(),
        expected.trim(),
        "Output does not match expected for ci_pipeline"
    );
}

#[test]
fn golden_file_double_line_boxes() {
    let input = fs::read_to_string("tests/data/unit/input/double_line_boxes.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/data/unit/expected/double_line_boxes.md")
        .expect("Failed to read expected fixture");

    let config = ascfix::config::Config::default();
    let result =
        ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, &input, false, &config);

    assert_eq!(
        result.trim(),
        expected.trim(),
        "Output does not match expected for double_line_boxes"
    );
}

#[test]
fn golden_file_rounded_boxes() {
    let input = fs::read_to_string("tests/data/unit/input/rounded_boxes.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/data/unit/expected/rounded_boxes.md")
        .expect("Failed to read expected fixture");

    let config = ascfix::config::Config::default();
    let result =
        ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, &input, false, &config);

    assert_eq!(
        result.trim(),
        expected.trim(),
        "Output does not match expected for rounded_boxes"
    );
}

#[test]
fn golden_file_nested_boxes() {
    let input = fs::read_to_string("tests/data/unit/input/nested_boxes.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/data/unit/expected/nested_boxes.md")
        .expect("Failed to read expected fixture");

    let config = ascfix::config::Config::default();
    let result =
        ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, &input, false, &config);

    assert_eq!(
        result.trim(),
        expected.trim(),
        "Output does not match expected for nested_boxes"
    );
}

#[test]
fn golden_file_side_by_side_boxes() {
    let input = fs::read_to_string("tests/data/unit/input/side_by_side_boxes.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/data/unit/expected/side_by_side_boxes.md")
        .expect("Failed to read expected fixture");

    let config = ascfix::config::Config::default();
    let result =
        ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, &input, false, &config);

    assert_eq!(
        result.trim(),
        expected.trim(),
        "Output does not match expected for side_by_side_boxes"
    );
}

#[test]
fn golden_file_connection_lines() {
    let input = fs::read_to_string("tests/data/unit/input/connection_lines.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/data/unit/expected/connection_lines.md")
        .expect("Failed to read expected fixture");

    let config = ascfix::config::Config::default();
    let result =
        ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, &input, false, &config);

    assert_eq!(
        result.trim(),
        expected.trim(),
        "Output does not match expected for connection_lines"
    );
}

#[test]
fn golden_file_mixed_features() {
    let input = fs::read_to_string("tests/data/unit/input/mixed_features.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/data/unit/expected/mixed_features.md")
        .expect("Failed to read expected fixture");

    let config = ascfix::config::Config::default();
    let result =
        ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, &input, false, &config);

    assert_eq!(
        result.trim(),
        expected.trim(),
        "Output does not match expected for mixed_features"
    );
}

#[test]
fn all_golden_files_have_expected_output() {
    let input_dir = Path::new("tests/data/unit/input");
    let expected_dir = Path::new("tests/data/unit/expected");

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
    let input_dir = Path::new("tests/data/unit/input");
    let expected_dir = Path::new("tests/data/unit/expected");

    assert!(
        input_dir.exists(),
        "Input directory tests/data/unit/input does not exist"
    );
    assert!(
        expected_dir.exists(),
        "Expected directory tests/data/unit/expected does not exist"
    );
}

#[test]
fn golden_file_ignore_ascii_art() {
    let input = fs::read_to_string("tests/data/unit/input/ignore_ascii_art.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/data/unit/expected/ignore_ascii_art.md")
        .expect("Failed to read expected fixture");

    // Process the input
    let config = ascfix::config::Config::default();
    let result =
        ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, &input, false, &config);

    // Compare
    assert_eq!(
        result.trim(),
        expected.trim(),
        "Output does not match expected for ignore_ascii_art - the ASCII art in ignore block should be preserved unchanged, but the box below should be normalized"
    );
}
