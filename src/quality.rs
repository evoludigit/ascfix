//! Quality validation for ASCII diagram processing
//!
//! This module provides comprehensive quality validation for diagram processing,
//! ensuring that all transformations maintain high quality standards and preserve
//! original content integrity.

// Quality validation module
use crate::cli::Mode;
use crate::config::Config;
use crate::modes;
use crate::transformation_analysis::{analyze_transformations, TransformationType};

/// Comprehensive quality report for diagram processing
#[derive(Debug, Clone)]
pub struct QualityReport {
    /// Overall quality score (0.0 to 1.0)
    pub score: f32,
    /// Specific quality issues found
    pub issues: Vec<QualityIssue>,
    /// Detailed quality metrics
    pub metrics: QualityMetrics,
}

/// Specific quality issues that can be detected
#[derive(Debug, Clone)]
pub enum QualityIssue {
    /// Text content was corrupted (e.g., arrows/pipes in text)
    TextCorruption {
        line: usize,
        expected: String,
        got: String,
    },
    /// Content was lost entirely
    DataLoss {
        content_type: LostContent,
        line: usize,
    },
    /// Structure corruption (e.g., malformed boxes)
    StructureCorruption {
        issue_type: StructureType,
        location: Point,
    },
    /// Visual inconsistency (e.g., misaligned elements)
    VisualInconsistency {
        issue_type: Inconsistency,
        location: Point,
    },
}

/// Types of content that can be lost
#[derive(Debug, Clone)]
pub enum LostContent {
    TextLine,
    BoxBorder,
    Arrow,
    ConnectionLine,
    Label,
}

/// Types of structure corruption
#[derive(Debug, Clone)]
pub enum StructureType {
    MalformedBox,
    BrokenArrow,
    InvalidConnection,
    NestedConflict,
}

/// Types of visual inconsistencies
#[derive(Debug, Clone)]
pub enum Inconsistency {
    MisalignedBoxes,
    InconsistentSpacing,
    BorderOverlap,
    TextOverflow,
}

/// Detailed quality metrics
#[derive(Debug, Clone)]
pub struct QualityMetrics {
    /// Percentage of original text preserved (0.0 to 1.0)
    pub text_preservation: f32,
    /// Percentage of structural elements preserved (0.0 to 1.0)
    pub structure_preservation: f32,
    /// Visual consistency score (0.0 to 1.0)
    pub visual_consistency: f32,
    /// Change in line count (can be negative)
    pub line_count_delta: i32,
    /// Number of text corruption instances
    pub text_corruption_count: usize,
    /// Number of data loss instances
    pub data_loss_count: usize,
}

/// 2D point for location reporting
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

/// Quality validation configuration
#[derive(Debug, Clone)]
pub struct QualityConfig {
    /// Minimum acceptable text preservation (default: 0.95)
    pub min_text_preservation: f32,
    /// Minimum acceptable structure preservation (default: 0.90)
    pub min_structure_preservation: f32,
    /// Maximum allowed line count delta (default: 0)
    pub max_line_count_delta: i32,
    /// Whether to allow any text corruption (default: false)
    pub allow_text_corruption: bool,
    /// Whether to allow any data loss (default: false)
    pub allow_data_loss: bool,
}

impl Default for QualityConfig {
    fn default() -> Self {
        Self {
            min_text_preservation: 0.95,
            min_structure_preservation: 0.90,
            max_line_count_delta: 0,
            allow_text_corruption: false,
            allow_data_loss: false,
        }
    }
}

impl QualityReport {
    /// Check if the quality meets acceptable standards
    pub fn is_acceptable(&self, config: &QualityConfig) -> bool {
        self.score >= 0.8
            && self.metrics.text_preservation >= config.min_text_preservation
            && self.metrics.structure_preservation >= config.min_structure_preservation
            && self.metrics.line_count_delta.abs() <= config.max_line_count_delta.abs()
            && (config.allow_text_corruption || self.metrics.text_corruption_count == 0)
            && (config.allow_data_loss || self.metrics.data_loss_count == 0)
    }
}

/// Validate the quality of diagram processing with intelligent transformation analysis
pub fn validate_quality(input: &str, output: &str) -> QualityReport {
    let mut report = QualityReport {
        score: 1.0,
        issues: Vec::new(),
        metrics: QualityMetrics {
            text_preservation: 1.0,
            structure_preservation: 1.0,
            visual_consistency: 1.0,
            line_count_delta: 0,
            text_corruption_count: 0,
            data_loss_count: 0,
        },
    };

    let input_lines: Vec<&str> = input.lines().collect();
    let output_lines: Vec<&str> = output.lines().collect();

    // Basic metrics
    report.metrics.line_count_delta = output_lines.len() as i32 - input_lines.len() as i32;

    // Advanced transformation analysis
    let transformation_analysis = analyze_transformations(input, output);

    // Update metrics based on transformation analysis
    report.metrics.text_corruption_count = transformation_analysis.summary.destructive_count;
    report.metrics.data_loss_count = transformation_analysis
        .transformations
        .iter()
        .filter(|t| matches!(t.transform_type, TransformationType::Destructive(_)))
        .count();

    // Calculate text preservation considering constructive transformations
    let original_text_chars = count_text_chars(&input_lines);
    let output_text_chars = count_text_chars(&output_lines);
    let base_preservation = if original_text_chars > 0 {
        output_text_chars as f32 / original_text_chars as f32
    } else {
        1.0
    };

    // Adjust preservation score based on constructive transformations
    // Constructive changes (like arrow duplication) shouldn't penalize preservation
    let constructive_bonus = transformation_analysis.summary.constructive_count as f32 * 0.02;
    report.metrics.text_preservation = (base_preservation + constructive_bonus).min(1.0);

    // Traditional checks (for compatibility)
    check_text_corruption(&input_lines, &output_lines, &mut report);
    check_data_loss(&input_lines, &output_lines, &mut report);
    check_visual_consistency(&output_lines, &mut report);

    // Enhanced structure preservation based on transformation analysis
    report.metrics.structure_preservation =
        calculate_structure_preservation(&transformation_analysis);

    // Calculate overall score with transformation awareness
    report.score = calculate_enhanced_overall_score(&report.metrics, &transformation_analysis);

    // Add transformation insights to issues
    for transformation in &transformation_analysis.transformations {
        if matches!(
            transformation.transform_type,
            TransformationType::Destructive(_)
        ) {
            report.issues.push(QualityIssue::TextCorruption {
                line: transformation.location.line,
                expected: "original content".to_string(),
                got: transformation.description.clone(),
            });
        }
    }

    report
}

/// Check for text corruption (arrows/pipes appearing in text content)
fn check_text_corruption(input_lines: &[&str], output_lines: &[&str], report: &mut QualityReport) {
    for (line_idx, output_line) in output_lines.iter().enumerate() {
        let chars: Vec<char> = output_line.chars().collect();

        for (col_idx, &ch) in chars.iter().enumerate() {
            // Check for corruption indicators - arrows that appear to be in text
            if ch == '↑' || ch == '↓' || ch == '←' || ch == '→' {
                // Only flag as corruption if arrow appears to be replacing a letter
                if is_arrow_corrupting_text(&chars, col_idx) {
                    report.issues.push(QualityIssue::TextCorruption {
                        line: line_idx,
                        expected: "text content".to_string(),
                        got: ch.to_string(),
                    });
                    report.metrics.text_corruption_count += 1;
                }
            }

            // Check for pipes that appear to be in text content (not borders)
            if ch == '│' {
                // Only flag if pipe appears to be in the middle of text
                if is_pipe_in_text_content(&chars, col_idx) {
                    report.issues.push(QualityIssue::TextCorruption {
                        line: line_idx,
                        expected: "text content".to_string(),
                        got: ch.to_string(),
                    });
                    report.metrics.text_corruption_count += 1;
                }
            }
        }
    }

    // Calculate text preservation score
    let original_text_chars = count_text_chars(input_lines);
    let output_text_chars = count_text_chars(output_lines);
    report.metrics.text_preservation = if original_text_chars > 0 {
        output_text_chars as f32 / original_text_chars as f32
    } else {
        1.0
    };
}

/// Check for data loss
fn check_data_loss(input_lines: &[&str], output_lines: &[&str], report: &mut QualityReport) {
    // Check if line count changed significantly
    let line_diff = output_lines.len().saturating_sub(input_lines.len());
    if line_diff > 2 {
        // Allow some line additions for formatting, but not losses
        report.metrics.data_loss_count += 1;
        report.issues.push(QualityIssue::DataLoss {
            content_type: LostContent::TextLine,
            line: input_lines.len(),
        });
    }

    // Check for significant content reduction
    let input_content_size = input_lines.iter().map(|l| l.len()).sum::<usize>();
    let output_content_size = output_lines.iter().map(|l| l.len()).sum::<usize>();

    if input_content_size > 100 && output_content_size < input_content_size / 2 {
        report.metrics.data_loss_count += 1;
        report.issues.push(QualityIssue::DataLoss {
            content_type: LostContent::TextLine,
            line: 0,
        });
    }
}

/// Check visual consistency
fn check_visual_consistency(output_lines: &[&str], report: &mut QualityReport) {
    // Check for obvious visual issues
    for (line_idx, line) in output_lines.iter().enumerate() {
        // Check for broken box borders
        if line.contains("┌") && !line.contains("┐") {
            report.metrics.visual_consistency -= 0.1;
            report.issues.push(QualityIssue::StructureCorruption {
                issue_type: StructureType::MalformedBox,
                location: Point {
                    row: line_idx,
                    col: 0,
                },
            });
        }

        // Check for inconsistent spacing (simplified)
        let spaces = line.chars().filter(|&c| c == ' ').count();
        let non_spaces = line.chars().filter(|&c| c != ' ').count();
        if non_spaces > 0 && spaces as f32 / non_spaces as f32 > 5.0 {
            report.metrics.visual_consistency -= 0.05;
        }
    }

    report.metrics.visual_consistency = report.metrics.visual_consistency.max(0.0);
}

/// Calculate structure preservation based on transformation analysis
fn calculate_structure_preservation(
    analysis: &crate::transformation_analysis::TransformationAnalysis,
) -> f32 {
    let destructive = analysis.summary.destructive_count as f32;
    let constructive = analysis.summary.constructive_count as f32;
    let neutral = analysis.summary.neutral_count as f32;
    let total = destructive + constructive + neutral;

    if total == 0.0 {
        return 1.0; // No transformations = perfect preservation
    }

    // Structure preservation considers constructive changes as neutral
    let preserved = constructive + neutral;
    preserved / total
}

/// Calculate enhanced overall score with transformation awareness
fn calculate_enhanced_overall_score(
    metrics: &QualityMetrics,
    analysis: &crate::transformation_analysis::TransformationAnalysis,
) -> f32 {
    let mut score = 1.0;

    // Base score from traditional metrics
    score *= metrics.text_preservation;
    score *= metrics.structure_preservation;
    score *= metrics.visual_consistency;

    // Adjust based on transformation quality impact
    score += analysis.summary.net_quality_impact * 0.1; // Scale the impact

    // Heavy penalty for destructive transformations
    let destructive_penalty = analysis.summary.destructive_count as f32 * 0.2;
    score -= destructive_penalty;

    // Light penalty for line count changes
    score -= (metrics.line_count_delta.abs() as f32 * 0.02).min(0.1);

    // Bonus for constructive transformations
    let constructive_bonus = analysis.summary.constructive_count as f32 * 0.05;
    score += constructive_bonus.min(0.2); // Cap the bonus

    score.max(0.0).min(1.0)
}

/// Calculate overall quality score (legacy function)
fn calculate_overall_score(metrics: &QualityMetrics) -> f32 {
    calculate_enhanced_overall_score(
        metrics,
        &crate::transformation_analysis::TransformationAnalysis {
            transformations: Vec::new(),
            summary: crate::transformation_analysis::TransformationSummary {
                destructive_count: metrics.text_corruption_count,
                constructive_count: 0,
                neutral_count: 0,
                net_quality_impact: 0.0,
                risk_score: metrics.text_corruption_count as f32 * 0.1,
            },
        },
    )
}

/// Check if an arrow appears to be corrupting text (replacing a letter)
fn is_arrow_corrupting_text(line_chars: &[char], col: usize) -> bool {
    if col == 0 || col >= line_chars.len() - 1 {
        return false;
    }

    let prev = line_chars[col - 1];
    let next = line_chars[col + 1];

    // If arrow is between letters, it's likely corruption
    prev.is_alphabetic() && next.is_alphabetic()
}

/// Check if a pipe appears to be in text content (not a legitimate border)
fn is_pipe_in_text_content(line_chars: &[char], col: usize) -> bool {
    if col == 0 || col >= line_chars.len() - 1 {
        return false;
    }

    let prev = line_chars[col - 1];
    let next = line_chars[col + 1];

    // Check for common text corruption patterns
    // Pipe between letters, or pipe where there should be a letter
    (prev.is_alphabetic() && next.is_alphabetic())
        || (prev.is_alphabetic() && next == ' ')
        || (prev == ' ' && next.is_alphabetic())
}

/// Check if a pipe is corrupting text content
fn is_pipe_in_text(line_chars: &[char], col: usize) -> bool {
    // Check if pipe is surrounded by text characters
    if col == 0 || col >= line_chars.len() - 1 {
        return false;
    }

    let prev = line_chars[col - 1];
    let next = line_chars[col + 1];

    // If surrounded by letters/spaces, likely text corruption
    (prev.is_alphabetic() || prev == ' ') && (next.is_alphabetic() || next == ' ')
}

/// Count text characters (letters, numbers, punctuation)
fn count_text_chars(lines: &[&str]) -> usize {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .filter(|&c| {
                    c.is_alphabetic() || c.is_numeric() || c.is_ascii_punctuation() || c == ' '
                })
                .count()
        })
        .sum()
}

/// Validate a fixture against quality standards
pub fn validate_fixture(
    input_path: &str,
    expected_path: &str,
    config: &QualityConfig,
) -> Result<(), String> {
    let input = std::fs::read_to_string(input_path)
        .map_err(|e| format!("Failed to read input {}: {}", input_path, e))?;

    let expected = std::fs::read_to_string(expected_path)
        .map_err(|e| format!("Failed to read expected {}: {}", expected_path, e))?;

    // Process the input
    let processed = modes::process_by_mode(&Mode::Diagram, &input, false, &Config::default());

    // Validate quality
    let report = validate_quality(&input, &processed);

    if !report.is_acceptable(config) {
        return Err(format!(
            "Quality validation failed for {}:\n\
             Score: {:.2}\n\
             Text preservation: {:.2} (min: {:.2})\n\
             Structure preservation: {:.2} (min: {:.2})\n\
             Line delta: {} (max: {})\n\
             Text corruption: {} (allowed: {})\n\
             Data loss: {} (allowed: {})\n\
             Issues: {}",
            input_path,
            report.score,
            report.metrics.text_preservation,
            config.min_text_preservation,
            report.metrics.structure_preservation,
            config.min_structure_preservation,
            report.metrics.line_count_delta,
            config.max_line_count_delta,
            report.metrics.text_corruption_count,
            config.allow_text_corruption,
            report.metrics.data_loss_count,
            config.allow_data_loss,
            report.issues.len()
        ));
    }

    // Also check that output matches expected (with normalization)
    let processed_normalized = normalize_output(&processed);
    let expected_normalized = normalize_output(&expected);

    if processed_normalized != expected_normalized {
        return Err(format!(
            "Output mismatch for {}\n\
             Expected length: {}\n\
             Got length: {}",
            input_path,
            expected_normalized.len(),
            processed_normalized.len()
        ));
    }

    Ok(())
}

/// Normalize output for comparison (strip trailing whitespace, normalize line endings)
fn normalize_output(output: &str) -> String {
    output
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_validation_clean_input() {
        let input = r#"
┌──────────┐
│ Clean    │
│ Text     │
└──────────┘
"#;

        let output = input; // Perfect preservation
        let report = validate_quality(input, output);

        assert!(report.score > 0.95, "Clean input should have high score");
        assert_eq!(report.metrics.text_corruption_count, 0);
        assert_eq!(report.metrics.data_loss_count, 0);
        assert!(report.metrics.text_preservation > 0.95);
    }

    #[test]
    fn test_quality_validation_text_corruption() {
        let input = r#"
┌──────────┐
│ Clean    │
│ Text     │
└──────────┘
"#;

        let output = r#"
┌──────────┐
│ Clean↑   │
│ Text     │
└──────────┘
"#;

        let report = validate_quality(input, output);

        assert!(
            report.score < 0.9,
            "Corrupted output should have lower score"
        );
        assert!(report.metrics.text_corruption_count > 0);
        assert!(report
            .issues
            .iter()
            .any(|issue| matches!(issue, QualityIssue::TextCorruption { .. })));
    }

    #[test]
    fn test_quality_validation_data_loss() {
        let input = r#"
┌──────────┐
│ Original │
│ Content  │
└──────────┘
"#;

        let output = r#"
┌──────────┐
│ Original │
└──────────┘
"#;

        let report = validate_quality(input, output);

        assert!(
            report.metrics.line_count_delta < 0,
            "Should detect line loss"
        );
        assert!(report.score < 1.0, "Data loss should reduce score");
    }
}
