# Phase 4: Connection Lines

## Objective
Implement L-shaped connection lines with proper detection, normalization, and rendering.

## Success Criteria
- [ ] ConnectionLine and Segment primitives defined
- [ ] L-shaped connection paths detected and traced
- [ ] Connection endpoints snapped to box edges
- [ ] Rendering with proper elbow characters
- [ ] Connection lines ordered before arrows in rendering
- [ ] All tests pass with no clippy warnings
- [ ] 4 cycles completed with TDD discipline

## TDD Cycles

### Cycle 12: Connection Primitives
- **RED**:
  - `test_segment_horizontal_creation()` creates horizontal segment
  - `test_segment_vertical_creation()` creates vertical segment
  - `test_connection_line_creation()` creates connection with segments
  - `test_connection_line_with_endpoints()` stores box references

- **GREEN**:
  - Add Segment enum: Horizontal { row, start_col, end_col }, Vertical { col, start_row, end_row }
  - Add ConnectionLine struct: segments: Vec<Segment>, from_box: Option<usize>, to_box: Option<usize>
  - Add to PrimitiveInventory

- **REFACTOR**:
  - Add helper methods to Segment for length, endpoints
  - Add validation that segments form continuous path

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(primitives): add ConnectionLine and Segment types"

### Cycle 13: Connection Detection
- **RED**:
  - `test_detect_simple_l_shaped_path()` finds basic L-shape
  - `test_detect_connection_distinguishes_from_box()` ignores box borders
  - `test_detect_connection_with_multiple_segments()` traces full path
  - `test_detect_connection_conservative_skip()` skips ambiguous patterns

- **GREEN**:
  - Add `detect_connection_lines()` function
  - Use BFS/DFS to trace paths from junction points
  - Skip if path touches box border or other arrows
  - Skip if path > 4 segments (suspicious)
  - Store connection lines in inventory

- **REFACTOR**:
  - Extract `trace_path_from()` helper
  - Extract `is_junction_point()` helper
  - Add conservative skip documentation

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(detector): detect L-shaped connection lines"

### Cycle 14: Connection Normalization
- **RED**:
  - `test_snap_connection_endpoints_to_box()` moves to edge
  - `test_snap_connection_straightens_bent_paths()` removes kinks
  - `test_normalize_connections_idempotent()` running twice identical

- **GREEN**:
  - Add `normalize_connection_lines()` function
  - Find nearest box edge for each endpoint
  - Snap endpoints if within 2 cells of box
  - Straighten segments (remove detours)
  - Ensure only 2-3 segments (L or Z shape)

- **REFACTOR**:
  - Extract `nearest_box_edge()` helper
  - Extract `straighten_path()` helper

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(normalizer): normalize connection endpoints and paths"

### Cycle 15: Connection Rendering
- **RED**:
  - `test_render_horizontal_segment()` uses `─`
  - `test_render_vertical_segment()` uses `│`
  - `test_render_elbow_junction()` uses correct corner character
  - `test_connection_renders_before_arrows()` proper ordering

- **GREEN**:
  - Add `draw_connection_line()` function
  - Render segments in order after boxes, before arrows
  - Use elbow characters (`┌ ┐ └ ┘`) at junctions
  - Update rendering order: boxes → connections → arrows → text → labels

- **REFACTOR**:
  - Extract `select_elbow_char()` helper
  - Document rendering order in modes.rs

- **CLEANUP**:
  - Run `cargo fmt --all`
  - Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Commit as: "feat(renderer): render L-shaped connection lines"

## Conservative Constraints

Skip detection when:
- Path branches (more than 2 endpoints)
- Path > 4 segments (too complex)
- Path touches arrow or box interior
- Endpoints ambiguous (not near boxes)

## Dependencies
- Requires: Phase 1 complete
- Blocks: Phase 7 (integration testing)

## Testing

Add tests to `tests/integration_tests.rs`:
```rust
#[test]
fn test_simple_l_connection_detected_and_rendered() { ... }

#[test]
fn test_z_shaped_connection() { ... }

#[test]
fn test_connection_snaps_to_boxes() { ... }

#[test]
fn test_connections_rendered_before_arrows() { ... }
```

Add golden files to `tests/golden/`:
- `connection_simple_l.txt` / `.expected.txt`
- `connection_z_shape.txt` / `.expected.txt`
- `connection_with_boxes.txt` / `.expected.txt`

## Status
[ ] Not Started | [ ] In Progress | [ ] Complete
