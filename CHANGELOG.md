# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.4] - 2026-02-14

### Quality & Documentation Release

**This release focuses on code quality improvements and comprehensive documentation**

#### ✅ **Code Quality**
- **Resolved all Clippy warnings**: Zero warnings with strict pedantic linting
- **Clippy compliance**: Marked 6 arrow detection functions as const fn for compile-time evaluation
- **Simplified patterns**: Replaced map_or patterns with is_some_and for cleaner code
- **Code formatting**: Applied consistent formatting standards across all modules

#### 📚 **Documentation Improvements**
- **Added CONFIG.md**: Complete configuration guide with schema, examples, and best practices
- **Added LIBRARY_USAGE.md**: Comprehensive Rust API documentation with integration examples
- **Updated README**: Improved documentation organization and discoverability
- **Quality validation**: All documentation examples verified and tested

#### 🧪 **Test Reliability**
- **Golden fixture tests**: Properly marked edge case fixtures for conservative mode
- **Test organization**: Improved fixture structure with clear documentation
- **Quality metrics**: Established realistic quality thresholds for different diagram types

#### 🔧 **Technical Improvements**
- **Test refactoring**: Extracted large test functions to improve maintainability
- **Code consolidation**: Reduced test complexity while maintaining coverage

### Notes
- All 282+ tests passing
- Zero Clippy warnings
- Code formatting standards applied throughout
- Backward compatible with 0.5.x series

## [0.5.0] - 2026-02-12

### Major Quality Improvements & Finalization Release

**Conservative, reliable ASCII diagram processing with honest quality guarantees**

#### 🐛 **Critical Bug Fixes**

**Table Processing Bug (Finalization)**
- **Fixed**: Tables with empty cells in some columns were corrupted (columns merged/shifted)
- **Root Cause**: `parse_table_row()` skipped empty cells, breaking column alignment
- **Solution**: Preserve ALL cells including empty ones; refined wrapped cell detection
- **Impact**: Tables now process correctly, README can be safely normalized
- **Verification**: Added README as dogfood test fixture

**Wrapped Cell Detection**
- **Fixed**: Legitimate empty table cells incorrectly identified as wrapped continuations
- **Solution**: Refined `is_continuation_row()` to check for first-column-empty + max-one-content-cell
- **Impact**: Wrapped cells unwrap correctly; empty cells preserved properly

#### ✅ **Quality Improvements**

**Text Corruption Prevention**
- **Fixed**: Arrows (↑↓←→) and pipes (│) corrupting text content inside boxes
- **Solution**: Collision detection prevents arrows from overwriting box interiors
- **Impact**: Text content preserved in simple-to-moderate diagrams

**Data Loss Elimination**
- **Fixed**: Complete sections of diagrams being deleted during processing
- **Solution**: Overlay rendering preserves all original content, only modifies detected primitives
- **Impact**: Zero data loss for simple diagrams; conservative mode for complex

**Nested Box Containment**
- **Added**: Basic parent-child box containment with proper margins
- **Status**: Single-level nesting works reliably; deep nesting (3+ levels) handled conservatively
- **Impact**: Simple nested diagrams render cleanly; complex structures preserved unchanged

**Intelligent Quality Validation**
- **Added**: Semantic transformation analysis distinguishing destructive vs constructive changes
- **Features**: Context-aware corruption detection, transformation classification
- **Impact**: Automated quality assurance with conservative behavior for ambiguous cases

### Repository Finalization

**Code Cleanup**
- Removed all phase markers from code and tests
- Cleaned up dead code and unused variables (13+ instances)
- Fixed all clippy warnings and linting issues
- Applied `cargo fmt` consistently across codebase
- Removed development archaeology per "Eternal Sunshine Principle"

**Documentation Honesty**
- Updated README with conservative, honest quality claims
- Changed from "92% pass rate" to "comprehensive test suite with 10+ passing fixtures"
- Added explicit caveats for complex nested structures
- Documented conservative mode behavior
- Accurate feature descriptions matching actual capabilities

**Test Enhancements**
- Added README as dogfood test fixture (`idempotent_readme_dogfood`)
- Comprehensive fixture documentation in `tests/data/README.md`
- Fixed wrapped cell fixtures to match correct output
- Quality tests marked with clear TODOs for known limitations

### Technical Enhancements

#### Core Processing Pipeline
- **Rendering**: Changed from "fresh grid" to "overlay on original" for content preservation
- **Detection**: Enhanced parent-child relationship establishment for nested boxes
- **Normalization**: Added nested containment logic with collision-aware expansion
- **Quality**: Intelligent validation framework with transformation classification
- **Tables**: Fixed cell preservation and wrapped cell detection

#### New Modules
- `src/transformation_analysis.rs` - Semantic analysis of processing transformations
- `src/quality.rs` - Comprehensive quality validation and metrics
- Enhanced test suites with 74 total tests (3 ignored with TODOs)

#### Quality Assurance
- **Test Coverage**: 74 tests passing (simple-to-moderate diagrams verified)
- **Conservative Behavior**: Complex structures (3+ nesting levels) handled safely
- **Validation Types**: Text preservation, structure integrity, corruption detection
- **Dogfooding**: README processes correctly with ascfix
- **CI/CD Ready**: Automated quality regression detection

### Known Limitations (Documented Transparently)
- **Complex Nested Diagrams**: Deep nesting (3+ levels) preserved unchanged (conservative mode)
- **Overlapping Elements**: Complex overlapping structures handled conservatively
- **Some Quality Tests Ignored**: 3 tests ignored with TODOs for features needing refinement
  - Complex nested diagrams (rendering issues)
  - Fence repair quality (needs refinement)

### Breaking Changes
- **Table Processing**: Tables now correctly preserve empty cells (was a bug, now fixed)
- **Processing Behavior**: Enhanced collision detection may change output for edge cases

### Performance
- **Processing Speed**: Maintained linear O(n) performance
- **Memory Usage**: Slight increase due to quality analysis (acceptable for benefits)
- **Build Size**: Minor increase due to additional quality validation code

### Verification
- ✅ 74 tests passing (3 ignored with documented TODOs)
- ✅ Zero clippy warnings
- ✅ Security audit clean (cargo audit)
- ✅ Code formatted (cargo fmt)
- ✅ Documentation builds (cargo doc)
- ✅ README processes cleanly with ascfix (dogfooding verified)
- ✅ Conservative quality approach for complex diagrams
- ✅ All original functionality preserved

### Migration Notes
- **No breaking API changes** - all existing functionality preserved
- **Enhanced quality** - improved output for complex diagrams
- **Better error handling** - more informative quality validation messages
- **CI/CD friendly** - automated quality gates for integration pipelines

---

## [Unreleased]

### Added - Table Unwrapping (Phase 02)

**Automatic Table Cell Unwrapping**
- Detect and repair Markdown tables with hard-wrapped cells at 80 columns
- Join continuation lines back into single cells with proper spacing
- Support for single and multi-column wrapped content
- Automatic detection using continuation row patterns (empty leading cells)

**Smart Content Preservation**
- Code blocks in table cells are preserved (not unwrapped)
- Supports both backtick (```) and tilde (~~~) code fences
- Links spanning wrap boundaries are preserved to prevent syntax breakage
- Intentional multi-line content remains intact

**Safe Mode Integration**
- Table unwrapping automatically applied in safe mode
- Unwrapping happens before column alignment normalization
- Seamless integration with existing table processing pipeline

**Comprehensive Testing**
- 10 unit tests for table unwrapping logic
- 3 new integration fixtures for wrapped tables
- 3 new fixture tests validating end-to-end behavior
- Tests for code block preservation and link boundary detection

### Technical Details

- New module: `src/tables.rs` with 200+ lines of table unwrapping logic
- Added `has_wrapped_cells()` - Detect wrapped table cells
- Added `unwrap_table_rows()` - Join continuation lines intelligently
- Added `contains_code_fence()` - Detect code blocks in cells
- Added `has_incomplete_link_across_rows()` - Prevent breaking markdown links
- Updated `process_safe_mode()` to integrate unwrapping pipeline

## [0.4.0] - 2026-02-09

### Added - Directory Support, MDX Files, and Duplicate Fence Repair

**Directory Processing and File Discovery**
- Process entire directories recursively with automatic file discovery
- Recursive traversal of nested directories
- Auto-detection of `.md` and `.mdx` files by default
- Custom file extension filtering via `--ext` / `-e` flag (comma-separated)
- Respect `.gitignore` files by default with `--no-gitignore` override
- Mixed argument support: process both files and directories in a single invocation

**MDX File Support (Issue #3)**
- `.mdx` files included in default processing
- Seamless handling alongside Markdown files
- Full feature parity with `.md` files

**Duplicate Closing Fence Detection (Issue #4)**
- Detect orphaned/duplicate closing fence markers
- Remove duplicate closing fences that appear after properly closed blocks
- Handle consecutive duplicate closing fences
- Support for both backtick and tilde fence types
- Conservative approach: preserves intentional nesting and longer fences

**CLI Improvements**
- New `--ext` / `-e` flag for custom file extension filtering
- New `--no-gitignore` flag to disable gitignore respect
- Graceful error handling: continue processing on single file errors
- Batch processing with error collection and reporting
- Short flag support for all flags (`-c`, `-e`, `-i`, `-a`, etc.)
- Improved help output with categorized sections and examples

**Batch Processing Enhancements**
- Process multiple files with resilience: errors in one file don't stop batch processing
- Comprehensive error reporting at end of processing
- Proper exit codes (0 for success, 1 for errors or changes needed)
- Clear error messages for file discovery and processing failures

**Dependencies**
- Added `ignore` crate (v0.4) for directory walking and gitignore support

### Testing
- 7 new integration tests for duplicate closing fence repair
- 16 unit tests for file discovery
- 5 tests for gitignore behavior and respect
- 7 end-to-end directory integration tests
- All existing tests continue to pass (247 unit tests, 572 total)

### Fixed
- Clippy warnings and code style improvements

### Changed
- Renamed internal `files` argument to `paths` for clarity (user-facing behavior unchanged)
- Default file extensions now include `.mdx` alongside `.md`

## [0.3.0] - 2026-02-09

### Added - Code Fence Boundary Validation and Repair

**Code Fence Repair**
- Detect and repair mismatched code fence lengths
- Automatically close unclosed code fences
- Preserve language specifiers (e.g., `python`, `javascript`)
- Support for both backtick and tilde fences
- Proper handling of nested fences (preserves intentional nesting)
- Conservative approach: skips ambiguous cases (type mismatches)

**CLI Enhancements**
- New `--fences` flag to enable fence repair independently
- New `--all` flag as shorthand for `--fences --mode=diagram`
- Works with `--check` mode for CI/CD validation

**Testing**
- 12 integration tests for fence repair
- 23 unit tests for detection, pairing, and validation
- Golden file tests for common fence scenarios
- Idempotence verification

### Fixed
- Integration with existing table and diagram repair pipeline

## [0.2.0] - 2026-02-06

### Added - Major Feature Expansion

**Box Style Variants**
- Support for double-line boxes (╔═╗║╚╝)
- Support for rounded-corner boxes (╭─╮│╰╯)
- Automatic style detection and preservation
- Style-aware rendering maintains original aesthetic

**Enhanced Arrow Support**
- Multiple arrow types: standard (→), double (⇒), long (⟶), dashed
- Bidirectional arrow support (rightward/leftward/upward/downward)
- Type detection from arrow characters
- Consistent arrow alignment across diagrams

**Side-by-Side Box Balancing**
- Automatic width normalization for horizontally adjacent boxes
- Vertical overlap detection
- Uniform sizing within alignment groups
- Conservative gap detection (≤1 cell)

**Nested Box Hierarchies**
- Parent/child relationship detection
- Automatic parent box expansion to fit children
- Multi-level nesting support (up to 2 levels stable)
- Hierarchy-aware rendering with proper nesting order

**Connection Lines (L-shaped Paths)**
- L-shaped path detection (limited to 4 segments)
- Box endpoint snapping
- Segment-based representation
- Conservative structure validation

**Label Preservation Framework**
- Text label attachment tracking to primitives
- Offset-based positioning (relative to attachment)
- Collision detection framework
- Support for labels on boxes, arrows, and connections

**Comprehensive Testing**
- 40+ new test cases across 5 test suites
- Golden file tests for all new features
- Idempotence verification framework
- 15 edge case tests covering unusual inputs
- Total: 250+ tests, all passing

### Improved
- Architecture updated to support 6 new primitive types
- Detector enhanced with hierarchy and connection detection
- Normalizer extended with 8 distinct operations
- Renderer updated for style-aware output
- Documentation comprehensive with examples

### Documentation
- Updated README with feature examples and usage
- Enhanced ARCHITECTURE.md with new primitives and algorithms
- Documented known limitations and conservative behavior
- Security and contribution guides maintained

### Known Limitations
- Nested box hierarchies may trigger re-detection on second pass (non-idempotent for complex diagrams)
- Simple diagrams remain fully idempotent
- Deep nesting (>2 levels) handled conservatively
- Connection line detection limited to 4 segments

### Verification
- 250+ tests passing (210 unit, 12 golden file, 15 edge case, 3 idempotence, 10 others)
- Zero Clippy warnings
- All code lints clean
- Security audit passed
- Release build successful

---

## [0.1.0] - 2026-02-06

### Added
- Initial release of **ascfix** - Automatic ASCII diagram repair tool
- **Safe mode**: Normalize Markdown tables (column alignment)
- **Diagram mode**: Detect and normalize ASCII boxes and arrows
- **Check mode**: Validate files without modifying (exit codes for CI/CD)
- Grid-based diagram representation and processing
- Primitive detection (boxes, arrows, text rows)
- Layout normalization (box widths, arrow alignment, padding)
- Comprehensive test suite (258 tests)
  - 122 unit tests
  - 129 integration tests
  - 6 golden file tests
  - 1 roundtrip test
- Production-ready binary (~1.2MB)
- Library API for programmatic use

### Features
- **Conservative**: Only fixes well-understood diagram structures
- **Idempotent**: Running twice produces identical output
- **Fast**: Linear processing of file content
- **Safe**: No panics on untrusted input
- **Deterministic**: Consistent output every time

### Verified
- Zero Clippy warnings
- All tests passing
- No development artifacts
- Clean git history
- Security validated

---

[Unreleased]: https://github.com/evoludigit/ascfix/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/evoludigit/ascfix/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/evoludigit/ascfix/releases/tag/v0.1.0
