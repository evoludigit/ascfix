# Architecture Overview

## High-Level Design

```
Input Markdown File
        ↓
   Scanner (extract diagram blocks)
        ↓
    Grid (2D character representation)
        ↓
  Detector (identify primitives)
        ↓
 Normalizer (fix alignment & padding)
        ↓
   Renderer (convert back to ASCII)
        ↓
    Output Markdown
```

Each component is **independent, testable, and focused**.

---

## Core Philosophy

### Conservative & Idempotent
- **Only process well-understood structures** (boxes, arrows, text)
- **Unknown patterns are preserved unchanged**
- **Running twice produces identical output** (safe to apply repeatedly)
- **No content is added or deleted** (only formatting improved)

### Minimal Assumptions
- Don't infer business logic
- Don't guess intent
- Process only what we can verify
- Fail safely (leave content unchanged if ambiguous)

---

## Module Structure

### `src/main.rs` - Entry Point (32 lines)
**Purpose:** CLI interface and error handling

```rust
fn main() -> Result<()> {
    let args = Args::parse_args();
    let processor = Processor::new(args);
    let exit_code = processor.process_all()?;
    std::process::exit(exit_code);
}
```

**Responsibilities:**
- Parse command-line arguments
- Create processor
- Handle exit codes (0 = success, 1 = check mode failure)
- Error reporting to stderr

---

### `src/lib.rs` - Library API (13 lines)
**Purpose:** Expose public API for library users

```rust
pub mod cli;      // CLI types
pub mod modes;    // Mode-specific processing
```

Enables programmatic usage:
```rust
use ascfix::modes::process_by_mode;
use ascfix::cli::Mode;

let result = process_by_mode(&Mode::Diagram, content);
```

---

### `src/cli.rs` - Command-Line Interface (100+ lines)
**Purpose:** Argument parsing and mode definition

**Key Types:**
```rust
#[derive(ValueEnum)]
pub enum Mode {
    Safe,     // Fix tables only
    Diagram,  // Fix tables + diagrams
    Check,    // Validate without writing
}

pub struct Args {
    pub files: Vec<PathBuf>,
    pub mode: Mode,
    pub in_place: bool,
    pub check: bool,
}
```

**Validation:**
- Files must exist
- Mode selected via `--mode` flag
- Check mode activated with `--check` flag

---

### `src/modes.rs` - Processing Strategies (355+ lines)
**Purpose:** Mode-specific processing implementations

**Three Modes:**

#### Safe Mode
- Detects Markdown table patterns (pipes + separators)
- Normalizes column widths
- Leaves diagrams untouched
- **Safest mode** - minimal changes

#### Diagram Mode
- Full pipeline: scan → detect → normalize → render
- Processes both tables and ASCII diagrams
- Only modifies blocks with detected primitives
- Preserves unknown structures

#### Check Mode
- Same as Diagram mode
- No file writes
- Returns exit code (1 if changes needed)
- Useful for CI/CD validation

**Key Function:**
```rust
pub fn process_by_mode(mode: &Mode, content: &str) -> String
```

---

### `src/scanner.rs` - Block Extraction (180+ lines)
**Purpose:** Identify diagram blocks in Markdown

**Algorithm:**
1. Scan for lines with ASCII diagram characters (box corners, arrows)
2. Group consecutive diagram lines into blocks
3. Preserve line numbers for later reinsertion

**Output:**
```rust
pub struct DiagramBlock {
    pub start_line: usize,
    pub lines: Vec<String>,
}
```

**Safety:**
- Ignores content inside code fences (backticks, tildes)
- Preserves original line positions
- No assumptions about diagram structure

---

### `src/grid.rs` - 2D Grid Representation (200+ lines)
**Purpose:** Convert ASCII diagrams to 2D grid for manipulation

**Design:**
```rust
pub struct Grid {
    cells: Vec<Vec<char>>,
}
```

**Operations:**
- `from_lines()` - Parse ASCII text into grid
- `get(row, col)` - Safe access with bounds checking
- `get_mut(row, col)` - Safe mutable access
- `render()` - Convert back to text
- Dimensions calculated from content

**Key Property:**
- All access is bounds-checked
- Returns `Option<char>` for safe access
- No panics on out-of-bounds

---

### `src/primitives.rs` - Type Definitions (80+ lines)
**Purpose:** Define diagram primitives and detection results

**Primitives:**
```rust
pub struct Box {
    pub top_left: (usize, usize),
    pub bottom_right: (usize, usize),
}

pub struct HorizontalArrow {
    pub row: usize,
    pub start_col: usize,
    pub end_col: usize,
}

pub struct VerticalArrow {
    pub start_row: usize,
    pub end_row: usize,
    pub col: usize,
}

pub struct TextRow {
    pub row: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub content: String,
}

pub struct PrimitiveInventory {
    pub boxes: Vec<Box>,
    pub horizontal_arrows: Vec<HorizontalArrow>,
    pub vertical_arrows: Vec<VerticalArrow>,
    pub text_rows: Vec<TextRow>,
}
```

---

### `src/detector.rs` - Primitive Detection (550+ lines)
**Purpose:** Identify boxes, arrows, and text rows in grids

**Detection Algorithm:**

#### Box Detection
1. Find box corners (┌, ┐, └, ┘)
2. Flood-fill from corner to find connected component
3. Verify rectangle shape (straight lines, all four corners)
4. Record top-left and bottom-right coordinates

#### Arrow Detection
- **Horizontal arrows** (→, ←, ─): Line-based detection with arrow tips required
- **Vertical arrows** (↓, ↑, │): Column-based detection with arrow tips required
- **Requirement:** Must have at least one arrow tip (→, ←, ↓, ↑) to be recognized

#### Text Detection
- Extract content from inside boxes
- Preserve original text exactly
- Record position within box

**Safety:**
- Conservative: Unknown patterns not identified
- Errors don't cause panics
- Ambiguous structures left unchanged

---

### `src/normalizer.rs` - Layout Improvement (800+ lines)
**Purpose:** Fix alignment, padding, and sizing issues

**Normalization Steps:**

1. **Box Width Expansion**
   - Find longest text row in each box
   - Calculate required width (content + 2 borders + padding)
   - Expand right edge if needed
   - Idempotent: only expands, never shrinks

2. **Horizontal Arrow Alignment**
   - Align arrow start/end columns with box edges
   - Maintain relative position
   - Uses BTreeMap for deterministic ordering

3. **Vertical Arrow Alignment**
   - Align arrow column with box center, left, or right edge
   - Choose closest alignment point
   - Preserve relative spacing

4. **Padding Normalization**
   - Enforce uniform 1-space padding inside boxes
   - Add space inside boxes if missing
   - Consistent formatting

**Key Property:**
- **Idempotent**: Running normalization twice produces identical output
- Verified by tests: `test_normalization_idempotent_*`

**Data Flow:**
```
PrimitiveInventory
        ↓
normalize_box_widths()
        ↓
align_horizontal_arrows()
        ↓
align_vertical_arrows()
        ↓
normalize_padding()
        ↓
PrimitiveInventory (normalized)
```

---

### `src/renderer.rs` - ASCII Reconstruction (250+ lines)
**Purpose:** Convert normalized primitives back to ASCII text

**Rendering Order:**
1. Create empty grid with sufficient dimensions
2. Draw boxes (borders + corners)
3. Draw text rows
4. Draw horizontal arrows
5. Draw vertical arrows

**Box Drawing:**
```
┌─────┐    ← corners (┌, ┐, └, ┘)
│ txt │    ← borders (─, │)
└─────┘
```

**Arrow Drawing:**
- Horizontal: Lines of ─ with → or ← tips
- Vertical: Lines of │ with ↓ or ↑ tips
- Only overwrites space cells (doesn't corrupt boxes)

**Output:**
```rust
pub fn render_diagram(inventory: &PrimitiveInventory) -> Grid
```

---

### `src/parser.rs` - Markdown Parsing (150+ lines)
**Purpose:** Extract non-diagram content (titles, text, code blocks)

**Features:**
- Ignores content in code fences (backticks, tildes)
- Extracts inline code preservation info
- Preserves line numbers

**Safety:**
- Respects code block boundaries
- No processing inside fenced code

---

### `src/processor.rs` - Main Orchestrator (200+ lines)
**Purpose:** Coordinate file I/O, mode selection, and output

**Key Function:**
```rust
pub fn process_all(&self) -> Result<i32>
```

**Responsibilities:**
1. Read files (with glob support)
2. Select mode based on CLI args
3. Apply processing
4. Handle output (stdout or in-place write)
5. Return appropriate exit code

**Exit Codes:**
- `0` - Success (or no changes needed in check mode)
- `1` - Check mode found changes OR error occurred

---

### `src/io.rs` - File Operations (150+ lines)
**Purpose:** Safe file reading and writing

**Features:**
- Handles glob patterns (e.g., `docs/*.md`)
- Respects file permissions
- Error handling for missing files
- In-place modification with validation

---

## Data Flow Example

### Processing a Diagram

```
Input: "┌──┐\n│Hi│\n└──┘"
        ↓
Scanner: [DiagramBlock { start_line: 0, lines: ["┌──┐", "│Hi│", "└──┘"] }]
        ↓
Grid: cells = [['┌', '─', '─', '┐'],
               ['│', 'H', 'i', '│'],
               ['└', '─', '─', '┘']]
        ↓
Detector: PrimitiveInventory {
  boxes: [Box { top_left: (0,0), bottom_right: (2,3) }],
  text_rows: [TextRow { row: 1, content: "Hi" }],
  ...
}
        ↓
Normalizer: (no changes needed - already normalized)
        ↓
Renderer: Grid with same cells
        ↓
Output: "┌──┐\n│Hi│\n└──┘"
```

### Processing with Changes

```
Input: "┌──┐\n│Hello│\n└──┘"  (too narrow!)
        ↓
Detector: Box width = 4, text = "Hello" (length 5)
        ↓
Normalizer: Expand box to width 7
        ↓
Renderer: "┌─────┐\n│Hello│\n└─────┘"
```

---

## Testing Strategy

### Unit Tests (122 tests)
Located in source files: `src/**/*.rs`
- Test individual functions
- No external dependencies
- Fast execution

### Integration Tests (129 tests)
Located in `tests/` directory
- Test mode processing
- Test full pipeline
- Real scenarios

### Golden File Tests (6 tests)
Located in `tests/golden_file_tests.rs`
- Test real examples
- Compare against expected output
- Catch regressions

### Test Coverage

| Module | Tests | Focus |
|--------|-------|-------|
| normalizer.rs | 110 | Idempotence, correctness |
| detector.rs | 31 | Primitive detection accuracy |
| renderer.rs | 6 | ASCII reconstruction |
| modes.rs | 129 | Mode-specific behavior |

---

## Design Decisions

### Why Grid-Based?
- **Easy reasoning** - All operations are spatial
- **Visual debugging** - Can print grids to understand state
- **Safe access** - Grid.get() returns Option, no panics
- **Testable** - Each grid state is verifiable

### Why Conservative Detection?
- **Safety** - Unknown patterns not modified
- **Predictability** - Only obvious fixes applied
- **No surprises** - Users always control output

### Why Modes?
- **Safe mode** - For any Markdown file (tables only)
- **Diagram mode** - For files with ASCII diagrams
- **Check mode** - For CI/CD validation
- **Flexibility** - Users choose safety level

### Why Idempotence?
- **Safe to apply repeatedly** - No degradation on re-runs
- **Tested property** - Verified by tests
- **User-friendly** - Can process files multiple times
- **CI/CD compatible** - Can be part of automated workflows

---

## Performance

### Time Complexity
- **Scanner**: O(n) - one pass through content
- **Detector**: O(n) - bounds checking on grid
- **Normalizer**: O(p) where p = primitives (typically small)
- **Renderer**: O(n) - render grid to text

**Overall**: O(n) - linear in file size

### Space Complexity
- **Grid**: O(w × h) where w, h = diagram dimensions
- **Primitives**: O(p) - constant typically
- **Overall**: O(n) - linear in file size

### Optimization
- Processes blocks independently (parallelizable)
- Single pass through content
- No unnecessary allocations

---

## Future Extensibility

### Adding New Primitives
1. Define in `src/primitives.rs`
2. Add detection in `src/detector.rs`
3. Add normalization in `src/normalizer.rs`
4. Add rendering in `src/renderer.rs`
5. Add tests for each step

### Adding New Modes
1. Add variant to `Mode` enum in `src/cli.rs`
2. Implement in `src/modes.rs`
3. Add tests

### Adding New Normalizations
1. Implement in `src/normalizer.rs`
2. Call from pipeline in `src/modes.rs`
3. Add tests

---

## Compilation & Optimization

### Build Modes
```bash
cargo build              # Debug (fast compile, slow runtime)
cargo build --release   # Release (slow compile, fast runtime)
```

### Linting
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Code Style
```bash
cargo fmt                # Format code
cargo fmt --check        # Check formatting
```

### Testing
```bash
cargo test               # All tests
cargo test --lib        # Unit tests only
cargo test --release    # Optimized tests
```

---

## Dependencies Rationale

### clap 4.5
- **Why**: Industry standard for CLI parsing
- **Benefit**: Declarative argument definition, auto help generation
- **Alternative considered**: manual argument parsing (too error-prone)

### anyhow 1.0
- **Why**: Ergonomic error handling
- **Benefit**: Minimal overhead, context preservation
- **Alternative considered**: custom error types (too verbose for this project)

### Dev Dependencies
- **tempfile**: Safe temporary file creation for tests
- **insta**: Snapshot testing for golden files

---

## References

- See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines
- See [SECURITY.md](SECURITY.md) for security policies
- See [CHANGELOG.md](CHANGELOG.md) for version history
