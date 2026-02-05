//! Primitive types representing ASCII diagram elements.

/// A rectangular box defined by its border.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Box {
    /// Top-left corner (row, col)
    pub top_left: (usize, usize),
    /// Bottom-right corner (row, col)
    pub bottom_right: (usize, usize),
}

impl Box {
    /// Get the width of the box (number of columns).
    #[allow(dead_code)]  // Reason: Used by normalization pipeline
    pub const fn width(&self) -> usize {
        self.bottom_right.1 - self.top_left.1 + 1
    }

    /// Get the height of the box (number of rows).
    #[allow(dead_code)]  // Reason: Used by normalization pipeline
    pub const fn height(&self) -> usize {
        self.bottom_right.0 - self.top_left.0 + 1
    }

    /// Check if a position is inside the box interior (not on border).
    #[allow(dead_code)]  // Reason: Used by normalization pipeline
    pub const fn contains_interior(&self, row: usize, col: usize) -> bool {
        row > self.top_left.0
            && row < self.bottom_right.0
            && col > self.top_left.1
            && col < self.bottom_right.1
    }

    /// Check if a position is on the box border.
    #[allow(dead_code)]  // Reason: Used by normalization pipeline
    pub const fn contains_border(&self, row: usize, col: usize) -> bool {
        (row == self.top_left.0 || row == self.bottom_right.0)
            && col >= self.top_left.1
            && col <= self.bottom_right.1
            || (col == self.top_left.1 || col == self.bottom_right.1)
                && row >= self.top_left.0
                && row <= self.bottom_right.0
    }
}

/// A horizontal arrow or connector.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]  // Reason: Used by main processing pipeline
pub struct HorizontalArrow {
    /// Row position
    pub row: usize,
    /// Starting column
    pub start_col: usize,
    /// Ending column
    pub end_col: usize,
}

/// A vertical arrow or connector.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]  // Reason: Used by main processing pipeline
pub struct VerticalArrow {
    /// Column position
    pub col: usize,
    /// Starting row
    pub start_row: usize,
    /// Ending row
    pub end_row: usize,
}

/// A row of text content.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]  // Reason: Used by main processing pipeline
pub struct TextRow {
    /// Row position
    pub row: usize,
    /// Starting column
    pub start_col: usize,
    /// Ending column (inclusive)
    pub end_col: usize,
    /// Text content
    pub content: String,
}

/// Check if a character is a box character.
#[allow(dead_code)]  // Reason: May be used by other detectors
const fn is_box_char(ch: char) -> bool {
    matches!(
        ch,
        '─' | '│' | '┌' | '┐' | '└' | '┘' | '├' | '┤' | '┼' | '┬' | '┴' | '┃'
    )
}

/// Check if a character is a box corner.
#[allow(dead_code)]  // Reason: May be used by other detectors
const fn is_box_corner(ch: char) -> bool {
    matches!(ch, '┌' | '┐' | '└' | '┘')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_dimensions() {
        let b = Box {
            top_left: (0, 0),
            bottom_right: (3, 5),
        };
        assert_eq!(b.width(), 6);
        assert_eq!(b.height(), 4);
    }

    #[test]
    fn test_box_contains_interior() {
        let b = Box {
            top_left: (0, 0),
            bottom_right: (3, 5),
        };
        assert!(b.contains_interior(1, 1));
        assert!(b.contains_interior(2, 3));
        assert!(!b.contains_interior(0, 1)); // Top edge
        assert!(!b.contains_interior(3, 1)); // Bottom edge
        assert!(!b.contains_interior(1, 0)); // Left edge
    }

    #[test]
    fn test_box_contains_border() {
        let b = Box {
            top_left: (0, 0),
            bottom_right: (3, 5),
        };
        assert!(b.contains_border(0, 0)); // Top-left
        assert!(b.contains_border(0, 3)); // Top
        assert!(b.contains_border(3, 5)); // Bottom-right
        assert!(!b.contains_border(1, 1)); // Interior
    }

    #[test]
    fn test_horizontal_arrow() {
        let arr = HorizontalArrow {
            row: 5,
            start_col: 2,
            end_col: 8,
        };
        assert_eq!(arr.row, 5);
        assert_eq!(arr.start_col, 2);
    }

    #[test]
    fn test_vertical_arrow() {
        let arr = VerticalArrow {
            col: 3,
            start_row: 1,
            end_row: 6,
        };
        assert_eq!(arr.col, 3);
        assert_eq!(arr.start_row, 1);
    }

    #[test]
    fn test_text_row() {
        let tr = TextRow {
            row: 2,
            start_col: 5,
            end_col: 10,
            content: "Hello".to_string(),
        };
        assert_eq!(tr.content, "Hello");
    }

    #[test]
    fn test_is_box_char() {
        assert!(is_box_char('─'));
        assert!(is_box_char('│'));
        assert!(is_box_char('┌'));
        assert!(!is_box_char('a'));
        assert!(!is_box_char(' '));
    }

    #[test]
    fn test_is_box_corner() {
        assert!(is_box_corner('┌'));
        assert!(is_box_corner('┐'));
        assert!(is_box_corner('└'));
        assert!(is_box_corner('┘'));
        assert!(!is_box_corner('─'));
        assert!(!is_box_corner('│'));
    }
}
