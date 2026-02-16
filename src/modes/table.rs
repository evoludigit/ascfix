//! Table utilities for Markdown table normalization.
//!
//! This module provides functions for detecting, parsing, and normalizing Markdown tables
//! in pipe-delimited format. It handles edge cases like:
//!
//! - **Link-aware parsing**: Doesn't split on `|` characters inside markdown link URLs
//! - **Column alignment**: Normalizes spacing while preserving column structure
//! - **Empty cells**: Maintains empty cells to preserve table structure
//!
//! ## Example
//!
//! ```rust,ignore
//! // Internal module - example for illustration
//! let header = "| Name | Age |";
//! let separator = "|------|-----|";
//! let rows = vec!["| Alice | 30 |", "| Bob | 25 |"];
//!
//! if let Some(normalized) = normalize_table(header, separator, &rows) {
//!     // Returns a normalized table with consistent spacing
//! }
//! ```
//!
//! ## Table Detection
//!
//! A valid Markdown table must have:
//! 1. A header row (starts and ends with `|`)
//! 2. A separator row (contains only `-`, `:`, and `|` characters)
//! 3. Zero or more data rows

use crate::links::{detect_links, is_inside_link_url};
use std::fmt::Write;

/// Check if a line is a table row (starts with |, ends with |).
#[allow(dead_code)] // Reason: Used in tests
pub fn is_table_row(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|') && trimmed.ends_with('|')
}

/// Check if a line is a table separator (pipes and dashes only).
#[allow(dead_code)] // Reason: Used in tests
pub fn is_table_separator(line: &str) -> bool {
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

/// Parse a table row into cells.
///
/// This function is link-aware and will not split on `|` characters that appear
/// inside markdown link URLs.
#[allow(dead_code)] // Reason: Used in tests
pub fn parse_table_row(row: &str) -> Option<Vec<String>> {
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
                    // End of current cell - preserve empty cells to maintain column structure
                    cells.push(current_cell.trim().to_string());
                    current_cell = String::new();
                }
                in_cell = true;
            }
        } else if in_cell {
            current_cell.push(ch);
        }
    }

    // The final | creates a trailing empty element that we should skip
    // (but keep intentional empty cells in the middle of the row)

    if cells.is_empty() {
        None
    } else {
        Some(cells)
    }
}

/// Parse and normalize a table.
#[allow(dead_code)] // Reason: Used in tests
pub fn normalize_table(header: &str, _separator: &str, rows: &[&str]) -> Option<String> {
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
            if i < col_widths.len() {
                let _ = write!(result, " {:<width$} |", cell, width = col_widths[i]);
            }
            // Extra cells beyond header count are silently truncated
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_module_exports() {
        // Verify table functions accessible
        assert!(is_table_row("| A | B |"));
        assert!(is_table_separator("|---|---|"));
        let cells = parse_table_row("| Name | Age |");
        assert_eq!(cells, Some(vec!["Name".to_string(), "Age".to_string()]));
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
    fn test_normalize_table() {
        let header = "| Name | Age |";
        let separator = "|------|-----|";
        let rows = vec!["| Alice | 30 |", "| Bob | 25 |"];
        let result = normalize_table(header, separator, &rows);
        assert!(result.is_some());
        let normalized = result.unwrap();
        assert!(normalized.contains("Alice"));
        assert!(normalized.contains("Bob"));
    }
}
