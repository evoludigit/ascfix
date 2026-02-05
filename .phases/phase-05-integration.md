# Phase 5: Integration, Testing & Modes

## Objective

Wire components together, implement CLI modes (safe/diagram/check), and add comprehensive testing.

## Success Criteria

- [ ] `safe` mode: Only fix Markdown tables
- [ ] `diagram` mode: Enable box + arrow normalization
- [ ] `check` mode: Exit non-zero if changes needed
- [ ] Golden file tests pass on real examples
- [ ] Idempotence tests pass
- [ ] All integration tests pass
- [ ] Clean exit codes and error messages

---

## TDD Cycles

### Cycle 1: Mode Routing
- **RED**: Write test that CLI routes to correct processor based on mode flag
- **GREEN**: Implement mode dispatch logic in main pipeline
- **REFACTOR**: Extract mode handlers into separate functions
- **CLEANUP**: Add clear error messages for invalid modes

### Cycle 2: Safe Mode (Tables)
- **RED**: Write test for Markdown table normalization
- **GREEN**: Implement table detection and column width normalization
- **REFACTOR**: Extract table logic into dedicated module
- **CLEANUP**: Handle edge cases (no tables, malformed tables)

### Cycle 3: Diagram Mode
- **RED**: Write test that diagram mode normalizes boxes and arrows
- **GREEN**: Wire detector + normalizer + renderer together
- **REFACTOR**: Consolidate mode-specific logic
- **CLEANUP**: Add safety checks (ambiguous structures → no change)

### Cycle 4: Check Mode
- **RED**: Write test that check mode compares input/output and exits with code 1 if different
- **GREEN**: Implement comparison logic and exit code
- **REFACTOR**: Extract comparison into utility
- **CLEANUP**: Verify no file writes in check mode

### Cycle 5: Golden File Tests
- **RED**: Create test fixtures (input diagrams + expected outputs)
- **GREEN**: Implement golden file testing framework
- **REFACTOR**: Parameterize test cases
- **CLEANUP**: Document test fixture format and location

### Cycle 6: Real Examples
- **RED**: Write test using AI-generated examples from PRD
- **GREEN**: Verify examples normalize as expected
- **REFACTOR**: Extract common patterns
- **CLEANUP**: Add more examples for regression testing

---

## Deliverables

- `src/modes.rs` — Mode-specific processors
- `src/processor.rs` — Main pipeline coordinator
- `tests/golden/` — Input/output fixtures
- `tests/integration_tests.rs` — Integration tests
- Documentation of test fixtures

---

## Dependencies

- Requires: Phase 4 complete

## Blocks

- Phase 6: Finalization

## Status

[ ] Not Started
