//! Table unwrapping for hard-wrapped Markdown tables.
//!
//! This module detects and repairs tables where cells have been hard-wrapped
//! at 80 columns, joining continuation lines back into single cells.

/// Check if a table has wrapped cells that need unwrapping.
///
/// Wrapped cells typically appear as:
/// ```markdown
/// | Name | Description |
/// |------|-------------|
/// | Item | This is a very |
/// |      | long description |
/// ```
///
/// The continuation line has empty cells (only whitespace + |) in the
/// columns that are not being continued.
///
/// # Examples
///
/// ```
/// use ascfix::tables::has_wrapped_cells;
///
/// let content = "| Name | Description |\n|------|-------------|\n| Item | This is a very |\n|      | long description |";
/// assert!(has_wrapped_cells(content));
/// ```
#[must_use]
#[allow(dead_code)] // Reason: Used in tests, will be used in production soon
pub fn has_wrapped_cells(content: &str) -> bool {
    let lines: Vec<&str> = content.lines().collect();

    for i in 0..lines.len() {
        let line = lines[i].trim();

        // Skip non-table rows
        if !is_table_row(line) {
            continue;
        }

        // Check if this row is a continuation of the previous row
        if i > 0 {
            let prev_line = lines[i - 1].trim();
            if is_table_row(prev_line) && is_continuation_row(line) {
                return true;
            }
        }
    }

    false
}

/// Check if a line looks like a table row (starts and ends with |).
fn is_table_row(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|') && trimmed.ends_with('|')
}

/// Check if a row is a continuation of the previous row.
///
/// A continuation row typically has:
/// 1. ALL leading cells empty (only whitespace), OR
/// 2. Exactly one non-empty cell (indicating that column is being continued)
///
/// This distinguishes wrapped cells from tables with legitimately empty columns.
///
/// Example of continuation rows:
/// ```markdown
/// | Name | Description with |
/// |      |  wrapped text    |  <- Leading cell empty
/// | API  | [Link](http://   |
/// |      | example.com)     |  <- Continuing URL column
/// ```
///
/// NOT a continuation (legitimate empty cells):
/// ```markdown
/// | Flag | | Description |  <- Multiple columns have content
/// ```
fn is_continuation_row(line: &str) -> bool {
    let cells = split_table_cells(line);

    if cells.is_empty() {
        return false;
    }

    // Count non-empty cells
    let non_empty_count = cells.iter().filter(|cell| !cell.trim().is_empty()).count();

    // A continuation row should have:
    // - At most one non-empty cell (the cell being continued)
    // - At least one empty cell
    // - First cell empty (continuation starts after the first column)
    non_empty_count <= 1
        && non_empty_count < cells.len()
        && cells.first().map_or(false, |c| c.trim().is_empty())
}

/// Split a table row into cells, handling the | delimiters.
fn split_table_cells(line: &str) -> Vec<&str> {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
        return Vec::new();
    }

    // Split by | and skip the first empty element before the opening |
    // The final | creates an empty element at the end which we filter out
    let parts: Vec<&str> = trimmed.split('|').skip(1).collect();

    // Filter out only the trailing empty cell (from the final |)
    // Keep any empty cells that are in the middle (which indicate wrapped cells)
    parts
        .iter()
        .enumerate()
        .filter(|(idx, s)| *idx < parts.len() - 1 || !s.is_empty())
        .map(|(_, s)| *s)
        .collect()
}

/// Unwrap table rows by joining continuation lines.
///
/// Takes rows that may include wrapped cells and joins continuation lines
/// into single rows. For example:
/// ```markdown
/// | Item | This is a very |
/// |      | long description |
/// ```
/// becomes:
/// ```markdown
/// | Item | This is a very long description |
/// ```
///
/// Intentional multi-line content with code blocks is preserved:
/// ```markdown
/// | Code | Example |
/// | ```python | of code |
/// | def hello(): | inside |
/// | ``` | cell |
/// ```
/// remains unchanged (preserved as 4 rows).
///
/// Links that span across wrap boundaries are also preserved to prevent
/// breaking markdown link syntax.
#[must_use]
#[allow(dead_code)] // Reason: Used in tests, will be used in production soon
pub fn unwrap_table_rows(rows: &[&str]) -> Vec<String> {
    if rows.is_empty() {
        return Vec::new();
    }

    // Check if any row contains a code fence - if so, preserve all rows as-is
    if rows_contain_code_fence(rows) {
        return rows.iter().map(|&s| s.to_string()).collect();
    }

    // Check if rows contain incomplete links that would be broken by unwrapping
    if has_incomplete_link_across_rows(rows) {
        return rows.iter().map(|&s| s.to_string()).collect();
    }

    let mut result: Vec<String> = Vec::new();
    let mut pending_row: Option<Vec<String>> = None;

    for row in rows {
        let cells = split_table_cells(row);
        if cells.is_empty() {
            continue;
        }

        if let Some(ref mut pending) = pending_row {
            // Check if this is a continuation of the pending row
            if is_continuation_row(row) {
                // Merge cells: for each cell, if pending has content and current is empty,
                // keep pending; if current has content, append to pending with space
                for (idx, cell) in cells.iter().enumerate() {
                    let cell_trimmed = cell.trim();
                    if idx < pending.len() && !cell_trimmed.is_empty() {
                        // Append to existing cell with space
                        if !pending[idx].is_empty() {
                            pending[idx].push(' ');
                        }
                        pending[idx].push_str(cell_trimmed);
                    }
                    // If cell is empty or idx >= pending.len(), keep the pending cell as-is
                }
            } else {
                // Not a continuation - finalize pending row and start new one
                let row_str = format_row(pending);
                result.push(row_str);
                pending_row = Some(cells.iter().map(|s| s.trim().to_string()).collect());
            }
        } else {
            // First row - start pending
            pending_row = Some(cells.iter().map(|s| s.trim().to_string()).collect());
        }
    }

    // Don't forget the last pending row
    if let Some(ref pending) = pending_row {
        let row_str = format_row(pending);
        result.push(row_str);
    }

    result
}

/// Format a row of cells back into a table row string.
fn format_row(cells: &[String]) -> String {
    let mut result = String::new();
    result.push('|');
    for cell in cells {
        result.push(' ');
        result.push_str(cell);
        result.push(' ');
        result.push('|');
    }
    result
}

/// Check if a line contains code fence markers.
/// Detects both backtick and tilde fences.
fn contains_code_fence(line: &str) -> bool {
    let trimmed = line.trim();
    // Check for backtick fences: ``` or ```python
    if trimmed.contains("```") {
        return true;
    }
    // Check for tilde fences: ~~~ or ~~~bash
    if trimmed.contains("~~~") {
        return true;
    }
    false
}

/// Check if a sequence of rows contains any code fences.
/// Used to determine if wrapped rows should be preserved as multi-line content.
fn rows_contain_code_fence(rows: &[&str]) -> bool {
    rows.iter().any(|row| contains_code_fence(row))
}

/// Check if rows contain an incomplete link that spans across the wrap boundary.
///
/// A link is incomplete if:
/// - It has an opening `[text](` pattern on one row
/// - But the closing `)` is not on the same row (it's on a continuation row)
///
/// This prevents unwrapping rows that would break a link across lines.
fn has_incomplete_link_across_rows(rows: &[&str]) -> bool {
    for (row_idx, row) in rows.iter().enumerate() {
        let chars: Vec<char> = row.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Look for opening bracket '['
            if chars[i] == '[' {
                // Try to find the complete link pattern
                if let Some(text_end) = find_closing_bracket(&chars, i + 1) {
                    // Check for opening parenthesis immediately after ']'
                    if text_end + 1 < chars.len() && chars[text_end + 1] == '(' {
                        // Look for closing parenthesis with proper nesting
                        if find_closing_parenthesis_balanced(&chars, text_end + 2).is_none() {
                            // Opening pattern found but no closing on this row
                            // Check if there are more rows (continuation)
                            if row_idx < rows.len() - 1 {
                                // Link spans across rows - don't unwrap
                                return true;
                            }
                        }
                    }
                }
            }
            i += 1;
        }
    }

    false
}

/// Find the closing bracket ']' starting from the given position.
/// Handles escaped brackets '\[' and '\]'.
fn find_closing_bracket(chars: &[char], start: usize) -> Option<usize> {
    let mut i = start;
    while i < chars.len() {
        if chars[i] == ']' && (i == 0 || chars[i - 1] != '\\') {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Find the closing parenthesis ')' with proper nesting.
/// Returns None if no balanced closing parenthesis is found.
fn find_closing_parenthesis_balanced(chars: &[char], start: usize) -> Option<usize> {
    let mut depth = 1; // We're already inside one level of parentheses
    let mut i = start;

    while i < chars.len() {
        match chars[i] {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }

    None // No balanced closing parenthesis found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_non_wrapped_table() {
        let content = "| Name | Description |\n|------|-------------|\n| Item | Short desc |";
        assert!(
            !has_wrapped_cells(content),
            "Should not detect wrapping in normal table"
        );
    }

    #[test]
    fn unwrap_single_wrapped_cell() {
        let rows = vec!["| Item | This is a very |", "|      | long description |"];
        let unwrapped = unwrap_table_rows(&rows);
        assert_eq!(unwrapped.len(), 1);
        assert_eq!(unwrapped[0], "| Item | This is a very long description |");
    }

    #[test]
    fn detect_wrapped_table_cell() {
        let content = "| Name | Description |\n|------|-------------|\n| Item | This is a very |\n|      | long description |";
        assert!(has_wrapped_cells(content), "Should detect wrapped cells");
    }

    #[test]
    fn unwrap_multiple_wrapped_columns() {
        // Test table where multiple columns have wrapped content
        let rows = vec!["| x | very | z |", "|   | long |   |", "|   | text |   |"];
        let unwrapped = unwrap_table_rows(&rows);
        assert_eq!(unwrapped.len(), 1);
        assert_eq!(unwrapped[0], "| x | very long text | z |");
    }

    #[test]
    fn preserve_intentional_multiline_code_block() {
        // Table cell with code block should NOT be unwrapped
        let rows = vec![
            "| Code | Example |",
            "| ```python | of code |",
            "| def hello(): | inside |",
            "| ``` | cell |",
        ];
        let unwrapped = unwrap_table_rows(&rows);
        // Should preserve all 4 rows since there's a code block
        assert_eq!(unwrapped.len(), 4);
    }

    #[test]
    fn unwrap_wrapped_but_not_code() {
        // Regular wrapped text should be joined, but code blocks preserved
        let rows = vec!["| A | This is a very |", "|   | long description |"];
        let unwrapped = unwrap_table_rows(&rows);
        assert_eq!(unwrapped.len(), 1);
        assert_eq!(unwrapped[0], "| A | This is a very long description |");
    }

    #[test]
    fn detect_code_fence_in_cell() {
        // Test detection of code fences inside table cells
        assert!(contains_code_fence("| ```python | code |"));
        assert!(contains_code_fence("| ``` | end |"));
        assert!(contains_code_fence("| ~~~bash | script |"));
        assert!(!contains_code_fence("| normal text | here |"));
        assert!(!contains_code_fence("| no code | fence |"));
    }

    #[test]
    fn preserve_incomplete_link_across_wrap() {
        // Link that spans wrap boundary should NOT be unwrapped
        let rows = vec![
            "| Item | [Docs](https://example.com/very |",
            "|      | long/path) more text |",
        ];
        let unwrapped = unwrap_table_rows(&rows);
        // Should preserve both rows since link spans boundary
        assert_eq!(
            unwrapped.len(),
            2,
            "Should preserve rows when link spans boundary"
        );
        assert!(unwrapped[0].contains("[Docs]"));
        assert!(unwrapped[1].contains("long/path)"));
    }

    #[test]
    fn unwrap_complete_link_on_one_line() {
        // Complete link on one line should allow unwrapping
        let rows = vec![
            "| Item | [Docs](https://example.com/path) |",
            "|      | more description text |",
        ];
        let unwrapped = unwrap_table_rows(&rows);
        // Should unwrap since link is complete on first line
        assert_eq!(unwrapped.len(), 1);
        assert!(unwrapped[0].contains("[Docs](https://example.com/path)"));
        assert!(unwrapped[0].contains("more description text"));
    }

    #[test]
    fn detect_incomplete_link_in_rows() {
        // Test detection of incomplete links
        assert!(has_incomplete_link_across_rows(&[
            "| [Link](http://x |",
            "| /path) |"
        ]));
        assert!(!has_incomplete_link_across_rows(&[
            "| [Link](http://x/path) |",
            "| more text |"
        ]));
    }
}
