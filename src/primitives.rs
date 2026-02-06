//! Primitive types representing ASCII diagram elements.

/// Box drawing style for different box types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Reason: Used by detector and renderer in upcoming phases
pub enum BoxStyle {
    /// Single-line boxes: ─ │ ┌ ┐ └ ┘
    Single,
    /// Double-line boxes: ═ ║ ╔ ╗ ╚ ╝
    Double,
    /// Rounded-corner boxes: ─ │ ╭ ╮ ╰ ╯
    Rounded,
}

/// Box drawing characters for a specific style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Reason: Used by detector and renderer in upcoming phases
pub struct BoxChars {
    /// Horizontal line character
    pub horizontal: char,
    /// Vertical line character
    pub vertical: char,
    /// Top-left corner
    pub top_left: char,
    /// Top-right corner
    pub top_right: char,
    /// Bottom-left corner
    pub bottom_left: char,
    /// Bottom-right corner
    pub bottom_right: char,
}

impl BoxStyle {
    /// Get the characters used for this box style.
    #[must_use]
    #[allow(dead_code)] // Reason: Used by detector and renderer in upcoming phases
    pub const fn chars(self) -> BoxChars {
        match self {
            Self::Single => BoxChars {
                horizontal: '─',
                vertical: '│',
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
            },
            Self::Double => BoxChars {
                horizontal: '═',
                vertical: '║',
                top_left: '╔',
                top_right: '╗',
                bottom_left: '╚',
                bottom_right: '╝',
            },
            Self::Rounded => BoxChars {
                horizontal: '─',
                vertical: '│',
                top_left: '╭',
                top_right: '╮',
                bottom_left: '╰',
                bottom_right: '╯',
            },
        }
    }
}

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
    #[allow(dead_code)] // Reason: Used by normalization pipeline
    #[must_use]
    pub const fn width(&self) -> usize {
        self.bottom_right.1 - self.top_left.1 + 1
    }

    /// Get the height of the box (number of rows).
    #[allow(dead_code)] // Reason: Used by normalization pipeline
    #[must_use]
    pub const fn height(&self) -> usize {
        self.bottom_right.0 - self.top_left.0 + 1
    }

    /// Check if a position is inside the box interior (not on border).
    #[allow(dead_code)] // Reason: Used by normalization pipeline
    #[must_use]
    pub const fn contains_interior(&self, row: usize, col: usize) -> bool {
        row > self.top_left.0
            && row < self.bottom_right.0
            && col > self.top_left.1
            && col < self.bottom_right.1
    }

    /// Check if a position is on the box border.
    #[allow(dead_code)] // Reason: Used by normalization pipeline
    #[must_use]
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
#[allow(dead_code)] // Reason: Used by main processing pipeline
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
#[allow(dead_code)] // Reason: Used by main processing pipeline
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
#[allow(dead_code)] // Reason: Used by main processing pipeline
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

/// Single-line box characters for character set constant.
#[allow(dead_code)] // Reason: Will be used by detector in upcoming phases
const SINGLE_LINE_HORIZ: char = '─';
/// Single-line box characters for character set constant.
#[allow(dead_code)] // Reason: Will be used by detector in upcoming phases
const SINGLE_LINE_VERT: char = '│';
/// Single-line box characters for character set constant.
#[allow(dead_code)] // Reason: Will be used by detector in upcoming phases
const SINGLE_LINE_CORNERS: &[char] = &['┌', '┐', '└', '┘'];

/// Double-line box characters for character set constant.
#[allow(dead_code)] // Reason: Will be used by detector in upcoming phases
const DOUBLE_LINE_HORIZ: char = '═';
/// Double-line box characters for character set constant.
#[allow(dead_code)] // Reason: Will be used by detector in upcoming phases
const DOUBLE_LINE_VERT: char = '║';
/// Double-line box characters for character set constant.
#[allow(dead_code)] // Reason: Will be used by detector in upcoming phases
const DOUBLE_LINE_CORNERS: &[char] = &['╔', '╗', '╚', '╝'];

/// Rounded box characters for character set constant.
#[allow(dead_code)] // Reason: Will be used by detector in upcoming phases
const ROUNDED_HORIZ: char = '─';
/// Rounded box characters for character set constant.
#[allow(dead_code)] // Reason: Will be used by detector in upcoming phases
const ROUNDED_VERT: char = '│';
/// Rounded box characters for character set constant.
#[allow(dead_code)] // Reason: Will be used by detector in upcoming phases
const ROUNDED_CORNERS: &[char] = &['╭', '╮', '╰', '╯'];

/// Check if a character is a box character.
#[allow(dead_code)] // Reason: Used by detector in upcoming phases
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

/// Check if a character is a box corner (any style).
#[allow(dead_code)] // Reason: Used by detector in upcoming phases
const fn is_box_corner(ch: char) -> bool {
    matches!(
        ch,
        '┌' | '┐' | '└' | '┘' | '╔' | '╗' | '╚' | '╝' | '╭' | '╮' | '╰' | '╯'
    )
}

/// Check if a character is a double-line box corner.
#[allow(dead_code)] // Reason: Used by detector in upcoming phases
const fn is_double_line_corner(ch: char) -> bool {
    matches!(ch, '╔' | '╗' | '╚' | '╝')
}

/// Check if a character is a rounded box corner.
#[allow(dead_code)] // Reason: Used by detector in upcoming phases
const fn is_rounded_corner(ch: char) -> bool {
    matches!(ch, '╭' | '╮' | '╰' | '╯')
}

/// Complete inventory of detected primitives in a diagram.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)] // Reason: Used by main processing pipeline
pub struct PrimitiveInventory {
    /// Detected boxes
    pub boxes: Vec<Box>,
    /// Detected horizontal arrows
    pub horizontal_arrows: Vec<HorizontalArrow>,
    /// Detected vertical arrows
    pub vertical_arrows: Vec<VerticalArrow>,
    /// Text rows (extracted from inside boxes)
    pub text_rows: Vec<TextRow>,
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

    // Phase 1, Cycle 1: RED - BoxStyle enum tests
    #[test]
    fn test_box_style_single_chars() {
        let chars = BoxStyle::Single.chars();
        // Single line: horizontal, vertical, top_left, top_right, bottom_left, bottom_right
        assert_eq!(chars.horizontal, '─');
        assert_eq!(chars.vertical, '│');
        assert_eq!(chars.top_left, '┌');
        assert_eq!(chars.top_right, '┐');
        assert_eq!(chars.bottom_left, '└');
        assert_eq!(chars.bottom_right, '┘');
    }

    #[test]
    fn test_box_style_double_chars() {
        let chars = BoxStyle::Double.chars();
        // Double line: horizontal, vertical, top_left, top_right, bottom_left, bottom_right
        assert_eq!(chars.horizontal, '═');
        assert_eq!(chars.vertical, '║');
        assert_eq!(chars.top_left, '╔');
        assert_eq!(chars.top_right, '╗');
        assert_eq!(chars.bottom_left, '╚');
        assert_eq!(chars.bottom_right, '╝');
    }

    #[test]
    fn test_box_style_rounded_chars() {
        let chars = BoxStyle::Rounded.chars();
        // Rounded: horizontal, vertical, top_left, top_right, bottom_left, bottom_right
        assert_eq!(chars.horizontal, '─');
        assert_eq!(chars.vertical, '│');
        assert_eq!(chars.top_left, '╭');
        assert_eq!(chars.top_right, '╮');
        assert_eq!(chars.bottom_left, '╰');
        assert_eq!(chars.bottom_right, '╯');
    }

    // Phase 1, Cycle 2: RED - Character recognition tests
    #[test]
    fn test_is_box_char_double_line() {
        // Double-line characters should be recognized
        assert!(is_box_char('═'));
        assert!(is_box_char('║'));
        assert!(is_box_char('╔'));
        assert!(is_box_char('╗'));
        assert!(is_box_char('╚'));
        assert!(is_box_char('╝'));
    }

    #[test]
    fn test_is_box_char_rounded() {
        // Rounded characters should be recognized
        assert!(is_box_char('╭'));
        assert!(is_box_char('╮'));
        assert!(is_box_char('╰'));
        assert!(is_box_char('╯'));
    }

    #[test]
    fn test_is_box_corner_all_styles() {
        // Single line corners
        assert!(is_box_corner('┌'));
        assert!(is_box_corner('┐'));
        assert!(is_box_corner('└'));
        assert!(is_box_corner('┘'));
        // Double line corners
        assert!(is_box_corner('╔'));
        assert!(is_box_corner('╗'));
        assert!(is_box_corner('╚'));
        assert!(is_box_corner('╝'));
        // Rounded corners
        assert!(is_box_corner('╭'));
        assert!(is_box_corner('╮'));
        assert!(is_box_corner('╰'));
        assert!(is_box_corner('╯'));
    }

    #[test]
    fn test_is_double_line_corner() {
        assert!(is_double_line_corner('╔'));
        assert!(is_double_line_corner('╗'));
        assert!(is_double_line_corner('╚'));
        assert!(is_double_line_corner('╝'));
        assert!(!is_double_line_corner('┌'));
        assert!(!is_double_line_corner('╭'));
    }

    #[test]
    fn test_is_rounded_corner() {
        assert!(is_rounded_corner('╭'));
        assert!(is_rounded_corner('╮'));
        assert!(is_rounded_corner('╰'));
        assert!(is_rounded_corner('╯'));
        assert!(!is_rounded_corner('┌'));
        assert!(!is_rounded_corner('╔'));
    }
}
