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
- **Input**: Files from `dirty/` directory
- **Expected Output**: Files from `clean/` directory
- **Validation**: `tests/quality_validation_tests.rs` ensures transformations are correct and safe
- **Coverage**: 13 dirty/clean pairs validated for correctness

### Regression and Edge Case Tests
- **Input**: Reference examples (root-level files)
- **Validation**: `tests/malformed_fixture_tests.rs` ensures no crashes and expected behavior
- **Coverage**: 25+ reference fixtures for comprehensive testing

## Adding New Fixtures

For quality validation (demonstrates fix capabilities):
1. Create malformed input → `dirty/fixture_name.md`
2. Create expected clean output → `clean/fixture_name.md`
3. Add to `validate_integration_fixtures()` in `quality_validation_tests.rs`

For regression/edge cases (tests robustness):
1. Create reference example → `fixture_name.md` (root level)
2. Add test case to `malformed_fixture_tests.rs`

## Statistics

- **Dirty/Clean Pairs**: 13 fixtures with quality validation
- **Reference Examples**: 25+ well-formed examples for regression testing
- **Total Integration Fixtures**: 38+ test cases
