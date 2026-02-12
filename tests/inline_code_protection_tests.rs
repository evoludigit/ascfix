//! Integration tests for inline code protection in diagram mode.
//!
//! These tests verify that inline code blocks with diagram-like characters
//! (arrows, box drawing, etc.) are preserved unchanged during diagram processing.

use ascfix::cli::Mode;
use ascfix::config::Config;
use ascfix::modes::process_by_mode;

#[test]
fn test_inline_code_with_arrows_preserved() {
    let config = Config::default();
    let content = "- Double arrows: `⇒ ⇓ ⇑ ⇐`\n- Normal text";
    let result = process_by_mode(&Mode::Diagram, content, false, &config);
    // The inline code arrows should be preserved exactly
    assert!(
        result.contains("`⇒ ⇓ ⇑ ⇐`"),
        "Arrows in inline code should be preserved"
    );
}

#[test]
fn test_inline_code_with_extended_arrows_preserved() {
    let config = Config::default();
    let content = "Extended arrows: `⟶ ⟹ ⟸` for notation";
    let result = process_by_mode(&Mode::Diagram, content, false, &config);
    assert!(
        result.contains("`⟶ ⟹ ⟸`"),
        "Extended arrows should be preserved"
    );
}

#[test]
fn test_inline_code_with_box_drawing_preserved() {
    let config = Config::default();
    let content = "Box chars: `┌─┐│└┘` for reference";
    let result = process_by_mode(&Mode::Diagram, content, false, &config);
    assert!(
        result.contains("`┌─┐│└┘`"),
        "Box drawing chars should be preserved"
    );
}

#[test]
fn test_multiple_inline_codes_preserved() {
    let config = Config::default();
    let content = "`first` and `second` and `⇒ third`";
    let result = process_by_mode(&Mode::Diagram, content, false, &config);
    assert!(result.contains("`first`"), "First code should be preserved");
    assert!(
        result.contains("`second`"),
        "Second code should be preserved"
    );
    assert!(
        result.contains("`⇒ third`"),
        "Third code with arrow should be preserved"
    );
}

#[test]
fn test_inline_code_mixed_with_diagram() {
    let config = Config::default();
    let content = "Inline `⇒` arrow:\n┌─┐\n│ │\n└─┘";
    let result = process_by_mode(&Mode::Diagram, content, false, &config);
    // Both inline code and diagram should be preserved
    assert!(result.contains("`⇒`"), "Inline code should be preserved");
    assert!(result.contains("┌─┐"), "Box should be preserved");
}

#[test]
fn test_inline_code_near_diagram_boundary() {
    let config = Config::default();
    let content = "Code: `⇒` text\n\nDiagram:\n┌──┐\n│xy│\n└──┘";
    let result = process_by_mode(&Mode::Diagram, content, false, &config);
    assert!(
        result.contains("`⇒`"),
        "Inline code at block boundary should be preserved"
    );
}

#[test]
fn test_empty_inline_code() {
    let config = Config::default();
    let content = "Empty `` code here";
    let result = process_by_mode(&Mode::Diagram, content, false, &config);
    assert!(result.contains("``"), "Empty code span should be preserved");
}

#[test]
fn test_readme_example_preservation() {
    let config = Config::default();
    // This is the exact example from the README that was failing
    let content = "## Diagram Types

ascfix recognizes various diagram types:

- Double arrows: `⇒ ⇓ ⇑ ⇐`
- Extended arrows: `⟶ ⟹`
- Box drawing: `┌─┐`";

    let result = process_by_mode(&Mode::Diagram, content, false, &config);

    // Verify all inline code is preserved
    assert!(
        result.contains("`⇒ ⇓ ⇑ ⇐`"),
        "Double arrows should be preserved"
    );
    assert!(
        result.contains("`⟶ ⟹`"),
        "Extended arrows should be preserved"
    );
    assert!(result.contains("`┌─┐`"), "Box drawing should be preserved");

    // Verify structure is intact
    assert!(
        result.contains("## Diagram Types"),
        "Headers should be preserved"
    );
    assert!(
        result.contains("ascfix recognizes"),
        "Text should be preserved"
    );
}

#[test]
fn test_inline_code_not_corrupted_by_detector() {
    let config = Config::default();
    // Content with diagram-like chars IN inline code mixed with normal content
    let content = "Use arrows `→ ⇒` for flow\nUse ┌ for corners\nNote: `├ ┤` are valid";
    let result = process_by_mode(&Mode::Diagram, content, false, &config);

    // Inline codes should be exactly preserved
    assert!(result.contains("`→ ⇒`"), "Arrow codes should be preserved");
    assert!(result.contains("`├ ┤`"), "Joint codes should be preserved");
}

#[test]
fn test_safe_mode_preserves_inline_code() {
    let config = Config::default();
    let content = "Safe mode test: `⇒ ⇓`";
    let result = process_by_mode(&Mode::Safe, content, false, &config);
    assert!(
        result.contains("`⇒ ⇓`"),
        "Safe mode should preserve inline code"
    );
}

#[test]
fn test_check_mode_preserves_inline_code() {
    let config = Config::default();
    let content = "Check mode test: `⇒ ⇓`";
    let result = process_by_mode(&Mode::Check, content, false, &config);
    assert!(
        result.contains("`⇒ ⇓`"),
        "Check mode should preserve inline code"
    );
}

#[test]
fn test_inline_code_with_special_characters() {
    let config = Config::default();
    let content = "Special: `<→>` and `«»` and `‖║`";
    let result = process_by_mode(&Mode::Diagram, content, false, &config);
    assert!(result.contains("`<→>`"), "Arrow combo should be preserved");
    assert!(result.contains("`«»`"), "Quotes should be preserved");
    assert!(result.contains("`‖║`"), "Lines should be preserved");
}

#[test]
fn test_inline_code_idempotency() {
    let config = Config::default();
    let content = "Line 1: `⇒ ⇓`\nLine 2: `normal`";

    // First pass
    let after_first = process_by_mode(&Mode::Diagram, content, false, &config);

    // Second pass - should be identical (idempotent)
    let after_second = process_by_mode(&Mode::Diagram, &after_first, false, &config);

    assert_eq!(after_first, after_second, "Processing should be idempotent");
    assert!(
        after_second.contains("`⇒ ⇓`"),
        "Content should still be preserved"
    );
}

#[test]
fn test_inline_code_doesnt_break_diagram_detection() {
    let config = Config::default();
    // Should still detect and fix the diagram, but preserve the inline code
    let content = "Info: `⇒` means arrow\n\n┌──┐\n│ab│\n└──┘";
    let result = process_by_mode(&Mode::Diagram, content, false, &config);

    // Inline code preserved
    assert!(result.contains("`⇒`"), "Inline code should be preserved");
    // Diagram is still detected and present
    assert!(result.contains("┌"), "Diagram should be detected");
}

#[test]
fn test_inline_code_with_newlines_outside_code() {
    let config = Config::default();
    let content = "`code` on line 1\n\n┌─┐\n│ │\n└─┘\n\n`code` on line after";
    let result = process_by_mode(&Mode::Diagram, content, false, &config);
    assert_eq!(
        result.matches("`code`").count(),
        2,
        "Both inline codes should be preserved"
    );
}
