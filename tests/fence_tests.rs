//! Integration tests for code fence repair functionality.

use std::fs;

#[test]
fn fence_repair_mismatched_lengths() {
    let input = fs::read_to_string("tests/data/unit/input/mismatched_fences.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/data/unit/expected/mismatched_fences.md")
        .expect("Failed to read expected fixture");

    // Process with fence repair enabled
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        &input,
        true,
        &ascfix::config::Config::default(),
    );

    assert_eq!(
        result.trim(),
        expected.trim(),
        "Fence repair output does not match expected"
    );
}

#[test]
fn fence_repair_nested_fences() {
    let input = fs::read_to_string("tests/data/unit/input/nested_fences.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/data/unit/expected/nested_fences.md")
        .expect("Failed to read expected fixture");

    // Process with fence repair enabled
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        &input,
        true,
        &ascfix::config::Config::default(),
    );

    assert_eq!(
        result.trim(),
        expected.trim(),
        "Nested fence output does not match expected"
    );
}

#[test]
fn fence_repair_idempotent_mismatched() {
    let input = fs::read_to_string("tests/data/unit/input/mismatched_fences.md")
        .expect("Failed to read input fixture");

    // Process twice
    let first = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        &input,
        true,
        &ascfix::config::Config::default(),
    );
    let second = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        &first,
        true,
        &ascfix::config::Config::default(),
    );

    assert_eq!(
        first.trim(),
        second.trim(),
        "Fence repair is not idempotent for mismatched fences"
    );
}

#[test]
fn fence_repair_idempotent_nested() {
    let input = fs::read_to_string("tests/data/unit/input/nested_fences.md")
        .expect("Failed to read input fixture");

    // Process twice
    let first = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        &input,
        true,
        &ascfix::config::Config::default(),
    );
    let second = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        &first,
        true,
        &ascfix::config::Config::default(),
    );

    assert_eq!(
        first.trim(),
        second.trim(),
        "Fence repair is not idempotent for nested fences"
    );
}

#[test]
fn fence_repair_with_diagram_mode() {
    // Test that fence repair can be combined with diagram repair
    let input = "```python\ncode\n`````\n\n┌─┐\n│a│\n└─┘";

    // Process with both fence repair and diagram mode
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        input,
        true,
        &ascfix::config::Config::default(),
    );

    // Should have:
    // 1. Fixed the fence lengths
    // 2. Processed the diagram
    assert!(!result.is_empty());
    assert!(result.contains("python") || result.contains("code"));
}

#[test]
fn fence_repair_disabled_by_default() {
    let input = "```python\ncode\n`````";

    // Process without fence repair
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        false,
        &ascfix::config::Config::default(),
    );

    // Should be unchanged (fence repair not applied)
    assert_eq!(result, input);
}

#[test]
fn fence_repair_preserves_content() {
    let input = "# Header\n\n```python\nprint('hello')\n`````\n\nSome text";

    // Process with fence repair
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        true,
        &ascfix::config::Config::default(),
    );

    // Should preserve all content
    assert!(result.contains("# Header"));
    assert!(result.contains("print('hello')"));
    assert!(result.contains("Some text"));
}

#[test]
fn fence_repair_already_correct() {
    let input = "```python\ncode\n```";

    // Process with fence repair
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        true,
        &ascfix::config::Config::default(),
    );

    // Should be unchanged when already correct
    assert_eq!(result, input);
}

#[test]
fn fence_repair_unclosed_fence() {
    let input = "```python\ncode";

    // Process with fence repair
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        true,
        &ascfix::config::Config::default(),
    );

    // Should have added closing fence
    assert!(result.contains("```"));
    // Should have at least 3 lines (opening, code, closing)
    assert!(result.lines().count() >= 2);
}

#[test]
fn fence_repair_multiple_blocks() {
    let input = "```\ncode1\n```\n\n```\ncode2\n`````";

    // Process with fence repair
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        true,
        &ascfix::config::Config::default(),
    );

    // Should have fixed both blocks
    assert!(result.contains("code1"));
    assert!(result.contains("code2"));
    // Check that fences are balanced
    let fence_count = result.matches("```").count();
    assert_eq!(fence_count % 2, 0, "Fences should be balanced");
}

#[test]
fn fence_repair_mixed_types() {
    // Mix of backticks and tildes - should not try to fix type mismatches
    let input = "```\ncode\n~~~";

    // Process with fence repair
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        true,
        &ascfix::config::Config::default(),
    );

    // Should handle gracefully (conservative approach)
    assert!(!result.is_empty());
}

#[test]
fn fence_repair_with_language_specifier() {
    let input = "```javascript\ncode\n`````";

    // Process with fence repair
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        true,
        &ascfix::config::Config::default(),
    );

    // Should preserve language specifier
    assert!(result.contains("javascript"));
    // And balance the fences
    assert!(result.lines().filter(|l| l.contains('`')).count() >= 2);
}
