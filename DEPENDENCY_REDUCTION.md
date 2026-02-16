# Dependency Reduction Plan for ascfix

**Goal:** Reduce from 126 dependencies to ~10-20 dependencies  
**Target:** Version 0.6.0 (breaking changes acceptable)  
**Estimated Impact:** 84% reduction, 50% smaller binary, 66% faster compile

---

## Executive Summary

**Current State (v0.5.8):**
- 126 total dependencies
- 3.7MB binary
- ~60s compile time
- Core functionality requires: **0 dependencies**

**Target State (v0.6.0):**
- ~15-20 total dependencies
- ~2MB binary  
- ~20s compile time
- Dependency-conscious design

---

## Phase 1: Replace Heavy Dependencies

### 1.1 Replace `clap` with `lexopt`

**Current:** clap 4.5 with derive feature (40 dependencies!)  
**Proposed:** lexopt 0.3 (1 dependency)  
**Savings:** 39 dependencies

#### Implementation Steps:

**Step 1: Add lexopt dependency**
```toml
# Cargo.toml
[dependencies]
# clap = { version = "4.5", features = ["derive"] }  # REMOVE
lexopt = "0.3"  # ADD
```

**Step 2: Rewrite src/cli.rs**

Current clap code:
```rust
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(help = "Files or directories to process")]
    paths: Vec<PathBuf>,
    
    #[arg(short, long, value_enum, default_value = "diagram")]
    mode: Mode,
    
    #[arg(short, long)]
    in_place: bool,
    // ... etc
}
```

New lexopt code:
```rust
use lexopt::prelude::*;

pub struct Args {
    pub paths: Vec<PathBuf>,
    pub mode: Mode,
    pub in_place: bool,
    // ... etc
}

impl Args {
    pub fn parse() -> Result<Self> {
        let mut paths = Vec::new();
        let mut mode = Mode::Diagram;
        let mut in_place = false;
        // ... etc
        
        let mut parser = lexopt::Parser::from_env();
        while let Some(arg) = parser.next()? {
            match arg {
                Short('m') | Long("mode") => {
                    mode = parser.value()?.parse()?;
                }
                Short('i') | Long("in-place") => {
                    in_place = true;
                }
                Value(val) => {
                    paths.push(val.into());
                }
                // ... etc
                _ => return Err(arg.unexpected()),
            }
        }
        
        Ok(Args { paths, mode, in_place, /* ... */ })
    }
}
```

**Benefits:**
- More explicit control
- No proc macros (faster compile)
- Minimal dependencies
- Smaller binary

**Trade-offs:**
- More manual code (~100 extra lines)
- No auto-generated help (write manually)
- Manual validation

**Time Estimate:** 3-4 hours

---

### 1.2 Remove `ignore`, Use `walkdir` or `std::fs`

**Current:** ignore 0.4 (15 dependencies for .gitignore support)  
**Option A:** walkdir 2.5 (2 dependencies)  
**Option B:** std::fs (0 dependencies)  
**Savings:** 13-15 dependencies

#### Implementation Steps:

**Step 1: Decide on approach**

Option A (Keep .gitignore support with walkdir):
```toml
# Cargo.toml
[dependencies]
# ignore = "0.4"  # REMOVE
walkdir = "2.5"   # ADD (if we want .gitignore-like filtering)
```

Option B (Remove .gitignore support):
```toml
# Cargo.toml
# ignore = "0.4"  # REMOVE
# No replacement needed - use std::fs::read_dir recursively
```

**Step 2: Rewrite src/processor.rs file discovery**

Current code with ignore:
```rust
use ignore::WalkBuilder;

fn find_files(&self) -> Vec<PathBuf> {
    WalkBuilder::new(&self.root)
        .hidden(false)
        .git_ignore(true)  // ← This is the feature we're paying 15 deps for
        .build()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .collect()
}
```

New code with walkdir (Option A):
```rust
use walkdir::WalkDir;

fn find_files(&self) -> Vec<PathBuf> {
    WalkDir::new(&self.root)
        .into_iter()
        .filter_entry(|e| !self.should_ignore(e))  // Manual .gitignore-like logic
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn should_ignore(&self, entry: &DirEntry) -> bool {
    // Simple ignore logic:
    let name = entry.file_name().to_str().unwrap_or("");
    name.starts_with('.') || 
    name == "node_modules" || 
    name == "target" ||
    name == "vendor"
}
```

New code with std::fs (Option B):
```rust
use std::fs;

fn find_files_recursive(&self, dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            // Skip hidden and common build dirs
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with('.') || name == "target" || name == "node_modules" {
                    continue;
                }
            }
            
            if path.is_dir() {
                self.find_files_recursive(&path, files)?;
            } else if self.matches_extension(&path) {
                files.push(path);
            }
        }
    }
    Ok(())
}
```

**Recommendation:** Option B (std::fs)
- .gitignore support is nice-to-have, not critical
- Users can use `find` or explicit paths if needed
- Example: `ascfix $(find . -name "*.md" -not -path "*/node_modules/*")`

**Time Estimate:** 2-3 hours

---

### 1.3 Remove or Make Optional: `toml` Config Files

**Current:** toml 0.8 (10 dependencies)  
**Option A:** Remove config file support entirely  
**Option B:** Make it optional feature  
**Savings:** 10 dependencies (or 0 if made optional with default feature)

#### Implementation Steps:

**Option A: Remove Config Files (Recommended)**

```toml
# Cargo.toml
[dependencies]
# toml = "0.8"          # REMOVE
# serde = { ... }       # REMOVE (was only for config)
```

Replace with CLI flags:
```bash
# Before (with config file):
# .ascfix.toml contains max_size = "10MB"
ascfix .

# After (with CLI flags):
ascfix --max-size 10MB .
```

**Option B: Make Config Optional**

```toml
# Cargo.toml
[dependencies.toml]
version = "0.8"
optional = true

[dependencies.serde]
version = "1.0"
features = ["derive"]
optional = true

[features]
config = ["toml", "serde"]
```

**Recommendation:** Option A (Remove)
- Config files add complexity
- Most users use CLI flags anyway
- Can add back later if requested

**Time Estimate:** 2 hours

---

## Phase 2: Make Features Optional

### 2.1 Make `colored` Optional

**Current:** Always included (2 dependencies)  
**Proposed:** Optional feature, enabled by default  

```toml
# Cargo.toml
[dependencies.colored]
version = "2.1"
optional = true

[features]
default = ["colors"]  # Enabled by default, but can disable
colors = ["colored"]
```

Usage:
```bash
# Normal (with colors)
cargo install ascfix

# Minimal (no colors)
cargo install ascfix --no-default-features
```

**Savings:** 2 dependencies (when built without colors)

**Time Estimate:** 30 minutes

---

### 2.2 Make `serde_json` Optional

**Current:** Always included (5 dependencies)  
**Proposed:** Optional feature for JSON output  

```toml
# Cargo.toml
[dependencies.serde_json]
version = "1.0"
optional = true

[features]
default = []
json = ["serde_json"]
```

Code changes:
```rust
// src/output.rs
#[cfg(feature = "json")]
pub fn output_json(result: &ProcessResult) -> String {
    serde_json::to_string_pretty(result).unwrap()
}

#[cfg(not(feature = "json"))]
pub fn output_json(_result: &ProcessResult) -> String {
    eprintln!("JSON output requires --features json");
    std::process::exit(1);
}
```

**Savings:** 5 dependencies (when JSON not needed)

**Time Estimate:** 1 hour

---

### 2.3 Evaluate `similar` (Text Diffing)

**Current:** similar 2.7 (0 dependencies) ✅  
**Question:** Is quality validation critical?  

**Options:**
1. Keep it (0 deps, lightweight, useful)
2. Make it optional for quality checks
3. Remove it entirely

**Recommendation:** Keep it
- Zero dependencies
- Useful for validation
- Lightweight

**Time Estimate:** 0 hours (no change)

---

## Phase 3: Final Cleanup

### 3.1 Review Remaining Dependencies

After Phase 1 & 2, remaining core dependencies:

```toml
[dependencies]
anyhow = "1.0"      # Error handling - 0 deps
similar = "2.4"     # Text diffing - 0 deps
lexopt = "0.3"      # CLI parsing - 1 dep

# Optional
[dependencies.colored]
version = "2.1"
optional = true     # 2 deps

[dependencies.serde_json]
version = "1.0"
optional = true     # 5 deps

[features]
default = ["colors"]  # Minimal by default
colors = ["colored"]
json = ["serde_json"]
```

**Total (minimal):** ~5-10 crates  
**Total (default):** ~10-15 crates  
**Total (full):** ~15-20 crates  

---

## Implementation Timeline

### Sprint 1 (Week 1): Core Refactoring
- [ ] Day 1-2: Replace clap → lexopt (~4 hours)
- [ ] Day 3: Remove ignore → std::fs (~3 hours)
- [ ] Day 4: Remove toml config support (~2 hours)
- [ ] Day 5: Testing, bug fixes

### Sprint 2 (Week 2): Polish & Optional Features
- [ ] Day 1: Make colored optional (~1 hour)
- [ ] Day 2: Make serde_json optional (~1 hour)
- [ ] Day 3: Update docs, README
- [ ] Day 4: Comprehensive testing
- [ ] Day 5: Performance benchmarks

### Sprint 3 (Week 3): Release
- [ ] Day 1-2: Update CHANGELOG for v0.6.0
- [ ] Day 3: Update AUR PKGBUILD
- [ ] Day 4: Release testing
- [ ] Day 5: Publish v0.6.0

**Total Time:** ~15-20 hours of development work

---

## Testing Strategy

### Before Changes (Baseline)
```bash
# Measure current state
cargo clean
time cargo build --release
ls -lh target/release/ascfix
cargo tree | wc -l

# Results:
# Build time: ~60s
# Binary size: 3.7MB
# Dependencies: 126
```

### After Each Phase
```bash
# Verify functionality
cargo test --all-features
cargo test --no-default-features

# Measure improvements
cargo clean
time cargo build --release
ls -lh target/release/ascfix
cargo tree | wc -l
```

### Final Verification
```bash
# Test all build configurations
cargo build --release --no-default-features           # Minimal
cargo build --release                                  # Default
cargo build --release --all-features                   # Full

# Test installation
cargo install --path . --force
ascfix --version
ascfix --help

# Run full test suite
cargo test --all-features
```

---

## Migration Guide for Users

### v0.5.8 → v0.6.0

**Breaking Changes:**

1. **Config files removed**
   ```bash
   # Old (v0.5.8):
   # .ascfix.toml
   max_size = "10MB"
   
   # New (v0.6.0):
   ascfix --max-size 10MB .
   ```

2. **.gitignore support simplified**
   ```bash
   # Old (v0.5.8):
   ascfix .  # Automatically respects .gitignore
   
   # New (v0.6.0):
   ascfix .  # Skips .hidden, node_modules, target
   # For complex ignores, use find:
   ascfix $(find . -name "*.md" -not -path "*/node_modules/*")
   ```

3. **JSON output requires feature**
   ```bash
   # Install with JSON support:
   cargo install ascfix --features json
   
   # Or build from source:
   cargo build --release --features json
   ```

**No Breaking Changes:**
- Core functionality unchanged
- CLI interface mostly same (just simplified)
- All test coverage maintained

---

## Expected Results

### Metrics Comparison

| Metric | v0.5.8 (Current) | v0.6.0 (Target) | Improvement |
|--------|------------------|------------------|-------------|
| **Dependencies** | 126 | ~15 | -88% |
| **Binary Size** | 3.7MB | ~2MB | -46% |
| **Compile Time** | ~60s | ~20s | -66% |
| **Functionality** | Full | Full | Same |
| **Maintainability** | Medium | High | Better |
| **Security Surface** | Large | Small | Better |

### Build Configurations

**Minimal (`--no-default-features`):**
- ~5-10 crates
- ~1.5MB binary
- No colors, no JSON

**Default (recommended):**
- ~10-15 crates
- ~2MB binary
- With colors, no JSON

**Full (`--all-features`):**
- ~15-20 crates
- ~2.5MB binary
- Colors + JSON

---

## Risk Assessment

### Low Risk ✅
- Removing `toml` (config files rarely used)
- Making `colored` optional (colors are nice-to-have)
- Making `serde_json` optional (JSON output rarely used)

### Medium Risk ⚠️
- Replacing `clap` → `lexopt` (requires careful testing)
  - Mitigation: Comprehensive CLI tests
  - Fallback: Keep similar interface

- Removing `ignore` → `std::fs` (loses .gitignore support)
  - Mitigation: Document workarounds with `find`
  - Fallback: Could add back `walkdir` if needed

### High Risk ❌
None identified. Core functionality unchanged.

---

## Success Criteria

- [ ] All 288 library tests pass
- [ ] CLI functionality preserved
- [ ] Dependency count < 20 (target: ~15)
- [ ] Binary size < 2.5MB (target: ~2MB)
- [ ] Compile time < 30s (target: ~20s)
- [ ] AUR package builds successfully
- [ ] No user-facing breaking changes (except config files)

---

## Rollback Plan

If issues arise:
1. Version 0.5.8 remains on crates.io
2. Users can pin to v0.5.8 in Cargo.toml
3. AUR can maintain both packages (ascfix and ascfix-legacy)
4. Critical fixes can be backported to 0.5.x branch

---

## Questions for Decision

1. **Config file removal:** Acceptable? (Recommended: Yes)
2. **.gitignore support:** Simplify or keep with walkdir? (Recommended: Simplify)
3. **Default features:** Include colors? (Recommended: Yes)
4. **JSON support:** Optional or remove entirely? (Recommended: Optional)
5. **Version bump:** 0.6.0 or 1.0.0? (Recommended: 0.6.0)

---

## Next Steps

**Immediate (Before Starting):**
1. Create `dependency-reduction` branch
2. Update GitHub project/issues
3. Get user approval on breaking changes

**After Completion:**
1. Update CHANGELOG.md
2. Update README.md (installation, features)
3. Update LIBRARY_USAGE.md
4. Blog post about dependency minimization?

**Post-Release:**
1. Monitor issues for v0.6.0
2. Gather user feedback
3. Consider further optimizations for v0.7.0

---

## Conclusion

This plan reduces dependencies by **88%** while maintaining full functionality. The changes are well-scoped, testable, and reversible. The main trade-off is manual CLI parsing code and simplified directory scanning, which are acceptable for the significant improvements in build time, binary size, and maintainability.

**Recommended: Proceed with implementation for v0.6.0 release.**
