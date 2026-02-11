//! Edge case tests for ascfix.
//!
//! These tests verify that the tool handles unusual or problematic inputs gracefully.
//! Edge cases should either normalize correctly or be skipped (conservative approach).

use std::fs;

#[test]
fn edge_case_single_character() {
    // Minimal valid input
    let input = "```\n┌┐\n└┘\n```";
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &ascfix::config::Config::default());
    // Should not crash, output should be valid
    assert!(!result.is_empty());
}

#[test]
fn edge_case_empty_box() {
    // Box with no content
    let input = "┌──┐\n│  │\n└──┘";
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &ascfix::config::Config::default());
    // Should handle gracefully
    assert!(!result.is_empty());
}

#[test]
fn edge_case_single_line_in_code_block() {
    // Very minimal markdown with diagram
    let input = "# Title\n\n```\n┌─┐\n│a│\n└─┘\n```";
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &ascfix::config::Config::default());
    assert!(!result.is_empty());
    // Should preserve markdown structure
    assert!(result.contains("# Title"));
}

#[test]
fn edge_case_multiple_code_blocks() {
    // Multiple diagrams in one markdown file
    let input = "# Section 1\n\n```\n┌─┐\n│1│\n└─┘\n```\n\n# Section 2\n\n```\n┌─┐\n│2│\n└─┘\n```";
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &ascfix::config::Config::default());
    assert!(!result.is_empty());
    // Both diagrams should be processed
    assert!(result.contains("Section 1"));
    assert!(result.contains("Section 2"));
}

#[test]
fn edge_case_box_text_overflow() {
    // Text content longer than box width
    let input = "```\n┌─────┐\n│Very Long Text│\n└─────┘\n```";
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &ascfix::config::Config::default());
    // Should either expand or handle gracefully
    assert!(!result.is_empty());
}

#[test]
fn edge_case_deeply_nested_boxes() {
    // 3+ levels of nesting
    let input = "```\n┌────────────────────────┐\n│ Level 1                │\n│  ┌──────────────────┐  │\n│  │ Level 2          │  │\n│  │  ┌────────────┐  │  │\n│  │  │ Level 3    │  │  │\n│  │  └────────────┘  │  │\n│  └──────────────────┘  │\n└────────────────────────┘\n```";
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &ascfix::config::Config::default());
    // Should handle without infinite loops or crashes
    assert!(!result.is_empty());
}

#[test]
fn edge_case_overlapping_boxes_partial() {
    // Boxes that partially overlap (not fully nested, not adjacent)
    let input = "```\n┌───────┐\n│ Box 1 │┐\n│       ││ Box 2\n└───────┘│\n        └┘\n```";
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &ascfix::config::Config::default());
    // Should detect boxes or skip if ambiguous
    assert!(!result.is_empty());
}

#[test]
fn edge_case_mixed_box_styles_in_hierarchy() {
    // Parent and child with different styles
    let input = "```\n┌─────────────────┐\n│ Single Parent   │\n│ ╔═════════════╗ │\n│ ║ Double Kid  ║ │\n│ ╚═════════════╝ │\n└─────────────────┘\n```";
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &ascfix::config::Config::default());
    assert!(!result.is_empty());
}

#[test]
fn edge_case_arrow_at_diagram_boundary() {
    // Arrow right at the edge
    let input = "```\n┌─┐→\n│a│\n└─┘\n```";
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &ascfix::config::Config::default());
    assert!(!result.is_empty());
}

#[test]
fn edge_case_very_wide_box() {
    // Box wider than 80 characters
    let input = "```\n┌─────────────────────────────────────────────────────────────────────────────┐\n│ This is a very wide box that tests horizontal rendering limits                 │\n└─────────────────────────────────────────────────────────────────────────────┘\n```";
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &ascfix::config::Config::default());
    assert!(!result.is_empty());
}

#[test]
fn edge_case_very_tall_box() {
    // Box with many lines of content
    let input = "```\n┌────────┐\n│ Line 1 │\n│ Line 2 │\n│ Line 3 │\n│ Line 4 │\n│ Line 5 │\n│ Line 6 │\n│ Line 7 │\n│ Line 8 │\n│ Line 9 │\n│Line 10 │\n└────────┘\n```";
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &ascfix::config::Config::default());
    assert!(!result.is_empty());
}

#[test]
fn edge_case_consecutive_boxes_no_space() {
    // Boxes with no gap between them
    let input = "```\n┌─┐┌─┐\n│a││b│\n└─┘└─┘\n```";
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &ascfix::config::Config::default());
    // Should detect as two separate boxes
    assert!(!result.is_empty());
}

#[test]
fn edge_case_unicode_box_characters_mixed() {
    // Mix of different unicode box drawing styles
    let input =
        "```\n┌─────┐ ╔═════╗ ╭─────╮\n│ Box │ ║Box2 ║ │Box3 │\n└─────┘ ╚═════╝ ╰─────╯\n```";
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &ascfix::config::Config::default());
    assert!(!result.is_empty());
}

#[test]
fn edge_case_broken_arrow_chain() {
    // Arrow chain with gaps that might not connect properly
    let input = "```\n┌───┐\n│ A │\n└─┬─┘\n  │\n┌─┴─┐\n│ B │\n└─┬─┘\n  │\n  (missing arrow)\n┌─┴─┐\n│ C │\n└───┘\n```";
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &ascfix::config::Config::default());
    assert!(!result.is_empty());
}

#[test]
fn edge_case_text_rows_preservation() {
    // Text outside boxes should be preserved
    let golden_input = fs::read_to_string("tests/data/unit/input/markdown_with_diagram.md")
        .expect("Failed to read fixture");
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, &golden_input, false, &ascfix::config::Config::default());
    // Should preserve surrounding text
    assert!(result.contains("Workflow") || result.contains("Start"));
}
