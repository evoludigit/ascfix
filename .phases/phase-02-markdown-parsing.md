# Phase 2: Markdown Parsing & Grid Representation

## Objective

Implement Markdown parsing to extract diagram blocks and convert them to a 2D grid representation.

## Success Criteria

- [ ] Detect diagram blocks (fenced text blocks outside code fences)
- [ ] Ignore fenced code blocks (```...```)
- [ ] Convert each diagram block to a 2D grid
- [ ] Preserve row/column indices for rendering
- [ ] All tests pass

---

## TDD Cycles

### Cycle 1: Code Block Detection
- **RED**: Write test that ignores content inside ` ``` ` fences
- **GREEN**: Implement simple regex/state machine to skip code blocks
- **REFACTOR**: Extract into dedicated parser module
- **CLEANUP**: Handle edge cases (nested backticks, unclosed fences)

### Cycle 2: Diagram Block Extraction
- **RED**: Write test to extract contiguous ASCII blocks (non-empty, whitespace-aware)
- **GREEN**: Implement heuristic detection (detect diagram boundaries)
- **REFACTOR**: Parameterize block detection logic
- **CLEANUP**: Fix lints, add comments for heuristics

### Cycle 3: 2D Grid Conversion
- **RED**: Write test converting a simple ASCII block to a 2D char grid
- **GREEN**: Implement line-by-line parsing into `Vec<Vec<char>>`
- **REFACTOR**: Extract into `Grid` struct with indexing methods
- **CLEANUP**: Add bounds checking, handle variable line lengths

### Cycle 4: Grid Preservation
- **RED**: Write test that round-trips a diagram through grid → rendering
- **GREEN**: Implement basic grid rendering back to string
- **REFACTOR**: Ensure grid preserves exact input (row/col addresses)
- **CLEANUP**: Verify idempotence, fix edge cases

---

## Deliverables

- `src/parser.rs` — Markdown block detection
- `src/grid.rs` — 2D grid representation and indexing
- `src/scanner.rs` — Diagram block extraction
- Tests for each component

---

## Dependencies

- Requires: Phase 1 complete

## Blocks

- Phase 3: Primitive Detection

## Status

[x] Complete
