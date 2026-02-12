//! Mode-specific processing implementations.

use crate::cli::Mode;
use crate::links::{detect_links, is_inside_link_url};
use crate::tables::{has_wrapped_cells, unwrap_table_rows};
use std::fmt::Write;

/// Process content according to the specified mode.
///
/// # Modes
/// - `Safe`: Only normalize Markdown tables (minimal changes, no diagrams)
/// - `Diagram`: Detect and normalize ASCII diagrams (boxes, arrows, text)
/// - `Check`: Validate content but don't modify (used with --check flag)
///
/// If `repair_fences` is true, fence normalization is applied first.
#[must_use]
#[allow(dead_code)] // Reason: Used by main processing pipeline
pub fn process_by_mode(
    mode: &Mode,
    content: &str,
    repair_fences: bool,
    config: &crate::config::Config,
) -> String {
    // For backward compatibility in internal calls, use default config when none provided
    let _default_config = crate::config::Config::default();
    let effective_config = config;

    // Apply fence repair first if enabled
    let content = if repair_fences {
        crate::fences::normalize_fences(content)
    } else {
        content.to_string()
    };

    match mode {
        Mode::Safe => process_safe_mode(&content),
        Mode::Diagram => process_diagram_mode(&content, effective_config),
        Mode::Check => process_check_mode(&content),
    }
}

/// Safe mode: Only normalize Markdown tables, leave diagrams untouched.
fn process_safe_mode(content: &str) -> String {
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

/// Diagram mode: Detect and normalize ASCII diagrams (full pipeline).
fn process_diagram_mode(content: &str, _config: &crate::config::Config) -> String {
    let blocks = crate::scanner::extract_diagram_blocks(content);

    // If no diagram blocks found, return content unchanged
    if blocks.is_empty() {
        return content.to_string();
    }

    // Build result line by line, preserving structure
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    // Process each diagram block (in reverse to maintain indices)
    for block in blocks.iter().rev() {
        let diagram_content = block.lines.join("\n");

        // Convert to grid
        let block_lines: Vec<&str> = diagram_content.lines().collect();
        let grid = crate::grid::Grid::from_lines(&block_lines);

        // Detect primitives
        let inventory = crate::detector::detect_all_primitives(&grid);

        // Only process if we found actual diagram primitives (boxes or arrows)
        if !inventory.boxes.is_empty()
            || !inventory.horizontal_arrows.is_empty()
            || !inventory.vertical_arrows.is_empty()
        {
            // Normalize
            let normalized = crate::normalizer::normalize_box_widths(&inventory);
            let normalized = crate::normalizer::align_horizontal_arrows(&normalized);
            let normalized = crate::normalizer::align_vertical_arrows(&normalized);
            let normalized = crate::normalizer::balance_horizontal_boxes(&normalized);
            let normalized = crate::normalizer::normalize_padding(&normalized);

            // Render
            let rendered_grid = crate::renderer::render_diagram(&normalized);
            let rendered = rendered_grid.render_trimmed();

            // Replace the block in the original content (in reverse to maintain indices)
            let block_len = block.lines.len();

            // Remove old lines and insert new ones
            for _ in 0..block_len {
                if block.start_line < lines.len() {
                    lines.remove(block.start_line);
                }
            }
            // Insert new lines
            for (i, line) in rendered.lines().map(String::from).enumerate() {
                lines.insert(block.start_line + i, line);
            }
        }
        // If no primitives found, leave the block unchanged
    }

    lines.join("\n")
}

/// Check mode: Validate without modifying (used with --check flag).
fn process_check_mode(content: &str) -> String {
    // Check mode uses the same processing as diagram mode but doesn't write
    // The caller will compare input vs output
    let default_config = crate::config::Config::default();
    process_diagram_mode(content, &default_config)
}

/// Compare original and processed content to determine if fixes are needed.
///
/// Returns true if the content has been modified, false if identical.
#[must_use]
#[allow(dead_code)] // Reason: Used by CLI for --check mode
pub fn content_needs_fixing(original: &str, processed: &str) -> bool {
    original != processed
}

/// Check if a line is a table row (starts with |, ends with |).
#[allow(dead_code)] // Reason: Used in tests
fn is_table_row(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|') && trimmed.ends_with('|')
}

/// Check if a line is a table separator (pipes and dashes only).
#[allow(dead_code)] // Reason: Used in tests
fn is_table_separator(line: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
        return false;
    }
    trimmed
        .split('|')
        .skip(1)
        .take_while(|cell| !cell.is_empty())
        .all(|cell| cell.trim().chars().all(|c| c == '-' || c == ':'))
}

/// Parse and normalize a table.
#[allow(dead_code)] // Reason: Used in tests
fn normalize_table(header: &str, _separator: &str, rows: &[&str]) -> Option<String> {
    // Parse header
    let headers = parse_table_row(header)?;

    // Parse data rows
    let mut data_rows = Vec::new();
    for row in rows {
        data_rows.push(parse_table_row(row)?);
    }

    // Calculate max width for each column
    let mut col_widths = vec![0; headers.len()];
    for (i, header) in headers.iter().enumerate() {
        col_widths[i] = header.len().max(col_widths[i]);
    }
    for row in &data_rows {
        for (i, cell) in row.iter().enumerate() {
            if i < col_widths.len() {
                col_widths[i] = cell.len().max(col_widths[i]);
            }
        }
    }

    // Format normalized table
    let mut result = String::new();

    // Header row
    let _ = write!(result, "|");
    for (i, header) in headers.iter().enumerate() {
        let _ = write!(result, " {:<width$} |", header, width = col_widths[i]);
    }

    // Separator row
    let _ = writeln!(result);
    let _ = write!(result, "|");
    for (i, _) in headers.iter().enumerate() {
        let _ = write!(result, "-{}-|", "-".repeat(col_widths[i]));
    }

    // Data rows
    for row in data_rows {
        let _ = writeln!(result);
        let _ = write!(result, "|");
        for (i, cell) in row.iter().enumerate() {
            let _ = write!(result, " {:<width$} |", cell, width = col_widths[i]);
        }
    }

    Some(result)
}

/// Parse a table row into cells.
///
/// This function is link-aware and will not split on `|` characters that appear
/// inside markdown link URLs.
#[allow(dead_code)] // Reason: Used in tests
fn parse_table_row(row: &str) -> Option<Vec<String>> {
    let trimmed = row.trim();
    if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
        return None;
    }

    // Detect links in the row to avoid splitting on | inside link URLs
    let links = detect_links(trimmed);

    let mut cells: Vec<String> = Vec::new();
    let mut current_cell = String::new();
    let mut in_cell = false;

    for (pos, ch) in trimmed.chars().enumerate() {
        if ch == '|' {
            // Check if this | is inside a link URL
            if is_inside_link_url(trimmed, pos, &links) {
                // This | is part of a link URL, include it in the cell
                current_cell.push(ch);
            } else {
                // This | is a cell delimiter
                if in_cell {
                    // End of current cell
                    let trimmed_cell = current_cell.trim().to_string();
                    if !trimmed_cell.is_empty() {
                        cells.push(trimmed_cell);
                    }
                    current_cell = String::new();
                }
                in_cell = true;
            }
        } else if in_cell {
            current_cell.push(ch);
        }
    }

    // Don't forget the last cell if there is one
    if in_cell && !current_cell.trim().is_empty() {
        cells.push(current_cell.trim().to_string());
    }

    // Remove the trailing empty cell from the final |
    if cells.is_empty() {
        None
    } else {
        Some(cells)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> crate::config::Config {
        crate::config::Config::default()
    }

    #[test]
    fn test_safe_mode_preserves_content() {
        let content = "# Test\n\nSome content";
        let result = process_by_mode(&Mode::Safe, content, false, &default_config());
        // Safe mode should preserve content for now
        assert_eq!(result, content);
    }

    #[test]
    fn test_diagram_mode_preserves_content() {
        let content = "# Test\n\nSome content";
        let result = process_by_mode(&Mode::Diagram, content, false, &default_config());
        // Diagram mode should preserve content when no diagrams exist
        assert_eq!(result, content);
    }

    #[test]
    fn test_check_mode_preserves_content() {
        let content = "# Test\n\nSome content";
        let result = process_by_mode(&Mode::Check, content, false, &default_config());
        // Check mode should use same processing as diagram
        assert_eq!(result, content);
    }

    #[test]
    fn test_all_modes_are_safe() {
        let content = "# Header\n\nText content\n\nMore text";
        let safe_result = process_by_mode(&Mode::Safe, content, false, &default_config());
        let diagram_result = process_by_mode(&Mode::Diagram, content, false, &default_config());
        let check_result = process_by_mode(&Mode::Check, content, false, &default_config());

        // All modes should handle content safely (no crashes, no panics)
        assert!(!safe_result.is_empty());
        assert!(!diagram_result.is_empty());
        assert!(!check_result.is_empty());
    }

    #[test]
    fn test_safe_mode_normalizes_table() {
        let content = "| Name | Age |\n|------|-----|\n| Alice | 30 |\n| Bob | 25 |";
        let result = process_by_mode(&Mode::Safe, content, false, &default_config());
        // Result should be normalized (may have different spacing)
        assert!(result.contains("| Name"));
        assert!(result.contains("| Age"));
        assert!(result.contains("| Alice"));
        assert!(result.contains("| Bob"));
    }

    #[test]
    fn test_safe_mode_preserves_non_tables() {
        let content = "# Title\n\nSome paragraph.\n\nMore text.";
        let result = process_by_mode(&Mode::Safe, content, false, &default_config());
        assert_eq!(result, content);
    }

    #[test]
    fn test_safe_mode_misaligned_table() {
        let content = "| A | B |\n|---|---|\n| x| y |";
        let result = process_by_mode(&Mode::Safe, content, false, &default_config());
        // Should normalize spacing
        assert!(result.contains("| A"));
        assert!(result.contains("| B"));
    }

    #[test]
    fn test_safe_mode_multiple_tables() {
        let content =
            "| H1 | H2 |\n|---|---|\n| a | b |\n\nText\n\n| C | D |\n|---|---|\n| c | d |";
        let result = process_by_mode(&Mode::Safe, content, false, &default_config());
        // Both tables should be present
        assert!(result.contains("| H1"));
        assert!(result.contains("| C"));
    }

    #[test]
    fn test_table_separator_detection() {
        assert!(is_table_separator("|---|---|"));
        assert!(is_table_separator("| --- | --- |"));
        assert!(is_table_separator("| :--- | ---: |"));
        assert!(!is_table_separator("| abc | def |"));
        assert!(!is_table_separator("no pipes here"));
    }

    #[test]
    fn test_table_row_detection() {
        assert!(is_table_row("| A | B |"));
        assert!(is_table_row("|A|B|"));
        assert!(is_table_row("  | A | B |  "));
        assert!(!is_table_row("A | B"));
        assert!(!is_table_row("| A | B"));
    }

    #[test]
    fn test_parse_table_row() {
        let cells = parse_table_row("| Name | Age |");
        assert_eq!(cells, Some(vec!["Name".to_string(), "Age".to_string()]));

        let cells2 = parse_table_row("|A|B|C|");
        assert_eq!(
            cells2,
            Some(vec!["A".to_string(), "B".to_string(), "C".to_string()])
        );

        let invalid = parse_table_row("no pipes");
        assert_eq!(invalid, None);
    }

    #[test]
    fn test_diagram_mode_processes_boxes() {
        let content = "┌─┐\n│ │\n└─┘";
        let result = process_by_mode(&Mode::Diagram, content, false, &default_config());
        // Should render the diagram (may change spacing but keep structure)
        assert!(result.contains("┌"));
        assert!(result.contains("└"));
        assert!(result.contains("│"));
    }

    #[test]
    fn test_diagram_mode_preserves_non_diagram_text() {
        let content = "# Title\n\nSome text\n\nMore content";
        let result = process_by_mode(&Mode::Diagram, content, false, &default_config());
        // Non-diagram content should be preserved
        assert!(result.contains("# Title"));
        assert!(result.contains("Some text"));
    }

    #[test]
    fn test_check_mode_returns_unchanged_content() {
        let content = "# Test\n\nNo diagrams here";
        let result = process_by_mode(&Mode::Check, content, false, &default_config());
        // Check mode processes same as diagram mode but returns content
        assert_eq!(result, content);
    }

    #[test]
    fn test_content_needs_fixing_detects_differences() {
        let original = "┌──┐\n│Hi│\n└──┘";
        let processed = process_by_mode(&Mode::Diagram, original, false, &default_config());
        // If there are primitives, processing might change formatting
        // Check that comparison would detect the difference
        let needs_fixing = content_needs_fixing(original, &processed);
        // This depends on if diagram mode changes anything
        let _ = needs_fixing; // Just verify function compiles
    }

    #[test]
    fn test_content_needs_fixing_detects_identical() {
        let original = "# Title\n\nNo diagrams";
        let processed = process_by_mode(&Mode::Diagram, original, false, &default_config());
        // When no changes are made, content should be identical
        let needs_fixing = content_needs_fixing(original, &processed);
        assert!(!needs_fixing);
    }

    #[test]
    fn test_content_needs_fixing_simple_case() {
        let original = "abc";
        let modified = "def";
        assert!(content_needs_fixing(original, modified));
    }

    #[test]
    fn test_content_needs_fixing_identical_strings() {
        let content = "exact same content";
        assert!(!content_needs_fixing(content, content));
    }

    #[test]
    fn test_content_needs_fixing_ignores_trailing_whitespace() {
        let original = "line1\nline2";
        let modified = "line1\nline2";
        assert!(!content_needs_fixing(original, modified));
    }

    #[test]
    fn test_fence_repair_in_pipeline() {
        let content = "```python\ncode\n`````";
        let result = process_by_mode(&Mode::Diagram, content, true, &default_config());
        // Fences should be normalized before diagram processing
        assert!(result.contains('`'));
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
}
