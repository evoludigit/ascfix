//! Stress tests for ascfix - large files, complex diagrams, unicode
//!
//! Run with: cargo test --features stress -- --ignored
//!
//! These tests verify robustness under extreme conditions:
//! - Large files (10MB+)
//! - Complex nested diagrams (10+ levels)
//! - Wide tables (50+ columns)
//! - Unicode edge cases (emoji, RTL, zero-width chars)

use ascfix::modes::process_by_mode;
use ascfix::cli::Mode;
use ascfix::config::Config;

mod helpers;

use helpers::generators::*;

#[cfg(test)]
mod large_files {
    use super::*;

    #[test]
    #[ignore = "stress test - run with --ignored"]
    fn stress_large_file_1mb() {
        let content = generate_large_markdown(1024 * 1024); // 1MB
        let result = std::panic::catch_unwind(|| {
            process_by_mode(&Mode::Diagram, &content, false, &Config::default())
        });
        assert!(result.is_ok(), "Should not panic on 1MB file");
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_large_file_10mb() {
        let content = generate_large_markdown(10 * 1024 * 1024); // 10MB
        let result = std::panic::catch_unwind(|| {
            process_by_mode(&Mode::Diagram, &content, false, &Config::default())
        });
        assert!(result.is_ok(), "Should not panic on 10MB file");
    }
}

#[cfg(test)]
mod complex_diagrams {
    use super::*;

    #[test]
    #[ignore = "stress test"]
    fn stress_deeply_nested_boxes() {
        let diagram = generate_nested_boxes(10); // 10 levels deep
        let result = std::panic::catch_unwind(|| {
            process_by_mode(&Mode::Diagram, &diagram, false, &Config::default())
        });
        assert!(result.is_ok(), "Should not panic on deeply nested boxes");
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_wide_table() {
        let table = generate_wide_table(50); // 50 columns
        let result = process_by_mode(&Mode::Safe, &table, false, &Config::default());
        assert!(!result.is_empty(), "Should handle wide tables");
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_large_diagram_grid() {
        let diagram = generate_box_grid(10, 10); // 10x10 grid of boxes
        let result = process_by_mode(&Mode::Diagram, &diagram, false, &Config::default());
        assert!(!result.is_empty(), "Should handle large diagram grids");
    }
}

#[cfg(test)]
mod unicode_stress {
    use super::*;

    #[test]
    #[ignore = "stress test"]
    fn stress_emoji_in_diagrams() {
        let content = "┌─────────┐\n│ 🎉 Test │\n└─────────┘";
        let result = process_by_mode(&Mode::Diagram, content, false, &Config::default());
        assert!(result.contains("🎉"), "Should preserve emoji");
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_rtl_text() {
        let content = "┌──────┐\n│ שָׁלוֹם │\n└──────┘"; // Hebrew "Shalom"
        let result = process_by_mode(&Mode::Diagram, content, false, &Config::default());
        assert!(!result.is_empty(), "Should handle RTL text");
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_zero_width_characters() {
        let content = "┌──────┐\n│ e\u{0301}test │\n└──────┘"; // e with combining acute
        let result = process_by_mode(&Mode::Diagram, content, false, &Config::default());
        assert!(!result.is_empty(), "Should handle combining characters");
    }
}
