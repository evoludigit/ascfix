# Phase 3: Primitive Detection

## Objective

Identify and classify ASCII diagram primitives: boxes, arrows, and text rows.

## Success Criteria

- [ ] Detect rectangular boxes (borders made of ─, │, ┌, ┐, └, ┘, ├, ┤, ┼)
- [ ] Identify text rows inside boxes
- [ ] Detect horizontal arrows (─→, ←─, ──)
- [ ] Detect vertical arrows (↓, ↑, ┃)
- [ ] All primitives tracked with position and dimensions
- [ ] All tests pass

---

## TDD Cycles

### Cycle 1: Box Detection
- **RED**: Write test detecting a simple rectangular box border
- **GREEN**: Implement DFS/flood-fill to find connected box characters
- **REFACTOR**: Extract into `BoxDetector` type
- **CLEANUP**: Handle incomplete or overlapping boxes conservatively

### Cycle 2: Box Content Extraction
- **RED**: Write test extracting text rows inside a detected box
- **GREEN**: Implement scanning of interior rows between top/bottom borders
- **REFACTOR**: Add validation that rows fit inside box bounds
- **CLEANUP**: Verify no content loss, handle edge cases

### Cycle 3: Horizontal Arrow Detection
- **RED**: Write test detecting `→` and `─` sequences as horizontal arrows
- **GREEN**: Implement pattern matching for arrow segments
- **REFACTOR**: Track arrow start/end positions and alignment
- **CLEANUP**: Handle multiple arrows in same row, validate alignment

### Cycle 4: Vertical Arrow Detection
- **RED**: Write test detecting `↓` and `↑` as vertical connectors
- **GREEN**: Implement detection of vertical sequences
- **REFACTOR**: Associate arrows with boxes and text rows
- **CLEANUP**: Validate vertical alignment with boxes above/below

### Cycle 5: Primitive Inventory
- **RED**: Write test building a complete primitive inventory from a grid
- **GREEN**: Implement unified detector that returns all boxes, arrows, text
- **REFACTOR**: Extract common detection patterns
- **CLEANUP**: Add comprehensive documentation

---

## Deliverables

- `src/primitives.rs` — Primitive types (Box, Arrow, TextRow)
- `src/detector.rs` — Detection logic for all primitives
- Tests covering all primitive types

---

## Dependencies

- Requires: Phase 2 complete

## Blocks

- Phase 4: Layout Normalization

## Status

[ ] Not Started
