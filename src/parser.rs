//! Markdown parsing for detecting and extracting diagram blocks.

/// Represents a line of Markdown and whether it's inside a code fence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Reason: Useful type for semantic clarity, used in tests
enum LineContext {
    /// Outside any code fence
    Normal,
    /// Inside a code fence
    InCodeFence,
}

/// Parse Markdown content and return lines with their context (inside/outside code fences).
#[allow(dead_code)] // Reason: Used by extract_normal_lines and tests
fn parse_line_contexts(text: &str) -> Vec<(usize, &str, LineContext)> {
    let mut result = Vec::new();
    let mut in_fence = false;

    for (line_num, line) in text.lines().enumerate() {
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

/// Extract all normal (non-code-fence) lines from Markdown content.
#[allow(dead_code)] // Reason: Used by main processing pipeline
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
}
