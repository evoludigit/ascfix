//! Arrow detection functionality.

use crate::grid::Grid;
use crate::primitives::ArrowType;

/// Check if a character is a vertical arrow (Unicode or box-drawing)
#[inline]
#[allow(dead_code)] // Reason: Used in arrow detection logic
fn is_vertical_arrow(ch: char) -> bool {
    matches!(ch,
        // Standard arrows
        '↓' | '↑' |
        // Double arrows
        '⇓' | '⇑' |
        // Box drawing
        '│' | '┃'
    )
}

/// Check if a character is a horizontal arrow (Unicode or box-drawing)
#[inline]
#[allow(dead_code)] // Reason: Used in arrow detection logic
fn is_horizontal_arrow(ch: char) -> bool {
    matches!(ch,
        // Standard arrows
        '→' | '←' |
        // Double arrows
        '⇒' | '⇐' |
        // Extended arrows
        '⟶' | '⟹' |
        // Box drawing
        '─'
    )
}

/// Check if a character is a downward arrow
#[inline]
#[allow(dead_code)] // Reason: Used in arrow detection logic
fn is_downward_arrow(ch: char) -> bool {
    matches!(ch, '↓' | '⇓')
}

/// Check if a character is an upward arrow
#[inline]
#[allow(dead_code)] // Reason: Used in arrow detection logic
fn is_upward_arrow(ch: char) -> bool {
    matches!(ch, '↑' | '⇑')
}

/// Check if a character is a rightward arrow
#[inline]
#[allow(dead_code)] // Reason: Used in arrow detection logic
fn is_rightward_arrow(ch: char) -> bool {
    matches!(ch, '→' | '⇒' | '⟶')
}

/// Check if a character is a leftward arrow
#[inline]
#[allow(dead_code)] // Reason: Used in arrow detection logic
fn is_leftward_arrow(ch: char) -> bool {
    matches!(ch, '←' | '⇐' | '⟹')
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
                if is_vertical_arrow(ch) {
                    let start_row = row;
                    let mut end_row = row;

                    // Extend through connected arrow characters
                    while end_row < grid.height() {
                        if let Some(c) = grid.get(end_row, col) {
                            if is_vertical_arrow(c) {
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
                    if start_row < end_row || is_downward_arrow(ch) || is_upward_arrow(ch) {
                        // Determine direction based on the arrow character at start_row
                        let is_downward = grid
                            .get(start_row, col)
                            .is_none_or(|start_char| !is_upward_arrow(start_char));

                        // Capture the original arrow character
                        let arrow_char = grid.get(start_row, col)
                            .and_then(|c| {
                                if is_downward_arrow(c) || is_upward_arrow(c) {
                                    Some(c)
                                } else {
                                    None
                                }
                            });

                        arrows.push(crate::primitives::VerticalArrow {
                            col,
                            start_row,
                            end_row,
                            arrow_type: ArrowType::Standard,
                            downward: is_downward,
                            arrow_char,
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
                if is_horizontal_arrow(ch) {
                    let start_col = col;
                    let mut end_col = col;

                    // Extend through connected arrow characters
                    while end_col < grid.width() {
                        if let Some(c) = grid.get(row, end_col) {
                            if is_horizontal_arrow(c) || c == '─' {
                                end_col += 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    end_col -= 1;

                    // Only add if it contains an arrow tip somewhere in the sequence
                    let has_arrow_tip = (start_col..=end_col)
                        .any(|c| grid.get(row, c).map_or(false, |ch| is_rightward_arrow(ch) || is_leftward_arrow(ch)));

                    if has_arrow_tip {
                        let arrow_char = (start_col..=end_col)
                            .find_map(|c| {
                                grid.get(row, c).and_then(|ch| {
                                    if ArrowType::from_char(ch).is_some() {
                                        Some(ch)
                                    } else {
                                        None
                                    }
                                })
                            });
                        arrows.push(crate::primitives::HorizontalArrow {
                            row,
                            start_col,
                            end_col,
                            arrow_type: ArrowType::Standard,
                            rightward: true,
                            arrow_char,
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
