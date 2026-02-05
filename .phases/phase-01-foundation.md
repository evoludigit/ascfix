# Phase 1: Foundation & CLI Setup

## Objective

Establish the Rust project structure, CLI framework, and basic file I/O handling.

## Success Criteria

- [ ] Cargo project initialized with strict Clippy linting
- [ ] Basic CLI with `--in-place`, `--mode`, `--check` flags
- [ ] Read/write Markdown files safely
- [ ] All tests pass and lints clean

---

## TDD Cycles

### Cycle 1: Project & CLI Structure
- **RED**: Write test for CLI argument parsing (modes: safe/diagram/check)
- **GREEN**: Implement basic CLI with clap or similar, handle file arguments
- **REFACTOR**: Extract CLI logic into separate module
- **CLEANUP**: Fix all Clippy warnings, format code

### Cycle 2: File I/O Safety
- **RED**: Write test for reading Markdown files and preserving content
- **GREEN**: Implement `read_markdown()` and `write_markdown()` functions
- **REFACTOR**: Extract I/O into dedicated module with error handling
- **CLEANUP**: Verify no unwrap() outside tests, proper error types

### Cycle 3: Round-Trip Preservation
- **RED**: Write test that reads a file and writes it back byte-for-byte
- **GREEN**: Implement round-trip validation in main pipeline
- **REFACTOR**: Extract pipeline into a `Processor` type
- **CLEANUP**: Add documentation, fix clippy pedantic warnings

---

## Deliverables

- `Cargo.toml` with strict Clippy lints
- `src/main.rs` with CLI entry point
- `src/cli.rs` — CLI argument handling
- `src/io.rs` — File reading/writing
- `tests/integration_tests.rs` — Round-trip tests

---

## Dependencies

- None (previous phases)

## Blocks

- Phase 2: Markdown Parsing & Grid Representation

## Status

[x] Complete
