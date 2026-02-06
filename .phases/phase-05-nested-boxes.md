# Phase 5: Nested Box Handling

## Objective
Implement parent-child relationships for nested boxes with hierarchy-aware normalization.

## Success Criteria
- [ ] Box struct includes parent_idx and child_indices fields
- [ ] Box hierarchy detection identifies containment relationships
- [ ] Nested boxes expanded to fit children with margins
- [ ] Rendering respects depth ordering (parents before children)
- [ ] Deep nesting constraints enforced (max 3 levels)
- [ ] All tests pass with no clippy warnings
- [ ] 4 cycles completed with TDD discipline

## TDD Cycles

### Cycle 16: Hierarchy Detection
- **RED**:
  - `test_detect_single_box_no_parent()` has no parent
  - `test_detect_nested_box_has_parent()` finds parent correctly
  - `test_detect_box_hierarchy_multiple_children()` finds all children
  - `test_detect_hierarchy_topological_sort()` orders innermost first

- **GREEN**:
  - Add `parent_idx: Option<usize>` and `child_indices: Vec<usize>` to Box
  - Add `detect_box_hierarchy()` function
  - For each box: check if contained by any other box
  - Set parent_idx if contained
  - Add to parent's child_indices
  - Use topological sort for deterministic ordering

- **REFACTOR**:
  - Extract `is_contained_by()` helper
  - Extract `topological_sort_boxes()` helper
  - Add depth calculation method

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(detector): detect nested box hierarchies"

### Cycle 17: Nested Normalization
- **RED**:
  - `test_parent_expands_to_fit_child()` grows to contain child
  - `test_parent_adds_margins_around_child()` adds 1-cell margin
  - `test_nested_boxes_normalized_innermost_first()` processes depth order
  - `test_nested_normalization_idempotent()` running twice identical

- **GREEN**:
  - Add `normalize_nested_boxes()` function
  - Process boxes in topological order (innermost to outermost)
  - For each box with children: expand to fit all children + 1-cell margin
  - Only expand, never shrink (conservative)
  - Update border to new size

- **REFACTOR**:
  - Extract `calculate_required_size()` helper
  - Extract `expand_box_with_margin()` helper
  - Document margin strategy

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(normalizer): expand parent boxes to fit children"

### Cycle 18: Nested Rendering
- **RED**:
  - `test_parent_renders_before_child()` correct depth order
  - `test_multiple_nesting_levels_render_correctly()` 3-level test
  - `test_depth_sort_is_deterministic()` same ordering always

- **GREEN**:
  - Update renderer to sort boxes by depth before rendering
  - Add `calculate_box_depth()` helper
  - Render parents first (depth 0), then children (depth 1), etc.

- **REFACTOR**:
  - Extract `sort_boxes_by_depth()` helper
  - Document depth calculation in renderer

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(renderer): render nested boxes in correct order"

### Cycle 19: Nested Edge Cases
- **RED**:
  - `test_deep_nesting_level_3_limit()` allows exactly 3 levels
  - `test_deep_nesting_level_4_skipped()` skips > 3 levels
  - `test_overlapping_boxes_conservative_skip()` doesn't process overlaps
  - `test_mixed_styles_nested()` preserves styles through nesting

- **GREEN**:
  - Add depth validation: skip if depth > 3
  - Add overlap check: if boxes not pure hierarchy, skip
  - Ensure styles preserved after expansion

- **REFACTOR**:
  - Extract `validate_nesting_structure()` helper
  - Document skip conditions

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(normalizer): add nested box edge case handling"

## Conservative Constraints

Skip processing when:
- Nesting depth > 3 levels
- Boxes overlap but not in pure hierarchy (ambiguous)
- Child box shares border with parent (invalid nesting)
- More than 5 children per parent (suspicious)

## Dependencies
- Requires: Phase 1 complete
- Blocks: Phase 7 (integration testing)

## Testing

Add tests to `tests/integration_tests.rs`:
```rust
#[test]
fn test_simple_nested_box() { ... }

#[test]
fn test_nested_box_with_content() { ... }

#[test]
fn test_three_level_nesting() { ... }

#[test]
fn test_nested_boxes_preserve_styles() { ... }

#[test]
fn test_deep_nesting_conservative_skip() { ... }
```

Add golden files to `tests/golden/`:
- `nested_simple.txt` / `.expected.txt`
- `nested_three_levels.txt` / `.expected.txt`
- `nested_with_content.txt` / `.expected.txt`

## Status
[ ] Not Started | [ ] In Progress | [ ] Complete
