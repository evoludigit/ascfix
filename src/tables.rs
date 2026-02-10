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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_wrapped_table_cell() {
        let content = "| Name | Description |\n|------|-------------|\n| Item | This is a very |\n|      | long description |";
        assert!(has_wrapped_cells(content), "Should detect wrapped cells");
    }

    #[test]
    fn detect_non_wrapped_table() {
        let content = "| Name | Description |\n|------|-------------|\n| Item | Short desc |";
        assert!(
            !has_wrapped_cells(content),
            "Should not detect wrapping in normal table"
        );
    }
}
