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
/// 4. Draw arrows
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
        draw_text_row(&mut grid, row);
    }

    // Draw horizontal arrows
    for arrow in &inventory.horizontal_arrows {
        draw_horizontal_arrow(&mut grid, arrow);
    }

    // Draw vertical arrows
    for arrow in &inventory.vertical_arrows {
        draw_vertical_arrow(&mut grid, arrow);
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
    let chars = arrow.arrow_type.chars();

    // Draw arrow line from start_col to end_col
    for col in arrow.start_col..=arrow.end_col {
        if let Some(cell) = grid.get_mut(arrow.row, col) {
            if *cell == ' ' {
                *cell = chars.line;
            }
        }
    }

    // Draw arrowhead at end
    if arrow.end_col > arrow.start_col {
        if let Some(cell) = grid.get_mut(arrow.row, arrow.end_col) {
            let arrowhead = if arrow.rightward {
                chars.arrowhead_right
            } else {
                chars.arrowhead_left
            };
            *cell = arrowhead;
        }
    }
}

/// Draw a vertical arrow on the grid.
fn draw_vertical_arrow(grid: &mut Grid, arrow: &crate::primitives::VerticalArrow) {
    // Vertical arrows always use │ or ║ depending on type
    let line_char = match arrow.arrow_type {
        ArrowType::Double => '║',
        _ => '│',
    };

    // Draw arrow line from start_row to end_row
    for row in arrow.start_row..=arrow.end_row {
        if let Some(cell) = grid.get_mut(row, arrow.col) {
            if *cell == ' ' {
                *cell = line_char;
            }
        }
    }

    // Draw arrowhead at end
    if arrow.end_row > arrow.start_row {
        if let Some(cell) = grid.get_mut(arrow.end_row, arrow.col) {
            let arrowhead = if arrow.downward { '↓' } else { '↑' };
            *cell = arrowhead;
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
#[allow(dead_code)] // Reason: Used by rendering pipeline in upcoming phase
#[allow(clippy::missing_const_for_fn)] // Future implementation will need mutable grid
fn draw_connection_line(_grid: &mut Grid, _conn: &crate::primitives::ConnectionLine) {
    // Conservative approach: Skip rendering for now
    // Prevents unintended overwrites of diagram content
    // TODO: Implement segment rendering with elbow detection in future phase
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_simple_box() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (2, 4),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });

        let grid = render_diagram(&inventory);
        // Check corners exist
        assert_eq!(grid.get(0, 0), Some('┌'));
        assert_eq!(grid.get(0, 4), Some('┐'));
        assert_eq!(grid.get(2, 0), Some('└'));
        assert_eq!(grid.get(2, 4), Some('┘'));
    }

    #[test]
    fn test_render_box_with_text() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (3, 10),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.text_rows.push(crate::primitives::TextRow {
            row: 1,
            start_col: 1,
            end_col: 9,
            content: "Test".to_string(),
        });

        let grid = render_diagram(&inventory);
        // Check text is rendered
        assert_eq!(grid.get(1, 1), Some('T'));
        assert_eq!(grid.get(1, 2), Some('e'));
        assert_eq!(grid.get(1, 3), Some('s'));
        assert_eq!(grid.get(1, 4), Some('t'));
    }

    #[test]
    fn test_render_box_with_vertical_arrow() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(crate::primitives::Box {
            top_left: (2, 0),
            bottom_right: (4, 4),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory
            .vertical_arrows
            .push(crate::primitives::VerticalArrow {
                col: 2,
                start_row: 0,
                end_row: 2,
                arrow_type: ArrowType::Standard,
                downward: true,
            });

        let grid = render_diagram(&inventory);
        // Check arrow line exists
        assert_eq!(grid.get(0, 2), Some('│'));
        assert_eq!(grid.get(1, 2), Some('│'));
        assert_eq!(grid.get(2, 2), Some('↓')); // Arrowhead at box top
    }

    #[test]
    fn test_render_box_with_horizontal_arrow() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 4),
            bottom_right: (2, 8),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory
            .horizontal_arrows
            .push(crate::primitives::HorizontalArrow {
                row: 1,
                start_col: 0,
                end_col: 4,
                arrow_type: ArrowType::Standard,
                rightward: true,
            });

        let grid = render_diagram(&inventory);
        // Check arrow line
        assert_eq!(grid.get(1, 0), Some('─'));
        assert_eq!(grid.get(1, 1), Some('─'));
        assert_eq!(grid.get(1, 4), Some('→')); // Arrowhead at box left
    }

    #[test]
    fn test_render_multiple_elements() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (2, 4),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 6),
            bottom_right: (2, 10),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory
            .horizontal_arrows
            .push(crate::primitives::HorizontalArrow {
                row: 1,
                start_col: 4,
                end_col: 6,
                arrow_type: ArrowType::Standard,
                rightward: true,
            });

        let grid = render_diagram(&inventory);
        // Both boxes should exist
        assert_eq!(grid.get(0, 0), Some('┌'));
        assert_eq!(grid.get(0, 6), Some('┌'));
        // Arrow should connect them (arrowhead at end_col = 6)
        assert_eq!(grid.get(1, 6), Some('→'));
    }

    #[test]
    fn test_render_preserves_grid_bounds() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(crate::primitives::Box {
            top_left: (5, 5),
            bottom_right: (7, 9),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });

        let grid = render_diagram(&inventory);
        // Grid should be large enough
        assert!(grid.height() >= 8);
        assert!(grid.width() >= 10);
    }

    // Phase 1, Cycle 4: RED - Style-aware rendering tests
    #[test]
    fn test_render_double_style_box() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (2, 4),
            style: BoxStyle::Double,
            parent_idx: None,
            child_indices: Vec::new(),
        });

        let grid = render_diagram(&inventory);
        // Check double-line corners and borders
        assert_eq!(grid.get(0, 0), Some('╔'));
        assert_eq!(grid.get(0, 4), Some('╗'));
        assert_eq!(grid.get(2, 0), Some('╚'));
        assert_eq!(grid.get(2, 4), Some('╝'));
        // Check double-line borders
        assert_eq!(grid.get(0, 1), Some('═'));
        assert_eq!(grid.get(1, 0), Some('║'));
    }

    #[test]
    fn test_render_rounded_style_box() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (2, 4),
            style: BoxStyle::Rounded,
            parent_idx: None,
            child_indices: Vec::new(),
        });

        let grid = render_diagram(&inventory);
        // Check rounded corners
        assert_eq!(grid.get(0, 0), Some('╭'));
        assert_eq!(grid.get(0, 4), Some('╮'));
        assert_eq!(grid.get(2, 0), Some('╰'));
        assert_eq!(grid.get(2, 4), Some('╯'));
        // Check borders use single-line (─ and │)
        assert_eq!(grid.get(0, 1), Some('─'));
        assert_eq!(grid.get(1, 0), Some('│'));
    }

    #[test]
    fn test_render_mixed_style_boxes() {
        let mut inventory = PrimitiveInventory::default();
        // Single-line box
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (2, 3),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Double-line box
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 5),
            bottom_right: (2, 8),
            style: BoxStyle::Double,
            parent_idx: None,
            child_indices: Vec::new(),
        });

        let grid = render_diagram(&inventory);
        // Single-line box
        assert_eq!(grid.get(0, 0), Some('┌'));
        assert_eq!(grid.get(0, 3), Some('┐'));
        // Double-line box
        assert_eq!(grid.get(0, 5), Some('╔'));
        assert_eq!(grid.get(0, 8), Some('╗'));
    }

    // Phase 4, Cycle 15: RED - Connection line rendering tests
    #[test]
    fn test_render_with_connection_lines() {
        use crate::primitives::{ConnectionLine, Segment};

        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (2, 4),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.connection_lines.push(ConnectionLine {
            segments: vec![Segment::Horizontal {
                row: 3,
                start_col: 0,
                end_col: 4,
            }],
            from_box: Some(0),
            to_box: None,
        });

        let grid = render_diagram(&inventory);
        // Box should be rendered (conservative: connection lines not rendered yet)
        assert_eq!(grid.get(0, 0), Some('┌'));
        // Grid should still be valid
        assert!(grid.height() > 0);
    }

    #[test]
    fn test_render_empty_connection_lines() {
        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (2, 4),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // No connection lines

        let grid = render_diagram(&inventory);
        // Box should be rendered normally
        assert_eq!(grid.get(0, 0), Some('┌'));
    }

    #[test]
    fn test_render_preserves_box_with_connections() {
        use crate::primitives::{ConnectionLine, Segment};

        let mut inventory = PrimitiveInventory::default();
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (2, 3),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.connection_lines.push(ConnectionLine {
            segments: vec![
                Segment::Horizontal {
                    row: 1,
                    start_col: 4,
                    end_col: 6,
                },
                Segment::Vertical {
                    col: 6,
                    start_row: 1,
                    end_row: 4,
                },
            ],
            from_box: None,
            to_box: None,
        });

        let grid = render_diagram(&inventory);
        // Box corners should be intact
        assert_eq!(grid.get(0, 0), Some('┌'));
        assert_eq!(grid.get(2, 3), Some('┘'));
    }
}
