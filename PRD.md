# Product Requirements Document (PRD)

## Product Name

**ascfix** (working name)

## Owner

TBD

## Status

Draft

## Target Release

MVP in 2 phases

---

## 1. Background & Problem Statement

Large Language Models frequently generate **Markdown files containing ASCII diagrams** such as:

* Workflow diagrams
* CI/CD pipelines
* Validation flows
* Architecture overviews

These diagrams often include:

* Boxes (sometimes nested)
* Arrows and connectors
* Checklists or fan-in patterns

While visually *intended* to be structured, the generated output is often:

* Misaligned
* Fragile to edits
* Inconsistent in spacing
* Hard to review or maintain

Manual fixing is time-consuming, error-prone, and does not scale.

There is currently **no reliable, automated tool** that can safely repair these ASCII layouts while preserving intent.

---

## 2. Goals

### Primary Goals

* Automatically **repair alignment and spacing issues** in ASCII diagrams embedded in Markdown
* Preserve **visual intent and semantics**
* Be **safe, conservative, and deterministic**
* Work well on **AI-generated content**
* Produce minimal, reviewable diffs

### Secondary Goals

* Integrate easily into CI, pre-commit hooks, and AI pipelines
* Be fast and dependency-light
* Ship as a single static binary

---

## 3. Non-Goals

* Perfect reconstruction of arbitrarily complex ASCII art
* Inferring business semantics from diagrams
* Supporting graphical output formats (SVG, PNG)
* Replacing diagram languages like Mermaid or Graphviz

---

## 4. Target Users

* Engineers using AI to generate documentation
* Technical writers
* DevOps teams maintaining Markdown-based docs
* CI systems validating documentation quality

---

## 5. Example Input (Representative)

```
 Workflow Diagram

  Docs Changed in PR
          ↓
  ┌───────────────────────────────────────┐
  │  Documentation Validation Pipeline    │
  ├───────────────────────────────────────┤
  │                                       │
  │  ✓ link-validation    ────┐          │
  │  ✓ markdown-lint      ──┐ │          │
  │  ✓ prose-lint (Vale) ─┐│ │  (NEW)   │
  │  ✓ doc-structure    ──┤│ │          │
  │  ✓ anchor-check      ──┤│ │          │
  │                        │││          │
  │  ✓ docs-validation-success          │
  │     (Waits for all jobs)            │
  │                                       │
  └───────────────────────────────────────┘
          ↓
     Pass/Fail
```

### Observed Issues

* Inconsistent arrow lengths
* Implicit, fragile fan-in alignment
* Box width barely fits content
* Uneven padding and spacing
* Easy to break with small text changes

---

## 6. Desired Output Characteristics

The tool should:

* Normalize box widths based on content
* Align arrows consistently
* Enforce uniform padding
* Preserve structure and wording
* Remain idempotent

**Important:**
The tool should improve robustness **without redesigning the diagram**.

---

## 7. Scope & Feature Tiers

### Tier 1 — Markdown Tables (MVP baseline)

* Detect Markdown tables
* Normalize column widths
* Preserve alignment markers
* Ignore tables in code blocks

### Tier 2 — Simple ASCII Boxes

* Detect closed rectangular boxes
* Normalize borders
* Expand boxes to fit content
* Preserve relative positioning

### Tier 3 — Structured Diagrams (MVP focus)

* Single outer box
* Inner rows of text
* Fan-in / fan-out arrow patterns
* Vertical flow arrows above/below boxes

This tier explicitly targets diagrams like the example above.

### Tier 4 — Advanced Diagrams (Future)

* Nested boxes
* Multiple boxes connected by arrows
* More complex routing

---

## 8. User Interface

### CLI Usage

```bash
ascii-layout-fix README.md
ascii-layout-fix docs/*.md --in-place
ascii-layout-fix diagram.md --mode=diagram
ascii-layout-fix diagram.md --check
```

### Modes

| Mode             | Description                         |
| ---------------- | ----------------------------------- |
| `safe` (default) | Only fix well-understood structures |
| `diagram`        | Enable box + arrow normalization    |
| `check`          | Exit non-zero if fixes are needed   |

---

## 9. Functional Requirements

### Input Handling

* Accept Markdown files
* Preserve non-diagram content byte-for-byte
* Ignore fenced code blocks

### Diagram Handling

* Detect diagram blocks heuristically
* Convert diagram blocks to a 2D grid
* Identify known primitives:

  * Boxes
  * Text rows
  * Horizontal arrows
  * Vertical arrows

### Repair Rules

* Boxes must fully enclose content
* Borders must be continuous
* Arrow endpoints must align consistently
* Padding must be uniform

### Safety Rules

* If structure is ambiguous → do nothing
* Unknown ASCII patterns must be preserved verbatim
* No content deletion or rewording

---

## 10. Architecture Overview

### High-Level Pipeline

```
Markdown
  ↓
Block Scanner
  ↓
ASCII Grid
  ↓
Primitive Detection
  ↓
Layout Normalization
  ↓
Renderer
  ↓
Markdown
```

### Core Concepts

* Grid-based layout (row/column addressing)
* Explicit representation of boxes and connectors
* Separation between detection and rendering

---

## 11. Quality & Testing

### Testing Strategy

* Golden file tests (input → expected output)
* Real AI-generated examples
* Idempotence tests
* Regression tests for known failures

### Success Metrics

* Reduced manual diagram fixes
* Low false-positive modification rate
* Clean diffs
* Idempotent output

---

## 12. Performance Requirements

* Linear or near-linear processing
* Handle files up to 10k lines
* Suitable for CI usage

---

## 13. Risks & Mitigations

| Risk                  | Mitigation                |
| --------------------- | ------------------------- |
| Ambiguous diagrams    | Conservative detection    |
| Over-aggressive fixes | Opt-in diagram mode       |
| Unicode width issues  | ASCII-only MVP            |
| User trust            | Minimal diffs, check mode |

---

## 14. Future Directions

* Nested diagram support
* Unicode box-drawing variants
* ASCII → Mermaid conversion
* Editor plugins (VS Code)
* Diagram confidence scoring

---

## 15. Why This Product Matters

* AI-generated documentation is now mainstream
* ASCII diagrams remain common in Markdown
* Existing tools parse or render — **none reliably repair**
* This tool addresses a growing, unmet need in AI-assisted workflows

---

If you want next steps, I can:

* Turn this into an **MVP-only PRD**
* Break it into **milestones & epics**
* Write a **technical design doc** for Tier 3
* Draft a **README + positioning**

Just tell me where you want to go next.

