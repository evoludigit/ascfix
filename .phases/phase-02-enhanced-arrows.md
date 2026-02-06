# Phase 2: Enhanced Arrow Handling

## Objective
Implement arrow type detection and rendering for multiple arrow styles and bidirectional support.

## Success Criteria
- [ ] ArrowType enum defined with Standard/Double/Long/Dashed variants
- [ ] Character recognition updated for all arrow types
- [ ] Horizontal and vertical arrows detect type from characters
- [ ] Renderer uses correct arrow characters for each type and direction
- [ ] Bidirectional arrows supported (arrows in both directions)
- [ ] All tests pass with no clippy warnings
- [ ] 4 cycles completed with TDD discipline

## TDD Cycles

### Cycle 5: ArrowType Enum
- **RED**:
  - `test_arrow_type_chars()` verifies each type returns correct characters
  - Test Standard: `→ ←`
  - Test Double: `⇒ ⇐`
  - Test Long: `⟶ ⟵`
  - Test Dashed: `--->`

- **GREEN**:
  - Implement ArrowType enum with Standard/Double/Long/Dashed
  - Add methods for getting line and arrowhead characters for direction

- **REFACTOR**:
  - Extract ArrowChars struct for consistency with BoxChars pattern
  - Create helpers for direction handling

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(primitives): add ArrowType enum and ArrowChars helper"

### Cycle 6: Arrow Detection
- **RED**:
  - `test_detect_standard_arrow()` finds `→` or `←`
  - `test_detect_double_arrow()` finds `⇒` or `⇐`
  - `test_detect_long_arrow()` finds `⟶` or `⟵`
  - `test_detect_dashed_arrow()` finds `--->`
  - `test_detect_arrow_direction()` correctly sets rightward flag

- **GREEN**:
  - Add character recognition for all arrow types
  - Update `HorizontalArrow` to include `arrow_type: ArrowType` and `rightward: bool`
  - Update `VerticalArrow` similarly with `downward: bool`
  - Detect direction from arrowhead position

- **REFACTOR**:
  - Extract arrow character sets as constants
  - Add `ArrowType::from_char()` detection method
  - Document direction detection logic

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(detector): detect arrow types and directions"

### Cycle 7: Arrow Rendering
- **RED**:
  - `test_render_standard_arrow_rightward()` uses `→`
  - `test_render_standard_arrow_leftward()` uses `←`
  - `test_render_double_arrow_rightward()` uses `⇒`
  - `test_render_long_arrow_rightward()` uses `⟶`
  - `test_render_dashed_arrow_rightward()` uses `--->`

- **GREEN**:
  - Update `draw_horizontal_arrow()` to use `arrow.arrow_type` and `arrow.rightward`
  - Select appropriate arrowhead based on both type and direction
  - Ensure line characters match type

- **REFACTOR**:
  - Extract `select_arrowhead()` helper
  - Extract `select_arrow_line()` helper
  - Simplify character selection logic

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(renderer): render arrows using detected type and direction"

### Cycle 8: Vertical Arrow Types
- **RED**:
  - `test_detect_vertical_standard_arrow()` finds `↓` and `↑`
  - `test_detect_vertical_double_arrow()` finds `⇓` and `⇑`
  - `test_detect_vertical_long_arrow()` finds `⟱` and `⟰`
  - `test_detect_vertical_arrow_direction()` correctly sets downward flag

- **GREEN**:
  - Apply same pattern to vertical arrows
  - Update VerticalArrow detection to capture type
  - Detect downward direction from arrowhead position
  - Update renderer for vertical arrows

- **REFACTOR**:
  - Consolidate direction detection logic (shared helpers)
  - Ensure consistency between horizontal and vertical handling

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat: add vertical arrow types and bidirectional support"

## Dependencies
- Requires: Phase 1 complete
- Blocks: Phase 3 (box balancing doesn't depend on arrows, but Phase 7 does)

## Testing

Add tests to `tests/integration_tests.rs`:
```rust
#[test]
fn test_standard_horizontal_arrows_preserved() { ... }

#[test]
fn test_double_arrows_detected_and_rendered() { ... }

#[test]
fn test_long_arrows_detected_and_rendered() { ... }

#[test]
fn test_bidirectional_arrows() { ... }

#[test]
fn test_vertical_arrows_all_types() { ... }
```

Add golden files to `tests/golden/`:
- `arrow_types_horizontal.txt` / `.expected.txt`
- `arrow_types_vertical.txt` / `.expected.txt`
- `arrow_bidirectional.txt` / `.expected.txt`

## Status
[ ] Not Started | [ ] In Progress | [ ] Complete
