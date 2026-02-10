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
/// A continuation row has some cells that are empty (only whitespace)
/// and at least one cell with content.
fn is_continuation_row(line: &str) -> bool {
    let cells = split_table_cells(line);

    // A continuation row should have:
    // - At least one empty cell (only whitespace)
    // - At least one non-empty cell
    let has_empty = cells.iter().any(|cell| cell.trim().is_empty());
    let has_content = cells.iter().any(|cell| !cell.trim().is_empty());

    has_empty && has_content
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
#[must_use]
#[allow(dead_code)] // Reason: Used in tests, will be used in production soon
pub fn unwrap_table_rows(rows: &[&str]) -> Vec<String> {
    if rows.is_empty() {
        return Vec::new();
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
}
