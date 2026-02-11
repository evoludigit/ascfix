//! Tests using malformed fixture files to verify ascfix robustness

use ascfix::cli::Mode;
use ascfix::config::Config;
use ascfix::modes::process_by_mode;

#[cfg(test)]
mod malformed_fixture_tests {
    use super::*;

    fn process_fixture_content(content: &str, mode: &Mode, repair_fences: bool) -> String {
        let config = Config::default();
        process_by_mode(mode, content, repair_fences, &config)
    }

    #[test]
    fn test_malformed_broken_box() {
        let content = include_str!("fixtures/malformed_broken_box.md");
        // Test that ascfix handles broken boxes gracefully
        let result = process_fixture_content(content, &Mode::Diagram, false);
        // Should not crash, may or may not fix the broken box
        assert!(!result.is_empty());
    }

    #[test]
    fn test_malformed_broken_arrows() {
        let content = include_str!("fixtures/malformed_broken_arrows.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Check that some arrows are preserved even if malformed
        assert!(result.contains("│A │"));
    }

    #[test]
    fn test_malformed_broken_tables() {
        // Test dirty input produces expected clean output
        let dirty_content = include_str!("fixtures/dirty/malformed_broken_tables.md");
        let expected_clean = include_str!("fixtures/clean/malformed_broken_tables.md");

        let result = process_fixture_content(dirty_content, &Mode::Safe, false);
        assert_eq!(result.trim(), expected_clean.trim());
    }

    #[test]
    fn test_malformed_broken_fences() {
        let content = include_str!("fixtures/malformed_broken_fences.md");
        let result = process_fixture_content(content, &Mode::Diagram, true);
        assert!(!result.is_empty());
        // Fence repair enabled
        assert!(result.contains("```"));
    }

    #[test]
    fn test_edge_case_minimal() {
        let content = include_str!("fixtures/edge_case_minimal.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_malformed_overlapping() {
        let content = include_str!("fixtures/malformed_overlapping.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle overlapping elements without crashing
    }

    #[test]
    fn test_malformed_nested() {
        let content = include_str!("fixtures/malformed_nested.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle nested structures
        assert!(result.contains("┌"));
    }

    #[test]
    fn test_malformed_box_alignment() {
        // Test dirty input produces expected clean output
        let dirty_content = include_str!("fixtures/dirty/malformed_box_alignment.md");
        let expected_clean = include_str!("fixtures/clean/malformed_box_alignment.md");

        let result = process_fixture_content(dirty_content, &Mode::Diagram, false);
        assert_eq!(result.trim(), expected_clean.trim());
    }

    #[test]
    fn test_complex_nested_with_labels() {
        // Test that complex nested diagrams don't crash ascfix
        let content = include_str!("fixtures/complex_nested_with_labels.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle nested boxes without crashing
        assert!(result.contains("┌"));
        assert!(result.contains("└"));
    }

    #[test]
    fn test_complex_connection_lines() {
        let content = include_str!("fixtures/complex_connection_lines.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle L-shaped connections and elbows
        assert!(result.contains("┌"));
        assert!(result.contains("└"));
        assert!(result.contains("─"));
        assert!(result.contains("│"));
    }

    #[test]
    fn test_complex_mixed_content() {
        let content = include_str!("fixtures/complex_mixed_content.md");
        let result = process_fixture_content(content, &Mode::Diagram, true);
        assert!(!result.is_empty());
        // Should handle diagrams, tables, and code blocks
        assert!(result.contains('┌'));
        assert!(result.contains('|'));
        assert!(result.contains("```"));
    }

    #[test]
    fn test_complex_unicode_diagrams() {
        let content = include_str!("fixtures/complex_unicode_diagrams.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle Unicode box characters
        assert!(result.contains("╔") || result.contains("╭"));
    }

    #[test]
    fn test_complex_large_diagram() {
        let content = include_str!("fixtures/complex_large_diagram.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle very large diagrams
        assert!(result.contains("┌"));
        assert!(result.lines().count() > 10);
    }

    #[test]
    fn test_edge_case_minimal_extended() {
        let content = include_str!("fixtures/edge_case_minimal_extended.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle minimal diagrams gracefully
    }

    #[test]
    fn test_complex_overlapping_elements() {
        let content = include_str!("fixtures/complex_overlapping_elements.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle overlapping elements
        assert!(result.contains("┌"));
    }

    #[test]
    fn test_complex_arrow_patterns() {
        let content = include_str!("fixtures/complex_arrow_patterns.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle various arrow styles
        assert!(result.contains('─') || result.contains('>') || result.contains('▶'));
    }

    #[test]
    fn test_complex_mixed_box_styles() {
        let content = include_str!("fixtures/complex_mixed_box_styles.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle different box styles
        assert!(result.contains('┌') || result.contains('╔') || result.contains('╭'));
    }

    #[test]
    fn test_complex_table_issues() {
        let content = include_str!("fixtures/complex_table_issues.md");
        let result = process_fixture_content(content, &Mode::Safe, false);
        assert!(!result.is_empty());
        // Should handle table normalization
        assert!(result.contains('|'));
        assert!(result.contains("Header") || result.contains("Column"));
    }

    #[test]
    fn test_complex_code_fence_issues() {
        let content = include_str!("fixtures/complex_code_fence_issues.md");
        let result = process_fixture_content(content, &Mode::Diagram, true);
        assert!(!result.is_empty());
        // Should handle fence repair
        assert!(result.contains("```") || result.contains("~~~"));
    }

    #[test]
    fn test_complex_links_in_diagrams() {
        let content = include_str!("fixtures/complex_links_in_diagrams.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle links in diagrams
        assert!(result.contains('[') || result.contains(']'));
        assert!(result.contains('┌'));
    }

    #[test]
    fn test_stress_large_table() {
        let content = include_str!("fixtures/stress_large_table.md");
        let result = process_fixture_content(content, &Mode::Safe, false);
        assert!(!result.is_empty());
        // Should handle very wide tables
        assert!(result.contains('|'));
        assert!(result.contains("Component"));
    }

    #[test]
    fn test_regression_github_flavored() {
        let content = include_str!("fixtures/regression_github_flavored.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle GFM features
        assert!(result.contains('┌') || result.contains('-') || result.contains('`'));
    }

    #[test]
    fn test_domain_devops_pipeline() {
        let content = include_str!("fixtures/domain_devops_pipeline.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle CI/CD pipeline diagrams
        assert!(result.contains('┌'));
        assert!(result.contains('─'));
    }

    #[test]
    fn test_domain_data_structures() {
        let content = include_str!("fixtures/domain_data_structures.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle tree structures
        assert!(result.contains('┌'));
        assert!(result.contains('└'));
    }

    #[test]
    fn test_domain_networking_osi() {
        let content = include_str!("fixtures/domain_networking_osi.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle layered network diagrams
        assert!(result.contains('┌'));
        assert!(result.contains('|'));
    }

    #[test]
    fn test_boundary_max_nesting() {
        let content = include_str!("fixtures/boundary_max_nesting.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle deeply nested structures gracefully
        assert!(result.contains('┌'));
    }

    #[test]
    fn test_edge_case_mathematical() {
        let content = include_str!("fixtures/edge_case_mathematical.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle mathematical notation
        assert!(result.contains('┌') || result.contains('∫') || result.contains('∑'));
    }

    #[test]
    fn test_international_multilingual() {
        let content = include_str!("fixtures/international_multilingual.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle international characters
        assert!(result.contains('┌'));
    }

    #[test]
    fn test_error_recovery_corrupted() {
        let content = include_str!("fixtures/error_recovery_corrupted.md");
        let result = process_fixture_content(content, &Mode::Diagram, true);
        assert!(!result.is_empty());
        // Should handle corrupted content gracefully
    }

    #[test]
    fn test_whitespace_handling() {
        let content = include_str!("fixtures/whitespace_handling.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle various whitespace patterns
        assert!(result.contains('┌'));
    }

    #[test]
    fn test_realworld_api_docs() {
        let content = include_str!("fixtures/realworld_api_docs.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle API documentation patterns
        assert!(result.contains('┌'));
        assert!(result.contains('|'));
    }

    #[test]
    fn test_realworld_database_schema() {
        let content = include_str!("fixtures/realworld_database_schema.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle database schema diagrams
        assert!(result.contains('┌'));
        assert!(result.contains("PK") || result.contains("FK"));
    }

    #[test]
    fn test_realworld_security_architecture() {
        let content = include_str!("fixtures/realworld_security_architecture.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle security architecture diagrams
        assert!(result.contains('┌'));
    }

    #[test]
    fn test_realworld_ml_pipeline() {
        let content = include_str!("fixtures/realworld_ml_pipeline.md");
        let result = process_fixture_content(content, &Mode::Diagram, false);
        assert!(!result.is_empty());
        // Should handle ML pipeline diagrams
        assert!(result.contains('┌'));
        assert!(result.contains('|'));
    }

    #[test]
    fn test_all_fixtures_exist() {
        // Verify all fixture files can be read
        let fixtures = vec![
            include_str!("fixtures/malformed_broken_box.md"),
            include_str!("fixtures/malformed_broken_arrows.md"),
            include_str!("fixtures/malformed_broken_tables.md"),
            include_str!("fixtures/malformed_broken_fences.md"),
            include_str!("fixtures/edge_case_minimal.md"),
            include_str!("fixtures/malformed_overlapping.md"),
            include_str!("fixtures/malformed_nested.md"),
            include_str!("fixtures/malformed_box_alignment.md"),
            // New complex fixtures
            include_str!("fixtures/complex_nested_with_labels.md"),
            include_str!("fixtures/complex_connection_lines.md"),
            include_str!("fixtures/complex_mixed_content.md"),
            include_str!("fixtures/complex_unicode_diagrams.md"),
            include_str!("fixtures/complex_large_diagram.md"),
            include_str!("fixtures/edge_case_minimal_extended.md"),
            include_str!("fixtures/complex_overlapping_elements.md"),
            include_str!("fixtures/complex_arrow_patterns.md"),
            include_str!("fixtures/complex_mixed_box_styles.md"),
            include_str!("fixtures/complex_table_issues.md"),
            include_str!("fixtures/complex_code_fence_issues.md"),
            include_str!("fixtures/complex_links_in_diagrams.md"),
            // Stress and performance fixtures
            include_str!("fixtures/stress_large_table.md"),
            // Regression fixtures
            include_str!("fixtures/regression_github_flavored.md"),
            // Domain-specific fixtures
            include_str!("fixtures/domain_devops_pipeline.md"),
            include_str!("fixtures/domain_data_structures.md"),
            include_str!("fixtures/domain_networking_osi.md"),
            // Boundary and edge case fixtures
            include_str!("fixtures/boundary_max_nesting.md"),
            include_str!("fixtures/edge_case_mathematical.md"),
            // International fixtures
            include_str!("fixtures/international_multilingual.md"),
            // Error recovery fixtures
            include_str!("fixtures/error_recovery_corrupted.md"),
            // Whitespace fixtures
            include_str!("fixtures/whitespace_handling.md"),
            // Real-world fixtures
            include_str!("fixtures/realworld_api_docs.md"),
            include_str!("fixtures/realworld_database_schema.md"),
            include_str!("fixtures/realworld_security_architecture.md"),
            include_str!("fixtures/realworld_ml_pipeline.md"),
        ];

        for fixture in fixtures {
            assert!(!fixture.is_empty(), "Fixture file should not be empty");
            assert!(fixture.contains('#'), "Fixture should have header comment");
        }

        // Verify dirty/clean pairs exist for key test cases
        #[allow(clippy::no_effect_underscore_binding)]
        {
            let _dirty_tables = include_str!("fixtures/dirty/malformed_broken_tables.md");
            let _clean_tables = include_str!("fixtures/clean/malformed_broken_tables.md");
            let _dirty_boxes = include_str!("fixtures/dirty/malformed_box_alignment.md");
            let _clean_boxes = include_str!("fixtures/clean/malformed_box_alignment.md");
            let _dirty_nested = include_str!("fixtures/dirty/complex_nested_with_labels.md");
            let _clean_nested = include_str!("fixtures/clean/complex_nested_with_labels.md");
        }
    }
}
