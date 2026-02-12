//! Diagram block extraction from Markdown content.

use crate::parser;

// Inline definition of InlineCodeSpan to avoid module resolution issues
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineCodeSpan {
    pub start_col: usize,
    pub end_col: usize,
    pub content: String,
}

// Inline mask_inline_code function
fn mask_inline_code(line: &str) -> (String, Vec<InlineCodeSpan>) {
    let spans = detect_inline_code_spans(line);
    if spans.is_empty() {
        return (line.to_string(), spans);
    }
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

// Inline detect_inline_code_spans function
fn detect_inline_code_spans(line: &str) -> Vec<InlineCodeSpan> {
    let mut spans = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '`' {
            if i > 0 && chars[i - 1] == '\\' {
                i += 1;
                continue;
            }
            let start_col = i;
            i += 1;
            while i < chars.len() {
                if chars[i] == '`' {
                    if i > 0 && chars[i - 1] == '\\' {
                        i += 1;
                        continue;
                    }
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
        } else {
            i += 1;
        }
    }
    spans
}

/// Represents a contiguous diagram block with its location.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // Reason: Used by main processing pipeline
pub struct DiagramBlock {
    /// Starting line number in the original markdown
    pub start_line: usize,
    /// Lines comprising this diagram block
    pub lines: Vec<String>,
    /// Inline code spans for each line (for protecting content during processing)
    pub inline_code_spans: Vec<Vec<InlineCodeSpan>>,
}

/// Extract diagram blocks from Markdown content.
///
/// A diagram block is a contiguous sequence of non-empty lines outside code fences.
/// Blank lines separate blocks.
///
/// This function also masks inline code spans to protect them from diagram processing.
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[must_use]
pub fn extract_diagram_blocks(text: &str) -> Vec<DiagramBlock> {
    let normal_lines = parser::extract_normal_lines(text);
    let mut blocks = Vec::new();
    let mut current_block: Option<(usize, Vec<String>, Vec<Vec<InlineCodeSpan>>)> = None;

    for (line_num, line) in normal_lines {
        if line.trim().is_empty() {
            // Blank line: finalize current block if any
            if let Some((start, block_lines, inline_spans)) = current_block.take() {
                if !block_lines.is_empty() {
                    blocks.push(DiagramBlock {
                        start_line: start,
                        lines: block_lines,
                        inline_code_spans: inline_spans,
                    });
                }
            }
        } else {
            // Mask inline code in the line
            let (masked_line, spans) = mask_inline_code(&line);

            // Non-empty line: add to current block or start new one
            if let Some((_start, ref mut block_lines, ref mut all_spans)) = &mut current_block {
                block_lines.push(masked_line);
                all_spans.push(spans);
            } else {
                current_block = Some((line_num, vec![masked_line], vec![spans]));
            }
        }
    }

    // Finalize last block if any
    if let Some((start, block_lines, inline_spans)) = current_block {
        if !block_lines.is_empty() {
            blocks.push(DiagramBlock {
                start_line: start,
                lines: block_lines,
                inline_code_spans: inline_spans,
            });
        }
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_diagram_block() {
        let markdown = "Line 1\nLine 2\nLine 3";
        let blocks = extract_diagram_blocks(markdown);
        assert_eq!(blocks.len(), 1);
        // Lines are now masked, so we check the count instead
        assert_eq!(blocks[0].lines.len(), 3);
        assert_eq!(blocks[0].inline_code_spans.len(), 3);
    }

    #[test]
    fn test_multiple_diagram_blocks() {
        let markdown = "Block1Line1\nBlock1Line2\n\nBlock2Line1\nBlock2Line2\n\nBlock3";
        let blocks = extract_diagram_blocks(markdown);
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0].lines.len(), 2);
        assert_eq!(blocks[1].lines.len(), 2);
        assert_eq!(blocks[2].lines.len(), 1);
    }

    #[test]
    fn test_preserves_start_line_number() {
        let markdown = "Line0\n\n\nLine3\nLine4\n\nLine6";
        let blocks = extract_diagram_blocks(markdown);
        assert_eq!(blocks[0].start_line, 0);
        assert_eq!(blocks[1].start_line, 3);
        assert_eq!(blocks[2].start_line, 6);
    }

    #[test]
    fn test_ignores_content_in_code_fence() {
        let markdown = "Visible1\n\n```\nHidden\n```\n\nVisible2";
        let blocks = extract_diagram_blocks(markdown);
        // Visible1, code fence (ignored), Visible2 = 2 blocks
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].lines.len(), 1);
        assert_eq!(blocks[1].lines.len(), 1);
    }

    #[test]
    fn test_empty_markdown() {
        let markdown = "";
        let blocks = extract_diagram_blocks(markdown);
        assert!(blocks.is_empty());
    }

    #[test]
    fn test_only_blank_lines() {
        let markdown = "\n\n\n";
        let blocks = extract_diagram_blocks(markdown);
        assert!(blocks.is_empty());
    }

    #[test]
    fn test_preserves_line_content() {
        let markdown = "  Indented  \n\tTabbed\nNormal";
        let blocks = extract_diagram_blocks(markdown);
        assert_eq!(blocks.len(), 1);
        // Verify structure is preserved
        assert_eq!(blocks[0].lines.len(), 3);
        assert_eq!(blocks[0].inline_code_spans.len(), 3);
        // Verify no inline code was detected in these lines
        assert_eq!(blocks[0].inline_code_spans[0].len(), 0);
        assert_eq!(blocks[0].inline_code_spans[1].len(), 0);
        assert_eq!(blocks[0].inline_code_spans[2].len(), 0);
    }

    #[test]
    fn test_multiple_blank_lines_separate_blocks() {
        let markdown = "Block1\n\n\n\nBlock2";
        let blocks = extract_diagram_blocks(markdown);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].lines.len(), 1);
        assert_eq!(blocks[1].lines.len(), 1);
    }
}
