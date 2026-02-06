# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability in ascfix, please **do not** open a public issue. Instead, please email the maintainers privately with details of the vulnerability.

When reporting, please include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if you have one)

We take security seriously and will work with you to address any issues promptly.

---

## Security Audit Status

### Current Status
✅ **No known vulnerabilities**

- Last audit: 2026-02-06
- Tool: `cargo audit`
- Result: Clean

### Continuous Monitoring
- **Weekly security audits** via GitHub Actions (see `.github/workflows/audit.yml`)
- **Automated dependency updates** via Dependabot
- All dependencies monitored for CVEs

---

## Code Safety Guarantees

### Unsafe Code
**Forbidden** - ascfix contains **zero unsafe code**

```toml
[lints.rust]
unsafe_code = "forbid"
```

All unsafe code is explicitly forbidden at compile-time. This ensures memory safety throughout the codebase.

### Panic Safety
No panics on untrusted input. All inputs are validated:

**Input Boundaries:**
- File paths: Validated before opening
- File content: No unwrap/expect on untrusted data
- CLI arguments: Parsed by `clap`, validated
- Diagram blocks: Safe iteration with bounds checking

**Error Handling:**
- `Result<T>` used throughout
- Graceful degradation for ambiguous structures
- Conservative approach: only fixes well-understood patterns

---

## Input Validation

### File Operations
```rust
// Safe file reading with proper error handling
let content = fs::read_to_string(&path)?;  // Returns Result
```

- Path validation prevents directory traversal
- File permissions respected
- Read-only operations (no write without explicit --in-place flag)

### Content Processing
```rust
// Safe diagram processing
- Grid bounds checked before access
- Iterator-based loops prevent out-of-bounds
- No index-based access to untrusted data
```

**Process:**
1. Content parsed line-by-line (safe iteration)
2. Diagram blocks extracted with bounds validation
3. Grid created with explicit dimensions
4. All cell access checked: `grid.get(row, col)` returns `Option`

### Markdown Parsing
- Ignores code fences (no processing of code blocks)
- Preserves unknown structures (conservative approach)
- No injection vulnerabilities (text-only processing)

---

## Dependencies

### Minimal, Audited Dependencies

```toml
[dependencies]
clap = "4.5"      # CLI parsing (widely used, well-maintained)
anyhow = "1.0"    # Error handling (minimal, focused)
```

**Dev Dependencies:**
```toml
[dev-dependencies]
tempfile = "3.8"  # Test file creation
insta = "1.36"    # Snapshot testing
```

All dependencies are:
- ✅ Actively maintained
- ✅ Audited for security
- ✅ Minimal scope (no bloat)
- ✅ Monitored via `cargo audit`

### Dependency Updates
- Automatic PRs via Dependabot (weekly)
- Security updates applied immediately
- All updates tested before merge

---

## Data Security

### No Sensitive Data Handling
ascfix does **not**:
- Store user data
- Send data to remote servers
- Create logs or telemetry
- Store temporary files (uses ephemeral temp directories)
- Process secrets or credentials

### File Operations
- Files are modified in-place (with `--in-place` flag)
- Original file permissions preserved
- No backup files created (user's responsibility)
- No data cached beyond operation lifetime

---

## Testing & Verification

### Test Coverage
- **258 automated tests** covering:
  - Unit tests (122 tests)
  - Integration tests (129 tests)
  - Golden file tests (6 tests)
  - Roundtrip tests (1 test)

### Security Testing
- ✅ No panics on random input
- ✅ Malformed diagrams handled gracefully
- ✅ Path traversal attempts rejected
- ✅ Large files processed safely
- ✅ All linting passes (Clippy pedantic)

### Reproducible Builds
```bash
cargo build --release
```

Produces deterministic binary suitable for verification.

---

## Best Practices for Users

### Safe Usage
```bash
# Always check output before in-place modification
ascfix file.md                           # Preview changes (stdout)

# Then apply if safe
ascfix file.md --in-place --mode=diagram

# Use check mode in CI/CD
ascfix file.md --check --mode=diagram
```

### In CI/CD
```yaml
# Example GitHub Actions
- name: Validate diagrams
  run: ascfix docs/*.md --check --mode=diagram
```

Exit codes:
- `0` - No changes needed
- `1` - Changes needed (in check mode) or error occurred

---

## Security Considerations

### Conservative Philosophy
ascfix follows a **conservative, defensive approach**:

1. **Unknown patterns are preserved** - If structure is ambiguous, diagram is left unchanged
2. **No inference** - We don't guess intent; we only fix obvious issues
3. **Text-only processing** - No executable content
4. **Idempotent** - Running twice produces identical output (predictable, safe)

### What ascfix WILL NOT do
- ❌ Add content not in original
- ❌ Delete content
- ❌ Reword diagrams
- ❌ Infer business logic
- ❌ Process code blocks
- ❌ Execute any code
- ❌ Write to system locations

---

## Changelog & Updates

See [CHANGELOG.md](CHANGELOG.md) for security-related updates and vulnerability fixes across versions.

---

## Questions?

For security questions or concerns, please contact the maintainers privately rather than opening public issues.

See [CONTRIBUTING.md](CONTRIBUTING.md) for more information about the project.
