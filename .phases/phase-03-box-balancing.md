# Phase 3: Side-by-Side Box Balancing

## Objective
Normalize widths of horizontally adjacent boxes to improve diagram appearance.

## Success Criteria
- [ ] Side-by-side boxes detected via overlap and adjacency analysis
- [ ] Boxes in same group expanded to match maximum width
- [ ] Text rows updated to match new widths
- [ ] Integration into normalization pipeline
- [ ] Idempotence verified (running twice produces same result)
- [ ] All tests pass with no clippy warnings
- [ ] 3 cycles completed with TDD discipline

## TDD Cycles

### Cycle 9: Overlap Detection
- **RED**:
  - `test_find_vertical_overlap_groups_single_box()` returns single group
  - `test_find_vertical_overlap_groups_separate_boxes()` returns empty groups
  - `test_find_vertical_overlap_groups_side_by_side()` identifies adjacent boxes
  - `test_find_vertical_overlap_groups_three_boxes()` groups three side-by-side
  - `test_find_vertical_overlap_groups_stacked()` doesn't group vertically stacked

- **GREEN**:
  - Add `find_vertical_overlap_groups()` function
  - Group boxes where rows overlap AND boxes are horizontally adjacent
  - Adjacency: no gap between rightmost col of left box and leftmost col of right box
  - Return Vec<Vec<usize>> where inner vec is indices of boxes in group

- **REFACTOR**:
  - Extract `boxes_are_adjacent()` helper
  - Extract `boxes_have_vertical_overlap()` helper
  - Add conservative skip conditions (e.g., if gap > 2 cells, skip)

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(normalizer): detect vertically overlapping box groups"

### Cycle 10: Balancing Algorithm
- **RED**:
  - `test_balance_horizontal_boxes_no_change()` single box unchanged
  - `test_balance_horizontal_boxes_equalizes_widths()` expands narrow to max
  - `test_balance_horizontal_boxes_maintains_content()` text preserved
  - `test_balance_horizontal_boxes_idempotent()` running twice identical

- **GREEN**:
  - Add `balance_horizontal_boxes()` function
  - For each group: find max width
  - Expand each box to max width (add padding on right)
  - Update text_rows inside boxes to match new width
  - Return modified PrimitiveInventory

- **REFACTOR**:
  - Extract `expand_box_width()` helper
  - Extract `adjust_text_rows_for_box()` helper
  - Document padding strategy

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(normalizer): balance widths of adjacent boxes"

### Cycle 11: Pipeline Integration
- **RED**:
  - `test_full_normalization_includes_balancing()` end-to-end test
  - `test_balancing_preserves_other_primitives()` arrows and text unaffected
  - `test_balancing_idempotent_in_pipeline()` pipeline runs twice identical

- **GREEN**:
  - Integrate `balance_horizontal_boxes()` into modes.rs normalization
  - Call after other normalizations to avoid conflicts
  - Pass inventory through both old and new functions

- **REFACTOR**:
  - Ensure calling order is deterministic
  - Update normalizer module documentation

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(modes): integrate box balancing into normalization pipeline"

## Conservative Constraints

Skip balancing when:
- Gap between boxes > 2 characters (not truly "adjacent")
- Box contains only edges (no content)
- Resulting width > 100 characters (suspicious pattern)
- Boxes have different styles (keep separate)

## Dependencies
- Requires: Phase 1 complete (needs box detection)
- Blocks: Phase 7 (integration testing)

## Testing

Add tests to `tests/integration_tests.rs`:
```rust
#[test]
fn test_two_boxes_side_by_side_balanced() { ... }

#[test]
fn test_three_boxes_balanced_simultaneously() { ... }

#[test]
fn test_balanced_boxes_with_content() { ... }

#[test]
fn test_balancing_ignores_vertical_stacking() { ... }
```

Add golden files to `tests/golden/`:
- `boxes_side_by_side.txt` / `.expected.txt`
- `boxes_three_adjacent.txt` / `.expected.txt`
- `boxes_balanced_with_arrows.txt` / `.expected.txt`

## Status
[ ] Not Started | [ ] In Progress | [ ] Complete
