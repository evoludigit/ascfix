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

/// Check if a position in content is inside any link URL.
///
/// This is useful for table parsing to avoid splitting on `|` characters
/// that appear inside link URLs.
#[must_use]
pub fn is_inside_link_url(_content: &str, pos: usize, links: &[Link]) -> bool {
    for link in links {
        // URL starts after the ]( and ends before the )
        let url_start = link.start_pos + link.text.len() + 2; // +2 for ](
        let url_end = link.end_pos;
        if pos >= url_start && pos < url_end {
            return true;
        }
    }
    false
}

/// Represents a reference-style link definition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // Reason: Used in tests, will be used in production soon
pub struct ReferenceLink {
    /// The reference label (e.g., "ref" in `[ref]: url`)
    pub label: String,
    /// The URL
    pub url: String,
    /// Optional title (e.g., `[ref]: url "title"`)
    pub title: Option<String>,
}

/// Detect all reference-style link definitions in content.
///
/// Reference links have the format:
/// ```markdown
/// [ref]: https://example.com/path
/// [ref]: https://example.com/path "optional title"
/// [ref]: https://example.com/path (optional title)
/// ```
///
/// # Examples
///
/// ```
/// use ascfix::links::detect_reference_links;
///
/// let content = "[ref]: https://example.com/path(1)";
/// let refs = detect_reference_links(content);
/// assert_eq!(refs.len(), 1);
/// assert_eq!(refs[0].url, "https://example.com/path(1)");
/// ```
#[must_use]
#[allow(dead_code)] // Reason: Used in tests, will be used in production soon
pub fn detect_reference_links(content: &str) -> Vec<ReferenceLink> {
    let mut refs = Vec::new();

    for line in content.lines() {
        if let Some(ref_link) = parse_reference_line(line) {
            refs.push(ref_link);
        }
    }

    refs
}

/// Parse a single reference link definition line.
///
/// Format: `[label]: url` or `[label]: url "title"` or `[label]: url (title)`
#[allow(dead_code)] // Reason: Used in tests
fn parse_reference_line(line: &str) -> Option<ReferenceLink> {
    let trimmed = line.trim();

    // Must start with '[' and have ']:' after the label
    let label_start = trimmed.find('[')?;
    let label_end = find_unescaped_bracket(trimmed, label_start + 1)?;

    // Check for ']:' after the label
    if !trimmed[label_end..].starts_with("]:") {
        return None;
    }

    // Extract label
    let label = trimmed[label_start + 1..label_end].trim().to_string();
    if label.is_empty() {
        return None;
    }

    // Everything after ']:' is the URL and optional title
    let after_colon = &trimmed[label_end + 2..].trim_start();

    // Parse URL and optional title
    let (url, title) = parse_url_and_title(after_colon)?;

    Some(ReferenceLink { label, url, title })
}

/// Find the closing ']' that matches the opening '[' at start position.
/// Handles escaped brackets: `\[` and `\]`
#[allow(dead_code)] // Reason: Used in tests
fn find_unescaped_bracket(s: &str, start: usize) -> Option<usize> {
    let chars: Vec<char> = s.chars().collect();
    let mut byte_pos = 0;
    let mut char_idx = 0;

    // Find the byte position corresponding to start character index
    for (idx, ch) in chars.iter().enumerate() {
        if idx == start {
            byte_pos = char_idx;
            break;
        }
        char_idx += ch.len_utf8();
    }

    // Now search from that position in the original string
    let rest = &s[byte_pos..];
    for (idx, ch) in rest.chars().enumerate() {
        if ch == ']' {
            // Check if it's escaped
            if idx > 0 {
                let prev_char = rest.chars().nth(idx - 1)?;
                if prev_char == '\\' {
                    continue;
                }
            }
            return Some(byte_pos + idx);
        }
    }
    None
}

/// Parse URL and optional title from reference link definition.
///
/// Returns (url, `optional_title`)
#[allow(dead_code)] // Reason: Used in tests
fn parse_url_and_title(s: &str) -> Option<(String, Option<String>)> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Check for title in quotes: "title"
    if let Some((url, title)) = parse_quoted_title(trimmed, '"') {
        return Some((url, Some(title)));
    }

    // Check for title in single quotes: 'title'
    if let Some((url, title)) = parse_quoted_title(trimmed, '\'') {
        return Some((url, Some(title)));
    }

    // Check for title in parentheses: (title)
    if let Some((url, title)) = parse_parenthesized_title(trimmed) {
        return Some((url, Some(title)));
    }

    // No title - the whole thing is the URL
    Some((trimmed.to_string(), None))
}

/// Parse title in quotes (double or single).
#[allow(dead_code)] // Reason: Used in tests
fn parse_quoted_title(s: &str, quote_char: char) -> Option<(String, String)> {
    // Find the last occurrence of the quote char that starts a title
    if let Some(quote_pos) = s.rfind(quote_char) {
        // Make sure there's something after the quote
        let after_quote = &s[quote_pos + 1..];
        if after_quote.trim().is_empty() || after_quote.trim().len() < 2 {
            // Check if the quote starts a title
            let before_quote = &s[..quote_pos].trim_end();
            if !before_quote.is_empty() {
                // The quoted part is the title
                if let Some(title_start) = s[..quote_pos].rfind(quote_char) {
                    if title_start < quote_pos {
                        let url = s[..title_start].trim_end().to_string();
                        let title = s[title_start + 1..quote_pos].to_string();
                        if !title.is_empty() {
                            return Some((url, title));
                        }
                    }
                }
            }
        }
    }
    None
}

/// Parse title in parentheses: (title)
#[allow(dead_code)] // Reason: Used in tests
fn parse_parenthesized_title(s: &str) -> Option<(String, String)> {
    // Find the last '(' that could start a title
    // Title in parens must be preceded by whitespace according to CommonMark
    if let Some(lparen_pos) = s.rfind(" (") {
        let url_part = &s[..lparen_pos];
        let title_part = &s[lparen_pos + 1..]; // Skip the space, include the (

        // Check if there's a matching ')' and nothing after it
        if let Some(rparen_pos) = title_part.rfind(')') {
            let after_rparen = &title_part[rparen_pos + 1..];
            if after_rparen.trim().is_empty() && rparen_pos > 0 {
                let url = url_part.trim_end().to_string();
                let title = title_part[1..rparen_pos].to_string();
                if !title.is_empty() && !title.contains('\n') {
                    return Some((url, title));
                }
            }
        }
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

    #[test]
    fn is_inside_link_url_detects_position() {
        let content = "[Link](https://example.com/path) text";
        let links = detect_links(content);
        assert_eq!(links.len(), 1);

        // Position inside URL
        let url_pos = content.find("example").unwrap();
        assert!(is_inside_link_url(content, url_pos, &links));

        // Position outside URL
        let outside_pos = content.find("text").unwrap();
        assert!(!is_inside_link_url(content, outside_pos, &links));
    }

    #[test]
    fn detect_reference_link_with_parens() {
        let content = "[ref]: https://example.com/path(1)";
        let refs = detect_reference_links(content);
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].label, "ref");
        assert_eq!(refs[0].url, "https://example.com/path(1)");
        assert_eq!(refs[0].title, None);
    }

    #[test]
    fn detect_reference_link_with_title() {
        let content = r#"[ref]: https://example.com "title here""#;
        let refs = detect_reference_links(content);
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].label, "ref");
        assert_eq!(refs[0].url, "https://example.com");
        assert_eq!(refs[0].title, Some("title here".to_string()));
    }

    #[test]
    fn detect_reference_link_with_parens_title() {
        let content = "[ref]: https://example.com (title here)";
        let refs = detect_reference_links(content);
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].label, "ref");
        assert_eq!(refs[0].url, "https://example.com");
        assert_eq!(refs[0].title, Some("title here".to_string()));
    }
}
