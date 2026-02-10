//! Link detection and preservation for Markdown content.
//!
//! This module detects markdown links, especially those with parentheses in URLs,
//! to prevent them from breaking during any reflow or processing operations.

/// Represents a detected markdown link.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    /// The link text (content inside square brackets)
    pub text: String,
    /// The URL (content inside parentheses)
    pub url: String,
    /// Start position in the original content
    pub start_pos: usize,
    /// End position in the original content
    pub end_pos: usize,
}

/// Detect all inline markdown links in content.
///
/// Handles links with balanced parentheses in URLs, such as:
/// `[text](https://example.com/path(1))`
///
/// # Examples
///
/// ```
/// use ascfix::links::detect_links;
///
/// let content = "[Wiki](https://en.wikipedia.org/wiki/Pointer_(computer_programming))";
/// let links = detect_links(content);
/// assert_eq!(links.len(), 1);
/// assert_eq!(links[0].url, "https://en.wikipedia.org/wiki/Pointer_(computer_programming)");
/// ```
#[must_use]
pub fn detect_links(content: &str) -> Vec<Link> {
    let mut links = Vec::new();
    let chars: Vec<char> = content.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Look for opening bracket '['
        if chars[i] == '[' {
            let start_pos = i;

            // Find closing bracket ']'
            if let Some(text_end) = find_closing_bracket(&chars, i + 1) {
                // Extract link text
                let text: String = chars[(i + 1)..text_end].iter().collect();

                // Check for opening parenthesis immediately after ']'
                if text_end + 1 < chars.len() && chars[text_end + 1] == '(' {
                    // Find the closing parenthesis, handling nested parens
                    if let Some(url_end) = find_closing_parenthesis_balanced(&chars, text_end + 2) {
                        // Extract URL
                        let url: String = chars[(text_end + 2)..url_end].iter().collect();

                        links.push(Link {
                            text,
                            url,
                            start_pos,
                            end_pos: url_end + 1,
                        });

                        i = url_end + 1;
                        continue;
                    }
                }
            }
        }
        i += 1;
    }

    links
}

/// Find the closing bracket ']' starting from position.
/// Returns None if not found.
fn find_closing_bracket(chars: &[char], start: usize) -> Option<usize> {
    let mut i = start;
    while i < chars.len() {
        if chars[i] == ']' {
            return Some(i);
        }
        // Skip escaped brackets
        if chars[i] == '\\' && i + 1 < chars.len() {
            i += 2;
        } else {
            i += 1;
        }
    }
    None
}

/// Find closing parenthesis ')' with balanced counting.
/// This handles URLs that contain parentheses like: `<https://example.com/path(1)>`
fn find_closing_parenthesis_balanced(chars: &[char], start: usize) -> Option<usize> {
    let mut depth = 1; // We're already inside the opening paren
    let mut i = start;

    while i < chars.len() {
        match chars[i] {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            '\\' if i + 1 < chars.len() => {
                // Skip escaped characters
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_link_with_parens() {
        let content = "[Wiki](https://en.wikipedia.org/wiki/Pointer_(computer_programming))";
        let links = detect_links(content);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].text, "Wiki");
        assert_eq!(
            links[0].url,
            "https://en.wikipedia.org/wiki/Pointer_(computer_programming)"
        );
    }

    #[test]
    fn detect_simple_link() {
        let content = "[Example](https://example.com)";
        let links = detect_links(content);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].text, "Example");
        assert_eq!(links[0].url, "https://example.com");
    }
}
