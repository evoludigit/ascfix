//! Arrow detection functionality.

use crate::grid::Grid;
use crate::primitives::ArrowType;

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
                        // Determine direction based on the arrow character at start_row
                        let is_downward = grid
                            .get(start_row, col)
                            .is_none_or(|start_char| start_char == '↓');

                        arrows.push(crate::primitives::VerticalArrow {
                            col,
                            start_row,
                            end_row,
                            arrow_type: ArrowType::Standard,
                            downward: is_downward,
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
