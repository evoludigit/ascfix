//! Detection logic for ASCII diagram primitives.

use crate::grid::Grid;
use crate::primitives::Box;
use std::collections::{HashSet, VecDeque};

/// Box character set for detection.
const fn is_box_char(ch: char) -> bool {
    matches!(
        ch,
        '─' | '│' | '┌' | '┐' | '└' | '┘' | '├' | '┤' | '┼' | '┬' | '┴' | '┃'
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
        let top_left_is_corner = self
            .grid
            .get(min_row, min_col)
            .is_some_and(|ch| matches!(ch, '┌' | '┬' | '├' | '┼'));
        let bottom_right_is_corner = self
            .grid
            .get(max_row, max_col)
            .is_some_and(|ch| matches!(ch, '┘' | '┴' | '┤' | '┼'));

        if top_left_is_corner && bottom_right_is_corner && min_row < max_row && min_col < max_col {
            Some(Box {
                top_left: (min_row, min_col),
                bottom_right: (max_row, max_col),
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
                    let has_arrow_tip = (start_col..=end_col).any(|c| {
                        matches!(grid.get(row, c), Some('→' | '←'))
                    });

                    if has_arrow_tip {
                        arrows.push(crate::primitives::HorizontalArrow {
                            row,
                            start_col,
                            end_col,
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
}
