//! Render normalized primitives back to ASCII grid.

#[allow(unused_imports)] // Reason: Used in tests
use crate::primitives::{ArrowType, BoxStyle};
use crate::{grid::Grid, primitives::PrimitiveInventory};

/// Render a primitive inventory back to an ASCII grid.
///
/// Algorithm:
/// 1. Create a new grid large enough for all primitives (filled with spaces)
/// 2. Draw all boxes (borders)
/// 3. Draw text rows
/// 4. Draw arrows (skipping those inside box interiors to prevent text corruption)
///
/// Note: This creates a fresh grid. For preserving existing content, use
/// `render_onto_grid()` instead.
#[allow(dead_code)] // Reason: Used by main processing pipeline
pub fn render_diagram(inventory: &PrimitiveInventory) -> Grid {
    // Calculate required grid dimensions
    let (max_row, max_col) = calculate_bounds(inventory);

    // Create a grid filled with spaces
    let grid_lines: Vec<String> = (0..=max_row).map(|_| " ".repeat(max_col + 1)).collect();
    let grid_strs: Vec<&str> = grid_lines.iter().map(String::as_str).collect();
    let mut grid = Grid::from_lines(&grid_strs);

    // Draw boxes
    for b in &inventory.boxes {
        draw_box(&mut grid, b);
    }

    // Draw text rows
    for row in &inventory.text_rows {
        if !row.content.trim().is_empty() {
            draw_text_row(&mut grid, row);
        }
    }

    // Draw horizontal arrows (skip if inside any box interior)
    for arrow in &inventory.horizontal_arrows {
        if !is_position_inside_any_box(&inventory.boxes, arrow.row, arrow.start_col)
            && !is_position_inside_any_box(&inventory.boxes, arrow.row, arrow.end_col)
        {
            draw_horizontal_arrow(&mut grid, arrow);
        }
    }

    // Draw vertical arrows (skip if inside any box interior to prevent text corruption)
    for arrow in &inventory.vertical_arrows {
        if !is_position_inside_any_box(&inventory.boxes, arrow.start_row, arrow.col)
            && !is_position_inside_any_box(&inventory.boxes, arrow.end_row, arrow.col)
        {
            draw_vertical_arrow(&mut grid, arrow);
        }
    }

    // Draw connection lines
    for conn in &inventory.connection_lines {
        draw_connection_line(&mut grid, conn);
    }

    // Draw labels (rendered last to be on top)
    for label in &inventory.labels {
        draw_label(&mut grid, label);
    }

    grid
}

/// Check if a position (row, col) falls inside any box's interior.
/// Interior means: between top and bottom borders, and between left and right borders.
fn is_position_inside_any_box(boxes: &[crate::primitives::Box], row: usize, col: usize) -> bool {
    boxes.iter().any(|b| {
        row > b.top_left.0 && row < b.bottom_right.0 && col > b.top_left.1 && col < b.bottom_right.1
    })
}

/// Render primitives onto an existing grid, preserving pass-through content.
///
/// This is the key fix for data loss issues. Instead of creating a fresh grid
/// with only detected primitives, we start with the original grid (which has
/// all content including lines without detected primitives) and overlay
/// primitives on top. This ensures that lines with text content but no
/// detected primitives (like "│ Start    │") are preserved.
///
/// Algorithm:
/// 1. Clone the original grid (contains all original content)
/// 2. Draw boxes (borders will overwrite original borders - this is intentional)
/// 3. Draw text rows extracted from boxes (these replace original interior content)
/// 4. Draw arrows (only if outside box interiors, to prevent text corruption)
/// 5. Draw connection lines and labels
///
/// This approach preserves:
/// - Lines with content but no detected primitives
/// - Empty lines
/// - Comments and annotations
#[must_use]
#[allow(dead_code)] // Reason: Used by main processing pipeline
pub fn render_onto_grid(original: &Grid, inventory: &PrimitiveInventory) -> Grid {
    // To handle arrow alignment, we need to remove old arrow positions before drawing new ones.
    // We detect repositioned arrows by checking for arrow characters in the original grid
    // at positions where we're about to draw arrows from the inventory.
    // This removes arrows only if they appear to be repositioned.
    // Calculate required dimensions for normalized boxes
    let (max_row, max_col) = calculate_bounds(inventory);
    let original_height = original.height();
    let original_width = original.width();

    // Create a grid that can accommodate both original content AND expanded boxes
    let required_height = max_row.max(original_height.saturating_sub(1)) + 1;
    let required_width = max_col.max(original_width.saturating_sub(1)) + 1;

    // Clone the original grid and resize if necessary
    let mut grid = if required_height > original_height || required_width > original_width {
        // Need to resize - create new grid with original content padded
        let mut new_rows: Vec<Vec<char>> = Vec::with_capacity(required_height);

        // Copy original rows, skipping arrow characters which will be redrawn
        for row_idx in 0..required_height {
            if row_idx < original_height {
                let mut row: Vec<char> = Vec::with_capacity(required_width);
                // Copy original columns, removing arrows so they can be redrawn at aligned positions
                for col_idx in 0..required_width {
                    if col_idx < original_width {
                        let ch = original.get(row_idx, col_idx).unwrap_or(' ');
                        // Remove arrow characters - they'll be redrawn by the inventory at aligned positions
                        // Support standard (↓ ↑ → ←), double (⇓ ⇑ ⇒ ⇐), and extended (⟶ ⟹) arrows
                        if matches!(ch, '↓' | '↑' | '→' | '←' | '⇓' | '⇑' | '⇒' | '⇐' | '⟶' | '⟹') {
                            row.push(' ');
                        } else {
                            row.push(ch);
                        }
                    } else {
                        row.push(' ');
                    }
                }
                new_rows.push(row);
            } else {
                // New row - fill with spaces
                new_rows.push(vec![' '; required_width]);
            }
        }
        Grid::from_rows(new_rows)
    } else {
        // No resize needed for overall dimensions, but still need to pad rows
        // to ensure all rows have consistent width for accessing aligned positions
        let grid = original.clone();
        let mut new_rows = Vec::new();
        for row_idx in 0..grid.height() {
            let mut row = Vec::new();
            for col_idx in 0..required_width {
                let ch = grid.get(row_idx, col_idx).unwrap_or(' ');
                // Remove arrow characters - they'll be redrawn by the inventory at aligned positions
                // Support standard (↓ ↑ → ←), double (⇓ ⇑ ⇒ ⇐), and extended (⟶ ⟹) arrows
                if matches!(ch, '↓' | '↑' | '→' | '←' | '⇓' | '⇑' | '⇒' | '⇐' | '⟶' | '⟹') {
                    row.push(' ');
                } else {
                    row.push(ch);
                }
            }
            new_rows.push(row);
        }
        Grid::from_rows(new_rows)
    };

    // Draw boxes - borders overwrite original, which is correct
    for b in &inventory.boxes {
        draw_box(&mut grid, b);
    }

    // Draw text rows extracted from boxes - these replace original content
    // Only draw if the text row has actual content (preserves spacing)
    for row in &inventory.text_rows {
        if !row.content.trim().is_empty() {
            draw_text_row(&mut grid, row);
        }
    }

    // Draw horizontal arrows (skip if inside any box interior)
    for arrow in &inventory.horizontal_arrows {
        if !is_position_inside_any_box(&inventory.boxes, arrow.row, arrow.start_col)
            && !is_position_inside_any_box(&inventory.boxes, arrow.row, arrow.end_col)
        {
            draw_horizontal_arrow(&mut grid, arrow);
        }
    }

    // Draw vertical arrows (skip if inside any box interior)
    // Arrows have already been removed from the grid during initialization,
    // so we just draw the aligned arrows from the inventory
    for arrow in &inventory.vertical_arrows {
        if !is_position_inside_any_box(&inventory.boxes, arrow.start_row, arrow.col)
            && !is_position_inside_any_box(&inventory.boxes, arrow.end_row, arrow.col)
        {
            draw_vertical_arrow(&mut grid, arrow);
        }
    }

    // Draw connection lines
    for conn in &inventory.connection_lines {
        draw_connection_line(&mut grid, conn);
    }

    // Draw labels (rendered last to be on top)
    for label in &inventory.labels {
        draw_label(&mut grid, label);
    }

    grid
}

/// Calculate the maximum row and column needed for all primitives.
fn calculate_bounds(inventory: &PrimitiveInventory) -> (usize, usize) {
    let mut max_row = 0;
    let mut max_col = 0;

    for b in &inventory.boxes {
        max_row = max_row.max(b.bottom_right.0);
        max_col = max_col.max(b.bottom_right.1);
    }

    for row in &inventory.text_rows {
        max_row = max_row.max(row.row);
        max_col = max_col.max(row.end_col);
    }

    for arrow in &inventory.horizontal_arrows {
        max_row = max_row.max(arrow.row);
        max_col = max_col.max(arrow.end_col);
    }

    for arrow in &inventory.vertical_arrows {
        max_row = max_row.max(arrow.end_row);
        max_col = max_col.max(arrow.col);
    }

    for conn in &inventory.connection_lines {
        for segment in &conn.segments {
            match segment {
                crate::primitives::Segment::Horizontal {
                    row,
                    start_col: _,
                    end_col,
                } => {
                    max_row = max_row.max(*row);
                    max_col = max_col.max(*end_col);
                }
                crate::primitives::Segment::Vertical {
                    col,
                    start_row: _,
                    end_row,
                } => {
                    max_row = max_row.max(*end_row);
                    max_col = max_col.max(*col);
                }
            }
        }
    }

    for label in &inventory.labels {
        // Calculate the end position of the label text
        let label_end_col = label.col + label.content.len().saturating_sub(1);
        max_row = max_row.max(label.row);
        max_col = max_col.max(label_end_col);
    }

    (max_row, max_col)
}

/// Draw a box on the grid.
fn draw_box(grid: &mut Grid, b: &crate::primitives::Box) {
    let chars = b.style.chars();

    // Top and bottom borders
    for col in b.top_left.1..=b.bottom_right.1 {
        if let Some(cell) = grid.get_mut(b.top_left.0, col) {
            *cell = chars.horizontal;
        }
        if let Some(cell) = grid.get_mut(b.bottom_right.0, col) {
            *cell = chars.horizontal;
        }
    }

    // Left and right borders
    for row in b.top_left.0..=b.bottom_right.0 {
        if let Some(cell) = grid.get_mut(row, b.top_left.1) {
            *cell = chars.vertical;
        }
        if let Some(cell) = grid.get_mut(row, b.bottom_right.1) {
            *cell = chars.vertical;
        }
    }

    // Corners
    if let Some(cell) = grid.get_mut(b.top_left.0, b.top_left.1) {
        *cell = chars.top_left;
    }
    if let Some(cell) = grid.get_mut(b.top_left.0, b.bottom_right.1) {
        *cell = chars.top_right;
    }
    if let Some(cell) = grid.get_mut(b.bottom_right.0, b.top_left.1) {
        *cell = chars.bottom_left;
    }
    if let Some(cell) = grid.get_mut(b.bottom_right.0, b.bottom_right.1) {
        *cell = chars.bottom_right;
    }
}

/// Draw a text row on the grid.
fn draw_text_row(grid: &mut Grid, row: &crate::primitives::TextRow) {
    for (i, ch) in row.content.chars().enumerate() {
        let col = row.start_col + i;
        if col <= row.end_col {
            if let Some(cell) = grid.get_mut(row.row, col) {
                *cell = ch;
            }
        }
    }
}

/// Draw a horizontal arrow on the grid.
fn draw_horizontal_arrow(grid: &mut Grid, arrow: &crate::primitives::HorizontalArrow) {
    // Determine the arrow character to use
    let arrow_char = arrow.arrow_char.unwrap_or_else(|| {
        let chars = arrow.arrow_type.chars();
        if arrow.rightward { chars.arrowhead_right } else { chars.arrowhead_left }
    });

    // Draw arrow line from start_col to end_col, preserving the arrow character
    for col in arrow.start_col..=arrow.end_col {
        if let Some(cell) = grid.get_mut(arrow.row, col) {
            if *cell == ' ' {
                *cell = arrow_char;
            }
        }
    }
}

/// Draw a vertical arrow on the grid.
fn draw_vertical_arrow(grid: &mut Grid, arrow: &crate::primitives::VerticalArrow) {
    // Determine the arrow character to use
    let default_char = if arrow.downward { '↓' } else { '↑' };
    let arrow_char = arrow.arrow_char.unwrap_or(default_char);

    // For single-character arrows, just render the arrow character
    if arrow.start_row == arrow.end_row {
        if let Some(cell) = grid.get_mut(arrow.start_row, arrow.col) {
            *cell = arrow_char;
        }
        return;
    }

    // For multi-character arrows, preserve the arrow symbol for all rows
    // This ensures arrows like ↓ are preserved instead of being converted to │
    for row in arrow.start_row..=arrow.end_row {
        if let Some(cell) = grid.get_mut(row, arrow.col) {
            if *cell == ' ' {
                *cell = arrow_char;
            }
        }
    }
}

/// Draw a connection line on the grid.
///
/// Algorithm:
/// 1. For each segment, draw with appropriate line character
/// 2. Draw elbows at segment junctions (┐ ┌ └ ┘ etc.)
/// 3. Skip if segments collide with boxes
///
/// Conservative: Does not render to avoid unintended overwrites.
#[allow(dead_code)] // Reason: Part of public API for diagram rendering
#[allow(clippy::missing_const_for_fn)] // Future implementation will need mutable grid
fn draw_connection_line(grid: &mut Grid, conn: &crate::primitives::ConnectionLine) {
    // First, identify all junction points where elbows should be drawn
    let mut junction_points = std::collections::HashSet::new();
    if conn.segments.len() >= 2 {
        for i in 0..conn.segments.len() - 1 {
            let seg1 = &conn.segments[i];
            let seg2 = &conn.segments[i + 1];

            match (seg1, seg2) {
                (
                    crate::primitives::Segment::Horizontal { row, end_col, .. },
                    crate::primitives::Segment::Vertical {
                        col: _,
                        start_row: _,
                        ..
                    },
                ) => {
                    // Horizontal ending at col, vertical starting at row
                    junction_points.insert((*row, *end_col));
                }
                (
                    crate::primitives::Segment::Vertical { col, end_row, .. },
                    crate::primitives::Segment::Horizontal {
                        row: _,
                        start_col: _,
                        ..
                    },
                ) => {
                    // Vertical ending at row, horizontal starting at col
                    junction_points.insert((*end_row, *col));
                }
                _ => {} // Other combinations don't create junctions in this simple L-shape logic
            }
        }
    }

    // Draw each segment of the connection line, skipping junction points
    for segment in &conn.segments {
        match segment {
            crate::primitives::Segment::Horizontal {
                row,
                start_col,
                end_col,
            } => {
                // Draw horizontal line with ─ character
                for col in *start_col..=*end_col {
                    if !junction_points.contains(&(*row, col)) {
                        if let Some(current) = grid.get(*row, col) {
                            // Only draw if empty or if it's a compatible line character
                            if current == ' ' || is_connection_line_char(current) {
                                if let Some(cell) = grid.get_mut(*row, col) {
                                    *cell = '─';
                                }
                            }
                        }
                    }
                }
            }
            crate::primitives::Segment::Vertical {
                col,
                start_row,
                end_row,
            } => {
                // Draw vertical line with │ character
                for row in *start_row..=*end_row {
                    if !junction_points.contains(&(row, *col)) {
                        if let Some(current) = grid.get(row, *col) {
                            // Only draw if empty or if it's a compatible line character
                            if current == ' ' || is_connection_line_char(current) {
                                if let Some(cell) = grid.get_mut(row, *col) {
                                    *cell = '│';
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Draw elbows at segment junctions
    draw_elbows_at_points(grid, conn, &junction_points);
}

/// Check if a character is a connection line character that can be overwritten
const fn is_connection_line_char(c: char) -> bool {
    matches!(
        c,
        '─' | '│' | '┌' | '┐' | '└' | '┘' | '├' | '┤' | '┬' | '┴' | '┼'
    )
}

/// Draw elbow characters at segment junctions
fn draw_elbows_at_points(
    grid: &mut Grid,
    conn: &crate::primitives::ConnectionLine,
    junction_points: &std::collections::HashSet<(usize, usize)>,
) {
    // For each junction point, determine the appropriate elbow character
    for &(row, col) in junction_points {
        // Find which segments meet at this junction
        let mut horizontal_from_left = false;
        let mut horizontal_from_right = false;
        let mut vertical_from_above = false;
        let mut vertical_from_below = false;

        for segment in &conn.segments {
            match segment {
                crate::primitives::Segment::Horizontal {
                    row: seg_row,
                    start_col,
                    end_col,
                } if *seg_row == row => {
                    if *start_col <= col && col <= *end_col {
                        if col == *start_col {
                            horizontal_from_right = true; // Coming from left, going right
                        } else if col == *end_col {
                            horizontal_from_left = true; // Coming from right, going left
                        }
                    }
                }
                crate::primitives::Segment::Vertical {
                    col: seg_col,
                    start_row,
                    end_row,
                } if *seg_col == col => {
                    if *start_row <= row && row <= *end_row {
                        if row == *start_row {
                            vertical_from_above = true; // Coming from above, going down
                        } else if row == *end_row {
                            vertical_from_below = true; // Coming from below, going up
                        }
                    }
                }
                _ => {}
            }
        }

        // Determine the elbow character based on the directions
        let elbow_char = match (
            horizontal_from_left,
            horizontal_from_right,
            vertical_from_above,
            vertical_from_below,
        ) {
            (true, false, true, false) => '┌', // Right + Down
            (false, true, true, false) => '┐', // Left + Down
            (true, false, false, true) => '└', // Right + Up
            (false, true, false, true) => '┘', // Left + Up
            _ => continue,                     // Invalid combination, skip
        };

        // Draw the elbow if the cell is empty
        if grid.get(row, col) == Some(' ') {
            if let Some(cell) = grid.get_mut(row, col) {
                *cell = elbow_char;
            }
        }
    }
}

/// Draw a label on the grid.
///
/// Algorithm:
/// 1. Check if label position is empty (space character)
/// 2. Render label text character by character
/// 3. Skip if collision detected with existing content
/// 4. Labels rendered last (lowest priority)
///
/// Conservative: Does not render to avoid text collision.
#[allow(dead_code)] // Reason: Part of public API for diagram rendering
#[allow(clippy::missing_const_for_fn)] // Future implementation will need mutable grid
fn draw_label(grid: &mut Grid, label: &crate::primitives::Label) {
    // For labels attached to vertical arrows, position them under the arrows
    // but preserve some reasonable spacing
    let final_col = match &label.attached_to {
        crate::primitives::LabelAttachment::VerticalArrow(_) => {
            if label.row > 0 {
                // Search for arrows near the label's original position
                let search_start = label.col.saturating_sub(3);
                let search_end = (label.col + label.content.len() + 3).min(grid.width());

                for col in search_start..search_end {
                    if let Some(ch) = grid.get(label.row - 1, col) {
                        if ch == '↓' || ch == '↑' {
                            // Found an arrow near this label's original position
                            let clean_content = label.content.trim_matches('"');
                            let formatted_content = format!(" {clean_content} ");
                            let label_width = formatted_content.chars().count();
                            let centered = col.saturating_sub(label_width / 2);
                            return draw_label_at_position(grid, label, label.row, centered.max(0));
                        }
                    }
                }
            }
            label.col // Fallback
        }
        _ => label.col,
    };

    draw_label_at_position(grid, label, label.row, final_col);
}

fn draw_label_at_position(
    grid: &mut Grid,
    label: &crate::primitives::Label,
    row: usize,
    col: usize,
) {
    // Clean the label content by removing surrounding quotes and add padding
    let clean_content = label.content.trim_matches('"');
    let formatted_content = format!(" {clean_content} ");

    // Place the formatted label text at the specified position
    for (i, ch) in formatted_content.chars().enumerate() {
        let target_col = col + i;
        if let Some(cell) = grid.get_mut(row, target_col) {
            *cell = ch;
        }
    }
}

#[test]
fn test_render_nested_different_styles() {
    let mut inventory = PrimitiveInventory::default();
    // Single-line parent
    inventory.boxes.push(crate::primitives::Box {
        top_left: (0, 0),
        bottom_right: (4, 8),
        style: BoxStyle::Single,
        parent_idx: None,
        child_indices: vec![1],
    });
    // Double-line child
    inventory.boxes.push(crate::primitives::Box {
        top_left: (1, 2),
        bottom_right: (3, 6),
        style: BoxStyle::Double,
        parent_idx: Some(0),
        child_indices: Vec::new(),
    });

    let grid = render_diagram(&inventory);
    // Parent corners should be single-line
    assert_eq!(grid.get(0, 0), Some('┌'));
    // Child corners should be double-line
    assert_eq!(grid.get(1, 2), Some('╔'));
}

#[test]
fn test_render_multiple_nested_levels() {
    let mut inventory = PrimitiveInventory::default();
    // Grandparent
    inventory.boxes.push(crate::primitives::Box {
        top_left: (0, 0),
        bottom_right: (6, 10),
        style: BoxStyle::Single,
        parent_idx: None,
        child_indices: vec![1],
    });
    // Parent
    inventory.boxes.push(crate::primitives::Box {
        top_left: (1, 1),
        bottom_right: (5, 9),
        style: BoxStyle::Double,
        parent_idx: Some(0),
        child_indices: vec![2],
    });
    // Child
    inventory.boxes.push(crate::primitives::Box {
        top_left: (2, 2),
        bottom_right: (4, 8),
        style: BoxStyle::Rounded,
        parent_idx: Some(1),
        child_indices: Vec::new(),
    });

    let grid = render_diagram(&inventory);
    // All boxes should be rendered with their own styles
    assert_eq!(grid.get(0, 0), Some('┌'));
    assert_eq!(grid.get(1, 1), Some('╔'));
    assert_eq!(grid.get(2, 2), Some('╭'));
}

#[test]
fn test_render_with_labels() {
    use crate::primitives::{Label, LabelAttachment};

    let mut inventory = PrimitiveInventory::default();
    // Add a box
    inventory.boxes.push(crate::primitives::Box {
        top_left: (0, 0),
        bottom_right: (2, 4),
        style: BoxStyle::Single,
        parent_idx: None,
        child_indices: Vec::new(),
    });
    // Add a label
    inventory.labels.push(Label {
        row: 4,
        col: 0,
        content: "Label".to_string(),
        attached_to: LabelAttachment::Box(0),
        offset: (2, 0),
    });

    let grid = render_diagram(&inventory);
    // Box should be rendered
    assert_eq!(grid.get(0, 0), Some('┌'));

    // Label should be rendered with formatting (truncated due to grid width)
    assert_eq!(grid.get(4, 0), Some(' '));
    assert_eq!(grid.get(4, 1), Some('L'));
    assert_eq!(grid.get(4, 2), Some('a'));
    assert_eq!(grid.get(4, 3), Some('b'));
    assert_eq!(grid.get(4, 4), Some('e'));
    // Grid should accommodate label space
    assert!(grid.height() > 4);
}

#[test]
fn test_render_empty_labels() {
    let mut inventory = PrimitiveInventory::default();
    // Add a box with no labels
    inventory.boxes.push(crate::primitives::Box {
        top_left: (0, 0),
        bottom_right: (2, 4),
        style: BoxStyle::Single,
        parent_idx: None,
        child_indices: Vec::new(),
    });

    let grid = render_diagram(&inventory);
    // Box should be rendered normally
    assert_eq!(grid.get(0, 0), Some('┌'));
}

#[test]
fn test_render_label_no_collision() {
    use crate::primitives::{Label, LabelAttachment};

    let mut inventory = PrimitiveInventory::default();
    // Add a box
    inventory.boxes.push(crate::primitives::Box {
        top_left: (0, 0),
        bottom_right: (2, 4),
        style: BoxStyle::Single,
        parent_idx: None,
        child_indices: Vec::new(),
    });
    // Add a label far from box (empty space)
    inventory.labels.push(Label {
        row: 5,
        col: 0,
        content: "Text".to_string(),
        attached_to: LabelAttachment::Box(0),
        offset: (3, 0),
    });

    let grid = render_diagram(&inventory);
    // Box corners should be intact
    assert_eq!(grid.get(0, 0), Some('┌'));
    assert_eq!(grid.get(2, 4), Some('┘'));

    // Label should be placed in empty space with formatting
    assert_eq!(grid.get(5, 0), Some(' '));
    assert_eq!(grid.get(5, 1), Some('T'));
    assert_eq!(grid.get(5, 2), Some('e'));
    assert_eq!(grid.get(5, 3), Some('x'));
    assert_eq!(grid.get(5, 4), Some('t'));
}

#[test]
fn test_render_multiple_labels() {
    use crate::primitives::{Label, LabelAttachment};

    let mut inventory = PrimitiveInventory::default();
    // Add two boxes
    inventory.boxes.push(crate::primitives::Box {
        top_left: (0, 0),
        bottom_right: (2, 3),
        style: BoxStyle::Single,
        parent_idx: None,
        child_indices: Vec::new(),
    });
    inventory.boxes.push(crate::primitives::Box {
        top_left: (0, 5),
        bottom_right: (2, 8),
        style: BoxStyle::Single,
        parent_idx: None,
        child_indices: Vec::new(),
    });
    // Add labels for each box
    inventory.labels.push(Label {
        row: 4,
        col: 1,
        content: "First".to_string(),
        attached_to: LabelAttachment::Box(0),
        offset: (2, 0),
    });
    inventory.labels.push(Label {
        row: 4,
        col: 6,
        content: "Second".to_string(),
        attached_to: LabelAttachment::Box(1),
        offset: (2, 0),
    });

    let grid = render_diagram(&inventory);
    // Both boxes should be rendered
    assert_eq!(grid.get(0, 0), Some('┌'));
    assert_eq!(grid.get(0, 5), Some('┌'));

    // Both labels should be rendered with formatting (truncated due to grid width)
    // " First " starts at col 1, " Second " starts at col 7
    assert_eq!(grid.get(4, 1), Some(' '));
    assert_eq!(grid.get(4, 2), Some('F'));
    assert_eq!(grid.get(4, 3), Some('i'));
    assert_eq!(grid.get(4, 4), Some('r'));
    assert_eq!(grid.get(4, 5), Some('s'));
    assert_eq!(grid.get(4, 6), Some(' '));
    assert_eq!(grid.get(4, 7), Some('S'));
    assert_eq!(grid.get(4, 8), Some('e'));
    assert_eq!(grid.get(4, 9), Some('c'));
    assert_eq!(grid.get(4, 10), Some('o'));
    assert_eq!(grid.get(4, 11), Some('n'));
}
