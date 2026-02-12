//! Inline code detection and protection for Markdown content.
//!
//! This module detects inline code spans (backtick pairs) and provides masking/restoration
//! functionality to protect them from diagram processing.

/// Represents an inline code span within a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineCodeSpan {
    /// Opening backtick position
    pub start_col: usize,
    /// Closing backtick position (inclusive)
    pub end_col: usize,
    /// Original content between backticks (including the backticks)
    pub content: String,
}

/// Detect all inline code spans in a line.
///
/// Scans the line for backtick pairs and extracts inline code spans.
/// Handles escaped backticks and unbalanced cases conservatively.
#[must_use]
pub fn detect_inline_code_spans(line: &str) -> Vec<InlineCodeSpan> {
    let mut spans = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Look for opening backtick
        if chars[i] == '`' {
            // Check if it's escaped
            if i > 0 && chars[i - 1] == '\\' {
                i += 1;
                continue;
            }

            let start_col = i;

            // Find closing backtick
            i += 1;
            while i < chars.len() {
                if chars[i] == '`' {
                    // Check if it's escaped
                    if i > 0 && chars[i - 1] == '\\' {
                        i += 1;
                        continue;
                    }

                    // Found closing backtick
                    let end_col = i;
                    let content: String = chars[start_col..=end_col].iter().collect();
                    spans.push(InlineCodeSpan {
                        start_col,
                        end_col,
                        content,
                    });
                    i += 1;
                    break;
                }
                i += 1;
            }

            // If we reached end without closing backtick, treat as unbalanced
            // (don't add to spans - conservative approach)
        } else {
            i += 1;
        }
    }

    spans
}

/// Mask inline code by replacing characters with spaces.
///
/// Returns the masked line and the spans that were masked.
/// Spaces are used to preserve column positions for grid-based processing.
#[must_use]
pub fn mask_inline_code(line: &str) -> (String, Vec<InlineCodeSpan>) {
    let spans = detect_inline_code_spans(line);

    if spans.is_empty() {
        return (line.to_string(), spans);
    }

    // Replace characters in spans with spaces (in reverse to maintain indices)
    let mut masked_chars: Vec<char> = line.chars().collect();
    for span in spans.iter().rev() {
        for j in span.start_col..=span.end_col {
            if j < masked_chars.len() {
                masked_chars[j] = ' ';
            }
        }
    }

    let masked = masked_chars.iter().collect::<String>();
    (masked, spans)
}

/// Restore masked inline code content.
///
/// Takes a masked line and inline code spans, restores the original content
/// from the spans back into the line.
#[must_use]
pub fn restore_inline_code(masked_line: &str, spans: &[InlineCodeSpan]) -> String {
    if spans.is_empty() {
        return masked_line.to_string();
    }

    let mut restored_chars: Vec<char> = masked_line.chars().collect();

    // Restore each span (safe to do in any order since spans don't overlap)
    for span in spans {
        let content_chars: Vec<char> = span.content.chars().collect();
        for (i, ch) in content_chars.iter().enumerate() {
            let pos = span.start_col + i;
            if pos < restored_chars.len() {
                restored_chars[pos] = *ch;
            }
        }
    }

    restored_chars.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_single_inline_code() {
        let line = "Some text `code` here";
        let spans = detect_inline_code_spans(line);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "`code`");
    }

    #[test]
    fn test_detect_multiple_inline_codes() {
        let line = "`first` and `second` text";
        let spans = detect_inline_code_spans(line);
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].content, "`first`");
        assert_eq!(spans[1].content, "`second`");
    }

    #[test]
    fn test_detect_inline_code_with_arrows() {
        let line = "Use these arrows: `⇒ ⇓ ⇑ ⇐`";
        let spans = detect_inline_code_spans(line);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "`⇒ ⇓ ⇑ ⇐`");
    }

    #[test]
    fn test_detect_inline_code_with_box_drawing() {
        let line = "Box chars: `┌─┐`";
        let spans = detect_inline_code_spans(line);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "`┌─┐`");
    }

    #[test]
    fn test_escaped_backticks_not_code() {
        let line = "Escaped: \\` not code";
        let spans = detect_inline_code_spans(line);
        assert_eq!(spans.len(), 0);
    }

    #[test]
    fn test_unbalanced_backticks() {
        let line = "Unbalanced `code without closing";
        let spans = detect_inline_code_spans(line);
        // Unbalanced cases should not be added (conservative)
        assert_eq!(spans.len(), 0);
    }

    #[test]
    fn test_empty_inline_code() {
        let line = "Empty: `` code";
        let spans = detect_inline_code_spans(line);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "``");
    }

    #[test]
    fn test_mask_inline_code() {
        let line = "Text `code` here";
        let (masked, spans) = mask_inline_code(line);
        assert_eq!(spans.len(), 1);
        // Verify masked line has spaces where code was
        assert!(masked.contains("     "));
        // Verify content is preserved in spans
        assert_eq!(spans[0].content, "`code`");
    }

    #[test]
    fn test_mask_preserves_column_positions() {
        let line = "A `bc` D";
        let (masked, _) = mask_inline_code(line);
        // Positions should be preserved
        assert_eq!(masked.len(), line.len());
        // Check specific characters
        assert_eq!(masked.chars().nth(0), Some('A'));
        assert_eq!(masked.chars().nth(2), Some(' ')); // Opening backtick replaced
        assert_eq!(masked.chars().nth(6), Some(' ')); // Closing backtick replaced
        assert_eq!(masked.chars().nth(7), Some('D'));
    }

    #[test]
    fn test_restore_inline_code() {
        let line = "Text `code` here";
        let (masked, spans) = mask_inline_code(line);
        let restored = restore_inline_code(&masked, &spans);
        assert_eq!(restored, line);
    }

    #[test]
    fn test_mask_and_restore_cycle() {
        let line = "Use `⇒ ⇓ ⇑ ⇐` arrows";
        let (masked, spans) = mask_inline_code(line);
        let restored = restore_inline_code(&masked, &spans);
        assert_eq!(restored, line);
    }

    #[test]
    fn test_multiple_spans_mask_restore() {
        let line = "`first` and `second` code";
        let (masked, spans) = mask_inline_code(line);
        assert_eq!(spans.len(), 2);
        let restored = restore_inline_code(&masked, &spans);
        assert_eq!(restored, line);
    }

    #[test]
    fn test_restore_without_spans() {
        let line = "Plain text without code";
        let (masked, spans) = mask_inline_code(line);
        assert_eq!(spans.len(), 0);
        let restored = restore_inline_code(&masked, &spans);
        assert_eq!(restored, line);
    }

    #[test]
    fn test_inline_code_with_unicode_escapes() {
        let line = "Extended: `⟶ ⟹ ⟸` arrows";
        let spans = detect_inline_code_spans(line);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "`⟶ ⟹ ⟸`");
        let (masked, masking_spans) = mask_inline_code(line);
        let restored = restore_inline_code(&masked, &masking_spans);
        assert_eq!(restored, line);
    }

    #[test]
    fn test_spans_include_backticks() {
        let line = "Code: `hello`";
        let spans = detect_inline_code_spans(line);
        assert_eq!(spans.len(), 1);
        // Spans should include the backticks
        assert_eq!(spans[0].content, "`hello`");
        assert!(spans[0].content.starts_with('`'));
        assert!(spans[0].content.ends_with('`'));
    }

    #[test]
    fn test_span_positions_accurate() {
        let line = "Start `code` end";
        let spans = detect_inline_code_spans(line);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].start_col, 6);
        assert_eq!(spans[0].end_col, 11);
        // Extract the substring to verify positions
        let line_chars: Vec<char> = line.chars().collect();
        let extracted: String = line_chars[spans[0].start_col..=spans[0].end_col]
            .iter()
            .collect();
        assert_eq!(extracted, "`code`");
    }
}
