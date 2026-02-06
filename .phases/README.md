# ascfix Feature Extensions - Phase Overview

## Project Goal

Extend ascfix with 7 new features while maintaining conservative philosophy, idempotence, and TDD discipline:

1. **Double-line boxes** (`╔═╗╚╝`) - Box style enum
2. **Rounded corners** (`╭─╮╰╯`) - Box style enum
3. **Enhanced arrow handling** - Arrow type enum, bidirectional support
4. **Side-by-side box balancing** - Width normalization
5. **Nested box handling** - Parent/child relationships
6. **Connection lines** - L-shaped paths with segments
7. **Label preservation** - Text attachment and movement

## Architecture Overview

### Current Pipeline
```
Grid → Detector → PrimitiveInventory → Normalizer → Renderer → Grid
```

### New Primitives Being Added
- `BoxStyle` enum: Single, Double, Rounded
- `ArrowType` enum: Standard, Double, Long, Dashed
- `ConnectionLine` struct: Segments with endpoints
- `Label` struct: Text with attachment points
- Enhanced `Box`, `HorizontalArrow`, `VerticalArrow` with new fields

## Phase Schedule

| Phase | Objective | Status | Cycles |
|-------|-----------|--------|--------|
| [Phase 1](phase-01-box-styles.md) | Box styles foundation | [ ] Not Started | 1-4 |
| [Phase 2](phase-02-enhanced-arrows.md) | Arrow type handling | [ ] Not Started | 5-8 |
| [Phase 3](phase-03-box-balancing.md) | Side-by-side box balancing | [ ] Not Started | 9-11 |
| [Phase 4](phase-04-connection-lines.md) | Connection line primitives | [ ] Not Started | 12-15 |
| [Phase 5](phase-05-nested-boxes.md) | Nested box hierarchy | [ ] Not Started | 16-19 |
| [Phase 6](phase-06-labels.md) | Label preservation | [ ] Not Started | 20-23 |
| [Phase 7](phase-07-integration.md) | Integration & testing | [ ] Not Started | 24-27 |
| [Finalize](phase-08-finalize.md) | Production readiness | [ ] Not Started | - |

## Key Principles

### Conservative Philosophy
- Only expand, never shrink structures
- Skip processing when ambiguous
- Use deterministic ordering
- Store offsets, not absolute positions

### Idempotence Strategy
- Detection is idempotent
- Normalization is fixed-point
- Operations are deterministic
- Rendering preserves detection

### TDD Discipline
**Per cycle:** RED → GREEN → REFACTOR → CLEANUP

- **RED**: Write failing test
- **GREEN**: Implement minimal code
- **REFACTOR**: Improve design
- **CLEANUP**: Lint, format, commit

## Critical Files Modified

- `src/primitives.rs` - New structs and enums (+300 lines)
- `src/detector.rs` - Detection functions (+400 lines)
- `src/normalizer.rs` - Normalization functions (+500 lines)
- `src/renderer.rs` - Rendering updates (+200 lines)
- `src/modes.rs` - Pipeline integration (+50 lines)
- `tests/golden_file_tests.rs` - Golden file tests (+200 lines)

## Estimated Impact

- **Total new code**: ~2000 lines
- **New tests**: ~80 (3 per cycle)
- **Breaking changes**: None

## Verification Process

After each cycle:
```bash
cargo test                    # All tests pass
cargo clippy --all-targets --all-features -- -D warnings  # No warnings
cargo fmt --check             # Format OK
```

## Repository Finalization

Last phase removes all phase markers, development artifacts, and comments. Final commit message format drops phase references.
