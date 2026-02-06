# Phase 7: Integration & Testing

## Objective
Integrate all features, verify idempotence across combinations, and harden edge cases.

## Success Criteria
- [ ] Golden file tests created for all features
- [ ] Mixed-feature diagrams test multiple features together
- [ ] Idempotence verified: `normalize(normalize(x)) == normalize(x)`
- [ ] All known edge cases tested and handled
- [ ] Documentation updated with new features
- [ ] All tests pass with no clippy warnings
- [ ] 4 cycles completed with TDD discipline

## TDD Cycles

### Cycle 24: Golden File Tests
- **RED**:
  - Create golden input files for each feature
  - Create expected output files
  - Tests fail initially (golden files don't match)

- **GREEN**:
  - Implement `tests/golden_file_tests.rs` test runner
  - For each feature: verify input → expected output through pipeline
  - Test diagrams:
    - All box styles (single, double, rounded)
    - All arrow types (standard, double, long, dashed)
    - Box balancing (2, 3, 4 boxes)
    - Connection lines (L-shape, Z-shape)
    - Nested boxes (2-level, 3-level)
    - Labels (box, arrow, multiple per object)
    - Mixed features (box + arrows + labels)

- **REFACTOR**:
  - Extract test helper functions
  - Organize golden files by feature

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "test(golden): add comprehensive golden file test suite"

### Cycle 25: Idempotence Verification
- **RED**:
  - `test_idempotent_box_styles()` run 3x, verify identical
  - `test_idempotent_arrows()` run 3x, verify identical
  - `test_idempotent_balancing()` run 3x, verify identical
  - `test_idempotent_connections()` run 3x, verify identical
  - `test_idempotent_nested()` run 3x, verify identical
  - `test_idempotent_labels()` run 3x, verify identical
  - `test_idempotent_combined_features()` run 3x, verify identical

- **GREEN**:
  - For each feature: run normalize 3 times, assert result[1] == result[2]
  - Test combinations of features (2-3 features together)
  - Document idempotence in modes.rs

- **REFACTOR**:
  - Extract idempotence test helper macro
  - Reuse for all feature combinations

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "test: verify idempotence for all features and combinations"

### Cycle 26: Edge Case Hardening
- **RED**:
  - `test_edge_mixed_box_styles_not_merged()` different styles stay separate
  - `test_edge_incomplete_connection_line_skipped()` fragmented paths ignored
  - `test_edge_deep_nesting_beyond_limit_skipped()` > 3 levels ignored
  - `test_edge_ambiguous_overlap_conservative()` complex overlaps skipped
  - `test_edge_very_long_labels_skipped()` > 20 chars ignored
  - `test_edge_extreme_dimensions()` very large boxes handled

- **GREEN**:
  - Test known edge cases and verify conservative behavior
  - Document specific skip conditions in code comments
  - Add test for each conservative skip condition
  - Ensure no panics on degenerate input

- **REFACTOR**:
  - Review all `is_valid()` checks
  - Add inline documentation for complex conditions

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "test: add edge case coverage and verify conservative behavior"

### Cycle 27: Documentation
- **RED**:
  - All examples in README produce expected output
  - ARCHITECTURE.md documents new primitives
  - Features listed with examples

- **GREEN**:
  - Update README.md with new features section
  - Add example diagrams for each feature
  - Update ARCHITECTURE.md with new primitive sections
  - Add feature compatibility matrix

- **REFACTOR**:
  - Ensure examples are tested and valid
  - Improve readability of architecture docs

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "docs: add feature documentation and examples"

## Testing Matrix

| Feature | Single | Double | Rounded | With Arrows | With Labels | Nested | Balanced |
|---------|--------|--------|---------|-------------|-------------|--------|----------|
| Box Styles | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Arrows | - | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Balancing | - | - | ✓ | ✓ | ✓ | ✓ | ✓ |
| Connections | - | - | - | ✓ | ✓ | ✓ | ✓ |
| Nesting | - | - | - | - | ✓ | ✓ | ✓ |
| Labels | - | - | - | - | - | ✓ | ✓ |

## Golden Files Checklist

**Box Styles:**
- [ ] box_single_line.txt
- [ ] box_double_line.txt
- [ ] box_rounded.txt

**Arrows:**
- [ ] arrow_standard.txt
- [ ] arrow_double.txt
- [ ] arrow_long.txt
- [ ] arrow_bidirectional.txt

**Balancing:**
- [ ] boxes_two_adjacent.txt
- [ ] boxes_three_adjacent.txt

**Connections:**
- [ ] connection_l_shape.txt
- [ ] connection_z_shape.txt

**Nesting:**
- [ ] nested_two_levels.txt
- [ ] nested_three_levels.txt

**Labels:**
- [ ] label_box.txt
- [ ] label_arrow.txt
- [ ] labels_multiple.txt

**Combined:**
- [ ] combined_all_features.txt
- [ ] combined_boxes_and_arrows.txt
- [ ] combined_nested_with_labels.txt

## Dependencies
- Requires: Phases 1-6 complete
- Blocks: Finalization phase

## Status
[ ] Not Started | [ ] In Progress | [ ] Complete
