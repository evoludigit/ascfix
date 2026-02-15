//! Edge case tests for ascfix.
//!
//! These tests verify that the tool handles unusual or problematic inputs gracefully.
//! Edge cases should either normalize correctly or be skipped (conservative approach).

use std::fs;

#[test]
fn edge_case_single_character() {
    // Minimal valid input
    let input = "```\n┌┐\n└┘\n```";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    // Should not crash, output should be valid
    assert!(!result.is_empty());
}

#[test]
fn edge_case_empty_box() {
    // Box with no content
    let input = "┌──┐\n│  │\n└──┘";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    // Should handle gracefully
    assert!(!result.is_empty());
}

#[test]
fn edge_case_single_line_in_code_block() {
    // Very minimal markdown with diagram
    let input = "# Title\n\n```\n┌─┐\n│a│\n└─┘\n```";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    assert!(!result.is_empty());
    // Should preserve markdown structure
    assert!(result.contains("# Title"));
}

#[test]
fn edge_case_multiple_code_blocks() {
    // Multiple diagrams in one markdown file
    let input = "# Section 1\n\n```\n┌─┐\n│1│\n└─┘\n```\n\n# Section 2\n\n```\n┌─┐\n│2│\n└─┘\n```";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    assert!(!result.is_empty());
    // Both diagrams should be processed
    assert!(result.contains("Section 1"));
    assert!(result.contains("Section 2"));
}

#[test]
fn edge_case_box_text_overflow() {
    // Text content longer than box width
    let input = "```\n┌─────┐\n│Very Long Text│\n└─────┘\n```";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    // Should either expand or handle gracefully
    assert!(!result.is_empty());
}

#[test]
fn edge_case_deeply_nested_boxes() {
    // 3+ levels of nesting
    let input = "```\n┌────────────────────────┐\n│ Level 1                │\n│  ┌──────────────────┐  │\n│  │ Level 2          │  │\n│  │  ┌────────────┐  │  │\n│  │  │ Level 3    │  │  │\n│  │  └────────────┘  │  │\n│  └──────────────────┘  │\n└────────────────────────┘\n```";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    // Should handle without infinite loops or crashes
    assert!(!result.is_empty());
}

#[test]
fn edge_case_overlapping_boxes_partial() {
    // Boxes that partially overlap (not fully nested, not adjacent)
    let input = "```\n┌───────┐\n│ Box 1 │┐\n│       ││ Box 2\n└───────┘│\n        └┘\n```";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    // Should detect boxes or skip if ambiguous
    assert!(!result.is_empty());
}

#[test]
fn edge_case_mixed_box_styles_in_hierarchy() {
    // Parent and child with different styles
    let input = "```\n┌─────────────────┐\n│ Single Parent   │\n│ ╔═════════════╗ │\n│ ║ Double Kid  ║ │\n│ ╚═════════════╝ │\n└─────────────────┘\n```";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    assert!(!result.is_empty());
}

#[test]
fn edge_case_arrow_at_diagram_boundary() {
    // Arrow right at the edge
    let input = "```\n┌─┐→\n│a│\n└─┘\n```";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    assert!(!result.is_empty());
}

#[test]
fn edge_case_very_wide_box() {
    // Box wider than 80 characters
    let input = "```\n┌─────────────────────────────────────────────────────────────────────────────┐\n│ This is a very wide box that tests horizontal rendering limits                 │\n└─────────────────────────────────────────────────────────────────────────────┘\n```";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    assert!(!result.is_empty());
}

#[test]
fn edge_case_very_tall_box() {
    // Box with many lines of content
    let input = "```\n┌────────┐\n│ Line 1 │\n│ Line 2 │\n│ Line 3 │\n│ Line 4 │\n│ Line 5 │\n│ Line 6 │\n│ Line 7 │\n│ Line 8 │\n│ Line 9 │\n│Line 10 │\n└────────┘\n```";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    assert!(!result.is_empty());
}

#[test]
fn edge_case_consecutive_boxes_no_space() {
    // Boxes with no gap between them
    let input = "```\n┌─┐┌─┐\n│a││b│\n└─┘└─┘\n```";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    // Should detect as two separate boxes
    assert!(!result.is_empty());
}

#[test]
fn edge_case_unicode_box_characters_mixed() {
    // Mix of different unicode box drawing styles
    let input =
        "```\n┌─────┐ ╔═════╗ ╭─────╮\n│ Box │ ║Box2 ║ │Box3 │\n└─────┘ ╚═════╝ ╰─────╯\n```";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    assert!(!result.is_empty());
}

#[test]
fn edge_case_broken_arrow_chain() {
    // Arrow chain with gaps that might not connect properly
    let input = "```\n┌───┐\n│ A │\n└─┬─┘\n  │\n┌─┴─┐\n│ B │\n└─┬─┘\n  │\n  (missing arrow)\n┌─┴─┐\n│ C │\n└───┘\n```";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    assert!(!result.is_empty());
}

#[test]
fn edge_case_text_rows_preservation() {
    // Text outside boxes should be preserved
    let golden_input = fs::read_to_string("tests/data/unit/input/markdown_with_diagram.md")
        .expect("Failed to read fixture");
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Diagram,
        &golden_input,
        false,
        &ascfix::config::Config::default(),
    );
    // Should preserve surrounding text
    assert!(result.contains("Workflow") || result.contains("Start"));
}

#[test]
fn edge_case_table_with_extra_columns_should_not_panic() {
    // Issue #7: Table with more cells than headers should not panic
    let input = "# Test Table\n\n| Col1 | Col2 | Col3 | Col4 |\n|------|------|------|------|\n| A | B | C | D |\n| E | F | G | H | I |\n";
    let result = std::panic::catch_unwind(|| {
        ascfix::modes::process_by_mode(
            &ascfix::cli::Mode::Safe,
            input,
            false,
            &ascfix::config::Config::default(),
        )
    });
    assert!(
        result.is_ok(),
        "Should not panic on table with extra columns"
    );
    let output = result.unwrap();
    assert!(!output.is_empty());
    // The row with extra column should be handled gracefully (truncated)
    // Note: cells are padded to column width during normalization
    assert!(output.contains("| E    | F    | G    | H    |"));
}

#[test]
fn edge_case_table_with_fewer_columns_should_work() {
    // Issue #7: Table with fewer cells than headers should work
    let input = "| Col1 | Col2 | Col3 | Col4 |\n|------|------|------|------|\n| A | B | C |\n";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    assert!(!result.is_empty());
    // Row with fewer columns should still render (missing cells are empty)
    assert!(result.contains("| A    | B    | C    |"));
}

#[test]
fn edge_case_list_blank_line_after_colon() {
    // Issue #8: Lists should have blank line before them for Pandoc compatibility
    let input = "Requirements:\n- Item 1\n- Item 2";
    let result = ascfix::lists::normalize_loose_lists(input);
    // Should insert blank line after "Requirements:"
    assert!(
        result.contains("Requirements:\n\n- Item 1"),
        "Should add blank line before list after colon. Got:\n{result}"
    );
}

#[test]
fn edge_case_list_blank_line_after_bold_colon() {
    // Issue #8: Bold text with colon should have blank line before list
    let input = "**Requirements:**\n- Item 1\n- Item 2";
    let result = ascfix::lists::normalize_loose_lists(input);
    assert!(
        result.contains("**Requirements:**\n\n- Item 1"),
        "Should add blank line after bold text with colon. Got:\n{result}"
    );
}

#[test]
fn edge_case_list_blank_line_after_word_bold_colon() {
    // Issue #8: Bold word followed by colon should have blank line before list
    let input = "Tools like **npm**:\n- Fast\n- Reliable";
    let result = ascfix::lists::normalize_loose_lists(input);
    assert!(
        result.contains("**npm**:\n\n- Fast"),
        "Should add blank line after bold word with colon. Got:\n{result}"
    );
}

#[test]
fn edge_case_list_no_duplicate_blank_line() {
    // Issue #8: Should not add blank line if one already exists
    let input = "Requirements:\n\n- Item 1\n- Item 2";
    let result = ascfix::lists::normalize_loose_lists(input);
    // Should not add extra blank line
    assert!(
        !result.contains("\n\n\n"),
        "Should not add duplicate blank line. Got:\n{result}"
    );
    assert!(
        result.contains("Requirements:\n\n- Item 1"),
        "Should preserve existing blank line. Got:\n{result}"
    );
}

#[test]
fn edge_case_list_nested_list_no_blank_line() {
    // Issue #8: Nested lists should not get blank line (previous line is list item)
    let input = "- Item 1\n  - Nested item";
    let result = ascfix::lists::normalize_loose_lists(input);
    // Should not add blank line between parent and nested list
    assert!(
        !result.contains("- Item 1\n\n  -"),
        "Should not add blank line in nested list. Got:\n{result}"
    );
}

#[test]
fn edge_case_list_in_code_block_preserved() {
    // Issue #8: Lists in code blocks should not be modified
    let input = "```markdown\nRequirements:\n- Item 1\n```";
    let result = ascfix::lists::normalize_loose_lists(input);
    // Should not modify content inside code block
    assert!(
        result.contains("Requirements:\n- Item 1"),
        "Should preserve list in code block. Got:\n{result}"
    );
    assert!(
        !result.contains("Requirements:\n\n- Item 1"),
        "Should not add blank line inside code block. Got:\n{result}"
    );
}

#[test]
fn edge_case_normalize_loose_lists_called_in_safe_mode() {
    // Issue #9: normalize_loose_lists should be called in safe mode
    let input = "# Test\n\n**Requirements:**\n- Item 1\n- Item 2";
    let result = ascfix::modes::process_by_mode(
        &ascfix::cli::Mode::Safe,
        input,
        false,
        &ascfix::config::Config::default(),
    );
    // Should add blank line in safe mode (not just in normalize_loose_lists directly)
    assert!(
        result.contains("**Requirements:**\n\n- Item 1"),
        "Safe mode should add blank lines before lists. Got:\n{result}"
    );
}
