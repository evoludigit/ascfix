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

    /// Determine box style from a corner character.
    ///
    /// Returns the style if the character is a corner of a box,
    /// or None if the character is not a corner.
    #[must_use]
    #[allow(dead_code)] // Reason: Used by detector in upcoming phases
    pub const fn from_corner(ch: char) -> Option<Self> {
        match ch {
            '┌' | '┐' | '└' | '┘' => Some(Self::Single),
            '╔' | '╗' | '╚' | '╝' => Some(Self::Double),
            '╭' | '╮' | '╰' | '╯' => Some(Self::Rounded),
            _ => None,
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
    /// Style of the box (single, double, or rounded lines)
    pub style: BoxStyle,
    /// Index of parent box (if nested inside another box)
    pub parent_idx: Option<usize>,
    /// Indices of child boxes (if this box contains other boxes)
    pub child_indices: Vec<usize>,
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

/// Arrow type for different arrow styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Reason: Used by detector and renderer in upcoming phases
pub enum ArrowType {
    /// Standard arrow: → ←
    Standard,
    /// Double arrow: ⇒ ⇐
    Double,
    /// Long arrow: ⟶ ⟵
    Long,
    /// Dashed arrow: · > <
    Dashed,
}

/// Arrow drawing characters for a specific style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Reason: Used by detector and renderer in upcoming phases
pub struct ArrowChars {
    /// Line character
    pub line: char,
    /// Arrowhead pointing right
    pub arrowhead_right: char,
    /// Arrowhead pointing left
    pub arrowhead_left: char,
}

impl ArrowType {
    /// Get the characters used for this arrow type.
    #[must_use]
    #[allow(dead_code)] // Reason: Used by detector and renderer in upcoming phases
    pub const fn chars(self) -> ArrowChars {
        match self {
            Self::Standard => ArrowChars {
                line: '─',
                arrowhead_right: '→',
                arrowhead_left: '←',
            },
            Self::Double => ArrowChars {
                line: '═',
                arrowhead_right: '⇒',
                arrowhead_left: '⇐',
            },
            Self::Long => ArrowChars {
                line: '─',
                arrowhead_right: '⟶',
                arrowhead_left: '⟵',
            },
            Self::Dashed => ArrowChars {
                line: '·',
                arrowhead_right: '>',
                arrowhead_left: '<',
            },
        }
    }

    /// Determine arrow type from an arrowhead character.
    ///
    /// Returns the type if the character is an arrowhead,
    /// or None if the character is not an arrowhead.
    #[must_use]
    #[allow(dead_code)] // Reason: Used by detector in upcoming phases
    pub const fn from_char(ch: char) -> Option<Self> {
        match ch {
            '→' | '←' => Some(Self::Standard),
            '⇒' | '⇐' => Some(Self::Double),
            '⟶' | '⟵' => Some(Self::Long),
            '>' | '<' => Some(Self::Dashed),
            _ => None,
        }
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
    /// Type of the arrow (standard, double, long, dashed)
    pub arrow_type: ArrowType,
    /// Direction: true if rightward (→), false if leftward (←)
    pub rightward: bool,
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
    /// Type of the arrow (standard, double, long, dashed)
    pub arrow_type: ArrowType,
    /// Direction: true if downward (↓), false if upward (↑)
    pub downward: bool,
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

/// A segment of a connection line (either horizontal or vertical).
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // Reason: Used by connection line detection and rendering
pub enum Segment {
    /// Horizontal segment: row and column range
    Horizontal {
        /// Row position
        row: usize,
        /// Starting column
        start_col: usize,
        /// Ending column
        end_col: usize,
    },
    /// Vertical segment: column and row range
    Vertical {
        /// Column position
        col: usize,
        /// Starting row
        start_row: usize,
        /// Ending row
        end_row: usize,
    },
}

impl Segment {
    /// Get the length of this segment (number of cells spanned).
    #[allow(dead_code)] // Reason: Used by normalization pipeline
    #[must_use]
    pub fn length(&self) -> usize {
        match self {
            Self::Horizontal {
                start_col, end_col, ..
            } => end_col - start_col + 1,
            Self::Vertical {
                start_row, end_row, ..
            } => end_row - start_row + 1,
        }
    }
}

/// An L-shaped or multi-segment connection line (e.g., between boxes).
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // Reason: Used by main processing pipeline
pub struct ConnectionLine {
    /// Segments that make up this connection line
    pub segments: Vec<Segment>,
    /// Index of the box this line originates from (if any)
    pub from_box: Option<usize>,
    /// Index of the box this line connects to (if any)
    pub to_box: Option<usize>,
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

/// Check if a character is a vertical arrow character (any style or direction).
#[allow(dead_code)] // Reason: Used by detector in upcoming phases
const fn is_vertical_arrow_char(ch: char) -> bool {
    matches!(ch, '↓' | '↑' | '⇓' | '⇑' | '⟱' | '⟰')
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
    /// Detected connection lines
    pub connection_lines: Vec<ConnectionLine>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_dimensions() {
        let b = Box {
            top_left: (0, 0),
            bottom_right: (3, 5),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        };
        assert_eq!(b.width(), 6);
        assert_eq!(b.height(), 4);
    }

    #[test]
    fn test_box_contains_interior() {
        let b = Box {
            top_left: (0, 0),
            bottom_right: (3, 5),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
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
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
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
            arrow_type: ArrowType::Standard,
            rightward: true,
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
            arrow_type: ArrowType::Standard,
            downward: true,
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

    // Phase 1, Cycle 3: RED - Box style detection tests
    #[test]
    fn test_box_style_from_single_corner() {
        assert_eq!(BoxStyle::from_corner('┌'), Some(BoxStyle::Single));
        assert_eq!(BoxStyle::from_corner('┐'), Some(BoxStyle::Single));
        assert_eq!(BoxStyle::from_corner('└'), Some(BoxStyle::Single));
        assert_eq!(BoxStyle::from_corner('┘'), Some(BoxStyle::Single));
    }

    #[test]
    fn test_box_style_from_double_corner() {
        assert_eq!(BoxStyle::from_corner('╔'), Some(BoxStyle::Double));
        assert_eq!(BoxStyle::from_corner('╗'), Some(BoxStyle::Double));
        assert_eq!(BoxStyle::from_corner('╚'), Some(BoxStyle::Double));
        assert_eq!(BoxStyle::from_corner('╝'), Some(BoxStyle::Double));
    }

    #[test]
    fn test_box_style_from_rounded_corner() {
        assert_eq!(BoxStyle::from_corner('╭'), Some(BoxStyle::Rounded));
        assert_eq!(BoxStyle::from_corner('╮'), Some(BoxStyle::Rounded));
        assert_eq!(BoxStyle::from_corner('╰'), Some(BoxStyle::Rounded));
        assert_eq!(BoxStyle::from_corner('╯'), Some(BoxStyle::Rounded));
    }

    #[test]
    fn test_box_style_from_non_corner() {
        assert_eq!(BoxStyle::from_corner('─'), None);
        assert_eq!(BoxStyle::from_corner('│'), None);
        assert_eq!(BoxStyle::from_corner('a'), None);
    }

    #[test]
    fn test_box_has_style_field() {
        let b = Box {
            top_left: (0, 0),
            bottom_right: (3, 5),
            style: BoxStyle::Single,
            parent_idx: None,
            child_indices: Vec::new(),
        };
        assert_eq!(b.style, BoxStyle::Single);
    }

    #[test]
    fn test_box_style_preserved() {
        let b = Box {
            top_left: (1, 2),
            bottom_right: (5, 8),
            style: BoxStyle::Double,
            parent_idx: None,
            child_indices: Vec::new(),
        };
        assert_eq!(b.style, BoxStyle::Double);
    }

    // Phase 1, Cycle 3: Tests ensuring from_corner() method works
    // (Note: tests in cycle 3 RED phase above)

    // Phase 2, Cycle 5: RED - ArrowType enum tests
    #[test]
    fn test_arrow_type_standard_chars() {
        let chars = ArrowType::Standard.chars();
        assert_eq!(chars.line, '─');
        assert_eq!(chars.arrowhead_right, '→');
        assert_eq!(chars.arrowhead_left, '←');
    }

    #[test]
    fn test_arrow_type_double_chars() {
        let chars = ArrowType::Double.chars();
        assert_eq!(chars.line, '═');
        assert_eq!(chars.arrowhead_right, '⇒');
        assert_eq!(chars.arrowhead_left, '⇐');
    }

    #[test]
    fn test_arrow_type_long_chars() {
        let chars = ArrowType::Long.chars();
        assert_eq!(chars.line, '─');
        assert_eq!(chars.arrowhead_right, '⟶');
        assert_eq!(chars.arrowhead_left, '⟵');
    }

    #[test]
    fn test_arrow_type_dashed_chars() {
        let chars = ArrowType::Dashed.chars();
        assert_eq!(chars.line, '·');
        assert_eq!(chars.arrowhead_right, '>');
        assert_eq!(chars.arrowhead_left, '<');
    }

    #[test]
    fn test_arrow_type_from_char() {
        assert_eq!(ArrowType::from_char('→'), Some(ArrowType::Standard));
        assert_eq!(ArrowType::from_char('←'), Some(ArrowType::Standard));
        assert_eq!(ArrowType::from_char('⇒'), Some(ArrowType::Double));
        assert_eq!(ArrowType::from_char('⇐'), Some(ArrowType::Double));
        assert_eq!(ArrowType::from_char('⟶'), Some(ArrowType::Long));
        assert_eq!(ArrowType::from_char('⟵'), Some(ArrowType::Long));
        assert_eq!(ArrowType::from_char('a'), None);
    }

    #[test]
    fn test_arrow_type_is_vertical_arrow_char() {
        assert!(is_vertical_arrow_char('↓'));
        assert!(is_vertical_arrow_char('↑'));
        assert!(is_vertical_arrow_char('⇓'));
        assert!(is_vertical_arrow_char('⇑'));
        assert!(is_vertical_arrow_char('⟱'));
        assert!(is_vertical_arrow_char('⟰'));
        assert!(!is_vertical_arrow_char('→'));
        assert!(!is_vertical_arrow_char('a'));
    }

    // Phase 2, Cycle 6: RED - Arrow detection tests
    #[test]
    fn test_horizontal_arrow_has_type_field() {
        let arr = HorizontalArrow {
            row: 5,
            start_col: 2,
            end_col: 8,
            arrow_type: ArrowType::Standard,
            rightward: true,
        };
        assert_eq!(arr.arrow_type, ArrowType::Standard);
        assert!(arr.rightward);
    }

    #[test]
    fn test_horizontal_arrow_leftward() {
        let arr = HorizontalArrow {
            row: 5,
            start_col: 8,
            end_col: 2,
            arrow_type: ArrowType::Double,
            rightward: false,
        };
        assert_eq!(arr.arrow_type, ArrowType::Double);
        assert!(!arr.rightward);
    }

    #[test]
    fn test_vertical_arrow_has_type_field() {
        let arr = VerticalArrow {
            col: 3,
            start_row: 1,
            end_row: 6,
            arrow_type: ArrowType::Long,
            downward: true,
        };
        assert_eq!(arr.arrow_type, ArrowType::Long);
        assert!(arr.downward);
    }

    #[test]
    fn test_vertical_arrow_upward() {
        let arr = VerticalArrow {
            col: 3,
            start_row: 6,
            end_row: 1,
            arrow_type: ArrowType::Dashed,
            downward: false,
        };
        assert_eq!(arr.arrow_type, ArrowType::Dashed);
        assert!(!arr.downward);
    }

    // Phase 4, Cycle 12: RED - ConnectionLine primitive tests
    #[test]
    fn test_segment_horizontal() {
        let seg = Segment::Horizontal {
            row: 5,
            start_col: 2,
            end_col: 8,
        };
        match seg {
            Segment::Horizontal {
                row,
                start_col,
                end_col,
            } => {
                assert_eq!(row, 5);
                assert_eq!(start_col, 2);
                assert_eq!(end_col, 8);
            }
            Segment::Vertical { .. } => panic!("Expected Horizontal segment"),
        }
    }

    #[test]
    fn test_segment_vertical() {
        let seg = Segment::Vertical {
            col: 3,
            start_row: 1,
            end_row: 6,
        };
        match seg {
            Segment::Vertical {
                col,
                start_row,
                end_row,
            } => {
                assert_eq!(col, 3);
                assert_eq!(start_row, 1);
                assert_eq!(end_row, 6);
            }
            Segment::Horizontal { .. } => panic!("Expected Vertical segment"),
        }
    }

    #[test]
    fn test_connection_line_basic() {
        let conn = ConnectionLine {
            segments: vec![
                Segment::Horizontal {
                    row: 2,
                    start_col: 0,
                    end_col: 5,
                },
                Segment::Vertical {
                    col: 5,
                    start_row: 2,
                    end_row: 5,
                },
            ],
            from_box: Some(0),
            to_box: Some(1),
        };
        assert_eq!(conn.segments.len(), 2);
        assert_eq!(conn.from_box, Some(0));
        assert_eq!(conn.to_box, Some(1));
    }

    #[test]
    fn test_connection_line_unattached() {
        let conn = ConnectionLine {
            segments: vec![Segment::Horizontal {
                row: 3,
                start_col: 0,
                end_col: 10,
            }],
            from_box: None,
            to_box: None,
        };
        assert_eq!(conn.segments.len(), 1);
        assert!(conn.from_box.is_none());
        assert!(conn.to_box.is_none());
    }

    #[test]
    fn test_connection_line_single_segment() {
        let conn = ConnectionLine {
            segments: vec![Segment::Vertical {
                col: 2,
                start_row: 0,
                end_row: 4,
            }],
            from_box: Some(0),
            to_box: None,
        };
        assert_eq!(conn.segments.len(), 1);
        assert_eq!(conn.from_box, Some(0));
    }
}
