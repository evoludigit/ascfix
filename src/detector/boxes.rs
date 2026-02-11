//! Box detection functionality.

use crate::grid::Grid;
use crate::primitives::{Box, BoxStyle};
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
