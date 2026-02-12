//! Regression tests for data loss issues (Phase 2)

/// Test that no lines are lost during processing
#[test]
fn test_no_line_loss_connection_lines() {
    let input = r"# Connection Lines Feature

Testing L-shaped connection paths:

┌──────────┐
│ Start    │
└─────┬────┘
      │
      └─────┬──────────┐
            │          │
        ┌───▼────┐  ┌──▼───┐
        │ Path 1 │  │ Path2 │
        └────────┘  └───────┘

This diagram tests connection line rendering.";

    // Process through the full pipeline
    let config = ascfix::config::Config::default();
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &config);

    let input_lines: Vec<&str> = input.lines().collect();
    let result_lines: Vec<&str> = result.lines().collect();

    // Line count should be preserved
    assert_eq!(
        result_lines.len(),
        input_lines.len(),
        "Line count mismatch: input had {}, output has {}\n\nInput:\n{}\n\nOutput:\n{}",
        input_lines.len(),
        result_lines.len(),
        input,
        result
    );

    // Key content should be preserved
    assert!(
        result.contains("│ Start    │"),
        "Start box content lost:\n{result}"
    );
    assert!(
        result.contains("└─────┬────┘"),
        "Start box bottom border lost:\n{result}"
    );
}

/// Test that nested box content is extracted and preserved
#[test]
fn test_nested_box_content_preserved() {
    let input = r"# Nested Boxes

┌─────────────────────┐
│ Parent              │
│  ┌──────────────┐   │
│  │ Child 1      │   │
│  └──────────────┘   │
└─────────────────────┘";

    let config = ascfix::config::Config::default();
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &config);

    // Both parent and child text should be preserved
    assert!(result.contains("Parent"), "Parent text lost:\n{result}");
    assert!(result.contains("Child 1"), "Child text lost:\n{result}");
}

/// Test that diagrams with only connection lines don't lose content
#[test]
fn test_connection_only_content_preserved() {
    let input = r"Test:

┌──────┐
│ Box  │
└──┬───┘
   │
   ▼
┌──────┐
│ Next │
└──────┘";

    let config = ascfix::config::Config::default();
    let result = ascfix::modes::process_by_mode(&ascfix::cli::Mode::Diagram, input, false, &config);

    // All boxes should be present (check for content, not exact spacing)
    assert!(result.contains("Box"), "First box content lost:\n{result}");
    assert!(
        result.contains("Next"),
        "Second box content lost:\n{result}"
    );
    assert!(
        result.contains("┌──────┐"),
        "First box top border lost:\n{result}"
    );
    assert!(
        result.contains("└──┬───┘") || result.contains("└───┬──┘"),
        "Connection point lost:\n{result}"
    );
}
