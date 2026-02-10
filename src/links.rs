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

/// Detect inline links that are outside of code blocks.
///
/// This function first identifies code block regions, then only detects
/// links in the regions between code blocks.
///
/// # Examples
///
/// ```
/// use ascfix::links::detect_links_outside_code_blocks;
///
/// let content = "```\n[inside](url)\n```\n[outside](url2)";
/// let links = detect_links_outside_code_blocks(content);
/// assert_eq!(links.len(), 1);
/// assert_eq!(links[0].text, "outside");
/// ```
#[must_use]
#[allow(dead_code)] // Reason: Used in tests, will be used in production soon
pub fn detect_links_outside_code_blocks(content: &str) -> Vec<Link> {
    // First, detect all links in the content
    let all_links = detect_links(content);

    // Get code block regions
    let code_regions = get_code_block_regions(content);

    // Filter out links that are inside code blocks
    all_links
        .into_iter()
        .filter(|link| !is_in_code_region(link.start_pos, &code_regions))
        .collect()
}

/// Represents a region of content inside a code block (`start_pos`, `end_pos`).
type CodeRegion = (usize, usize);

/// Get all code block regions as (start, end) character positions.
fn get_code_block_regions(content: &str) -> Vec<CodeRegion> {
    let mut regions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut in_code_block = false;
    let mut block_start_pos = 0;
    let mut current_pos = 0;

    for line in &lines {
        let line_start = current_pos;
        let line_end = current_pos + line.len();

        let trimmed = line.trim();

        // Check for fence markers
        if is_fence_line(trimmed) {
            if in_code_block {
                // Closing fence - end the region (include the newline after)
                let block_end = line_end;
                regions.push((block_start_pos, block_end));
                in_code_block = false;
            } else {
                // Opening fence
                in_code_block = true;
                block_start_pos = line_start;
            }
        }

        // Move to next line (+1 for newline character)
        current_pos = line_end + 1;
    }

    // Handle unclosed code blocks
    if in_code_block {
        regions.push((block_start_pos, content.len()));
    }

    regions
}

/// Check if a line is a code fence marker (triple backtick or tilde).
fn is_fence_line(line: &str) -> bool {
    if line.len() < 3 {
        return false;
    }

    // Check for backtick fence - 3+ backticks at start
    if line.starts_with("```") {
        // Count consecutive backticks from start
        let backtick_count = line.chars().take_while(|&c| c == '`').count();
        if backtick_count >= 3 {
            // Everything after the backticks should be the language specifier or empty
            let after_fences = &line[backtick_count..];
            // Language specifier can't contain backticks
            return !after_fences.contains('`');
        }
    }

    // Check for tilde fence - 3+ tildes at start
    if line.starts_with("~~~") {
        // Count consecutive tildes from start
        let tilde_count = line.chars().take_while(|&c| c == '~').count();
        if tilde_count >= 3 {
            // Everything after the tildes should be the language specifier or empty
            let after_fences = &line[tilde_count..];
            // Language specifier can't contain tildes
            return !after_fences.contains('~');
        }
    }

    false
}

/// Check if a position falls within any code block region.
fn is_in_code_region(pos: usize, regions: &[CodeRegion]) -> bool {
    regions
        .iter()
        .any(|(start, end)| pos >= *start && pos < *end)
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

    #[test]
    fn detect_links_outside_code_blocks_only() {
        let content =
            "```\n[text](https://inside.code.block)\n```\n\n[Outside](https://outside.link)";
        let links = detect_links_outside_code_blocks(content);
        // Should only find the outside link
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].text, "Outside");
        assert_eq!(links[0].url, "https://outside.link");
    }

    #[test]
    fn detect_links_skips_backtick_fence() {
        let content = "Some text\n\n```python\n# [Link](https://example.com)\n```\n\nMore text";
        let links = detect_links_outside_code_blocks(content);
        // Should not detect the link inside the code fence
        assert!(
            links.is_empty(),
            "Links inside code blocks should be skipped"
        );
    }

    #[test]
    fn detect_links_skips_tilde_fence() {
        let content = "Some text\n\n~~~\n[Link](https://example.com)\n~~~\n\nMore text";
        let links = detect_links_outside_code_blocks(content);
        // Should not detect the link inside the tilde fence
        assert!(
            links.is_empty(),
            "Links inside tilde code blocks should be skipped"
        );
    }

    #[test]
    fn detect_multiple_links_on_one_line() {
        let content = "[First](https://a.com) and [Second](https://b.com/path(1))";
        let links = detect_links(content);
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].text, "First");
        assert_eq!(links[0].url, "https://a.com");
        assert_eq!(links[1].text, "Second");
        assert_eq!(links[1].url, "https://b.com/path(1)");
    }

    #[test]
    fn detect_link_with_deeply_nested_parens() {
        let content = "[Link](https://example.com/a(b(c(d))))";
        let links = detect_links(content);
        assert_eq!(links.len(), 1);
        // URL has 3 opening and 3 closing parens inside, 4th ) closes the link
        assert_eq!(links[0].url, "https://example.com/a(b(c(d)))");
    }

    #[test]
    fn detect_link_with_special_chars_in_url() {
        let content = "[API](https://api.example.com/v1?foo=bar&baz=qux)";
        let links = detect_links(content);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].url, "https://api.example.com/v1?foo=bar&baz=qux");
    }

    #[test]
    fn detect_link_with_parens_in_text() {
        // Parentheses in link text should not affect URL parsing
        let content = "[click (here)](https://example.com)";
        let links = detect_links(content);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].text, "click (here)");
        assert_eq!(links[0].url, "https://example.com");
    }

    #[test]
    fn link_detection_is_idempotent() {
        // Running detect_links twice should produce identical results
        let content = "[Link](https://example.com/path(1)) and [Another](https://test.org)";

        let first_pass = detect_links(content);
        // Create a new content string from the detected links (simulating processing)
        let processed: String = first_pass
            .iter()
            .map(|l| format!("[{}]({})", l.text, l.url))
            .collect::<Vec<_>>()
            .join(" and ");

        let second_pass = detect_links(&processed);

        // Should have same number of links
        assert_eq!(first_pass.len(), second_pass.len());

        // Each link should have same text and URL
        for (first, second) in first_pass.iter().zip(second_pass.iter()) {
            assert_eq!(first.text, second.text);
            assert_eq!(first.url, second.url);
        }
    }

    #[test]
    fn link_detection_preserves_content_unchanged() {
        // Content without links should be unchanged
        let content = "Just plain text without any links";
        let links = detect_links(content);
        assert!(links.is_empty());
    }
}
