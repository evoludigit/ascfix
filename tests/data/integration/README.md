# Test Fixtures Directory Structure

This directory contains comprehensive test fixtures for ascfix validation, organized into three categories:

## Directory Structure

### `dirty/` - Malformed Input Files
Files with formatting issues that ascfix should repair:
- Table wrapping issues (`malformed_wrapped_*.md`)
- Box alignment problems (`malformed_box_alignment.md`)
- Arrow connection issues (`malformed_broken_arrows.md`)
- Broken/incomplete boxes (`malformed_broken_box.md`)
- Nested structure issues (`malformed_nested.md`)
- Overlapping elements (`malformed_overlapping.md`)
- Code fence problems (`malformed_broken_fences.md`)
- Corrupted content (`error_recovery_corrupted.md`)

### `clean/` - Expected Output Files
Expected results after ascfix processing corresponding dirty files.
These demonstrate the tool's repair capabilities.

### `*.md` - Reference Examples (Root Level)
Well-formed examples used for regression and edge case testing:
- `complex_*.md` - Complex diagram examples
- `domain_*.md` - Domain-specific patterns (DevOps, networking, data structures)
- `realworld_*.md` - Real-world use cases (API docs, ML pipelines, databases, security)
- `edge_case_*.md` - Boundary condition testing
- `*_dogfood.md` - Self-referential tests on README

## Testing Approach

### Quality Validation Tests
- **Input**: Files from `dirty/` directory (well-understood repair cases)
- **Expected Output**: Files from `clean/` directory
- **Validation**: `tests/quality_validation_tests.rs` ensures transformations are correct and safe
- **Coverage**: 6 stable dirty/clean pairs validated for correctness
- **Scope**: Table normalization, cell unwrapping, arrow alignment - areas where ascfix reliably succeeds

### Regression and Robustness Tests
- **Input**: All fixture files (both dirty/ and reference examples)
- **Validation**: `tests/malformed_fixture_tests.rs` ensures:
  - No crashes on any input
  - Conservative handling of complex/ambiguous structures
  - Content preservation and no data loss
- **Coverage**: 38+ fixtures including edge cases
- **Note**: Edge case fixtures (broken arrows, complex nesting, etc.) are tested for safe behavior, not correctness, as ascfix uses conservative mode for ambiguous structures

## Adding New Fixtures

For quality validation (demonstrates fix capabilities):
1. Create malformed input → `dirty/fixture_name.md`
2. Create expected clean output → `clean/fixture_name.md`
3. Add to `validate_integration_fixtures()` in `quality_validation_tests.rs`

For regression/edge cases (tests robustness):
1. Create reference example → `fixture_name.md` (root level)
2. Add test case to `malformed_fixture_tests.rs`

## Statistics

- **Dirty/Clean Pairs (Quality Validated)**: 8 fixtures
  - 4 table wrapping & structure issues
  - 2 arrow alignment issues
  - 1 box alignment issue
  - 1 nested box issue
- **Reference Examples (Regression Only)**: 31+ test cases
- **Total Integration Tests**: 39 cases all passing

## Design Philosophy

**Focus on realistic LLM problems**: All quality-validated fixtures represent problems that LLMs commonly generate:
- Cells wrapping at column boundaries (very common)
- Boxes too narrow for content (very common)
- Arrows at inconsistent positions (common)
- Mixed Unicode/ASCII styles (somewhat common)

**Conservative with edge cases**: Complex structures that ascfix handles conservatively (ambiguous nesting, overlapping elements) are tested for safety (no crash, no data loss) but NOT validated for specific correctness, as ascfix intentionally preserves these unchanged to prioritize safety over aggressive transformation.
