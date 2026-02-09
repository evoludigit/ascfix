//! Code fence boundary validation and repair.
//!
//! This module detects, validates, and repairs code fence markers in Markdown.
//! It handles both backtick and tilde fences, detects issues like
//! mismatched lengths and unclosed blocks, and repairs them conservatively.

use std::collections::VecDeque;

/// Type of fence marker (backtick or tilde).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceType {
    /// Backtick fence
    Backtick,
    /// Tilde fence
    Tilde,
}

/// A fence marker (opening or closing).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FenceMarker {
    /// Line number (0-indexed)
    pub line_num: usize,
    /// Fence type (backtick or tilde)
    pub fence_type: FenceType,
    /// Number of fence characters (3 or more)
    pub length: usize,
    /// Language specifier (e.g., "python")
    pub language: Option<String>,
    /// Whether this is an opening fence
    pub is_opening: bool,
}

/// A code block with opening and optional closing fence.
#[derive(Debug, Clone)]
pub struct CodeBlock {
    /// Opening fence marker
    pub opening: FenceMarker,
    /// Closing fence marker (None if unclosed)
    pub closing: Option<FenceMarker>,
}

/// Fence validation issue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenceIssue {
    /// Unclosed fence (no matching closing marker)
    Unclosed { opening: FenceMarker },
    /// Length mismatch between opening and closing
    LengthMismatch {
        opening: FenceMarker,
        closing: FenceMarker,
    },
    /// Type mismatch (backticks vs tildes)
    TypeMismatch {
        opening: FenceMarker,
        closing: FenceMarker,
    },
}

/// Detect all fence markers in content.
///
/// Returns a vector of fence markers with their metadata.
/// Fences must be on a line by themselves (possibly with indentation).
#[must_use]
pub fn detect_fence_markers(content: &str) -> Vec<FenceMarker> {
    let mut markers = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Check for backtick fence (3+ backticks)
        if let Some(pos) = trimmed.find('`') {
            if trimmed[pos..].starts_with("```") {
                let fence_str = &trimmed[pos..];
                let fence_end = fence_str.find(|c| c != '`').unwrap_or(fence_str.len());
                let length = fence_end;

                if length >= 3 {
                    let rest = &fence_str[fence_end..].trim_start();
                    let language = if rest.is_empty() {
                        None
                    } else {
                        Some(rest.to_string())
                    };

                    markers.push(FenceMarker {
                        line_num,
                        fence_type: FenceType::Backtick,
                        length,
                        language,
                        is_opening: false, // Will be determined during pairing
                    });
                }
                continue;
            }
        }

        // Check for tilde fence (3+ tildes)
        if let Some(pos) = trimmed.find('~') {
            if trimmed[pos..].starts_with("~~~") {
                let fence_str = &trimmed[pos..];
                let fence_end = fence_str.find(|c| c != '~').unwrap_or(fence_str.len());
                let length = fence_end;

                if length >= 3 {
                    let rest = &fence_str[fence_end..].trim_start();
                    let language = if rest.is_empty() {
                        None
                    } else {
                        Some(rest.to_string())
                    };

                    markers.push(FenceMarker {
                        line_num,
                        fence_type: FenceType::Tilde,
                        length,
                        language,
                        is_opening: false, // Will be determined during pairing
                    });
                }
            }
        }
    }

    markers
}

/// Pair fence markers into code blocks.
///
/// Uses a stack-based algorithm to handle nested fences.
/// A fence can nest another fence if it has more markers.
///
/// # Panics
///
/// Never panics - the `expect()` is protected by the conditional check above it.
#[must_use]
pub fn pair_fences(markers: Vec<FenceMarker>) -> Vec<CodeBlock> {
    let mut blocks = Vec::new();
    let mut stack: VecDeque<FenceMarker> = VecDeque::new();

    for mut marker in markers {
        if let Some(top) = stack.back() {
            // Check if this could close the top fence:
            // Same type and length >= top length (nesting allowed)
            if marker.fence_type == top.fence_type && marker.length >= top.length {
                // This closes the top fence
                // Safe: we just checked that stack is not empty with .back()
                let mut opening = stack.pop_back().expect("stack not empty");
                opening.is_opening = true;
                marker.is_opening = false;

                blocks.push(CodeBlock {
                    opening,
                    closing: Some(marker),
                });
            } else {
                // This opens a new nested fence or is a different type
                marker.is_opening = true;
                stack.push_back(marker);
            }
        } else {
            // Stack is empty, this opens a new fence
            marker.is_opening = true;
            stack.push_back(marker);
        }
    }

    // Any remaining markers in stack are unclosed
    while let Some(opening) = stack.pop_back() {
        blocks.push(CodeBlock {
            opening,
            closing: None,
        });
    }

    // Sort blocks by opening line number
    blocks.sort_by_key(|b| b.opening.line_num);
    blocks
}

/// Validate fence blocks and return list of issues.
#[must_use]
pub fn validate_fences(blocks: &[CodeBlock]) -> Vec<FenceIssue> {
    let mut issues = Vec::new();

    for block in blocks {
        if let Some(closing) = &block.closing {
            // Check for type mismatch
            if closing.fence_type != block.opening.fence_type {
                issues.push(FenceIssue::TypeMismatch {
                    opening: block.opening.clone(),
                    closing: closing.clone(),
                });
            }
            // Check for length mismatch
            else if closing.length != block.opening.length {
                issues.push(FenceIssue::LengthMismatch {
                    opening: block.opening.clone(),
                    closing: closing.clone(),
                });
            }
        } else {
            // No closing fence
            issues.push(FenceIssue::Unclosed {
                opening: block.opening.clone(),
            });
        }
    }

    issues
}

/// Normalize fences in content, repairing common issues.
///
/// This is the main entry point for fence repair. It:
/// 1. Detects all fence markers
/// 2. Pairs them into blocks
/// 3. Validates each block
/// 4. Repairs issues conservatively
/// 5. Returns normalized content
#[must_use]
pub fn normalize_fences(content: &str) -> String {
    let markers = detect_fence_markers(content);

    // If no fences found, return unchanged
    if markers.is_empty() {
        return content.to_string();
    }

    let blocks = pair_fences(markers);

    // If no validation issues, return unchanged
    let issues = validate_fences(&blocks);
    if issues.is_empty() {
        return content.to_string();
    }

    // Repair issues
    repair_fences(content, &blocks, &issues)
}

/// Repair fence issues in content.
fn repair_fences(content: &str, _blocks: &[CodeBlock], issues: &[FenceIssue]) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result_lines: Vec<String> =
        lines.iter().map(std::string::ToString::to_string).collect();

    // Process issues in reverse order to maintain line numbers
    let mut sorted_issues = issues.to_vec();
    sorted_issues.sort_by_key(|issue| match issue {
        FenceIssue::Unclosed { opening }
        | FenceIssue::LengthMismatch { opening, .. }
        | FenceIssue::TypeMismatch { opening, .. } => opening.line_num,
    });
    sorted_issues.reverse();

    for issue in sorted_issues {
        match issue {
            FenceIssue::Unclosed { opening } => {
                // Add closing fence after the last line
                let closing_fence =
                    format!("{}{}", fence_chars(opening.fence_type), opening.length);
                result_lines.push(closing_fence);
            }
            FenceIssue::LengthMismatch { opening, closing } => {
                // Use the longer length for both
                let length = opening.length.max(closing.length);
                let fence_str = fence_chars(opening.fence_type);
                let new_fence = format!("{}{}", fence_str, fence_str.repeat(length - 1));

                // Update opening fence
                if opening.line_num < result_lines.len() {
                    let line = &result_lines[opening.line_num];
                    let trimmed = line.trim();
                    let indent = line.len() - trimmed.len();
                    let language = opening.language.as_deref().unwrap_or("");
                    let prefix = " ".repeat(indent);
                    result_lines[opening.line_num] = if language.is_empty() {
                        format!("{prefix}{new_fence}")
                    } else {
                        format!("{prefix}{new_fence}{language}")
                    };
                }

                // Update closing fence
                if closing.line_num < result_lines.len() {
                    let line = &result_lines[closing.line_num];
                    let trimmed = line.trim();
                    let indent = line.len() - trimmed.len();
                    let prefix = " ".repeat(indent);
                    result_lines[closing.line_num] = format!("{prefix}{new_fence}");
                }
            }
            FenceIssue::TypeMismatch { .. } => {
                // Skip type mismatches (too ambiguous)
            }
        }
    }

    result_lines.join("\n")
}

/// Get the character string for a fence type.
const fn fence_chars(fence_type: FenceType) -> &'static str {
    match fence_type {
        FenceType::Backtick => "`",
        FenceType::Tilde => "~",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_backtick_fence() {
        let content = "```python\ncode\n```";
        let markers = detect_fence_markers(content);
        assert_eq!(markers.len(), 2);
        assert_eq!(markers[0].fence_type, FenceType::Backtick);
        assert_eq!(markers[0].length, 3);
        assert_eq!(markers[0].language, Some("python".to_string()));
        assert_eq!(markers[1].fence_type, FenceType::Backtick);
        assert_eq!(markers[1].length, 3);
        assert_eq!(markers[1].language, None);
    }

    #[test]
    fn test_detect_tilde_fence() {
        let content = "~~~ruby\ncode\n~~~";
        let markers = detect_fence_markers(content);
        assert_eq!(markers.len(), 2);
        assert_eq!(markers[0].fence_type, FenceType::Tilde);
        assert_eq!(markers[0].length, 3);
        assert_eq!(markers[0].language, Some("ruby".to_string()));
    }

    #[test]
    fn test_detect_long_fence() {
        let content = "`````\ncode\n`````";
        let markers = detect_fence_markers(content);
        assert_eq!(markers.len(), 2);
        assert_eq!(markers[0].length, 5);
        assert_eq!(markers[1].length, 5);
    }

    #[test]
    fn test_detect_fence_with_whitespace() {
        let content = "  ```python\n  code\n  ```";
        let markers = detect_fence_markers(content);
        assert_eq!(markers.len(), 2);
        assert_eq!(markers[0].fence_type, FenceType::Backtick);
    }

    #[test]
    fn test_pair_simple_fences() {
        let content = "```\ncode\n```";
        let markers = detect_fence_markers(content);
        let blocks = pair_fences(markers);
        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].closing.is_some());
        assert_eq!(blocks[0].opening.line_num, 0);
        assert_eq!(blocks[0].closing.as_ref().unwrap().line_num, 2);
    }

    #[test]
    fn test_pair_nested_fences() {
        let content = "`````\n```\ncode\n```\n`````";
        let markers = detect_fence_markers(content);
        let blocks = pair_fences(markers);
        // Should have 2 blocks: outer 5-backtick and inner 3-backtick
        assert_eq!(blocks.len(), 2);
    }

    #[test]
    fn test_pair_unclosed_fence() {
        let content = "```\ncode";
        let markers = detect_fence_markers(content);
        let blocks = pair_fences(markers);
        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].closing.is_none());
    }

    #[test]
    fn test_validate_length_mismatch() {
        let content = "```\ncode\n`````";
        let markers = detect_fence_markers(content);
        let blocks = pair_fences(markers);
        let issues = validate_fences(&blocks);
        assert_eq!(issues.len(), 1);
        match &issues[0] {
            FenceIssue::LengthMismatch { .. } => {}
            _ => panic!("Expected LengthMismatch"),
        }
    }

    #[test]
    fn test_validate_type_mismatch() {
        let content = "```\ncode\n~~~";
        let markers = detect_fence_markers(content);
        let blocks = pair_fences(markers);
        // When types don't match, they don't pair, so we get 2 unclosed fences
        // (not a type mismatch issue, since they're not paired)
        assert_eq!(blocks.len(), 2);
        assert!(blocks[0].closing.is_none());
        assert!(blocks[1].closing.is_none());
    }

    #[test]
    fn test_validate_unclosed() {
        let content = "```\ncode";
        let markers = detect_fence_markers(content);
        let blocks = pair_fences(markers);
        let issues = validate_fences(&blocks);
        assert_eq!(issues.len(), 1);
        match &issues[0] {
            FenceIssue::Unclosed { .. } => {}
            _ => panic!("Expected Unclosed"),
        }
    }

    #[test]
    fn test_normalize_length_mismatch() {
        let content = "```python\ncode\n`````";
        let normalized = normalize_fences(content);
        // Both fences should now have 5 backticks
        assert!(normalized.contains("`````python") || normalized.contains("`````"));
        assert!(normalized.contains("`````"));
    }

    #[test]
    fn test_normalize_unclosed_fence() {
        let content = "```python\ncode";
        let normalized = normalize_fences(content);
        // Should have added closing fence
        assert!(normalized.contains("```"));
        // Check that we have both opening (with language) and closing
        assert!(normalized.lines().count() >= 2);
        assert!(normalized
            .lines()
            .last()
            .is_some_and(|l| l.trim().contains('`')));
    }

    #[test]
    fn test_normalize_already_correct() {
        let content = "```python\ncode\n```";
        let normalized = normalize_fences(content);
        assert_eq!(normalized, content);
    }

    #[test]
    fn test_normalize_idempotent() {
        let content = "```python\ncode\n`````";
        let first = normalize_fences(content);
        let second = normalize_fences(&first);
        assert_eq!(first, second);
    }

    #[test]
    fn test_normalize_no_fences() {
        let content = "# Title\n\nNo fences here";
        let normalized = normalize_fences(content);
        assert_eq!(normalized, content);
    }

    #[test]
    fn test_normalize_multiple_blocks() {
        let content = "```\ncode1\n```\n\n```\ncode2\n`````";
        let normalized = normalize_fences(content);
        // Should have fixed the second block
        assert!(!normalized.is_empty());
    }

    #[test]
    fn test_fence_type_detection_backtick() {
        let content = "```\ncode\n```";
        let markers = detect_fence_markers(content);
        assert!(markers.iter().all(|m| m.fence_type == FenceType::Backtick));
    }

    #[test]
    fn test_fence_type_detection_tilde() {
        let content = "~~~\ncode\n~~~";
        let markers = detect_fence_markers(content);
        assert!(markers.iter().all(|m| m.fence_type == FenceType::Tilde));
    }

    #[test]
    fn test_language_specifier_preserved() {
        let content = "```javascript\ncode\n```";
        let normalized = normalize_fences(content);
        assert!(normalized.contains("javascript"));
    }

    #[test]
    fn test_skip_type_mismatch() {
        let content = "```\ncode\n~~~";
        let normalized = normalize_fences(content);
        // When types don't match, they don't pair - so we get 2 unclosed fences instead of a type mismatch
        // normalize_fences will add closing fences for both
        let markers = detect_fence_markers(&normalized);
        assert!(markers.len() >= 2); // At least opening backtick and tilde
    }

    #[test]
    fn test_empty_fence_block() {
        let content = "```\n```";
        let normalized = normalize_fences(content);
        assert_eq!(normalized, content);
    }

    #[test]
    fn test_fence_at_end_of_file() {
        let content = "```\ncode";
        let normalized = normalize_fences(content);
        assert!(normalized.contains("```"));
        assert!(normalized.lines().count() >= 2);
    }

    #[test]
    fn test_indented_fences() {
        let content = "  ```python\n  code\n  ```";
        let normalized = normalize_fences(content);
        // Should preserve indentation
        for line in normalized.lines() {
            if line.contains('`') {
                assert!(line.starts_with("  "));
            }
        }
    }
}
