//! Safe mode: Only normalize Markdown tables, leave diagrams untouched.
//!
//! This module provides conservative content normalization that:
//! - Normalizes Markdown tables (alignment, spacing)
//! - Unwraps hard-wrapped table cells
//! - Normalizes list formatting (indentation, bullets)
//! - Preserves ASCII diagrams and special content

use crate::tables::{has_wrapped_cells, unwrap_table_rows};

use super::table::{is_table_row, is_table_separator, normalize_table};

/// Safe mode: Only normalize Markdown tables, leave diagrams untouched.
pub fn process_safe_mode(content: &str) -> String {
    // First normalize lists in the content
    let content = crate::lists::normalize_lists(content);
    // Then add blank lines before lists for Pandoc compatibility
    let content = crate::lists::normalize_loose_lists(&content);

    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        // Look for table pattern: header row -> separator row -> data rows
        if i + 1 < lines.len() && is_table_row(lines[i]) && is_table_separator(lines[i + 1]) {
            // Found a table, collect all rows
            let header = lines[i];
            let separator = lines[i + 1];
            i += 2;

            let mut table_rows = vec![];
            while i < lines.len() && is_table_row(lines[i]) {
                table_rows.push(lines[i]);
                i += 1;
            }

            // Check if table has wrapped cells and unwrap if needed
            let all_table_lines: Vec<&str> = std::iter::once(header)
                .chain(std::iter::once(separator))
                .chain(table_rows.iter().copied())
                .collect();

            let table_content = all_table_lines.join("\n");

            if has_wrapped_cells(&table_content) {
                // Unwrap the table rows
                let unwrapped_rows = unwrap_table_rows(&table_rows);
                // Convert unwrapped rows back to &str for normalize_table
                let unwrapped_refs: Vec<&str> = unwrapped_rows.iter().map(String::as_str).collect();

                if let Some(normalized) = normalize_table(header, separator, &unwrapped_refs) {
                    result.push(normalized);
                } else {
                    // If parsing fails, use unwrapped rows
                    result.push(header.to_string());
                    result.push(separator.to_string());
                    for row in unwrapped_rows {
                        result.push(row);
                    }
                }
            } else {
                // No wrapping - normalize normally
                if let Some(normalized) = normalize_table(header, separator, &table_rows) {
                    result.push(normalized);
                } else {
                    // If parsing fails, keep original lines
                    result.push(header.to_string());
                    result.push(separator.to_string());
                    for row in &table_rows {
                        result.push(row.to_string());
                    }
                    i -= table_rows.len();
                    i += 2;
                }
            }
        } else {
            result.push(lines[i].to_string());
            i += 1;
        }
    }

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_mode_preserves_content() {
        let content = "# Test\n\nSome content";
        let result = process_safe_mode(content);
        // Safe mode should preserve content for now
        assert_eq!(result, content);
    }

    #[test]
    fn test_safe_mode_normalizes_table() {
        let content = "| Name | Age |\n|------|-----|\n| Alice | 30 |\n| Bob | 25 |";
        let result = process_safe_mode(content);
        // Result should be normalized (may have different spacing)
        assert!(result.contains("| Name"));
        assert!(result.contains("| Age"));
        assert!(result.contains("| Alice"));
        assert!(result.contains("| Bob"));
    }

    #[test]
    fn test_safe_mode_preserves_non_tables() {
        let content = "# Title\n\nSome paragraph.\n\nMore text.";
        let result = process_safe_mode(content);
        assert_eq!(result, content);
    }

    #[test]
    fn test_safe_mode_misaligned_table() {
        let content = "| A | B |\n|---|---|\n| x| y |";
        let result = process_safe_mode(content);
        // Should normalize spacing
        assert!(result.contains("| A"));
        assert!(result.contains("| B"));
    }

    #[test]
    fn test_safe_mode_multiple_tables() {
        let content =
            "| H1 | H2 |\n|---|---|\n| a | b |\n\nText\n\n| C | D |\n|---|---|\n| c | d |";
        let result = process_safe_mode(content);
        // Both tables should be present
        assert!(result.contains("| H1"));
        assert!(result.contains("| C"));
    }

    #[test]
    fn test_link_in_table_cell_preserved() {
        // Test that links with parentheses in URLs are preserved in table cells
        let content = "| [API](https://example.com/api(v2)) | Description |\n|-----------------------------------|-------------|\n| Link | Value |";
        let result = process_safe_mode(content);
        // The URL with parentheses should be preserved
        assert!(
            result.contains("https://example.com/api(v2)"),
            "Link URL with parentheses should be preserved in table cell. Result:\n{result}"
        );
    }

    #[test]
    fn test_link_with_pipe_in_table_cell() {
        // Test that links containing | character don't break table parsing
        // This is a more challenging case that requires link-aware parsing
        let content = "| [Docs](https://example.com/doc|section) | Description |\n|------------------------------------------|-------------|\n| Link | Value |";
        let result = process_safe_mode(content);
        // The link should be preserved with its full URL
        assert!(
            result.contains("https://example.com/doc|section"),
            "Link URL with pipe should be preserved. Result:\n{result}"
        );
    }

    #[test]
    fn test_safe_mode_unwraps_wrapped_table_cells() {
        // Test that wrapped table cells are unwrapped in safe mode
        let content = "| Name | Description |\n|------|-------------|\n| Item | This is a very |\n|      | long description |";
        let result = process_safe_mode(content);
        // The wrapped cell should be joined into one row
        assert!(
            result.contains("This is a very long description"),
            "Wrapped cell should be unwrapped. Result:\n{result}"
        );
        // Should not have the continuation row pattern
        assert!(
            !result.contains("|      | long description |"),
            "Continuation row should be removed. Result:\n{result}"
        );
    }

    #[test]
    fn test_safe_mode_preserves_multiline_code_in_tables() {
        // Test that intentional multi-line content (code blocks) is preserved
        let content = "| Code | Example |\n|------|---------|\n| ```python | of code |\n| def hello(): | inside |\n| ``` | cell |";
        let result = process_safe_mode(content);
        // Code blocks should be preserved (not unwrapped)
        assert!(
            result.contains("```python"),
            "Code fence should be preserved. Result:\n{result}"
        );
        assert!(
            result.contains("def hello():"),
            "Code content should be preserved. Result:\n{result}"
        );
    }

    #[test]
    fn test_safe_mode_normalizes_list_indentation() {
        // Test that inconsistent list indentation is normalized
        let content = "- Item 1\n    - Nested with 4 spaces\n- Item 2";
        let result = process_safe_mode(content);
        // Nested item should be 2 spaces, not 4
        assert!(
            result.contains("  - Nested with 4 spaces"),
            "List indentation should be normalized to 2 spaces. Result:\n{result}"
        );
        assert!(
            !result.contains("    - Nested"),
            "4-space indentation should be removed. Result:\n{result}"
        );
    }

    #[test]
    fn test_safe_mode_normalizes_bullet_styles() {
        // Test that mixed bullet styles are normalized
        let content = "- Item 1\n* Item 2\n+ Item 3";
        let result = process_safe_mode(content);
        // All bullets should be normalized to dash
        assert!(
            result.contains("- Item 1"),
            "Dash bullet should remain. Result:\n{result}"
        );
        assert!(
            result.contains("- Item 2"),
            "Asterisk should become dash. Result:\n{result}"
        );
        assert!(
            result.contains("- Item 3"),
            "Plus should become dash. Result:\n{result}"
        );
    }

    #[test]
    fn test_safe_mode_preserves_task_lists() {
        // Test that task list checkboxes are preserved
        let content = "- [ ] Todo item\n- [x] Done item\n- [X] Also done";
        let result = process_safe_mode(content);
        assert!(
            result.contains("- [ ] Todo item"),
            "Unchecked task should be preserved. Result:\n{result}"
        );
        assert!(
            result.contains("- [x] Done item"),
            "Checked task should be preserved. Result:\n{result}"
        );
    }

    #[test]
    fn test_safe_mode_preserves_lists_in_code_blocks() {
        // Test that lists inside code blocks are not normalized
        let content =
            "```markdown\n- Item in code block\n* Another item\n```\n\n- Real item outside";
        let result = process_safe_mode(content);
        // List in code block should preserve mixed bullets
        assert!(
            result.contains("- Item in code block"),
            "List in code block should be preserved. Result:\n{result}"
        );
        assert!(
            result.contains("* Another item"),
            "Asterisk in code block should be preserved. Result:\n{result}"
        );
        // Outside list should be normalized
        assert!(
            result.contains("- Real item outside"),
            "Outside list should be normalized. Result:\n{result}"
        );
    }
}
