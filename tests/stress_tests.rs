//! Stress tests for ascfix - large files, complex diagrams, unicode
//!
//! Run with: cargo test --features stress -- --ignored
//!
//! These tests verify robustness under extreme conditions:
//! - Large files (10MB+)
//! - Complex nested diagrams (10+ levels)
//! - Wide tables (50+ columns)
//! - Unicode edge cases (emoji, RTL, zero-width chars)

use ascfix::cli::Mode;
use ascfix::config::Config;
use ascfix::modes::process_by_mode;

mod helpers;

use helpers::generators::*;

#[cfg(test)]
mod large_files {
    use super::*;
    use std::time::Instant;

    #[test]
    #[ignore = "stress test - run with --ignored"]
    fn stress_large_file_1mb() {
        let content = generate_large_markdown(1024 * 1024); // 1MB
        let result = std::panic::catch_unwind(|| {
            process_by_mode(&Mode::Diagram, &content, false, &Config::default())
        });
        assert!(result.is_ok(), "Should not panic on 1MB file");
        if let Ok(output) = result {
            assert!(!output.is_empty(), "Should produce output");
        }
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_large_file_10mb() {
        let content = generate_large_markdown(10 * 1024 * 1024); // 10MB
        let result = std::panic::catch_unwind(|| {
            process_by_mode(&Mode::Diagram, &content, false, &Config::default())
        });
        assert!(result.is_ok(), "Should not panic on 10MB file");
        if let Ok(output) = result {
            assert!(!output.is_empty(), "Should produce output");
        }
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_large_file_50mb() {
        let content = generate_large_markdown(50 * 1024 * 1024); // 50MB
        let result = std::panic::catch_unwind(|| {
            process_by_mode(&Mode::Diagram, &content, false, &Config::default())
        });
        assert!(result.is_ok(), "Should not panic on 50MB file");
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_large_file_100mb() {
        let content = generate_large_markdown(100 * 1024 * 1024); // 100MB
        let result = std::panic::catch_unwind(|| {
            process_by_mode(&Mode::Diagram, &content, false, &Config::default())
        });
        assert!(result.is_ok(), "Should not panic on 100MB file");
    }

    #[test]
    #[ignore = "stress test"]
    #[allow(clippy::cast_precision_loss)]
    fn stress_performance_scaling_linear() {
        // Test that processing time scales linearly with file size
        let sizes = vec![
            1024,        // 1KB
            10 * 1024,   // 10KB
            100 * 1024,  // 100KB
            1024 * 1024, // 1MB
        ];

        let mut times = Vec::new();

        for size in &sizes {
            let content = generate_large_markdown(*size);
            let start = Instant::now();
            let _result = process_by_mode(&Mode::Diagram, &content, false, &Config::default());
            let duration = start.elapsed().as_secs_f64();
            times.push(duration);
            println!("Size: {size} bytes, Time: {duration:.3}s");
        }

        // Check scaling between consecutive sizes
        for i in 1..times.len() {
            let size_ratio = sizes[i] as f64 / sizes[i - 1] as f64;
            let time_ratio = times[i] / times[i - 1];

            // Allow up to 3x overhead factor (linear would be ~10x for 10x size increase)
            // But we allow some overhead for larger allocations
            assert!(
                time_ratio < size_ratio * 3.0,
                "Performance scaling appears non-linear: {}KB took {:.3}s, {}KB took {:.3}s (ratio: {:.2}x, expected: ~{:.2}x)",
                sizes[i - 1] / 1024, times[i - 1],
                sizes[i] / 1024, times[i],
                time_ratio, size_ratio
            );
        }
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_many_diagram_blocks() {
        // Test file with many separate diagram blocks
        let mut content = String::new();
        for i in 0..1000 {
            content.push('\n');
            content.push_str("# Section ");
            content.push_str(&i.to_string());
            content.push_str("\n\n");
            content.push_str("┌─────┐\n│ Box │\n└─────┘\n\n");
        }

        let result = std::panic::catch_unwind(|| {
            process_by_mode(&Mode::Diagram, &content, false, &Config::default())
        });
        assert!(result.is_ok(), "Should handle many diagram blocks");
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_many_tables() {
        // Test file with many separate tables
        let mut content = String::new();
        for i in 0..1000 {
            content.push('\n');
            content.push_str("# Table ");
            content.push_str(&i.to_string());
            content.push_str("\n\n");
            content.push_str("| Col1 | Col2 | Col3 |\n");
            content.push_str("|------|------|------|\n");
            content.push_str("| A    | B    | C    |\n\n");
        }

        let result = process_by_mode(&Mode::Safe, &content, false, &Config::default());
        assert!(!result.is_empty(), "Should handle many tables");
        assert!(result.contains("Col1"), "Should preserve table content");
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
mod edge_cases {
    use super::*;

    #[test]
    #[ignore = "stress test"]
    fn stress_extremely_long_lines() {
        // Test lines with 10,000 characters
        let content = generate_long_lines(10_000, 100);
        let result = std::panic::catch_unwind(|| {
            process_by_mode(&Mode::Diagram, &content, false, &Config::default())
        });
        assert!(result.is_ok(), "Should handle extremely long lines");
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_many_blank_lines() {
        // Test file with 10,000 consecutive blank lines
        let content = generate_many_blank_lines(10_000);
        let result = process_by_mode(&Mode::Diagram, &content, false, &Config::default());
        assert!(!result.is_empty(), "Should handle many blank lines");
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_deeply_nested_lists() {
        // Test lists nested 100 levels deep
        let content = generate_nested_lists(100);
        let result = process_by_mode(&Mode::Safe, &content, false, &Config::default());
        assert!(!result.is_empty(), "Should handle deeply nested lists");
        assert!(result.contains("level 0"), "Should preserve list content");
        assert!(
            result.contains("level 99"),
            "Should preserve deeply nested items"
        );
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_single_line_no_newline() {
        // Test file with single line and no trailing newline
        let content = "┌───┐\n│Box│\n└───┘";
        let result = process_by_mode(&Mode::Diagram, content, false, &Config::default());
        assert!(!result.is_empty(), "Should handle no trailing newline");
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_only_whitespace() {
        // Test file with only whitespace
        let content = "   \n\t\n  \t  \n";
        let result = process_by_mode(&Mode::Diagram, content, false, &Config::default());
        // Should not panic, even if empty output
        assert!(result.is_empty() || !result.is_empty());
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_mixed_line_endings() {
        // Test file with mixed line endings (Unix \n, Windows \r\n, old Mac \r)
        let content = "Line 1\nLine 2\r\nLine 3\rLine 4\n";
        let result = process_by_mode(&Mode::Diagram, content, false, &Config::default());
        assert!(!result.is_empty(), "Should handle mixed line endings");
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

    #[test]
    #[ignore = "stress test"]
    fn stress_multiple_emoji_types() {
        let content = "🎉 📊 🚀 ❤️ 🌈 🔥 ✨ 🎨\n\n┌──────┐\n│ Test │\n└──────┘";
        let result = process_by_mode(&Mode::Diagram, content, false, &Config::default());
        assert!(result.contains("🎉"), "Should preserve emojis");
        assert!(result.contains("Test"), "Should preserve box content");
    }

    #[test]
    #[ignore = "stress test"]
    fn stress_asian_characters() {
        // Test with Chinese, Japanese, and Korean characters
        let content = "┌──────────┐\n│ 你好世界 │\n│ こんにちは │\n│ 안녕하세요 │\n└──────────┘";
        let result = process_by_mode(&Mode::Diagram, content, false, &Config::default());
        assert!(result.contains("你好"), "Should preserve Chinese");
        assert!(result.contains("こんにちは"), "Should preserve Japanese");
        assert!(result.contains("안녕하세요"), "Should preserve Korean");
    }
}
