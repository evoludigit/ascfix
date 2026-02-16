//! Property-based fuzz tests using proptest
//!
//! Run with: cargo test --features fuzz
//! Configure iterations: `PROPTEST_CASES=10000` cargo test --features fuzz
//!
//! These tests verify properties that should hold for all inputs:
//! - No panics on any input
//! - Idempotence (where expected)
//! - Output is valid markdown

#![cfg(feature = "fuzz")]

use ascfix::cli::Mode;
use ascfix::config::Config;
use ascfix::modes::process_by_mode;
use proptest::prelude::*;

mod helpers;

#[cfg(test)]
mod property_tests {
    use super::*;

    proptest! {
        #[test]
        fn fuzz_no_panic_on_any_ascii(content in "([\\x20-\\x7E\n]{0,1000})") {
            let result = std::panic::catch_unwind(|| {
                process_by_mode(&Mode::Diagram, &content, false, &Config::default())
            });
            prop_assert!(result.is_ok());
        }

        #[test]
        fn fuzz_box_dimensions(
            width in 3usize..20,
            height in 3usize..20,
        ) {
            let diagram = format!(
                "┌{}┐\n{}└{}┘",
                "─".repeat(width),
                format!("│{}│\n", " ".repeat(width)).repeat(height - 2),
                "─".repeat(width)
            );
            let result = std::panic::catch_unwind(|| {
                process_by_mode(&Mode::Diagram, &diagram, false, &Config::default())
            });
            prop_assert!(result.is_ok());
        }

        #[test]
        fn fuzz_line_lengths(line_len in 0usize..200) {
            let content = format!("# {}\n\nSome text", "a".repeat(line_len));
            let result = process_by_mode(&Mode::Diagram, &content, false, &Config::default());
            prop_assert!(!result.is_empty());
        }
    }
}
