# Configuration Guide

ascfix can be configured using a `.ascfix.toml` file in your project root to customize its behavior across your codebase.

## Configuration File Format

Create a `.ascfix.toml` file with the following options:

```toml
# Processing mode: "safe", "diagram", or "check"
# - safe: Fix tables and basic formatting (conservative, minimal risk)
# - diagram: Fix tables and ASCII diagrams (comprehensive, recommended)
# - check: Validate without writing changes
mode = "diagram"

# File extensions to process (default: [".md", ".mdx"])
extensions = [".md", ".mdx", ".txt"]

# Maximum file size to process (default: "100MB")
# Supports: B, KB, MB, GB
max_size = "100MB"

# Respect .gitignore when finding files (default: true)
respect_gitignore = true

# Enable conservative mode for complex diagrams (default: true)
# Conservative mode: preserve unusual diagrams as-is to prevent damage
conservative = true

# Enable fix attempts for various issues (can be disabled per issue type)
[fixes]
fix_tables = true
fix_arrows = true
fix_boxes = true
```

## Processing Modes

### Safe Mode
```toml
mode = "safe"
```

**What it fixes:**
- Malformed Markdown tables
- Wrapped table cells
- List formatting issues

**What it preserves:**
- ASCII diagram content (unchanged)
- Code blocks
- All non-table formatting

**Risk level:** Minimal

**Use when:**
- You want conservative, safe-only fixes
- Working with files that might have hand-drawn diagrams
- You need guaranteed data preservation

### Diagram Mode (Recommended)
```toml
mode = "diagram"
```

**What it fixes:**
- Everything from Safe mode, plus:
- ASCII box diagrams
- Arrow alignment and positioning
- Diagram spacing and alignment
- Connection lines
- Unbalanced diagram elements

**What it preserves:**
- Content within diagrams
- Code blocks
- Comments

**Risk level:** Low (with quality validation)

**Use when:**
- Processing LLM-generated content (common issues)
- You want comprehensive ASCII fixing
- You have documentation with diagrams

### Check Mode
```toml
mode = "check"
```

**What it does:**
- Validates files without making changes
- Reports issues that would be fixed
- Useful for CI/CD pipelines

**Use when:**
- Running in CI to detect issues
- You want to review fixes before applying
- Auditing repository quality

## Configuration Examples

### Minimal Configuration (Safe Defaults)
```toml
mode = "safe"
```
This uses all defaults - safe mode, processes .md and .mdx, respects .gitignore.

### Full Diagram Processing
```toml
mode = "diagram"
extensions = [".md", ".mdx", ".txt", ".rst"]
max_size = "200MB"
respect_gitignore = true
conservative = true
```

### Strict Mode (for Generated Docs)
```toml
mode = "diagram"
extensions = [".md"]
conservative = false  # Aggressively fix even complex diagrams
respect_gitignore = false  # Process all files
```

### Documentation Focus
```toml
mode = "diagram"
extensions = [".md", ".mdx"]
max_size = "50MB"
```

## File Size Limits

The `max_size` option protects against processing very large files:

```toml
max_size = "10MB"      # Only process files under 10MB
max_size = "1024KB"    # Equivalent to 1MB
max_size = "5242880B"  # Bytes notation
```

## Conservative Mode

When `conservative = true`, ascfix is extra cautious with complex nested structures:

- **Nested boxes:** Preserved as-is (not expanded)
- **Complex arrows:** Minimal adjustments only
- **Mixed styles:** Handled carefully to prevent corruption

This mode is recommended for hand-crafted diagrams where you want to preserve the original structure even if imperfect.

When `conservative = false`, ascfix makes more aggressive improvements:

- Expands boxes to fit content
- Aligns arrows comprehensively
- Normalizes diagram elements

Use false only when processing automatically-generated content (like LLM output).

## Environment Variables

Configuration can also be set via environment variables (takes precedence over `.ascfix.toml`):

```bash
export ASCFIX_MODE=diagram
export ASCFIX_MAX_SIZE=100MB
export ASCFIX_CONSERVATIVE=true
ascfix *.md
```

## CLI Override

Command-line arguments override both config file and environment variables:

```bash
ascfix --mode=safe --max-size=50MB *.md
```

## Default Behavior

If no `.ascfix.toml` exists, ascfix uses these defaults:

```toml
mode = "safe"
extensions = [".md", ".mdx"]
max_size = "100MB"
respect_gitignore = true
conservative = true
```

## Tips and Best Practices

1. **Start with `mode = "safe"`** - Get comfortable with conservative fixes first
2. **Use `check` mode in CI** - Detect issues before they're committed
3. **Version your config** - Commit `.ascfix.toml` to version control
4. **Test on a branch** - Try `diagram` mode on a feature branch first
5. **Review the changelog** - Check what gets fixed before committing

## Troubleshooting

**Q: My custom diagrams keep getting "fixed"**
```toml
conservative = true  # Preserve unusual structures
```

**Q: I want ascfix to fix everything**
```toml
mode = "diagram"
conservative = false
```

**Q: File is too large, getting skipped**
```toml
max_size = "500MB"
```

**Q: Want ascfix to ignore certain files**
```bash
# Add to .gitignore, then:
respect_gitignore = true
```
