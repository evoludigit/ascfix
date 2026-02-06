# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/evoludigit/ascfix/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/evoludigit/ascfix/releases/tag/v0.1.0
