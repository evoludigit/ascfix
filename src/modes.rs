//! Mode-specific processing implementations.

use crate::cli::Mode;

/// Process content according to the specified mode.
///
/// # Modes
/// - `Safe`: Only normalize Markdown tables (minimal changes, no diagrams)
/// - `Diagram`: Detect and normalize ASCII diagrams (boxes, arrows, text)
/// - `Check`: Validate content but don't modify (used with --check flag)
#[allow(dead_code)] // Reason: Used by main processing pipeline
pub fn process_by_mode(mode: &Mode, content: &str) -> String {
    match mode {
        Mode::Safe => process_safe_mode(content),
        Mode::Diagram => process_diagram_mode(content),
        Mode::Check => process_check_mode(content),
    }
}

/// Safe mode: Only normalize Markdown tables, leave diagrams untouched.
fn process_safe_mode(content: &str) -> String {
    // For now, return content unchanged
    // TODO: Implement Markdown table normalization
    content.to_string()
}

/// Diagram mode: Detect and normalize ASCII diagrams (full pipeline).
fn process_diagram_mode(content: &str) -> String {
    // For now, return content unchanged
    // TODO: Implement full diagram detection, normalization, rendering pipeline
    content.to_string()
}

/// Check mode: Validate without modifying (used with --check flag).
fn process_check_mode(content: &str) -> String {
    // Check mode uses the same processing as diagram mode but doesn't write
    // The caller will compare input vs output
    process_diagram_mode(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_mode_preserves_content() {
        let content = "# Test\n\nSome content";
        let result = process_by_mode(&Mode::Safe, content);
        // Safe mode should preserve content for now
        assert_eq!(result, content);
    }

    #[test]
    fn test_diagram_mode_preserves_content() {
        let content = "# Test\n\nSome content";
        let result = process_by_mode(&Mode::Diagram, content);
        // Diagram mode should preserve content when no diagrams exist
        assert_eq!(result, content);
    }

    #[test]
    fn test_check_mode_preserves_content() {
        let content = "# Test\n\nSome content";
        let result = process_by_mode(&Mode::Check, content);
        // Check mode should use same processing as diagram
        assert_eq!(result, content);
    }

    #[test]
    fn test_all_modes_are_safe() {
        let content = "# Header\n\nText content\n\nMore text";
        let safe_result = process_by_mode(&Mode::Safe, content);
        let diagram_result = process_by_mode(&Mode::Diagram, content);
        let check_result = process_by_mode(&Mode::Check, content);

        // All modes should handle content safely (no crashes, no panics)
        assert!(!safe_result.is_empty());
        assert!(!diagram_result.is_empty());
        assert!(!check_result.is_empty());
    }
}
