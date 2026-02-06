# Phase 1: Box Styles Foundation

## Objective
Implement box style detection and rendering for single, double, and rounded line styles.

## Success Criteria
- [ ] BoxStyle enum defined with Single/Double/Rounded variants
- [ ] Character recognition updated for double-line (`═║╔╗╚╝`) and rounded (`╭╮╰╯`)
- [ ] Box struct includes style field and detects it from corners
- [ ] Renderer uses style when drawing boxes
- [ ] All tests pass with no clippy warnings
- [ ] 4 cycles completed with TDD discipline

## TDD Cycles

### Cycle 1: BoxStyle Enum
- **RED**:
  - `test_box_style_chars()` verifies each style returns correct characters
  - Test Single: `─│┌┐└┘`
  - Test Double: `═║╔╗╚╝`
  - Test Rounded: `─│╭╮╰╯`

- **GREEN**:
  - Implement BoxStyle enum with Single/Double/Rounded
  - Add chars() method returning (&str, &str, &str, &str, &str, &str) for (horiz, vert, top_left, top_right, bottom_left, bottom_right)

- **REFACTOR**:
  - Extract BoxChars struct for cleaner API
  - Create helper methods for accessing individual characters

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(primitives): add BoxStyle enum and BoxChars helper"

### Cycle 2: Character Recognition
- **RED**:
  - `test_is_box_char_double()` recognizes `═║╔╗╚╝`
  - `test_is_box_char_rounded()` recognizes `─│╭╮╰╯`
  - `test_is_box_corner_all_styles()` recognizes all corner types

- **GREEN**:
  - Update `is_box_char()` to include double and rounded characters
  - Update `is_box_corner()` to include `╔╗╚╝╭╮╰╯`
  - Add `is_double_line_corner()` and `is_rounded_corner()` helpers

- **REFACTOR**:
  - Extract character set constants at module top
  - Document each character set

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(detector): recognize double-line and rounded box characters"

### Cycle 3: Box Style Detection
- **RED**:
  - `test_box_style_from_single_corner()` determines Single style
  - `test_box_style_from_double_corner()` determines Double style
  - `test_box_style_from_rounded_corner()` determines Rounded style
  - `test_extract_box_at_preserves_style()` full box detection preserves style

- **GREEN**:
  - Add `BoxStyle::from_corner(ch: char)` method
  - Update Box struct to include `style: BoxStyle` field
  - Update `extract_box_at()` to detect and set style from corners
  - Default to Single if detection fails (conservative)

- **REFACTOR**:
  - Add validation that all 4 corners use same style
  - Document mixed-style detection behavior

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(detector): detect box style from corners"

### Cycle 4: Style-Aware Rendering
- **RED**:
  - `test_render_single_style_box()` uses Single line characters
  - `test_render_double_style_box()` uses Double line characters
  - `test_render_rounded_style_box()` uses Rounded line characters

- **GREEN**:
  - Update `draw_box()` to use `box.style.chars()` instead of hardcoded characters
  - Ensure all 6 character positions (corners, sides) use correct style

- **REFACTOR**:
  - Extract `select_characters()` helper
  - Simplify character indexing logic

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(renderer): render boxes using their detected style"

## Dependencies
- Requires: Clean codebase (Phase 0 implicit)
- Blocks: Phase 2 (enhanced arrows require similar pattern)

## Testing

Add tests to `tests/integration_tests.rs`:
```rust
#[test]
fn test_single_line_boxes_unchanged() { ... }

#[test]
fn test_double_line_box_detection_and_rendering() { ... }

#[test]
fn test_rounded_box_detection_and_rendering() { ... }

#[test]
fn test_mixed_style_boxes_preserved() { ... }
```

Add golden files to `tests/golden/`:
- `box_single_line.txt` / `box_single_line.expected.txt`
- `box_double_line.txt` / `box_double_line.expected.txt`
- `box_rounded.txt` / `box_rounded.expected.txt`

## Status
[ ] Not Started | [~] In Progress | [ ] Complete
