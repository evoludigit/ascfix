# Phase 6: Label Preservation

## Objective
Implement text attachment to primitives and movement preservation during normalization.

## Success Criteria
- [ ] Label and LabelAttachment primitives defined
- [ ] Labels detected and attached to nearby primitives
- [ ] Label offsets calculated and preserved
- [ ] Labels moved with attachments during normalization
- [ ] Rendering places labels in empty spaces only
- [ ] All tests pass with no clippy warnings
- [ ] 4 cycles completed with TDD discipline

## TDD Cycles

### Cycle 20: Label Primitives
- **RED**:
  - `test_label_creation()` creates label with content
  - `test_label_attachment_to_box()` attaches to box
  - `test_label_attachment_to_arrow()` attaches to arrow
  - `test_label_stores_offset()` preserves position offset

- **GREEN**:
  - Add Label struct: row, col, content, attached_to, offset
  - Add LabelAttachment enum: Box(usize), HorizontalArrow(usize), VerticalArrow(usize), ConnectionLine(usize)
  - Add to PrimitiveInventory

- **REFACTOR**:
  - Add validation that offset is small (±2 cells)
  - Add methods for calculating label bounds

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(primitives): add Label and LabelAttachment types"

### Cycle 21: Label Detection
- **RED**:
  - `test_detect_label_near_box()` finds text 1-2 cells away
  - `test_detect_label_attached_to_arrow()` finds label on arrow
  - `test_detect_label_calculates_offset()` offset stored correctly
  - `test_detect_label_conservative_limits()` length < 20 chars

- **GREEN**:
  - Add `detect_labels()` function
  - Scan all TextRows near primitives (distance ≤ 2 cells)
  - Calculate offset from attachment point
  - Skip if content > 20 chars (suspicious)
  - Skip if attached to interior of box (too ambiguous)

- **REFACTOR**:
  - Extract `is_label_candidate()` helper
  - Extract `calculate_attachment_offset()` helper
  - Document proximity thresholds

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(detector): detect text labels near primitives"

### Cycle 22: Label Normalization
- **RED**:
  - `test_label_moves_with_attachment()` offset applied
  - `test_label_normalized_with_box_movement()` follows box changes
  - `test_label_normalized_idempotent()` running twice identical

- **GREEN**:
  - Add `normalize_labels()` function
  - For each label: calculate new position from attachment + offset
  - Update label position when attachment moves
  - Preserve offset through changes

- **REFACTOR**:
  - Extract `calculate_label_position()` helper
  - Document position calculation strategy

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(normalizer): update label positions with attachments"

### Cycle 23: Label Rendering
- **RED**:
  - `test_render_label_in_empty_space()` renders if not occupied
  - `test_render_label_skipped_if_collision()` doesn't overwrite
  - `test_render_labels_last()` renders after diagram

- **GREEN**:
  - Add `draw_label()` function
  - Check for collisions before rendering
  - Skip if space occupied by diagram element
  - Render as last step in pipeline
  - Update rendering order: boxes → connections → arrows → text → labels

- **REFACTOR**:
  - Extract `label_space_occupied()` helper
  - Extract `get_label_bounds()` helper

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(renderer): render labels in preserved positions"

## Conservative Constraints

Skip label detection when:
- Text > 20 characters (too long)
- Distance > 2 cells from nearest primitive
- Inside box border (too ambiguous)
- Attachment primitive is ambiguous (multiple possible attachments)

Skip label rendering when:
- Label space occupied by diagram
- Label outside grid bounds
- Attachment target removed or moved outside grid

## Dependencies
- Requires: Phase 1 complete
- Blocks: Phase 7 (integration testing)

## Testing

Add tests to `tests/integration_tests.rs`:
```rust
#[test]
fn test_label_near_box_detected_and_preserved() { ... }

#[test]
fn test_label_moves_with_arrow() { ... }

#[test]
fn test_multiple_labels_per_primitive() { ... }

#[test]
fn test_label_skipped_on_collision() { ... }

#[test]
fn test_label_preserved_through_normalization() { ... }
```

Add golden files to `tests/golden/`:
- `label_box.txt` / `.expected.txt`
- `label_arrow.txt` / `.expected.txt`
- `labels_multiple.txt` / `.expected.txt`

## Status
[ ] Not Started | [ ] In Progress | [ ] Complete
