//! Box detection functionality.
//!
//! Uses a corner-tracing algorithm: for each top-left corner found in the grid,
//! trace the four walls of the rectangle to detect the box independently.
//! This correctly handles boxes connected by junction characters (┬, ┴, etc.).

use crate::grid::Grid;
use crate::primitives::{Box, BoxStyle};

/// Check if a character is a top-left corner (any style).
const fn is_top_left_corner(ch: char) -> bool {
    matches!(ch, '┌' | '╔' | '╭')
}

/// Check if a character is a top-right corner (any style).
const fn is_top_right_corner(ch: char) -> bool {
    matches!(ch, '┐' | '╗' | '╮')
}

/// Check if a character is a bottom-left corner (any style).
const fn is_bottom_left_corner(ch: char) -> bool {
    matches!(ch, '└' | '╚' | '╰')
}

/// Check if a character is a bottom-right corner (any style).
const fn is_bottom_right_corner(ch: char) -> bool {
    matches!(ch, '┘' | '╝' | '╯')
}

/// Check if a character is valid on a horizontal border (top or bottom wall).
/// Includes horizontal lines, junction characters, and arrow symbols that
/// commonly sit on box borders as connection indicators.
const fn is_horizontal_border(ch: char) -> bool {
    matches!(
        ch,
        '─' | '┬'
            | '┴'
            | '┼'
            | '├'
            | '┤'
            | '═'
            | '╦'
            | '╩'
            | '╬'
            | '▼'
            | '▲'
            | '↓'
            | '↑'
            | '⇓'
            | '⇑'
    )
}

/// Check if a character is valid on a vertical border (left or right wall).
/// Includes vertical lines, junction characters, and arrow symbols that
/// commonly sit on box borders as connection indicators.
const fn is_vertical_border(ch: char) -> bool {
    matches!(
        ch,
        '│' | '├'
            | '┤'
            | '┼'
            | '┬'
            | '┴'
            | '║'
            | '╠'
            | '╣'
            | '╬'
            | '►'
            | '◄'
            | '→'
            | '←'
            | '⇒'
            | '⇐'
    )
}

/// Detects rectangular boxes in ASCII diagrams using corner-tracing.
pub struct BoxDetector<'a> {
    grid: &'a Grid,
}

impl<'a> BoxDetector<'a> {
    /// Create a new box detector for a grid.
    #[must_use]
    pub const fn new(grid: &'a Grid) -> Self {
        BoxDetector { grid }
    }

    /// Detect all rectangular boxes in the grid.
    ///
    /// Algorithm:
    /// 1. Scan for top-left corners (┌, ╔, ╭)
    /// 2. For each, trace the four walls to find the complete rectangle
    /// 3. Emit a box if all four walls check out
    #[must_use]
    pub fn detect(self) -> Vec<Box> {
        let mut boxes = Vec::new();

        for row in 0..self.grid.height() {
            for col in 0..self.grid.width() {
                if let Some(ch) = self.grid.get(row, col) {
                    if is_top_left_corner(ch) {
                        if let Some(b) = self.trace_box(row, col, ch) {
                            boxes.push(b);
                        }
                    }
                }
            }
        }

        boxes
    }

    /// Trace a rectangle starting from a top-left corner at (row, col).
    ///
    /// Uses a two-anchor approach: traces from BOTH the top-left and bottom-left
    /// corners to find the rectangle dimensions. This handles common misalignment
    /// where the top/bottom borders differ by ±1 in width.
    ///
    /// 1. Trace left border down from top-left to find bottom-left corner
    /// 2. Trace right along BOTH top and bottom borders to find top-right and bottom-right
    /// 3. Accept the box using the max of the two right-column values
    /// 4. Verify the opposite walls at the resolved positions
    fn trace_box(&self, top_row: usize, left_col: usize, tl_char: char) -> Option<Box> {
        let style = BoxStyle::from_corner(tl_char)?;

        // Step 1: Trace left border down from top-left to find bottom-left corner
        let bottom_row = self.trace_vertical(left_col, top_row + 1, is_bottom_left_corner)?;

        // Step 2: Trace right along top border to find top-right corner
        let top_right_col = self.trace_horizontal(top_row, left_col + 1, is_top_right_corner)?;

        // Step 3: Trace right along bottom border to find bottom-right corner
        let bottom_right_col =
            self.trace_horizontal(bottom_row, left_col + 1, is_bottom_right_corner)?;

        // Step 4: Use the max of top and bottom right columns as the box width.
        // This handles off-by-one misalignment (common in AI-generated diagrams).
        let right_col = top_right_col.max(bottom_right_col);

        // Step 5: Verify right border connects top-right to bottom-right.
        // Also discovers the actual max right column (some rows may extend further).
        let actual_right_col =
            self.find_right_border_extent(right_col, top_right_col, top_row + 1, bottom_row)?;

        Some(Box {
            top_left: (top_row, left_col),
            bottom_right: (bottom_row, actual_right_col.max(right_col)),
            style,
            parent_idx: None,
            child_indices: Vec::new(),
        })
    }

    /// Trace horizontally from (row, `start_col`) until finding a character
    /// that satisfies the `is_target_corner` predicate.
    /// Returns the column of the target corner, or None if not found.
    fn trace_horizontal(
        &self,
        row: usize,
        start_col: usize,
        is_target_corner: fn(char) -> bool,
    ) -> Option<usize> {
        let mut col = start_col;
        loop {
            let ch = self.grid.get(row, col)?;
            if is_target_corner(ch) {
                return Some(col);
            }
            if !is_horizontal_border(ch) {
                return None;
            }
            col += 1;
        }
    }

    /// Trace vertically from (`start_row`, col) until finding a character
    /// that satisfies the `is_target_corner` predicate.
    /// Returns the row of the target corner, or None if not found.
    fn trace_vertical(
        &self,
        col: usize,
        start_row: usize,
        is_target_corner: fn(char) -> bool,
    ) -> Option<usize> {
        let mut row = start_row;
        loop {
            let ch = self.grid.get(row, col)?;
            if is_target_corner(ch) {
                return Some(row);
            }
            if !is_vertical_border(ch) {
                return None;
            }
            row += 1;
        }
    }

    /// Verify a vertical right border exists at each row (allowing ±1 column),
    /// and return the maximum column where a border was found.
    /// This ensures the box encompasses all content, even when some rows are wider.
    /// Returns None if any row has no border character nearby.
    fn find_right_border_extent(
        &self,
        primary_col: usize,
        alt_col: usize,
        start_row: usize,
        end_row: usize,
    ) -> Option<usize> {
        // Columns to check: primary, alt, and ±1 from primary
        let candidates: [Option<usize>; 4] = [
            Some(primary_col),
            if alt_col == primary_col {
                None
            } else {
                Some(alt_col)
            },
            primary_col.checked_sub(1),
            Some(primary_col + 1),
        ];

        let mut max_col = primary_col;

        for row in start_row..end_row {
            let mut found = false;
            for col_opt in &candidates {
                if let Some(col) = *col_opt {
                    if matches!(self.grid.get(row, col), Some(ch) if is_vertical_border(ch)) {
                        max_col = max_col.max(col);
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                return None;
            }
        }
        Some(max_col)
    }
}

/// Convenience function to detect boxes in a grid.
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[must_use]
pub fn detect_boxes(grid: &Grid) -> Vec<Box> {
    BoxDetector::new(grid).detect()
}
