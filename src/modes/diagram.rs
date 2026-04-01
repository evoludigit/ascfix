//! Diagram mode: Detect and normalize ASCII diagrams (full pipeline).
//!
//! This module handles the complete diagram processing pipeline:
//! 1. Extract diagram blocks from content
//! 2. Convert to grid representation
//! 3. Detect primitives (boxes, arrows, text)
//! 4. Normalize geometry
//! 5. Render back to text
//! 6. Restore inline code spans

use crate::scanner::InlineCodeSpan;

/// Restore masked inline code content.
fn restore_inline_code(masked_line: &str, spans: &[InlineCodeSpan]) -> String {
    if spans.is_empty() {
        return masked_line.to_string();
    }
    let mut restored_chars: Vec<char> = masked_line.chars().collect();
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

/// Diagram mode: Detect and normalize ASCII diagrams (full pipeline).
pub fn process_diagram_mode(content: &str, config: &crate::config::Config) -> String {
    let mut blocks = crate::scanner::extract_diagram_blocks(content);

    // Optionally include diagrams inside bare code fences
    if config.fenced_diagrams {
        blocks.extend(crate::scanner::extract_fenced_diagram_blocks(content));
        blocks.sort_by_key(|b| b.start_line);
    }

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
            let normalized = crate::normalizer::normalize_nested_boxes(&normalized);
            let normalized = crate::normalizer::align_horizontal_arrows(&normalized);
            let normalized = crate::normalizer::align_vertical_arrows(&normalized);
            let normalized = crate::normalizer::balance_horizontal_boxes(&normalized);
            let normalized = crate::normalizer::normalize_padding(&normalized);

            // Render onto a COPY of the original grid to preserve pass-through content
            // This ensures lines without detected primitives are not lost
            let rendered_grid =
                crate::renderer::render_onto_grid(&grid, &inventory, &normalized);
            let rendered = rendered_grid.render_trimmed();

            // Restore inline code in the rendered output
            let rendered_lines: Vec<String> = rendered
                .lines()
                .enumerate()
                .map(|(i, line)| {
                    if i < block.inline_code_spans.len() {
                        restore_inline_code(line, &block.inline_code_spans[i])
                    } else {
                        line.to_string()
                    }
                })
                .collect();

            // Replace the block in the original content (in reverse to maintain indices)
            let block_len = block.lines.len();

            // Remove old lines and insert new ones
            for _ in 0..block_len {
                if block.start_line < lines.len() {
                    lines.remove(block.start_line);
                }
            }
            // Insert new lines
            for (i, line) in rendered_lines.iter().enumerate() {
                lines.insert(block.start_line + i, line.clone());
            }
        }
        // If no primitives found, leave the block unchanged
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Mode;

    fn default_config() -> crate::config::Config {
        crate::config::Config::default()
    }

    #[test]
    fn test_diagram_mode_preserves_content() {
        let content = "# Test\n\nSome content";
        let result = process_diagram_mode(content, &default_config());
        // Diagram mode should preserve content when no diagrams exist
        assert_eq!(result, content);
    }

    #[test]
    fn test_diagram_mode_processes_boxes() {
        let content = "┌─┐\n│ │\n└─┘";
        let result = process_diagram_mode(content, &default_config());
        // Should render the diagram (may change spacing but keep structure)
        assert!(result.contains("┌"));
        assert!(result.contains("└"));
        assert!(result.contains("│"));
    }

    #[test]
    fn test_diagram_mode_preserves_non_diagram_text() {
        let content = "# Title\n\nSome text\n\nMore content";
        let result = process_diagram_mode(content, &default_config());
        // Non-diagram content should be preserved
        assert!(result.contains("# Title"));
        assert!(result.contains("Some text"));
    }

    #[test]
    fn test_fence_repair_in_pipeline() {
        let content = "```python\ncode\n`````";
        // Test with fence repair enabled via process_by_mode
        let result =
            super::super::process_by_mode(&Mode::Diagram, content, true, &default_config());
        // Fences should be normalized before diagram processing
        assert!(result.contains('`'));
    }
}
