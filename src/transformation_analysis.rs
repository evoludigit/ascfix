//! Advanced transformation analysis for quality validation
//!
//! This module analyzes the semantic meaning of transformations to distinguish
//! between destructive changes (bad), constructive changes (good), and neutral
//! changes (acceptable).

// Transformation analysis module

/// Classification of transformation types
#[derive(Debug, Clone, PartialEq)]
pub enum TransformationType {
    /// Destructive changes that must be prevented
    Destructive(DestructiveReason),
    /// Constructive changes that improve quality
    Constructive(ConstructiveReason),
    /// Neutral changes that don't affect meaning
    Neutral(NeutralReason),
}

/// Reasons for destructive transformations
#[derive(Debug, Clone, PartialEq)]
pub enum DestructiveReason {
    ContentDeletion,
    CharacterCorruption,
    SemanticChange,
    DataLoss,
}

/// Reasons for constructive transformations
#[derive(Debug, Clone, PartialEq)]
pub enum ConstructiveReason {
    ArrowDuplication, // ↓ → ↓↓ for alignment
    BoxExpansion,     // Parent boxes grow to contain children
    WhitespaceNormalization,
    VisualAlignment,  // Side-by-side box alignment
    BorderCorrection, // Fixing malformed borders
}

/// Reasons for neutral transformations
#[derive(Debug, Clone, PartialEq)]
pub enum NeutralReason {
    CaseNormalization,
    EncodingNormalization,
    LineEndingNormalization,
    WhitespaceNormalization,
}

/// Analysis of transformations between input and output
#[derive(Debug, Clone)]
pub struct TransformationAnalysis {
    pub transformations: Vec<Transformation>,
    pub summary: TransformationSummary,
}

/// Individual transformation instance
#[derive(Debug, Clone)]
pub struct Transformation {
    pub transform_type: TransformationType,
    pub location: Location,
    pub description: String,
    pub impact_score: f32, // -1.0 (very bad) to +1.0 (very good)
}

/// Location of transformation
#[derive(Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub col_start: usize,
    pub col_end: usize,
}

/// Summary of all transformations
#[derive(Debug, Clone)]
pub struct TransformationSummary {
    pub destructive_count: usize,
    pub constructive_count: usize,
    pub neutral_count: usize,
    pub net_quality_impact: f32, // Overall quality change
    pub risk_score: f32,         // 0.0 (safe) to 1.0 (risky)
}

/// Analyze transformations between input and output
pub fn analyze_transformations(input: &str, output: &str) -> TransformationAnalysis {
    let input_lines: Vec<&str> = input.lines().collect();
    let output_lines: Vec<&str> = output.lines().collect();

    let mut transformations = Vec::new();

    // Analyze line-by-line changes
    for (line_idx, (input_line, output_line)) in
        input_lines.iter().zip(output_lines.iter()).enumerate()
    {
        analyze_line_transformations(line_idx, input_line, output_line, &mut transformations);
    }

    // Handle different line counts
    let max_lines = input_lines.len().max(output_lines.len());
    if input_lines.len() != output_lines.len() {
        transformations.push(Transformation {
            transform_type: if output_lines.len() > input_lines.len() {
                TransformationType::Constructive(ConstructiveReason::VisualAlignment)
            } else {
                TransformationType::Destructive(DestructiveReason::DataLoss)
            },
            location: Location {
                line: max_lines,
                col_start: 0,
                col_end: 0,
            },
            description: format!(
                "Line count changed: {} → {}",
                input_lines.len(),
                output_lines.len()
            ),
            impact_score: if output_lines.len() > input_lines.len() {
                0.1
            } else {
                -0.5
            },
        });
    }

    // Calculate summary
    let summary = calculate_summary(&transformations);

    TransformationAnalysis {
        transformations,
        summary,
    }
}

/// Analyze transformations within a single line
fn analyze_line_transformations(
    line_idx: usize,
    input_line: &str,
    output_line: &str,
    transformations: &mut Vec<Transformation>,
) {
    let input_chars: Vec<char> = input_line.chars().collect();
    let output_chars: Vec<char> = output_line.chars().collect();

    // Find differences
    let mut i = 0;
    let mut j = 0;

    while i < input_chars.len() && j < output_chars.len() {
        if input_chars[i] != output_chars[j] {
            // Found a difference - analyze the transformation
            analyze_character_difference(
                line_idx,
                i,
                j,
                &input_chars,
                &output_chars,
                transformations,
            );

            // Skip ahead to find next match
            while i < input_chars.len()
                && j < output_chars.len()
                && input_chars[i] != output_chars[j]
            {
                i += 1;
                j += 1;
            }
        } else {
            i += 1;
            j += 1;
        }
    }

    // Handle remaining characters
    if i < input_chars.len() || j < output_chars.len() {
        transformations.push(Transformation {
            transform_type: TransformationType::Destructive(DestructiveReason::DataLoss),
            location: Location {
                line: line_idx,
                col_start: i.min(j),
                col_end: input_chars.len().max(output_chars.len()),
            },
            description: format!(
                "Content length mismatch: {} vs {} chars",
                input_chars.len(),
                output_chars.len()
            ),
            impact_score: -0.3,
        });
    }
}

/// Analyze a specific character difference
fn analyze_character_difference(
    line_idx: usize,
    input_pos: usize,
    output_pos: usize,
    input_chars: &[char],
    output_chars: &[char],
    transformations: &mut Vec<Transformation>,
) {
    let input_char = input_chars.get(input_pos).copied().unwrap_or(' ');
    let output_char = output_chars.get(output_pos).copied().unwrap_or(' ');

    // Analyze the transformation type
    let (transform_type, impact_score) = classify_character_transformation(
        input_char,
        output_char,
        input_chars,
        output_chars,
        input_pos,
        output_pos,
    );

    transformations.push(Transformation {
        transform_type,
        location: Location {
            line: line_idx,
            col_start: input_pos,
            col_end: output_pos + 1,
        },
        description: format!("'{}' → '{}'", input_char, output_char),
        impact_score,
    });
}

/// Classify the type of character transformation
fn classify_character_transformation(
    input_char: char,
    output_char: char,
    input_chars: &[char],
    output_chars: &[char],
    input_pos: usize,
    output_pos: usize,
) -> (TransformationType, f32) {
    // First check for constructive transformations
    if let Some(result) = check_constructive_transformation(
        input_char,
        output_char,
        input_chars,
        output_chars,
        input_pos,
        output_pos,
    ) {
        return result;
    }

    // Then check for destructive transformations
    if let Some(result) = check_destructive_transformation(
        input_char,
        output_char,
        input_chars,
        output_chars,
        input_pos,
        output_pos,
    ) {
        return result;
    }

    // Default to neutral for unrecognized changes
    (
        TransformationType::Neutral(NeutralReason::EncodingNormalization),
        0.0,
    )
}

/// Check for constructive transformations
fn check_constructive_transformation(
    input_char: char,
    output_char: char,
    input_chars: &[char],
    output_chars: &[char],
    input_pos: usize,
    output_pos: usize,
) -> Option<(TransformationType, f32)> {
    // Arrow duplication for alignment (constructive)
    if input_char == '↓'
        && output_char == '↓'
        && is_arrow_duplication(input_chars, output_chars, input_pos, output_pos)
    {
        return Some((
            TransformationType::Constructive(ConstructiveReason::ArrowDuplication),
            0.3,
        ));
    }

    // Arrow addition for alignment (constructive)
    if input_char == ' '
        && matches!(output_char, '↓' | '↑' | '←' | '→')
        && is_arrow_duplication(input_chars, output_chars, input_pos, output_pos)
    {
        return Some((
            TransformationType::Constructive(ConstructiveReason::ArrowDuplication),
            0.3,
        ));
    }

    // Box expansion (constructive)
    if is_box_expansion(
        input_char,
        output_char,
        input_chars,
        output_chars,
        input_pos,
        output_pos,
    ) {
        return Some((
            TransformationType::Constructive(ConstructiveReason::BoxExpansion),
            0.2,
        ));
    }

    None
}

/// Check for destructive transformations
fn check_destructive_transformation(
    input_char: char,
    output_char: char,
    input_chars: &[char],
    output_chars: &[char],
    input_pos: usize,
    output_pos: usize,
) -> Option<(TransformationType, f32)> {
    // Arrow appearing in text content (destructive)
    if matches!(output_char, '↑' | '↓' | '←' | '→') && is_in_text_content(input_chars, input_pos)
    {
        return Some((
            TransformationType::Destructive(DestructiveReason::CharacterCorruption),
            -0.8,
        ));
    }

    // Pipe appearing in text content (destructive)
    if output_char == '│' && is_in_text_content(input_chars, input_pos) {
        return Some((
            TransformationType::Destructive(DestructiveReason::CharacterCorruption),
            -0.6,
        ));
    }

    // Content deletion (destructive)
    if input_char != ' '
        && output_char == ' '
        && !is_diagram_whitespace_change(input_chars, output_chars, input_pos, output_pos)
    {
        return Some((
            TransformationType::Destructive(DestructiveReason::ContentDeletion),
            -0.5,
        ));
    }

    None
}

/// Check if this is arrow duplication for alignment
fn is_arrow_duplication(
    input_chars: &[char],
    output_chars: &[char],
    input_pos: usize,
    output_pos: usize,
) -> bool {
    // Look for pattern where spaces become arrows for alignment
    // Check if this is part of a larger arrow alignment pattern
    if input_chars.get(input_pos) == Some(&' ')
        && matches!(output_chars.get(output_pos), Some('↓' | '↑' | '←' | '→'))
    {
        // Check surrounding context for arrow alignment patterns
        let start = output_pos.saturating_sub(5);
        let end = (output_pos + 6).min(output_chars.len());

        let context: String = output_chars[start..end].iter().collect();

        // Look for multiple arrows indicating alignment
        let arrow_count = context
            .chars()
            .filter(|&c| matches!(c, '↓' | '↑' | '←' | '→'))
            .count();

        arrow_count >= 2
    } else {
        false
    }
}

/// Check if this is box expansion (constructive)
/// Currently simplified - could be enhanced to detect specific expansion patterns
fn is_box_expansion(
    _input_char: char,
    _output_char: char,
    _input_chars: &[char],
    _output_chars: &[char],
    _input_pos: usize,
    _output_pos: usize,
) -> bool {
    // For now, assume box expansion is handled by the normalization pipeline
    // Could be enhanced to detect specific expansion patterns in future
    false
}

/// Check if whitespace change is diagram-related (not destructive)
/// Currently conservative - could be enhanced for diagram alignment detection
fn is_diagram_whitespace_change(
    _input_chars: &[char],
    _output_chars: &[char],
    _input_pos: usize,
    _output_pos: usize,
) -> bool {
    // For now, treat whitespace changes as potentially destructive
    // Could be enhanced to detect diagram alignment whitespace in future
    false
}

/// Check if position is in text content (not diagram elements)
fn is_in_text_content(chars: &[char], pos: usize) -> bool {
    // Check surrounding context for text patterns
    let start = pos.saturating_sub(10);
    let end = (pos + 11).min(chars.len());

    let context: String = chars[start..end].iter().collect();

    // Look for text patterns (letters, spaces, punctuation)
    context.chars().any(|c| c.is_alphabetic()) && !context.contains("┌┐└┘│─") // Not in box borders
}

/// Calculate transformation summary
fn calculate_summary(transformations: &[Transformation]) -> TransformationSummary {
    let mut destructive_count = 0;
    let mut constructive_count = 0;
    let mut neutral_count = 0;
    let mut net_quality_impact = 0.0;

    for transformation in transformations {
        net_quality_impact += transformation.impact_score;

        match transformation.transform_type {
            TransformationType::Destructive(_) => destructive_count += 1,
            TransformationType::Constructive(_) => constructive_count += 1,
            TransformationType::Neutral(_) => neutral_count += 1,
        }
    }

    // Calculate risk score based on destructive transformations
    let total_transformations = transformations.len() as f32;
    let risk_score = if total_transformations > 0.0 {
        destructive_count as f32 / total_transformations
    } else {
        0.0
    };

    TransformationSummary {
        destructive_count,
        constructive_count,
        neutral_count,
        net_quality_impact,
        risk_score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrow_duplication_classification() {
        let input = "     ↓\n┌────┐\n│    │\n└────┘\n     ↓";
        let output = "    ↓↓\n┌────┐\n│    │\n└────┘\n    ↓↓";

        let analysis = analyze_transformations(input, output);

        // Should classify arrow duplication as constructive
        assert!(analysis.transformations.iter().any(|t| matches!(
            t.transform_type,
            TransformationType::Constructive(ConstructiveReason::ArrowDuplication)
        )));

        // Should have positive quality impact
        assert!(analysis.summary.net_quality_impact > 0.0);

        // Should have low risk score
        assert!(analysis.summary.risk_score < 0.5);
    }

    #[test]
    fn test_destructive_transformation() {
        let input = "Hello World";
        let output = "H↑llo W↑rld";

        let analysis = analyze_transformations(input, output);

        // Should classify as destructive
        assert!(analysis
            .transformations
            .iter()
            .any(|t| matches!(t.transform_type, TransformationType::Destructive(_))));

        // Should have negative quality impact
        assert!(analysis.summary.net_quality_impact < 0.0);

        // Should have high risk score
        assert!(analysis.summary.risk_score > 0.5);
    }
}
