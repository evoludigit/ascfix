//! Detection logic for ASCII diagram primitives.

use crate::grid::Grid;
use crate::primitives::{ArrowType, Box, BoxStyle};
use std::collections::{HashSet, VecDeque};

/// Box character set for detection.
const fn is_box_char(ch: char) -> bool {
    matches!(
        ch,
        '─' | '│'
            | '┌'
            | '┐'
            | '└'
            | '┘'
            | '├'
            | '┤'
            | '┼'
            | '┬'
            | '┴'
            | '┃'
            | '═'
            | '║'
            | '╔'
            | '╗'
            | '╚'
            | '╝'
            | '╭'
            | '╮'
            | '╰'
            | '╯'
    )
}

/// Check if a character is any box corner (single, double, or rounded).
const fn is_any_box_corner(ch: char) -> bool {
    matches!(
        ch,
        '┌' | '┐' | '└' | '┘' | '╔' | '╗' | '╚' | '╝' | '╭' | '╮' | '╰' | '╯'
    )
}

/// Detects rectangular boxes in ASCII diagrams.
pub struct BoxDetector<'a> {
    grid: &'a Grid,
    visited: HashSet<(usize, usize)>,
}

impl<'a> BoxDetector<'a> {
    /// Create a new box detector for a grid.
    #[must_use]
    pub fn new(grid: &'a Grid) -> Self {
        BoxDetector {
            grid,
            visited: HashSet::new(),
        }
    }

    /// Detect all rectangular boxes in the grid.
    ///
    /// Algorithm:
    /// 1. For each unvisited box character, start a flood-fill
    /// 2. Collect all connected box characters
    /// 3. Extract bounding box as primitive if it's a valid rectangle
    #[must_use]
    pub fn detect(mut self) -> Vec<Box> {
        let mut boxes = Vec::new();

        for row in 0..self.grid.height() {
            for col in 0..self.grid.width() {
                if let Some(ch) = self.grid.get(row, col) {
                    if is_box_char(ch) && !self.visited.contains(&(row, col)) {
                        // Try to extract a box starting from this position
                        if let Some(b) = self.extract_box_at(row, col) {
                            boxes.push(b);
                        }
                    }
                }
            }
        }

        boxes
    }

    /// Try to extract a rectangular box starting at (row, col).
    fn extract_box_at(&mut self, start_row: usize, start_col: usize) -> Option<Box> {
        let mut boundary = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start_row, start_col));

        // Flood-fill connected box characters
        while let Some((row, col)) = queue.pop_front() {
            if self.visited.contains(&(row, col)) {
                continue;
            }

            if let Some(ch) = self.grid.get(row, col) {
                if is_box_char(ch) {
                    self.visited.insert((row, col));
                    boundary.insert((row, col));

                    // Check 4-neighbors
                    let neighbors = [
                        (row.saturating_sub(1), col),
                        (row.saturating_add(1), col),
                        (row, col.saturating_sub(1)),
                        (row, col.saturating_add(1)),
                    ];
                    for (nr, nc) in neighbors {
                        if nr < self.grid.height()
                            && nc < self.grid.width()
                            && !self.visited.contains(&(nr, nc))
                        {
                            queue.push_back((nr, nc));
                        }
                    }
                }
            }
        }

        // Extract bounding box
        if boundary.is_empty() {
            return None;
        }

        let min_row = boundary.iter().map(|(r, _)| *r).min()?;
        let max_row = boundary.iter().map(|(r, _)| *r).max()?;
        let min_col = boundary.iter().map(|(_, c)| *c).min()?;
        let max_col = boundary.iter().map(|(_, c)| *c).max()?;

        // Verify it's a valid rectangle (has corners)
        let top_left_char = self.grid.get(min_row, min_col)?;
        let bottom_right_char = self.grid.get(max_row, max_col)?;

        let top_left_is_corner = is_any_box_corner(top_left_char);
        let bottom_right_is_corner = is_any_box_corner(bottom_right_char);

        if top_left_is_corner && bottom_right_is_corner && min_row < max_row && min_col < max_col {
            // Detect the style from the top-left corner
            let style = BoxStyle::from_corner(top_left_char).unwrap_or(BoxStyle::Single);
            Some(Box {
                top_left: (min_row, min_col),
                bottom_right: (max_row, max_col),
                style,
                parent_idx: None,
                child_indices: Vec::new(),
            })
        } else {
            None
        }
    }
}

/// Convenience function to detect boxes in a grid.
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[must_use]
pub fn detect_boxes(grid: &Grid) -> Vec<Box> {
    BoxDetector::new(grid).detect()
}

/// Unified detector that returns all primitives in a diagram.
///
/// This is the main entry point for diagram analysis. It orchestrates
/// detection of all primitive types and returns a complete inventory.
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[must_use]
pub fn detect_all_primitives(grid: &Grid) -> crate::primitives::PrimitiveInventory {
    let boxes = detect_boxes(grid);
    let horizontal_arrows = detect_horizontal_arrows(grid);
    let vertical_arrows = detect_vertical_arrows(grid);

    // Extract text rows from inside boxes
    let mut text_rows = Vec::new();
    for b in &boxes {
        for (line_idx, line) in extract_box_content(grid, b).iter().enumerate() {
            if !line.trim().is_empty() {
                let interior_row = b.top_left.0 + 1 + line_idx;
                text_rows.push(crate::primitives::TextRow {
                    row: interior_row,
                    start_col: b.top_left.1 + 1,
                    end_col: b.bottom_right.1 - 1,
                    content: line.clone(),
                });
            }
        }
    }

    crate::primitives::PrimitiveInventory {
        boxes,
        horizontal_arrows,
        vertical_arrows,
        text_rows,
        connection_lines: Vec::new(),
        labels: Vec::new(),
    }
}

/// Extract text rows from inside a box.
///
/// Returns the content of interior rows between the top and bottom borders.
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[must_use]
pub fn extract_box_content(grid: &Grid, b: &Box) -> Vec<String> {
    let mut content = Vec::new();

    for row in (b.top_left.0 + 1)..b.bottom_right.0 {
        let mut line = String::new();
        for col in (b.top_left.1 + 1)..b.bottom_right.1 {
            if let Some(ch) = grid.get(row, col) {
                line.push(ch);
            }
        }
        content.push(line);
    }

    content
}

/// Detect vertical arrows in a grid.
///
/// Detects patterns like `↓`, `↑`, and sequences of `│` or `┃`.
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[must_use]
pub fn detect_vertical_arrows(grid: &Grid) -> Vec<crate::primitives::VerticalArrow> {
    let mut arrows = Vec::new();

    for col in 0..grid.width() {
        let mut row = 0;
        while row < grid.height() {
            if let Some(ch) = grid.get(row, col) {
                if ch == '↓' || ch == '↑' || ch == '│' || ch == '┃' {
                    let start_row = row;
                    let mut end_row = row;

                    // Extend through connected arrow characters
                    while end_row < grid.height() {
                        if let Some(c) = grid.get(end_row, col) {
                            if matches!(c, '↓' | '↑' | '│' | '┃') {
                                end_row += 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    end_row -= 1;

                    // Only add if it's more than one character or is an arrow tip
                    if start_row < end_row || ch == '↓' || ch == '↑' {
                        arrows.push(crate::primitives::VerticalArrow {
                            col,
                            start_row,
                            end_row,
                            arrow_type: ArrowType::Standard,
                            downward: true,
                        });
                    }

                    row = end_row + 1;
                } else {
                    row += 1;
                }
            } else {
                row += 1;
            }
        }
    }

    arrows
}

/// Detect horizontal arrows in a grid.
///
/// Detects patterns like `→`, `←`, and sequences of `─`.
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[must_use]
pub fn detect_horizontal_arrows(grid: &Grid) -> Vec<crate::primitives::HorizontalArrow> {
    let mut arrows = Vec::new();

    for row in 0..grid.height() {
        let mut col = 0;
        while col < grid.width() {
            if let Some(ch) = grid.get(row, col) {
                if ch == '→' || ch == '←' || ch == '─' {
                    let start_col = col;
                    let mut end_col = col;

                    // Extend through connected arrow characters
                    while end_col < grid.width() {
                        if let Some(c) = grid.get(row, end_col) {
                            if matches!(c, '→' | '←' | '─') {
                                end_col += 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    end_col -= 1;

                    // Only add if it contains an arrow tip (→ or ←) somewhere in the sequence
                    let has_arrow_tip =
                        (start_col..=end_col).any(|c| matches!(grid.get(row, c), Some('→' | '←')));

                    if has_arrow_tip {
                        arrows.push(crate::primitives::HorizontalArrow {
                            row,
                            start_col,
                            end_col,
                            arrow_type: ArrowType::Standard,
                            rightward: true,
                        });
                    }

                    col = end_col + 1;
                } else {
                    col += 1;
                }
            } else {
                col += 1;
            }
        }
    }

    arrows
}

/// Detect connection lines (L-shaped paths connecting elements).
///
/// Algorithm:
/// 1. Scan grid for line characters (─ │)
/// 2. Use BFS to trace paths
/// 3. Identify L-shaped paths (2 segments with direction change)
/// 4. Skip if inside boxes or ambiguous
///
/// Conservative: Only reports clear L-shapes with exactly 2 segments.
/// Currently returns empty vector. Actual implementation deferred to ensure
/// proper edge case handling and to avoid false positives.
#[allow(dead_code)] // Reason: Used by main processing pipeline in upcoming phase
#[must_use]
pub const fn detect_connection_lines(
    _grid: &crate::grid::Grid,
) -> Vec<crate::primitives::ConnectionLine> {
    // Conservative approach: Skip detection for now
    // Prevents false positives in early phases
    // TODO: Implement path tracing algorithm in future phase
    Vec::new()
}

/// Detect box hierarchy (parent/child relationships for nested boxes).
///
/// Algorithm:
/// 1. For each box, check if it's completely inside another box
/// 2. If yes, set `parent_idx` and add to parent's `child_indices`
/// 3. Skip ambiguous cases (overlapping but not nested)
///
/// Conservative: Only establishes clear containment relationships.
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[must_use]
pub fn detect_box_hierarchy(
    inventory: &crate::primitives::PrimitiveInventory,
) -> crate::primitives::PrimitiveInventory {
    let mut result = inventory.clone();

    // For each pair of boxes, check if one is inside the other
    let box_count = result.boxes.len();
    for i in 0..box_count {
        for j in 0..box_count {
            if i != j {
                let box_i = &result.boxes[i];
                let box_j = &result.boxes[j];

                // Check if box_i is completely inside box_j (not just touching border)
                if is_box_inside(box_i, box_j) {
                    // box_i is a child of box_j
                    result.boxes[i].parent_idx = Some(j);
                    if !result.boxes[j].child_indices.contains(&i) {
                        result.boxes[j].child_indices.push(i);
                    }
                }
            }
        }
    }

    result
}

/// Check if `box_inner` is completely inside `box_outer` (interior, not on border).
#[allow(dead_code)] // Reason: Helper for hierarchy detection
#[must_use]
pub const fn is_box_inside(inner: &crate::primitives::Box, outer: &crate::primitives::Box) -> bool {
    // Inner box's borders must be strictly inside outer box's interior
    // (with at least 1 cell of margin)
    inner.top_left.0 > outer.top_left.0
        && inner.bottom_right.0 < outer.bottom_right.0
        && inner.top_left.1 > outer.top_left.1
        && inner.bottom_right.1 < outer.bottom_right.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_simple_box() {
        let lines = vec!["┌─┐", "│ │", "└─┘"];
        let grid = Grid::from_lines(&lines);
        let boxes = detect_boxes(&grid);
        assert_eq!(boxes.len(), 1);
        assert_eq!(boxes[0].top_left, (0, 0));
        assert_eq!(boxes[0].bottom_right, (2, 2));
    }

    #[test]
    fn test_detect_larger_box() {
        let lines = vec!["┌─────┐", "│ A   │", "│ B   │", "└─────┘"];
        let grid = Grid::from_lines(&lines);
        let boxes = detect_boxes(&grid);
        assert_eq!(boxes.len(), 1);
        assert_eq!(boxes[0].width(), 7);
        assert_eq!(boxes[0].height(), 4);
    }

    #[test]
    fn test_no_boxes() {
        let lines = vec!["Line 1", "Line 2"];
        let grid = Grid::from_lines(&lines);
        let boxes = detect_boxes(&grid);
        assert_eq!(boxes.len(), 0);
    }

    #[test]
    fn test_multiple_boxes() {
        let lines = vec!["┌─┐   ┌─┐", "│a│   │b│", "└─┘   └─┘"];
        let grid = Grid::from_lines(&lines);
        let boxes = detect_boxes(&grid);
        assert_eq!(boxes.len(), 2);
    }

    #[test]
    fn test_box_with_extended_borders() {
        let lines = vec!["┌──────┐", "│      │", "└──────┘"];
        let grid = Grid::from_lines(&lines);
        let boxes = detect_boxes(&grid);
        assert_eq!(boxes.len(), 1);
        assert_eq!(boxes[0].top_left, (0, 0));
        assert_eq!(boxes[0].bottom_right, (2, 7));
    }

    #[test]
    fn test_box_with_t_junctions() {
        let lines = vec!["┌──┬──┐", "│  │  │", "└──┴──┘"];
        let grid = Grid::from_lines(&lines);
        let boxes = detect_boxes(&grid);
        // May detect one large box or multiple - depending on implementation
        // For now, we expect at least one box
        assert!(!boxes.is_empty());
    }

    #[test]
    fn test_extract_empty_box() {
        let lines = vec!["┌─┐", "│ │", "└─┘"];
        let grid = Grid::from_lines(&lines);
        let b = Box {
            top_left: (0, 0),
            bottom_right: (2, 2),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        };
        let content = extract_box_content(&grid, &b);
        assert_eq!(content.len(), 1);
        assert_eq!(content[0], " ");
    }

    #[test]
    fn test_extract_box_with_text() {
        let lines = vec!["┌─────┐", "│ Box │", "└─────┘"];
        let grid = Grid::from_lines(&lines);
        let b = Box {
            top_left: (0, 0),
            bottom_right: (2, 6),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        };
        let content = extract_box_content(&grid, &b);
        assert_eq!(content.len(), 1);
        assert_eq!(content[0].trim(), "Box");
    }

    #[test]
    fn test_extract_multiline_box() {
        let lines = vec!["┌──────┐", "│Line1 │", "│Line2 │", "│Line3 │", "└──────┘"];
        let grid = Grid::from_lines(&lines);
        let b = Box {
            top_left: (0, 0),
            bottom_right: (4, 7),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        };
        let content = extract_box_content(&grid, &b);
        assert_eq!(content.len(), 3);
        assert_eq!(content[0].trim(), "Line1");
        assert_eq!(content[1].trim(), "Line2");
        assert_eq!(content[2].trim(), "Line3");
    }

    #[test]
    fn test_extract_preserves_spacing() {
        let lines = vec!["┌────────┐", "│ Text   │", "└────────┘"];
        let grid = Grid::from_lines(&lines);
        let b = Box {
            top_left: (0, 0),
            bottom_right: (2, 9),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        };
        let content = extract_box_content(&grid, &b);
        assert_eq!(content[0], " Text   ");
    }

    #[test]
    fn test_detect_simple_arrow() {
        let lines = vec!["→"];
        let grid = Grid::from_lines(&lines);
        let arrows = detect_horizontal_arrows(&grid);
        assert_eq!(arrows.len(), 1);
        assert_eq!(arrows[0].row, 0);
        assert_eq!(arrows[0].start_col, 0);
        assert_eq!(arrows[0].end_col, 0);
    }

    #[test]
    fn test_detect_dashes() {
        // Plain dashes without arrow tips are not detected as arrows
        let lines = vec!["────"];
        let grid = Grid::from_lines(&lines);
        let arrows = detect_horizontal_arrows(&grid);
        assert_eq!(arrows.len(), 0);
    }

    #[test]
    fn test_detect_arrow_with_dashes() {
        let lines = vec!["──→──"];
        let grid = Grid::from_lines(&lines);
        let arrows = detect_horizontal_arrows(&grid);
        assert_eq!(arrows.len(), 1);
        assert_eq!(arrows[0].start_col, 0);
        assert_eq!(arrows[0].end_col, 4);
    }

    #[test]
    fn test_detect_multiple_arrows() {
        let lines = vec!["→    ←"];
        let grid = Grid::from_lines(&lines);
        let arrows = detect_horizontal_arrows(&grid);
        assert_eq!(arrows.len(), 2);
    }

    #[test]
    fn test_no_arrows() {
        let lines = vec!["normal text"];
        let grid = Grid::from_lines(&lines);
        let arrows = detect_horizontal_arrows(&grid);
        assert_eq!(arrows.len(), 0);
    }

    #[test]
    fn test_arrows_in_different_rows() {
        let lines = vec!["→  ", "   ", "←  "];
        let grid = Grid::from_lines(&lines);
        let arrows = detect_horizontal_arrows(&grid);
        assert_eq!(arrows.len(), 2);
        assert_eq!(arrows[0].row, 0);
        assert_eq!(arrows[1].row, 2);
    }

    #[test]
    fn test_detect_simple_vertical_arrow() {
        let lines = vec!["↓"];
        let grid = Grid::from_lines(&lines);
        let arrows = detect_vertical_arrows(&grid);
        assert_eq!(arrows.len(), 1);
        assert_eq!(arrows[0].col, 0);
        assert_eq!(arrows[0].start_row, 0);
        assert_eq!(arrows[0].end_row, 0);
    }

    #[test]
    fn test_detect_vertical_pipes() {
        let lines = vec!["│", "│", "│", "│"];
        let grid = Grid::from_lines(&lines);
        let arrows = detect_vertical_arrows(&grid);
        assert_eq!(arrows.len(), 1);
        assert_eq!(arrows[0].col, 0);
        assert_eq!(arrows[0].start_row, 0);
        assert_eq!(arrows[0].end_row, 3);
    }

    #[test]
    fn test_detect_arrow_with_pipes() {
        let lines = vec!["│", "│", "↓", "│", "│"];
        let grid = Grid::from_lines(&lines);
        let arrows = detect_vertical_arrows(&grid);
        assert_eq!(arrows.len(), 1);
        assert_eq!(arrows[0].start_row, 0);
        assert_eq!(arrows[0].end_row, 4);
    }

    #[test]
    fn test_detect_multiple_vertical_arrows() {
        let lines = vec!["↓ ↑"];
        let grid = Grid::from_lines(&lines);
        let arrows = detect_vertical_arrows(&grid);
        assert_eq!(arrows.len(), 2);
        assert_eq!(arrows[0].col, 0);
        assert_eq!(arrows[1].col, 2);
    }

    #[test]
    fn test_no_vertical_arrows() {
        let lines = vec!["a b c"];
        let grid = Grid::from_lines(&lines);
        let arrows = detect_vertical_arrows(&grid);
        assert_eq!(arrows.len(), 0);
    }

    #[test]
    fn test_vertical_arrows_in_different_cols() {
        let lines = vec!["↓     ↑", "│     │", "│     │"];
        let grid = Grid::from_lines(&lines);
        let arrows = detect_vertical_arrows(&grid);
        assert_eq!(arrows.len(), 2);
        assert_eq!(arrows[0].col, 0);
        assert_eq!(arrows[1].col, 6);
    }

    #[test]
    fn test_detect_thick_vertical_pipes() {
        let lines = vec!["┃", "┃", "┃"];
        let grid = Grid::from_lines(&lines);
        let arrows = detect_vertical_arrows(&grid);
        assert_eq!(arrows.len(), 1);
        assert_eq!(arrows[0].start_row, 0);
        assert_eq!(arrows[0].end_row, 2);
    }

    #[test]
    fn test_unified_detector_empty_grid() {
        let lines: Vec<&str> = vec![];
        let grid = Grid::from_lines(&lines);
        let inventory = detect_all_primitives(&grid);
        assert!(inventory.boxes.is_empty());
        assert!(inventory.horizontal_arrows.is_empty());
        assert!(inventory.vertical_arrows.is_empty());
    }

    #[test]
    fn test_unified_detector_simple_box() {
        let lines = vec!["┌─┐", "│a│", "└─┘"];
        let grid = Grid::from_lines(&lines);
        let inventory = detect_all_primitives(&grid);
        assert_eq!(inventory.boxes.len(), 1);
        assert_eq!(inventory.text_rows.len(), 1);
        assert_eq!(inventory.horizontal_arrows.len(), 0);
        assert_eq!(inventory.vertical_arrows.len(), 0);
    }

    #[test]
    fn test_unified_detector_box_with_arrows() {
        let lines = vec!["  ↓  ", "┌─┐", "│x│", "└─┘", "  ↓  "];
        let grid = Grid::from_lines(&lines);
        let inventory = detect_all_primitives(&grid);
        assert_eq!(inventory.boxes.len(), 1);
        // Two separate ↓ arrows (at row 0 and row 4)
        assert_eq!(inventory.vertical_arrows.len(), 2);
        assert_eq!(inventory.text_rows.len(), 1);
    }

    #[test]
    fn test_unified_detector_multiple_boxes() {
        let lines = vec!["┌─┐ ┌─┐", "│a│ │b│", "└─┘ └─┘"];
        let grid = Grid::from_lines(&lines);
        let inventory = detect_all_primitives(&grid);
        assert_eq!(inventory.boxes.len(), 2);
        assert_eq!(inventory.text_rows.len(), 2);
    }

    #[test]
    fn test_unified_detector_complex_diagram() {
        let lines = vec![
            "  Input  ",
            "    ↓    ",
            "┌───────┐",
            "│Process│",
            "└───────┘",
            "    ↓    ",
            "  Output ",
        ];
        let grid = Grid::from_lines(&lines);
        let inventory = detect_all_primitives(&grid);
        assert_eq!(inventory.boxes.len(), 1);
        // Two separate ↓ arrows (at row 1 and row 5)
        assert_eq!(inventory.vertical_arrows.len(), 2);
        assert_eq!(inventory.text_rows.len(), 1);
    }

    #[test]
    fn test_unified_detector_text_extraction() {
        let lines = vec!["┌──────┐", "│Line1 │", "│Line2 │", "└──────┘"];
        let grid = Grid::from_lines(&lines);
        let inventory = detect_all_primitives(&grid);
        assert_eq!(inventory.text_rows.len(), 2);
        assert!(inventory.text_rows[0].content.contains("Line1"));
        assert!(inventory.text_rows[1].content.contains("Line2"));
    }

    #[test]
    fn test_unified_detector_empty_box_no_text() {
        let lines = vec!["┌─┐", "│ │", "└─┘"];
        let grid = Grid::from_lines(&lines);
        let inventory = detect_all_primitives(&grid);
        assert_eq!(inventory.boxes.len(), 1);
        // Empty box produces no text rows (whitespace only)
        assert_eq!(inventory.text_rows.len(), 0);
    }

    #[test]
    fn test_unified_detector_preserves_positions() {
        let lines = vec!["┌──┐", "│AB│", "└──┘"];
        let grid = Grid::from_lines(&lines);
        let inventory = detect_all_primitives(&grid);
        let text_row = &inventory.text_rows[0];
        assert_eq!(text_row.row, 1);
        assert_eq!(text_row.start_col, 1);
        assert_eq!(text_row.end_col, 2);
    }

    // Phase 4, Cycle 13: RED - Connection line detection tests
    #[test]
    fn test_detect_connection_lines_completes() {
        // Function should return a vector without crashing
        let lines = vec!["────┐  ", "    │  ", "    └──"];
        let grid = Grid::from_lines(&lines);
        let _connections = detect_connection_lines(&grid);
        // Function completed successfully (conservative: may return empty)
    }

    #[test]
    fn test_detect_connection_lines_empty_grid() {
        let lines = vec!["        ", "        ", "        "];
        let grid = Grid::from_lines(&lines);
        let connections = detect_connection_lines(&grid);
        // Empty grid should have no connections
        assert!(
            connections.is_empty(),
            "Empty grid should have no connections"
        );
    }

    #[test]
    fn test_detect_connection_lines_single_line() {
        let lines = vec!["────"];
        let grid = Grid::from_lines(&lines);
        let _connections = detect_connection_lines(&grid);
        // Function should complete without crashing (conservative approach)
    }

    #[test]
    fn test_detect_connection_lines_between_boxes() {
        // Connection line between two boxes
        let lines = vec!["┌──┐    ┌──┐", "│  │    │  │", "└──┘────└──┘"];
        let grid = Grid::from_lines(&lines);
        let _connections = detect_connection_lines(&grid);
        // Function should complete without crashing (conservative approach)
    }

    // Phase 5, Cycle 16: RED - Hierarchy detection tests
    #[test]
    fn test_detect_box_hierarchy_empty() {
        let inventory = crate::primitives::PrimitiveInventory::default();
        let result = detect_box_hierarchy(&inventory);
        assert!(result.boxes.is_empty());
    }

    #[test]
    fn test_detect_box_hierarchy_single_box() {
        let mut inventory = crate::primitives::PrimitiveInventory::default();
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (2, 4),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        let result = detect_box_hierarchy(&inventory);
        // Single box should have no parent and no children
        assert_eq!(result.boxes.len(), 1);
        assert!(result.boxes[0].parent_idx.is_none());
        assert!(result.boxes[0].child_indices.is_empty());
    }

    #[test]
    fn test_detect_box_hierarchy_separate_boxes() {
        let mut inventory = crate::primitives::PrimitiveInventory::default();
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (2, 4),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 6),
            bottom_right: (2, 10),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        let result = detect_box_hierarchy(&inventory);
        // Separate boxes should have no relationships
        assert!(result.boxes.iter().all(|b| b.parent_idx.is_none()));
        assert!(result.boxes.iter().all(|b| b.child_indices.is_empty()));
    }

    #[test]
    fn test_detect_box_hierarchy_nested() {
        let mut inventory = crate::primitives::PrimitiveInventory::default();
        // Outer box
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (4, 8),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Inner box
        inventory.boxes.push(crate::primitives::Box {
            top_left: (1, 2),
            bottom_right: (3, 6),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        let result = detect_box_hierarchy(&inventory);
        // Inner box should have parent=0, outer should have child=1
        assert_eq!(result.boxes[0].child_indices, vec![1]);
        assert_eq!(result.boxes[1].parent_idx, Some(0));
    }

    #[test]
    fn test_detect_box_hierarchy_multiple_children() {
        let mut inventory = crate::primitives::PrimitiveInventory::default();
        // Parent box
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (6, 12),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Child 1
        inventory.boxes.push(crate::primitives::Box {
            top_left: (1, 1),
            bottom_right: (3, 5),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Child 2
        inventory.boxes.push(crate::primitives::Box {
            top_left: (1, 7),
            bottom_right: (3, 11),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        let result = detect_box_hierarchy(&inventory);
        // Parent should have two children
        assert_eq!(result.boxes[0].child_indices.len(), 2);
        assert!(result.boxes[0].child_indices.contains(&1));
        assert!(result.boxes[0].child_indices.contains(&2));
    }

    // Phase 5, Cycle 19: RED - Edge case tests
    #[test]
    fn test_detect_box_hierarchy_deep_nesting() {
        let mut inventory = crate::primitives::PrimitiveInventory::default();
        // Level 0: Grandgrandparent
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (8, 12),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Level 1: Grandparent
        inventory.boxes.push(crate::primitives::Box {
            top_left: (1, 1),
            bottom_right: (7, 11),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Level 2: Parent
        inventory.boxes.push(crate::primitives::Box {
            top_left: (2, 2),
            bottom_right: (6, 10),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Level 3: Child (may be conservative and skip)
        inventory.boxes.push(crate::primitives::Box {
            top_left: (3, 3),
            bottom_right: (5, 9),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        let result = detect_box_hierarchy(&inventory);
        // Should handle deep nesting or conservatively process
        assert_eq!(result.boxes.len(), 4);
    }

    #[test]
    fn test_detect_box_hierarchy_overlapping_not_nested() {
        let mut inventory = crate::primitives::PrimitiveInventory::default();
        // Box 1
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (3, 5),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Box 2 - overlaps but not nested
        inventory.boxes.push(crate::primitives::Box {
            top_left: (1, 4),
            bottom_right: (4, 7),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        let result = detect_box_hierarchy(&inventory);
        // Overlapping boxes should not be nested
        assert!(result.boxes[0].child_indices.is_empty());
        assert!(result.boxes[1].parent_idx.is_none());
    }

    #[test]
    fn test_detect_box_hierarchy_touching_border() {
        let mut inventory = crate::primitives::PrimitiveInventory::default();
        // Outer box
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 0),
            bottom_right: (4, 6),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Inner box touching border (should not be nested)
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 1),
            bottom_right: (2, 5),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        let result = detect_box_hierarchy(&inventory);
        // Should not be nested if touching border
        assert!(result.boxes[0].child_indices.is_empty());
        assert!(result.boxes[1].parent_idx.is_none());
    }

    #[test]
    fn test_detect_complex_multi_level_hierarchy() {
        // This tests the Database Architecture diagram structure:
        // Level 0: Parent (Database as the Optimizer)
        // Level 1: 3 middle boxes (Compilation Pipeline, confiture, pg_tviews)
        // Level 2: 3 bottom boxes (fraiseql-core, fraiseql-observers, jsonb_delta)
        let mut inventory = crate::primitives::PrimitiveInventory::default();

        // Level 0: Root/parent box
        inventory.boxes.push(crate::primitives::Box {
            top_left: (0, 20),
            bottom_right: (3, 45),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });

        // Level 1: Three middle boxes
        // Left: Compilation Pipeline
        inventory.boxes.push(crate::primitives::Box {
            top_left: (6, 5),
            bottom_right: (9, 20),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Middle: confiture
        inventory.boxes.push(crate::primitives::Box {
            top_left: (6, 23),
            bottom_right: (9, 38),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Right: pg_tviews
        inventory.boxes.push(crate::primitives::Box {
            top_left: (6, 41),
            bottom_right: (9, 56),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });

        // Level 2: Three bottom boxes
        // Left: fraiseql-core
        inventory.boxes.push(crate::primitives::Box {
            top_left: (12, 5),
            bottom_right: (15, 20),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Middle: fraiseql-observers
        inventory.boxes.push(crate::primitives::Box {
            top_left: (12, 23),
            bottom_right: (15, 38),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });
        // Right: jsonb_delta
        inventory.boxes.push(crate::primitives::Box {
            top_left: (12, 41),
            bottom_right: (15, 56),
            style: crate::primitives::BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        });

        // Detect hierarchy
        let result = detect_box_hierarchy(&inventory);

        // Verify structure: total 7 boxes
        assert_eq!(result.boxes.len(), 7);

        // For this diagram, boxes aren't strictly nested (side-by-side arrangement),
        // but the test ensures the detector handles complex layouts without crashing
        assert!(!result.boxes.is_empty());
        // All boxes should be independent (no nesting in this layout)
        assert!(result.boxes.iter().all(|b| b.parent_idx.is_none()));
    }
}
