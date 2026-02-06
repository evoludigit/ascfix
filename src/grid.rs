//! 2D grid representation for ASCII diagrams.

/// A 2D grid of characters representing an ASCII diagram.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // Reason: Used by main processing pipeline
pub struct Grid {
    /// Grid data: rows of characters
    rows: Vec<Vec<char>>,
}

impl Grid {
    /// Create a grid from lines of text.
    #[allow(dead_code)] // Reason: Used by main processing pipeline
    #[must_use]
    pub fn from_lines(lines: &[&str]) -> Self {
        let rows = lines.iter().map(|line| line.chars().collect()).collect();
        Self { rows }
    }

    /// Get the number of rows in the grid.
    #[allow(dead_code)] // Reason: Used by main processing pipeline
    #[must_use]
    pub const fn height(&self) -> usize {
        self.rows.len()
    }

    /// Get the number of columns in the grid (width of the longest row).
    #[allow(dead_code)] // Reason: Used by main processing pipeline
    #[must_use]
    pub fn width(&self) -> usize {
        self.rows.iter().map(Vec::len).max().unwrap_or(0)
    }

    /// Get a character at (row, col). Returns None if out of bounds.
    #[allow(dead_code)] // Reason: Used by main processing pipeline
    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        self.rows.get(row).and_then(|r| r.get(col).copied())
    }

    /// Get a mutable reference to a cell. Returns None if out of bounds.
    #[allow(dead_code)] // Reason: Used by main processing pipeline
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut char> {
        self.rows.get_mut(row).and_then(|r| r.get_mut(col))
    }

    /// Render the grid back to a string (including trailing whitespace).
    #[allow(dead_code)] // Reason: Used by main processing pipeline
    #[must_use]
    pub fn render(&self) -> String {
        self.rows
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Render the grid, trimming trailing whitespace from each line.
    #[allow(dead_code)] // Reason: Used by main processing pipeline
    #[must_use]
    pub fn render_trimmed(&self) -> String {
        self.rows
            .iter()
            .map(|row| {
                let s = row.iter().collect::<String>();
                s.trim_end().to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_from_simple_lines() {
        let lines = vec!["abc", "def"];
        let grid = Grid::from_lines(&lines);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid.width(), 3);
    }

    #[test]
    fn test_grid_single_line() {
        let lines = vec!["hello"];
        let grid = Grid::from_lines(&lines);
        assert_eq!(grid.height(), 1);
        assert_eq!(grid.width(), 5);
    }

    #[test]
    fn test_grid_empty() {
        let lines: Vec<&str> = vec![];
        let grid = Grid::from_lines(&lines);
        assert_eq!(grid.height(), 0);
        assert_eq!(grid.width(), 0);
    }

    #[test]
    fn test_grid_variable_widths() {
        let lines = vec!["a", "bb", "ccc"];
        let grid = Grid::from_lines(&lines);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid.width(), 3);
    }

    #[test]
    fn test_grid_get_character() {
        let lines = vec!["abc", "def"];
        let grid = Grid::from_lines(&lines);
        assert_eq!(grid.get(0, 0), Some('a'));
        assert_eq!(grid.get(0, 1), Some('b'));
        assert_eq!(grid.get(1, 0), Some('d'));
        assert_eq!(grid.get(1, 2), Some('f'));
    }

    #[test]
    fn test_grid_get_out_of_bounds() {
        let lines = vec!["ab"];
        let grid = Grid::from_lines(&lines);
        assert_eq!(grid.get(0, 5), None);
        assert_eq!(grid.get(5, 0), None);
    }

    #[test]
    fn test_grid_render() {
        let lines = vec!["abc", "def"];
        let grid = Grid::from_lines(&lines);
        assert_eq!(grid.render(), "abc\ndef");
    }

    #[test]
    fn test_grid_roundtrip() {
        let original = "line1\nline2\nline3";
        let lines: Vec<&str> = original.lines().collect();
        let grid = Grid::from_lines(&lines);
        assert_eq!(grid.render(), original);
    }

    #[test]
    fn test_grid_with_special_chars() {
        let lines = vec!["┌─┐", "│ │", "└─┘"];
        let grid = Grid::from_lines(&lines);
        assert_eq!(grid.get(0, 0), Some('┌'));
        assert_eq!(grid.get(0, 1), Some('─'));
        assert_eq!(grid.get(0, 2), Some('┐'));
    }

    #[test]
    fn test_grid_get_mut() {
        let lines = vec!["ab"];
        let mut grid = Grid::from_lines(&lines);
        if let Some(cell) = grid.get_mut(0, 0) {
            *cell = 'x';
        }
        assert_eq!(grid.get(0, 0), Some('x'));
    }

    #[test]
    fn test_grid_render_trimmed() {
        let lines = vec!["abc  ", "d    "];
        let grid = Grid::from_lines(&lines);
        assert_eq!(grid.render_trimmed(), "abc\nd");
    }

    #[test]
    fn test_roundtrip_simple_box() {
        let original = "┌─────┐\n│ Box │\n└─────┘";
        let lines: Vec<&str> = original.lines().collect();
        let grid = Grid::from_lines(&lines);
        assert_eq!(grid.render(), original);
    }

    #[test]
    fn test_roundtrip_with_arrows() {
        let original = "  ↓\n┌─┐\n│ │\n└─┘\n  ↓";
        let lines: Vec<&str> = original.lines().collect();
        let grid = Grid::from_lines(&lines);
        assert_eq!(grid.render(), original);
    }

    #[test]
    fn test_idempotent_grid_render() {
        let original = "line1\nline2\nline3";
        let lines1: Vec<&str> = original.lines().collect();
        let grid1 = Grid::from_lines(&lines1);
        let rendered1 = grid1.render();

        let lines2: Vec<&str> = rendered1.lines().collect();
        let grid2 = Grid::from_lines(&lines2);
        let rendered2 = grid2.render();

        assert_eq!(rendered1, rendered2);
    }
}
