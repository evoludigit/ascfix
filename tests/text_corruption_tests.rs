//! Regression tests for text corruption issues (Phase 1)

use ascfix::{
    grid::Grid,
    primitives::{Box as DiagramBox, BoxStyle, PrimitiveInventory, TextRow},
    renderer::render_diagram,
};

/// Test that text content is preserved and not corrupted by arrows or pipes
#[test]
fn test_no_text_corruption_in_simple_box() {
    let input = r"╭────────────────╮
│ Rounded        │
│ Corner Box     │
╰────────────────╯";

    // Parse the input into a grid (used to verify the test input)
    let lines: Vec<&str> = input.lines().collect();
    let _grid = Grid::from_lines(&lines);

    // Create a simple inventory with the box and text
    let mut inventory = PrimitiveInventory::default();
    inventory.boxes.push(DiagramBox {
        top_left: (0, 0),
        bottom_right: (3, 17),
        style: BoxStyle::Rounded,
        parent_idx: None,
        child_indices: Vec::new(),
    });
    // Text row for "Rounded"
    inventory.text_rows.push(TextRow {
        row: 1,
        start_col: 2,
        end_col: 15,
        content: "Rounded".to_string(),
    });
    // Text row for "Corner Box"
    inventory.text_rows.push(TextRow {
        row: 2,
        start_col: 2,
        end_col: 15,
        content: "Corner Box".to_string(),
    });

    // Render the diagram
    let result_grid = render_diagram(&inventory);
    let result = result_grid.render();

    // The text content should be preserved exactly
    assert!(
        result.contains("Rounded"),
        "Text 'Rounded' should be preserved in output:\n{result}"
    );
    assert!(
        result.contains("Corner Box"),
        "Text 'Corner Box' should be preserved in output:\n{result}"
    );

    // No corruption: arrows should not appear in text
    assert!(
        !result.contains("Corner↑Box"),
        "Arrow should not replace text. Output:\n{result}"
    );

    // Verify specific cell at position (2, 7) which should be 'B' not '↑'
    // "Corner Box" is at row 2, starting at col 2
    // Position 2,7 should be ' ' (space between "Corner" and "Box")
    if let Some(ch) = result_grid.get(2, 7) {
        assert!(
            ch == ' ' || ch.is_alphabetic(),
            "Cell at (2,7) should be space or letter, got '{ch}'"
        );
    }
}

/// Test that nested boxes don't corrupt text with parent borders
#[test]
fn test_no_corruption_in_nested_boxes() {
    let mut inventory = PrimitiveInventory::default();

    // Parent box
    inventory.boxes.push(DiagramBox {
        top_left: (0, 0),
        bottom_right: (4, 18),
        style: BoxStyle::Single,
        parent_idx: None,
        child_indices: vec![1],
    });

    // Child box (nested)
    inventory.boxes.push(DiagramBox {
        top_left: (2, 2),
        bottom_right: (3, 12),
        style: BoxStyle::Single,
        parent_idx: Some(0),
        child_indices: Vec::new(),
    });

    // Text inside child box
    inventory.text_rows.push(TextRow {
        row: 2,
        start_col: 3,
        end_col: 11,
        content: "Child 1".to_string(),
    });

    // Render
    let result_grid = render_diagram(&inventory);
    let result = result_grid.render();

    // Text should be preserved
    assert!(
        result.contains("Child 1"),
        "Text 'Child 1' should be preserved:\n{result}"
    );

    // No pipes should appear inside the text
    assert!(
        !result.contains("│ Child  │"),
        "Pipes should not corrupt text:\n{result}"
    );
}

/// Test that rendering order doesn't corrupt text
/// Boxes should be drawn first (borders), then text should fill interior
#[test]
fn test_rendering_order_preserves_text() {
    let mut inventory = PrimitiveInventory::default();

    // Simple box
    inventory.boxes.push(DiagramBox {
        top_left: (0, 0),
        bottom_right: (2, 10),
        style: BoxStyle::Single,
        parent_idx: None,
        child_indices: Vec::new(),
    });

    // Text row positioned inside the box
    inventory.text_rows.push(TextRow {
        row: 1,
        start_col: 2,
        end_col: 8,
        content: "Hello".to_string(),
    });

    let result_grid = render_diagram(&inventory);

    // Check specific positions
    // Box interior: row 1, cols 1-9
    // Text "Hello" at row 1, cols 2-6

    // Position 1,1 should be border
    assert_eq!(result_grid.get(0, 0), Some('┌')); // Top-left corner
    assert_eq!(result_grid.get(2, 10), Some('┘')); // Bottom-right corner

    // Position 1,2 should be 'H' (text content)
    assert_eq!(
        result_grid.get(1, 2),
        Some('H'),
        "Text should be at position (1,2), not overwritten by border"
    );

    // Position 1,3 should be 'e'
    assert_eq!(
        result_grid.get(1, 3),
        Some('e'),
        "Text should be at position (1,3)"
    );
}
