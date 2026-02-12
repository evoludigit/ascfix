//! Markdown parsing for detecting and extracting diagram blocks.

/// Represents a line of Markdown and whether it's inside a code fence or ignore block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Reason: Useful type for semantic clarity, used in tests
enum LineContext {
    /// Outside any code fence or ignore block
    Normal,
    /// Inside a code fence
    InCodeFence,
    /// Inside an ignore block (between ascfix:ignore markers)
    InIgnoreBlock,
}

/// Check if a line contains an ascfix ignore start marker.
///
/// Supports:
/// - `<!-- ascfix:ignore -->`
/// - `<!-- ascfix-ignore-start -->`
#[allow(dead_code)] // Reason: Used by parse_line_contexts
fn is_ignore_start(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.contains("<!-- ascfix:ignore -->") || trimmed.contains("<!-- ascfix-ignore-start -->")
}

/// Check if a line contains an ascfix ignore end marker.
///
/// Supports:
/// - `<!-- /ascfix:ignore -->`
/// - `<!-- ascfix-ignore-end -->`
#[allow(dead_code)] // Reason: Used by parse_line_contexts
fn is_ignore_end(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.contains("<!-- /ascfix:ignore -->") || trimmed.contains("<!-- ascfix-ignore-end -->")
}

/// Parse Markdown content and return lines with their context (inside/outside code fences and ignore blocks).
#[allow(dead_code)] // Reason: Used by extract_normal_lines and tests
fn parse_line_contexts(text: &str) -> Vec<(usize, &str, LineContext)> {
    let mut result = Vec::new();
    let mut in_fence = false;
    let mut in_ignore = false;

    for (line_num, line) in text.lines().enumerate() {
        // Check for ignore markers first (they take precedence)
        if is_ignore_start(line) {
            in_ignore = true;
            // Ignore marker lines are skipped entirely
            continue;
        } else if is_ignore_end(line) {
            in_ignore = false;
            // Ignore marker lines are skipped entirely
            continue;
        }

        // If we're in an ignore block, mark all lines as ignored
        if in_ignore {
            result.push((line_num, line, LineContext::InIgnoreBlock));
            continue;
        }

        // Count code fence markers (``` or ~~~)
        let backtick_count = line.matches("```").count();
        let tilde_count = line.matches("~~~").count();
        let is_fence_marker = backtick_count > 0 || tilde_count > 0;

        // Toggle fence state if we encounter an odd number of markers
        if backtick_count % 2 == 1 || tilde_count % 2 == 1 {
            in_fence = !in_fence;
        }

        // Determine context: fence markers are always skipped, interior lines stay in fence
        let line_ctx = if is_fence_marker {
            // Fence marker lines are skipped entirely
            continue;
        } else if in_fence {
            LineContext::InCodeFence
        } else {
            LineContext::Normal
        };

        result.push((line_num, line, line_ctx));
    }

    result
}

/// Extract all normal (non-code-fence, non-ignored) lines from Markdown content.
///
/// Lines inside code fences and ignore blocks are filtered out.
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[must_use]
pub fn extract_normal_lines(text: &str) -> Vec<(usize, String)> {
    parse_line_contexts(text)
        .into_iter()
        .filter_map(|(line_num, line, line_ctx)| {
            if line_ctx == LineContext::Normal {
                Some((line_num, line.to_string()))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignores_content_in_backtick_fence() {
        let markdown = "# Header\n\n```\ncode line\nmore code\n```\n\nAfter fence";
        let normal_lines = extract_normal_lines(markdown);

        let line_texts: Vec<&str> = normal_lines.iter().map(|(_, l)| l.as_str()).collect();
        assert!(!line_texts.contains(&"code line"));
        assert!(!line_texts.contains(&"more code"));
        assert!(line_texts.contains(&"# Header"));
        assert!(line_texts.contains(&"After fence"));
    }

    #[test]
    fn test_ignores_content_in_tilde_fence() {
        let markdown = "Before\n\n~~~\nfenced content\n~~~\n\nAfter";
        let normal_lines = extract_normal_lines(markdown);

        let line_texts: Vec<&str> = normal_lines.iter().map(|(_, l)| l.as_str()).collect();
        assert!(!line_texts.contains(&"fenced content"));
        assert!(line_texts.contains(&"Before"));
        assert!(line_texts.contains(&"After"));
    }

    #[test]
    fn test_preserves_line_numbers() {
        let markdown = "Line 0\n\n```\nLine 3 (hidden)\n```\n\nLine 6";
        let normal_lines = extract_normal_lines(markdown);

        let line_nums: Vec<usize> = normal_lines.iter().map(|(num, _)| *num).collect();
        // Lines 2 and 4 are fence markers and are skipped
        assert_eq!(line_nums, vec![0, 1, 5, 6]);
    }

    #[test]
    fn test_nested_triple_backticks_in_content() {
        // Triple backticks in text should toggle fence state
        let markdown = "text\n```\nhidden\n```\nvisible";
        let normal_lines = extract_normal_lines(markdown);

        let line_texts: Vec<&str> = normal_lines.iter().map(|(_, l)| l.as_str()).collect();
        assert!(line_texts.contains(&"text"));
        assert!(line_texts.contains(&"visible"));
        assert!(!line_texts.contains(&"hidden"));
    }

    #[test]
    fn test_multiple_code_blocks() {
        let markdown = "Before\n\n```\nBlock1\n```\n\nMiddle\n\n```\nBlock2\n```\n\nAfter";
        let normal_lines = extract_normal_lines(markdown);

        let line_texts: Vec<&str> = normal_lines.iter().map(|(_, l)| l.as_str()).collect();
        assert!(!line_texts.contains(&"Block1"));
        assert!(!line_texts.contains(&"Block2"));
        assert!(line_texts.contains(&"Before"));
        assert!(line_texts.contains(&"Middle"));
        assert!(line_texts.contains(&"After"));
    }

    #[test]
    fn test_empty_markdown() {
        let markdown = "";
        let normal_lines = extract_normal_lines(markdown);
        assert!(normal_lines.is_empty());
    }

    #[test]
    fn test_no_code_blocks() {
        let markdown = "# Header\n\nSome text\nMore text";
        let normal_lines = extract_normal_lines(markdown);
        assert_eq!(normal_lines.len(), 4);
    }

    #[test]
    fn test_ignore_block_basic() {
        let markdown = "Before\n<!-- ascfix:ignore -->\nIgnored content\n<!-- /ascfix:ignore -->\nAfter";
        let normal_lines = extract_normal_lines(markdown);

        let line_texts: Vec<&str> = normal_lines.iter().map(|(_, l)| l.as_str()).collect();
        assert!(line_texts.contains(&"Before"));
        assert!(line_texts.contains(&"After"));
        assert!(!line_texts.contains(&"Ignored content"));
    }

    #[test]
    fn test_ignore_block_alternative_syntax() {
        let markdown =
            "Before\n<!-- ascfix-ignore-start -->\nIgnored\n<!-- ascfix-ignore-end -->\nAfter";
        let normal_lines = extract_normal_lines(markdown);

        let line_texts: Vec<&str> = normal_lines.iter().map(|(_, l)| l.as_str()).collect();
        assert!(line_texts.contains(&"Before"));
        assert!(line_texts.contains(&"After"));
        assert!(!line_texts.contains(&"Ignored"));
    }

    #[test]
    fn test_ignore_block_multiple() {
        let markdown = "Text1\n<!-- ascfix:ignore -->\nIgnored1\n<!-- /ascfix:ignore -->\nText2\n<!-- ascfix:ignore -->\nIgnored2\n<!-- /ascfix:ignore -->\nText3";
        let normal_lines = extract_normal_lines(markdown);

        let line_texts: Vec<&str> = normal_lines.iter().map(|(_, l)| l.as_str()).collect();
        assert!(line_texts.contains(&"Text1"));
        assert!(line_texts.contains(&"Text2"));
        assert!(line_texts.contains(&"Text3"));
        assert!(!line_texts.contains(&"Ignored1"));
        assert!(!line_texts.contains(&"Ignored2"));
    }

    #[test]
    fn test_ignore_block_multiline() {
        let markdown = "Before\n<!-- ascfix:ignore -->\nLine1\nLine2\nLine3\n<!-- /ascfix:ignore -->\nAfter";
        let normal_lines = extract_normal_lines(markdown);

        let line_texts: Vec<&str> = normal_lines.iter().map(|(_, l)| l.as_str()).collect();
        assert_eq!(line_texts.len(), 2);
        assert!(line_texts.contains(&"Before"));
        assert!(line_texts.contains(&"After"));
    }

    #[test]
    fn test_ignore_block_with_code_fence() {
        let markdown = "Text\n<!-- ascfix:ignore -->\n```\ncode\n```\n<!-- /ascfix:ignore -->\nAfter";
        let normal_lines = extract_normal_lines(markdown);

        let line_texts: Vec<&str> = normal_lines.iter().map(|(_, l)| l.as_str()).collect();
        assert!(line_texts.contains(&"Text"));
        assert!(line_texts.contains(&"After"));
        // Everything in ignore block should be filtered
        assert!(!line_texts.contains(&"code"));
    }

    #[test]
    fn test_code_fence_with_ignore_markers_inside() {
        let markdown =
            "Before\n```\n<!-- ascfix:ignore -->\ncode\n<!-- /ascfix:ignore -->\n```\nAfter";
        let normal_lines = extract_normal_lines(markdown);

        let line_texts: Vec<&str> = normal_lines.iter().map(|(_, l)| l.as_str()).collect();
        assert!(line_texts.contains(&"Before"));
        assert!(line_texts.contains(&"After"));
        // Code fence content is filtered, including fake ignore markers
        assert!(!line_texts.contains(&"code"));
        assert!(!line_texts.contains(&"<!-- ascfix:ignore -->"));
    }

    #[test]
    fn test_ignore_start_detection() {
        assert!(is_ignore_start("<!-- ascfix:ignore -->"));
        assert!(is_ignore_start("  <!-- ascfix:ignore -->  "));
        assert!(is_ignore_start("<!-- ascfix-ignore-start -->"));
        assert!(is_ignore_start("  <!-- ascfix-ignore-start -->  "));
        assert!(!is_ignore_start("<!-- /ascfix:ignore -->"));
        assert!(!is_ignore_start("<!-- ascfix-ignore-end -->"));
        assert!(!is_ignore_start("regular text"));
    }

    #[test]
    fn test_ignore_end_detection() {
        assert!(is_ignore_end("<!-- /ascfix:ignore -->"));
        assert!(is_ignore_end("  <!-- /ascfix:ignore -->  "));
        assert!(is_ignore_end("<!-- ascfix-ignore-end -->"));
        assert!(is_ignore_end("  <!-- ascfix-ignore-end -->  "));
        assert!(!is_ignore_end("<!-- ascfix:ignore -->"));
        assert!(!is_ignore_end("<!-- ascfix-ignore-start -->"));
        assert!(!is_ignore_end("regular text"));
    }

    #[test]
    fn test_ignore_block_preserves_line_numbers() {
        let markdown = "Line0\n<!-- ascfix:ignore -->\nLine2\nLine3\n<!-- /ascfix:ignore -->\nLine5";
        let normal_lines = extract_normal_lines(markdown);

        let line_nums: Vec<usize> = normal_lines.iter().map(|(num, _)| *num).collect();
        // Line 0 and Line 5 should be present
        assert_eq!(line_nums, vec![0, 5]);
    }

    #[test]
    fn test_unclosed_ignore_block() {
        // If an ignore block is not closed, everything after it should be ignored
        let markdown = "Before\n<!-- ascfix:ignore -->\nIgnored1\nIgnored2\nAlso ignored";
        let normal_lines = extract_normal_lines(markdown);

        let line_texts: Vec<&str> = normal_lines.iter().map(|(_, l)| l.as_str()).collect();
        assert_eq!(line_texts.len(), 1);
        assert!(line_texts.contains(&"Before"));
    }
}
