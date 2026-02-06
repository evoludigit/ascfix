//! Diagram block extraction from Markdown content.

use crate::parser;

/// Represents a contiguous diagram block with its location.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // Reason: Used by main processing pipeline
pub struct DiagramBlock {
    /// Starting line number in the original markdown
    pub start_line: usize,
    /// Lines comprising this diagram block
    pub lines: Vec<String>,
}

/// Extract diagram blocks from Markdown content.
///
/// A diagram block is a contiguous sequence of non-empty lines outside code fences.
/// Blank lines separate blocks.
#[allow(dead_code)] // Reason: Used by main processing pipeline
#[must_use] 
pub fn extract_diagram_blocks(text: &str) -> Vec<DiagramBlock> {
    let normal_lines = parser::extract_normal_lines(text);
    let mut blocks = Vec::new();
    let mut current_block: Option<(usize, Vec<String>)> = None;

    for (line_num, line) in normal_lines {
        if line.trim().is_empty() {
            // Blank line: finalize current block if any
            if let Some((start, lines)) = current_block.take() {
                if !lines.is_empty() {
                    blocks.push(DiagramBlock {
                        start_line: start,
                        lines,
                    });
                }
            }
        } else {
            // Non-empty line: add to current block or start new one
            if let Some((_start, ref mut lines)) = &mut current_block {
                lines.push(line);
            } else {
                current_block = Some((line_num, vec![line]));
            }
        }
    }

    // Finalize last block if any
    if let Some((start, lines)) = current_block {
        if !lines.is_empty() {
            blocks.push(DiagramBlock {
                start_line: start,
                lines,
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
        assert_eq!(blocks[0].lines, vec!["Line 1", "Line 2", "Line 3"]);
    }

    #[test]
    fn test_multiple_diagram_blocks() {
        let markdown = "Block1Line1\nBlock1Line2\n\nBlock2Line1\nBlock2Line2\n\nBlock3";
        let blocks = extract_diagram_blocks(markdown);
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0].lines, vec!["Block1Line1", "Block1Line2"]);
        assert_eq!(blocks[1].lines, vec!["Block2Line1", "Block2Line2"]);
        assert_eq!(blocks[2].lines, vec!["Block3"]);
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
        assert_eq!(blocks[0].lines, vec!["Visible1"]);
        assert_eq!(blocks[1].lines, vec!["Visible2"]);
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
        assert_eq!(blocks[0].lines[0], "  Indented  ");
        assert_eq!(blocks[0].lines[1], "\tTabbed");
        assert_eq!(blocks[0].lines[2], "Normal");
    }

    #[test]
    fn test_multiple_blank_lines_separate_blocks() {
        let markdown = "Block1\n\n\n\nBlock2";
        let blocks = extract_diagram_blocks(markdown);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].lines, vec!["Block1"]);
        assert_eq!(blocks[1].lines, vec!["Block2"]);
    }
}
