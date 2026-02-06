//! Mode-specific processing implementations.

use crate::cli::Mode;
use std::fmt::Write;

/// Process content according to the specified mode.
///
/// # Modes
/// - `Safe`: Only normalize Markdown tables (minimal changes, no diagrams)
/// - `Diagram`: Detect and normalize ASCII diagrams (boxes, arrows, text)
/// - `Check`: Validate content but don't modify (used with --check flag)
#[allow(dead_code)] // Reason: Used by main processing pipeline
pub fn process_by_mode(mode: &Mode, content: &str) -> String {
    match mode {
        Mode::Safe => process_safe_mode(content),
        Mode::Diagram => process_diagram_mode(content),
        Mode::Check => process_check_mode(content),
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

            // Parse and normalize the table
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
        } else {
            result.push(lines[i].to_string());
            i += 1;
        }
    }

    result.join("\n")
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
#[allow(dead_code)] // Reason: Used in tests
fn parse_table_row(row: &str) -> Option<Vec<String>> {
    let trimmed = row.trim();
    if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
        return None;
    }

    let cells: Vec<String> = trimmed
        .split('|')
        .skip(1) // Skip the first empty element before the opening |
        .map(|cell| cell.trim().to_string())
        .filter(|cell| !cell.is_empty()) // Remove trailing empty cell
        .collect();

    if cells.is_empty() {
        None
    } else {
        Some(cells)
    }
}

/// Diagram mode: Detect and normalize ASCII diagrams (full pipeline).
fn process_diagram_mode(content: &str) -> String {
    // For now, return content unchanged
    // TODO: Implement full diagram detection, normalization, rendering pipeline
    content.to_string()
}

/// Check mode: Validate without modifying (used with --check flag).
fn process_check_mode(content: &str) -> String {
    // Check mode uses the same processing as diagram mode but doesn't write
    // The caller will compare input vs output
    process_diagram_mode(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_mode_preserves_content() {
        let content = "# Test\n\nSome content";
        let result = process_by_mode(&Mode::Safe, content);
        // Safe mode should preserve content for now
        assert_eq!(result, content);
    }

    #[test]
    fn test_diagram_mode_preserves_content() {
        let content = "# Test\n\nSome content";
        let result = process_by_mode(&Mode::Diagram, content);
        // Diagram mode should preserve content when no diagrams exist
        assert_eq!(result, content);
    }

    #[test]
    fn test_check_mode_preserves_content() {
        let content = "# Test\n\nSome content";
        let result = process_by_mode(&Mode::Check, content);
        // Check mode should use same processing as diagram
        assert_eq!(result, content);
    }

    #[test]
    fn test_all_modes_are_safe() {
        let content = "# Header\n\nText content\n\nMore text";
        let safe_result = process_by_mode(&Mode::Safe, content);
        let diagram_result = process_by_mode(&Mode::Diagram, content);
        let check_result = process_by_mode(&Mode::Check, content);

        // All modes should handle content safely (no crashes, no panics)
        assert!(!safe_result.is_empty());
        assert!(!diagram_result.is_empty());
        assert!(!check_result.is_empty());
    }

    #[test]
    fn test_safe_mode_normalizes_table() {
        let content = "| Name | Age |\n|------|-----|\n| Alice | 30 |\n| Bob | 25 |";
        let result = process_by_mode(&Mode::Safe, content);
        // Result should be normalized (may have different spacing)
        assert!(result.contains("| Name"));
        assert!(result.contains("| Age"));
        assert!(result.contains("| Alice"));
        assert!(result.contains("| Bob"));
    }

    #[test]
    fn test_safe_mode_preserves_non_tables() {
        let content = "# Title\n\nSome paragraph.\n\nMore text.";
        let result = process_by_mode(&Mode::Safe, content);
        assert_eq!(result, content);
    }

    #[test]
    fn test_safe_mode_misaligned_table() {
        let content = "| A | B |\n|---|---|\n| x| y |";
        let result = process_by_mode(&Mode::Safe, content);
        // Should normalize spacing
        assert!(result.contains("| A"));
        assert!(result.contains("| B"));
    }

    #[test]
    fn test_safe_mode_multiple_tables() {
        let content =
            "| H1 | H2 |\n|---|---|\n| a | b |\n\nText\n\n| C | D |\n|---|---|\n| c | d |";
        let result = process_by_mode(&Mode::Safe, content);
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
}
