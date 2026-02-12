//! Regression tests for nested box containment (Phase 3)

use ascfix::{
    normalizer::{
        align_horizontal_arrows, align_vertical_arrows, balance_horizontal_boxes,
        normalize_box_widths, normalize_nested_boxes, normalize_padding,
    },
    primitives::{Box as DiagramBox, BoxStyle, PrimitiveInventory, TextRow},
    renderer::render_diagram,
};

/// Check if a character represents a border of the given box
fn is_border_char_of_box(ch: char, b: &DiagramBox) -> bool {
    let chars = b.style.chars();

    // Check corners
    if ch == chars.top_left
        || ch == chars.top_right
        || ch == chars.bottom_left
        || ch == chars.bottom_right
    {
        return true;
    }

    // Check horizontal borders
    if ch == chars.horizontal {
        return true;
    }

    // Check vertical borders
    if ch == chars.vertical {
        return true;
    }

    false
}

/// Test that parent boxes expand to contain children with proper margins
#[test]
fn test_parent_expands_to_contain_children() {
    let mut inventory = PrimitiveInventory::default();

    // Parent box (too small initially)
    inventory.boxes.push(DiagramBox {
        top_left: (0, 0),
        bottom_right: (4, 15), // 4 rows, 15 cols - too small
        style: BoxStyle::Single,
        parent_idx: None,
        child_indices: vec![1, 2],
    });

    // Child 1
    inventory.boxes.push(DiagramBox {
        top_left: (2, 2),
        bottom_right: (4, 10), // 2 rows, 8 cols
        style: BoxStyle::Single,
        parent_idx: Some(0),
        child_indices: Vec::new(),
    });

    // Child 2
    inventory.boxes.push(DiagramBox {
        top_left: (2, 12),
        bottom_right: (4, 20), // 2 rows, 8 cols, extends beyond parent
        style: BoxStyle::Single,
        parent_idx: Some(0),
        child_indices: Vec::new(),
    });

    // Text in children
    inventory.text_rows.push(TextRow {
        row: 3,
        start_col: 3,
        end_col: 9,
        content: "Child A".to_string(),
    });
    inventory.text_rows.push(TextRow {
        row: 3,
        start_col: 13,
        end_col: 19,
        content: "Child B".to_string(),
    });

    // Run the full normalization pipeline
    let after_widths = normalize_box_widths(&inventory);
    println!("After normalize_box_widths:");
    for (i, b) in after_widths.boxes.iter().enumerate() {
        println!(
            "  Box {}: ({},{}) -> ({},{})",
            i, b.top_left.0, b.top_left.1, b.bottom_right.0, b.bottom_right.1
        );
    }

    let after_nested = normalize_nested_boxes(&after_widths);
    println!("After normalize_nested_boxes:");
    for (i, b) in after_nested.boxes.iter().enumerate() {
        println!(
            "  Box {}: ({},{}) -> ({},{})",
            i, b.top_left.0, b.top_left.1, b.bottom_right.0, b.bottom_right.1
        );
    }

    let after_arrows_h = align_horizontal_arrows(&after_nested);
    println!("After align_horizontal_arrows:");
    for (i, b) in after_arrows_h.boxes.iter().enumerate() {
        println!(
            "  Box {}: ({},{}) -> ({},{})",
            i, b.top_left.0, b.top_left.1, b.bottom_right.0, b.bottom_right.1
        );
    }

    let after_arrows_v = align_vertical_arrows(&after_arrows_h);
    println!("After align_vertical_arrows:");
    for (i, b) in after_arrows_v.boxes.iter().enumerate() {
        println!(
            "  Box {}: ({},{}) -> ({},{})",
            i, b.top_left.0, b.top_left.1, b.bottom_right.0, b.bottom_right.1
        );
    }

    let after_balance = balance_horizontal_boxes(&after_arrows_v);
    println!("After balance_horizontal_boxes:");
    for (i, b) in after_balance.boxes.iter().enumerate() {
        println!(
            "  Box {}: ({},{}) -> ({},{})",
            i, b.top_left.0, b.top_left.1, b.bottom_right.0, b.bottom_right.1
        );
    }

    let normalized = normalize_padding(&after_balance);

    // Debug: print text rows
    println!("Text rows after normalization:");
    for (i, row) in normalized.text_rows.iter().enumerate() {
        println!(
            "  Text row {}: row={}, col={}-{}, content='{}'",
            i, row.row, row.start_col, row.end_col, row.content
        );
    }

    let result_grid = render_diagram(&normalized);

    // Debug: check row 3
    println!("Row 3 contents:");
    for col in 0..result_grid.width() {
        let ch = result_grid.get(3, col).unwrap_or('?');
        print!("{}", ch);
    }
    println!();

    // Main requirement: no text corruption in nested boxes
    let result = result_grid.render();
    println!("Full result:\n{}", result);
    assert!(!result.contains('↑'), "Arrow in text: {}", result);
    assert!(!result.contains('│'), "Pipe in text: {}", result);

    // Also verify parent expanded correctly
    assert!(
        normalized.boxes[0].width() >= 22,
        "Parent should be at least 22 wide, got {}",
        normalized.boxes[0].width()
    );
}

/// Test that complex 3-level nesting works
#[test]
fn test_three_level_nesting() {
    let mut inventory = PrimitiveInventory::default();

    // Grandparent
    inventory.boxes.push(DiagramBox {
        top_left: (0, 0),
        bottom_right: (10, 25),
        style: BoxStyle::Single,
        parent_idx: None,
        child_indices: vec![1],
    });

    // Parent
    inventory.boxes.push(DiagramBox {
        top_left: (2, 2),
        bottom_right: (8, 23),
        style: BoxStyle::Double,
        parent_idx: Some(0),
        child_indices: vec![2],
    });

    // Child
    inventory.boxes.push(DiagramBox {
        top_left: (4, 5),
        bottom_right: (6, 15),
        style: BoxStyle::Rounded,
        parent_idx: Some(1),
        child_indices: Vec::new(),
    });

    // Run the full normalization pipeline
    let normalized = normalize_box_widths(&inventory);
    let normalized = normalize_nested_boxes(&normalized);
    let normalized = align_horizontal_arrows(&normalized);
    let normalized = align_vertical_arrows(&normalized);
    let normalized = balance_horizontal_boxes(&normalized);
    let normalized = normalize_padding(&normalized);

    let result_grid = render_diagram(&normalized);

    // All three boxes should be rendered with their styles
    assert_eq!(result_grid.get(0, 0), Some('┌')); // Grandparent: single
    assert_eq!(result_grid.get(2, 2), Some('╔')); // Parent: double
    assert_eq!(result_grid.get(4, 5), Some('╭')); // Child: rounded

    // No text corruption
    let result = result_grid.render();
    assert!(!result.contains('↑'), "Arrow in text: {}", result);
    assert!(!result.contains('│'), "Pipe in text: {}", result);
}

/// Test with parent and child to isolate the issue
#[test]
fn test_parent_child_rendering() {
    let mut inventory = PrimitiveInventory::default();

    // Parent box
    inventory.boxes.push(DiagramBox {
        top_left: (0, 0),
        bottom_right: (6, 15), // Parent
        style: BoxStyle::Single,
        parent_idx: None,
        child_indices: vec![1],
    });

    // Child box
    inventory.boxes.push(DiagramBox {
        top_left: (2, 2),
        bottom_right: (4, 10), // Child
        style: BoxStyle::Single,
        parent_idx: Some(0),
        child_indices: Vec::new(),
    });

    // Text inside child
    inventory.text_rows.push(TextRow {
        row: 3,
        start_col: 3, // Child interior
        end_col: 9,
        content: "Hello".to_string(),
    });

    // Run normalization
    let normalized = normalize_box_widths(&inventory);
    let normalized = normalize_nested_boxes(&normalized);
    let normalized = normalize_padding(&normalized);

    println!("Final boxes:");
    for (i, b) in normalized.boxes.iter().enumerate() {
        println!(
            "  Box {}: ({},{}) -> ({},{})",
            i, b.top_left.0, b.top_left.1, b.bottom_right.0, b.bottom_right.1
        );
    }

    println!("Final text:");
    for (i, row) in normalized.text_rows.iter().enumerate() {
        println!(
            "  Text {}: row={}, col={}-{}, content='{}'",
            i, row.row, row.start_col, row.end_col, row.content
        );
    }

    let result_grid = render_diagram(&normalized);
    let result = result_grid.render();

    println!("Result:\n{}", result);

    // Check row 3 where text should be
    let lines: Vec<&str> = result.lines().collect();
    if lines.len() > 3 {
        let row3 = lines[3];
        println!("Row 3: '{}'", row3);

        // Text should be clean
        assert!(
            !row3.contains('│'),
            "Pipe found in text row. Row 3: '{}'",
            row3
        );
    }
}
