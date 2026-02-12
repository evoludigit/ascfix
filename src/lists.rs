//! List normalization for Markdown lists.
//!
//! This module detects and normalizes Markdown lists, fixing inconsistent
//! indentation and bullet styles.

/// Represents a detected list item.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct ListItem {
    /// The bullet character (-, *, +) or number for ordered lists
    pub marker: String,
    /// The content of the list item (after the bullet)
    pub content: String,
    /// Indentation level (0 = top level, 1 = nested, etc.)
    pub level: usize,
    /// Whether this is a task list item (- [ ], - [x])
    pub is_task: bool,
    /// Checkbox state for task lists (Some(true) = checked, Some(false) = unchecked, None = not a task)
    pub checked: Option<bool>,
    /// The line number where this item starts
    pub line_number: usize,
}

/// Represents a complete list.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct List {
    /// All items in the list
    pub items: Vec<ListItem>,
    /// The starting line number of the list
    pub start_line: usize,
    /// The ending line number of the list
    pub end_line: usize,
    /// Whether this is an ordered (numbered) list
    pub is_ordered: bool,
}

/// Detect all lists in the given content.
///
/// Identifies both bullet lists (-, *, +) and ordered lists (1., 2), etc.).
/// Lists inside code blocks are ignored.
///
/// # Examples
///
/// ```
/// use ascfix::lists::detect_lists;
///
/// let content = "- Item 1\n- Item 2\n- Item 3";
/// let lists = detect_lists(content);
/// assert_eq!(lists.len(), 1);
/// assert_eq!(lists[0].items.len(), 3);
/// ```
#[must_use]
#[allow(dead_code)]
pub fn detect_lists(content: &str) -> Vec<List> {
    let mut lists = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    // Get code block line ranges to skip
    let code_line_ranges = get_code_block_line_ranges(content);

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];

        // Skip lines inside code blocks
        if is_in_code_region(i, &code_line_ranges) {
            i += 1;
            continue;
        }

        // Check if this line starts a list item
        if let Some(item) = parse_list_item(line, i) {
            // Check if we're continuing an existing list or starting a new one
            let mut current_list_items = vec![item];
            let start_line = i;
            i += 1;

            // Continue collecting list items
            while i < lines.len() {
                let next_line = lines[i];

                // Skip code block lines
                if is_in_code_region(i, &code_line_ranges) {
                    break;
                }

                // Check for continuation (blank lines or nested content)
                if next_line.trim().is_empty() {
                    // Look ahead to see if list continues
                    if i + 1 < lines.len() {
                        if let Some(next_item) = parse_list_item(lines[i + 1], i + 1) {
                            // Check if this continues the same list
                            if is_same_list(&current_list_items, &next_item) {
                                i += 1; // Skip blank line
                                continue;
                            }
                        }
                    }
                    break;
                }

                // Check for new list item
                if let Some(next_item) = parse_list_item(next_line, i) {
                    // Check if same list or nested
                    if is_same_list(&current_list_items, &next_item)
                        || is_nested_list(&current_list_items, &next_item)
                    {
                        current_list_items.push(next_item);
                        i += 1;
                        continue;
                    }
                }

                // Check for continuation line (indented content)
                if is_continuation_line(next_line, &current_list_items) {
                    i += 1;
                    continue;
                }

                break;
            }

            // Create the list
            if !current_list_items.is_empty() {
                let is_ordered = current_list_items[0].marker.parse::<i32>().is_ok();
                lists.push(List {
                    items: current_list_items,
                    start_line,
                    end_line: i - 1,
                    is_ordered,
                });
            }
        } else {
            i += 1;
        }
    }

    lists
}

/// Get code block regions as line number ranges (`start_line`, `end_line`).
#[allow(dead_code)]
fn get_code_block_line_ranges(content: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut in_code_block = false;
    let mut block_start = 0;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Check for fence markers
        if is_fence_line(trimmed) {
            if in_code_block {
                // End of code block
                ranges.push((block_start, i));
                in_code_block = false;
            } else {
                // Start of code block
                block_start = i;
                in_code_block = true;
            }
        }
    }

    // Handle unclosed code block
    if in_code_block {
        ranges.push((block_start, lines.len() - 1));
    }

    ranges
}

/// Check if a line is a fence marker (starts with backticks or tildes).
#[allow(dead_code)]
fn is_fence_line(line: &str) -> bool {
    line.starts_with("```") || line.starts_with("~~~")
}

/// Parse a single line to see if it's a list item.
#[allow(dead_code)]
fn parse_list_item(line: &str, line_number: usize) -> Option<ListItem> {
    let trimmed = line.trim_start();

    // Check for bullet list markers: -, *, +
    if let Some(rest) = trimmed.strip_prefix("- ") {
        return parse_task_or_item("-", rest, line_number);
    }
    if let Some(rest) = trimmed.strip_prefix("* ") {
        return Some(ListItem {
            marker: "*".to_string(),
            content: rest.to_string(),
            level: 0, // Will be calculated based on indentation
            is_task: false,
            checked: None,
            line_number,
        });
    }
    if let Some(rest) = trimmed.strip_prefix("+ ") {
        return Some(ListItem {
            marker: "+".to_string(),
            content: rest.to_string(),
            level: 0,
            is_task: false,
            checked: None,
            line_number,
        });
    }

    // Check for ordered list markers: 1., 2), etc.
    // Pattern: number followed by . or )
    let chars: Vec<char> = trimmed.chars().collect();
    if !chars.is_empty() && chars[0].is_ascii_digit() {
        // Find the end of the number
        let mut num_end = 0;
        while num_end < chars.len() && chars[num_end].is_ascii_digit() {
            num_end += 1;
        }

        if num_end < chars.len() && (chars[num_end] == '.' || chars[num_end] == ')') {
            let number = &trimmed[0..num_end];
            let rest = trimmed[num_end + 1..].trim_start();
            return Some(ListItem {
                marker: number.to_string(),
                content: rest.to_string(),
                level: 0,
                is_task: false,
                checked: None,
                line_number,
            });
        }
    }

    None
}

/// Check if content is a task list item and parse accordingly.
#[allow(dead_code)]
fn parse_task_or_item(marker: &str, content: &str, line_number: usize) -> Option<ListItem> {
    let trimmed = content.trim_start();

    // Check for task list: [ ], [x], [X]
    if let Some(rest) = trimmed.strip_prefix("[ ] ") {
        return Some(ListItem {
            marker: marker.to_string(),
            content: rest.to_string(),
            level: 0,
            is_task: true,
            checked: Some(false),
            line_number,
        });
    }
    if let Some(rest) = trimmed.strip_prefix("[x] ") {
        return Some(ListItem {
            marker: marker.to_string(),
            content: rest.to_string(),
            level: 0,
            is_task: true,
            checked: Some(true),
            line_number,
        });
    }
    if let Some(rest) = trimmed.strip_prefix("[X] ") {
        return Some(ListItem {
            marker: marker.to_string(),
            content: rest.to_string(),
            level: 0,
            is_task: true,
            checked: Some(true),
            line_number,
        });
    }

    // Regular bullet item
    Some(ListItem {
        marker: marker.to_string(),
        content: content.to_string(),
        level: 0,
        is_task: false,
        checked: None,
        line_number,
    })
}

/// Check if a new item belongs to the same list as existing items.
#[allow(dead_code)]
fn is_same_list(existing_items: &[ListItem], new_item: &ListItem) -> bool {
    if existing_items.is_empty() {
        return true;
    }

    // Same bullet style or both ordered
    let first = &existing_items[0];

    // Check if both use same bullet type
    if ["-", "*", "+"].contains(&first.marker.as_str())
        && ["-", "*", "+"].contains(&new_item.marker.as_str())
    {
        return true;
    }

    // Check if both are ordered lists
    if first.marker.parse::<i32>().is_ok() && new_item.marker.parse::<i32>().is_ok() {
        return true;
    }

    false
}

/// Check if a new item is a nested list item.
#[allow(dead_code)]
fn is_nested_list(existing_items: &[ListItem], new_item: &ListItem) -> bool {
    // A nested item has more indentation
    // For now, just check if it's indented more than the first item
    if existing_items.is_empty() {
        return false;
    }

    // Compare line numbers to determine nesting
    // In practice, we'd need the actual indentation levels
    new_item.line_number > existing_items[0].line_number
}

/// Check if a line is a continuation of the current list item.
#[allow(dead_code)]
fn is_continuation_line(line: &str, current_items: &[ListItem]) -> bool {
    if current_items.is_empty() {
        return false;
    }

    // A continuation line is indented (has leading whitespace)
    // and doesn't start a new list item
    if line.trim().is_empty() {
        return false;
    }

    let leading_spaces = line.len() - line.trim_start().len();
    // Must have at least 2 spaces of indentation to be a continuation
    leading_spaces >= 2 && parse_list_item(line, 0).is_none()
}

/// Check if a line number is inside a code block region.
#[allow(dead_code)]
fn is_in_code_region(line_num: usize, regions: &[(usize, usize)]) -> bool {
    for (start, end) in regions {
        if line_num >= *start && line_num <= *end {
            return true;
        }
    }
    false
}

/// Normalize list indentation to standard 2-space increments.
///
/// Takes content with lists and normalizes the indentation of nested items
/// to use consistent 2-space increments per nesting level.
///
/// # Examples
///
/// ```
/// use ascfix::lists::normalize_list_indentation;
///
/// let content = "- Item 1\n    - Nested item\n- Item 2";
/// let normalized = normalize_list_indentation(content);
/// assert!(normalized.contains("  - Nested item")); // 2 spaces, not 4
/// ```
#[must_use]
#[allow(dead_code)]
pub fn normalize_list_indentation(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return String::new();
    }

    // Get code block regions to skip
    let code_ranges = get_code_block_line_ranges(content);

    let mut result = Vec::new();
    let mut list_stack: Vec<usize> = Vec::new(); // Stack of indentation levels for each list level

    for (i, line) in lines.iter().enumerate() {
        // Skip lines inside code blocks
        if is_in_code_region(i, &code_ranges) {
            result.push(line.to_string());
            continue;
        }

        // Check if this line is a list item
        if let Some(item) = parse_list_item(line, i) {
            let current_indent = line.len() - line.trim_start().len();

            // Determine the nesting level based on indentation
            let level = if list_stack.is_empty() {
                // First item in a list
                list_stack.push(current_indent);
                0
            } else {
                // Find the appropriate level based on indentation
                let mut level = list_stack.len();
                for (idx, &indent) in list_stack.iter().enumerate() {
                    if current_indent <= indent {
                        level = idx;
                        break;
                    }
                }

                // Trim stack to current level
                list_stack.truncate(level);

                // If this is a new nesting level, add it
                if level == list_stack.len() {
                    list_stack.push(current_indent);
                }

                level
            };

            // Calculate normalized indentation: 2 spaces per level
            let normalized_indent = "  ".repeat(level);
            let reconstructed = format!("{}{} {}", normalized_indent, item.marker, item.content);
            result.push(reconstructed);
        } else {
            // Non-list line - reset the stack if it's not a continuation
            if line.trim().is_empty() {
                list_stack.clear();
            }
            result.push(line.to_string());
        }
    }

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_simple_bullet_list() {
        let content = "- Item 1\n- Item 2\n- Item 3";
        let lists = detect_lists(content);
        assert_eq!(lists.len(), 1);
        assert_eq!(lists[0].items.len(), 3);
        assert_eq!(lists[0].items[0].content, "Item 1");
        assert_eq!(lists[0].items[1].content, "Item 2");
        assert_eq!(lists[0].items[2].content, "Item 3");
    }

    #[test]
    fn detect_list_with_mixed_bullets() {
        let content = "- Item 1\n* Item 2\n+ Item 3";
        let lists = detect_lists(content);
        assert_eq!(lists.len(), 1);
        assert_eq!(lists[0].items.len(), 3);
    }

    #[test]
    fn detect_ordered_list() {
        let content = "1. First item\n2. Second item\n3. Third item";
        let lists = detect_lists(content);
        assert_eq!(lists.len(), 1);
        assert_eq!(lists[0].items.len(), 3);
        assert!(lists[0].is_ordered);
    }

    #[test]
    fn detect_task_list() {
        let content = "- [ ] Todo item\n- [x] Done item\n- [X] Also done";
        let lists = detect_lists(content);
        assert_eq!(lists.len(), 1);
        assert_eq!(lists[0].items.len(), 3);
        assert!(lists[0].items[0].is_task);
        assert!(!lists[0].items[0].checked.unwrap());
        assert!(lists[0].items[1].is_task);
        assert!(lists[0].items[1].checked.unwrap());
    }

    #[test]
    fn ignore_lists_in_code_blocks() {
        let content =
            "```markdown\n- Item in code block\n- Another item\n```\n\n- Real item outside";
        let lists = detect_lists(content);
        assert_eq!(lists.len(), 1);
        assert_eq!(lists[0].items.len(), 1);
        assert_eq!(lists[0].items[0].content, "Real item outside");
    }

    #[test]
    fn detect_multiple_lists() {
        let content = "- First list item 1\n- First list item 2\n\nSome text\n\n* Second list item 1\n* Second list item 2";
        let lists = detect_lists(content);
        assert_eq!(lists.len(), 2);
        assert_eq!(lists[0].items.len(), 2);
        assert_eq!(lists[1].items.len(), 2);
    }

    #[test]
    fn no_lists_in_plain_text() {
        let content = "This is just a paragraph.\nNo lists here.\nJust text.";
        let lists = detect_lists(content);
        assert_eq!(lists.len(), 0);
    }

    #[test]
    fn normalize_indentation_to_two_spaces() {
        // 4-space indentation should become 2-space
        let content = "- Item 1\n    - Nested item\n- Item 2";
        let normalized = normalize_list_indentation(content);
        assert!(normalized.contains("- Item 1"));
        assert!(normalized.contains("  - Nested item")); // 2 spaces
        assert!(!normalized.contains("    - Nested")); // No 4 spaces
    }

    #[test]
    fn normalize_deeply_nested_list() {
        // Mixed indentation should be normalized
        let content = "- Level 1\n    - Level 2\n        - Level 3\n- Back to 1";
        let normalized = normalize_list_indentation(content);
        assert!(normalized.contains("- Level 1"));
        assert!(normalized.contains("  - Level 2")); // 2 spaces
        assert!(normalized.contains("    - Level 3")); // 4 spaces (2 per level)
    }

    #[test]
    fn preserve_content_when_normalizing() {
        // Content should remain unchanged, only indentation fixed
        let content = "- First item with text\n    - Second item with more text";
        let normalized = normalize_list_indentation(content);
        assert!(normalized.contains("First item with text"));
        assert!(normalized.contains("Second item with more text"));
    }

    #[test]
    fn no_change_to_already_normalized() {
        // Already 2-space indented lists should remain unchanged
        let content = "- Item 1\n  - Nested\n  - Another nested\n- Item 2";
        let normalized = normalize_list_indentation(content);
        assert_eq!(normalized, content);
    }

    #[test]
    fn normalize_mixed_indentation_styles() {
        // Mixed 2-space and 4-space should become all 2-space relative
        let content = "- Item 1\n  - Two space\n    - Four space (should be 4)\n- Item 2";
        let normalized = normalize_list_indentation(content);
        assert!(normalized.contains("- Item 1"));
        assert!(normalized.contains("  - Two space"));
        assert!(normalized.contains("    - Four space")); // 4 spaces is correct for 2nd level
    }
}
