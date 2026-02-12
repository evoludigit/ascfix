# README Dogfood Fixture

This is a copy of the project's actual README.md used as a test fixture.

## Purpose

The README is an excellent real-world test because it contains:
- Tables (including complex ones with empty cells)
- Lists (bullet points with proper spacing)
- Code blocks (with language specifiers)
- Mixed Markdown content
- Real-world formatting patterns

## Known Issues Caught

**Table Processing Bug**: The Flags table has multiple rows with empty cells in the "Short" column. When processed, the table normalization incorrectly merges these rows, creating corrupted output. This demonstrates why the tool's conservative approach is important.

**Test Status**: Currently marked as `#[ignore]` until table bug is fixed.

## Value

This fixture proves the tool works on real-world documents and helps catch regressions. Any future changes should be validated against this fixture.
