# Test Data Directory

This directory contains the test data structure for ascfix.

## Structure

- **`unit/`**: Golden file tests for focused, fast unit testing of diagram features
  - `input/` - Test input files with various diagram patterns
  - `expected/` - Expected output after processing
  - `diagrams/` - Additional diagram test cases
- **`integration/`**: Integration tests for complex, real-world scenarios and error handling
  - `clean/` - Well-formed inputs that should remain mostly unchanged
  - `dirty/` - Malformed inputs requiring repair
  - `expected/` - Expected outputs for specific integration scenarios
  - Various domain-specific fixtures (API docs, database schemas, ML pipelines)

## Unit Test Fixtures

### Verified Working (Quality Validated)
- `simple_box.txt` - Basic single box
- `box_with_arrow.txt` - Box with arrow connection
- `ci_pipeline.md` - Vertical workflow with arrows
- `side_by_side_boxes.md` - Horizontal box alignment
- `nested_boxes.md` - Single-level box nesting
- `double_line_boxes.md` - Double-line box style (╔═╗)
- `rounded_boxes.md` - Rounded corner style (╭─╮)
- `connection_lines.md` - L-shaped connection patterns
- `markdown_with_diagram.md` - Mixed markdown and diagrams

### Complex Cases (Conservative Mode)
- `mixed_features.md` - Multiple box styles with deep nesting (preserved unchanged)
- `mismatched_fences.md` - Code fence normalization (requires --fences flag)
- `nested_fences.md` - Nested code blocks (requires --fences flag)

## Integration Test Fixtures

### Clean Examples
Well-formed diagrams that test preservation:
- `complex_nested_with_labels.md` - Multi-level nesting with labels (conservative)
- `malformed_box_alignment.md` - Box alignment edge cases
- `malformed_broken_tables.md` - Table recovery scenarios
- `malformed_wrapped_cells.md` - Cell wrapping handling
- `malformed_wrapped_with_code.md` - Mixed code and tables
- `malformed_wrapped_with_links.md` - Links in tables

### Dirty Examples
Malformed inputs requiring repair:
- Same filenames as clean/ but with formatting issues

### Domain-Specific Tests
Real-world use cases:
- `realworld_api_docs.md` - API documentation patterns
- `realworld_database_schema.md` - Database schema diagrams
- `realworld_ml_pipeline.md` - ML workflow diagrams
- `realworld_security_architecture.md` - Security architecture
- `domain_devops_pipeline.md` - DevOps workflow
- `domain_data_structures.md` - Data structure diagrams
- `domain_networking_osi.md` - Network layer diagrams

### Edge Cases & Boundaries
- `edge_case_minimal.md` - Minimal valid diagrams
- `edge_case_minimal_extended.md` - Minimal with extensions
- `edge_case_mathematical.md` - Mathematical notation
- `boundary_max_nesting.md` - Maximum nesting depth test
- `error_recovery_corrupted.md` - Corruption recovery
- `stress_large_table.md` - Large table stress test
- `whitespace_handling.md` - Whitespace edge cases
- `international_multilingual.md` - Unicode and i18n

### Complex Patterns (Conservative Mode Active)
- `complex_arrow_patterns.md` - Advanced arrow routing
- `complex_connection_lines.md` - Multi-segment connections
- `complex_large_diagram.md` - Very large diagrams
- `complex_links_in_diagrams.md` - URLs in diagrams
- `complex_mixed_box_styles.md` - Multiple box styles
- `complex_mixed_content.md` - Heavily mixed content
- `complex_overlapping_elements.md` - Overlapping structures
- `complex_unicode_diagrams.md` - Extended Unicode

## Testing Philosophy

**Conservative by Design**: When diagram structures are ambiguous or highly complex, ascfix preserves the original unchanged rather than risk corruption. This means some "complex" fixtures serve as regression tests to ensure the tool doesn't break working diagrams, even if it doesn't improve them.

**Quality Gates**: Unit fixtures undergo automated quality validation to ensure:
- Text preservation > 85%
- Structure preservation > 80%
- Zero text corruption (characters replaced incorrectly)
- Zero data loss (content deleted)

**Fixture Organization**: Tests are organized by complexity and purpose:
- **Unit**: Focused, fast, specific feature tests
- **Integration**: Real-world scenarios, edge cases, conservative behavior verification