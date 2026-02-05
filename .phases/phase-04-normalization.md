# Phase 4: Layout Normalization & Repair

## Objective

Implement layout repair logic: normalize box widths, align arrows, fix spacing, and ensure visual consistency.

## Success Criteria

- [ ] Boxes expand to fit longest content row
- [ ] Horizontal arrows align consistently
- [ ] Vertical arrows maintain column alignment
- [ ] Padding is uniform (1 space on each side inside boxes)
- [ ] Output is idempotent (running twice produces same result)
- [ ] All tests pass

---

## TDD Cycles

### Cycle 1: Box Width Normalization
- **RED**: Write test that box expands to fit longest interior text
- **GREEN**: Implement box width calculation and border adjustment
- **REFACTOR**: Extract width computation into dedicated function
- **CLEANUP**: Handle edge cases (empty boxes, very long lines)

### Cycle 2: Horizontal Arrow Alignment
- **RED**: Write test that multiple horizontal arrows in same column align
- **GREEN**: Implement arrow repositioning within same row
- **REFACTOR**: Track arrow anchor points (start/end columns)
- **CLEANUP**: Validate no arrow overlap or collision

### Cycle 3: Vertical Arrow Alignment
- **RED**: Write test that vertical arrows maintain column alignment with boxes
- **GREEN**: Implement vertical arrow column normalization
- **REFACTOR**: Parameterize alignment strategy
- **CLEANUP**: Handle arrows between multiple boxes

### Cycle 4: Interior Padding
- **RED**: Write test that all interior rows have consistent 1-space padding from borders
- **GREEN**: Implement padding adjustment on all text rows
- **REFACTOR**: Extract padding logic into utility function
- **CLEANUP**: Verify no content truncation

### Cycle 5: Rendering Repaired Diagram
- **RED**: Write test that normalized primitives render back to ASCII correctly
- **GREEN**: Implement `render()` that places all primitives into a new grid
- **REFACTOR**: Separate grid construction from character placement
- **CLEANUP**: Verify output matches expected layout

### Cycle 6: Idempotence
- **RED**: Write test that processing output twice produces identical result
- **GREEN**: Implement idempotence by detecting already-normalized diagrams
- **REFACTOR**: Extract idempotence check into separate function
- **CLEANUP**: Verify idempotence on real examples

---

## Deliverables

- `src/normalizer.rs` — Normalization logic
- `src/renderer.rs` — Grid rendering from primitives
- Tests for all repair operations

---

## Dependencies

- Requires: Phase 3 complete

## Blocks

- Phase 5: Integration & Testing

## Status

[ ] Not Started
