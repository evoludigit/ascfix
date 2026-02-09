# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
