# ascfix Development Phases

## Project Overview

**ascfix** is a tool to automatically repair ASCII diagrams in Markdown files.

**Goal:** Make misaligned ASCII diagrams (especially AI-generated) reliable and maintainable.

**MVP Scope:** Tier 3 — Structured diagrams with single outer boxes, inner rows, fan-in/fan-out patterns, and vertical flow arrows.

---

## Phase Structure

| Phase | Title | Status |
|-------|-------|--------|
| [Phase 1](./phase-01-foundation.md) | Foundation & CLI Setup | [ ] Pending |
| [Phase 2](./phase-02-markdown-parsing.md) | Markdown Parsing & Grid Representation | [ ] Pending |
| [Phase 3](./phase-03-primitive-detection.md) | Primitive Detection (Boxes, Arrows, Text) | [ ] Pending |
| [Phase 4](./phase-04-normalization.md) | Layout Normalization & Repair | [ ] Pending |
| [Phase 5](./phase-05-integration.md) | Integration, Testing & Modes | [ ] Pending |
| [Phase 6](./phase-06-finalize.md) | Finalization & Repository Polish | [ ] Pending |

---

## Current Status

**Overall:** Not started

**Next:** Begin Phase 1 — Foundation & CLI Setup

---

## Development Notes

- Language: **Rust**
- Toolchain: Cargo + Clippy (strict mode)
- Testing: `cargo test` with TDD discipline
- Each phase follows RED → GREEN → REFACTOR → CLEANUP cycles
- Final phase (Phase 6) removes all development artifacts before shipping
