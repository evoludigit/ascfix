//! Check mode: Validate without modifying (used with --check flag).
//!
//! Check mode processes content the same way as diagram mode but doesn't write changes.
//! The caller compares input vs output to determine if fixes are needed.

/// Check mode: Validate without modifying (used with --check flag).
pub fn process_check_mode(content: &str) -> String {
    // Check mode uses the same processing as diagram mode but doesn't write
    // The caller will compare input vs output
    let default_config = crate::config::Config::default();
    super::diagram::process_diagram_mode(content, &default_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_mode_preserves_content() {
        let content = "# Test\n\nSome content";
        let result = process_check_mode(content);
        // Check mode should use same processing as diagram
        assert_eq!(result, content);
    }

    #[test]
    fn test_check_mode_returns_unchanged_content() {
        let content = "# Test\n\nNo diagrams here";
        let result = process_check_mode(content);
        // Check mode processes same as diagram mode but returns content
        assert_eq!(result, content);
    }
}
