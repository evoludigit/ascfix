# v0.5.0 Release Checklist

## Pre-Release Verification ✅

- [x] Version bumped to 0.5.0 in `Cargo.toml`
- [x] CHANGELOG.md updated with release notes
- [x] README.md updated with accurate claims
- [x] All tests passing (74 tests)
- [x] Ignored tests documented with TODOs (3 tests)
- [x] Zero clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`
- [x] Code formatted: `cargo fmt --check`
- [x] Documentation builds: `cargo doc --no-deps`
- [x] Security audit clean: `cargo audit`
- [x] Release binary builds: `cargo build --release`
- [x] Manual testing: README processes correctly
- [x] Phase markers removed
- [x] Dead code cleaned up
- [x] .phases directory removed

## Quality Verification ✅

- [x] Table bug fixed (empty cells preserved)
- [x] README dogfood test added
- [x] Wrapped cell detection refined
- [x] Conservative behavior documented
- [x] Known limitations listed

## Git Status ✅

- [x] All changes committed
- [x] Commit message follows convention
- [x] Annotated tag created: `v0.5.0`
- [x] Tag message descriptive
- [x] Co-authored attribution included

## Release Notes ✅

- [x] Release notes written
- [x] Highlights documented
- [x] Bug fixes listed
- [x] Breaking changes noted
- [x] Known limitations documented
- [x] Installation instructions included

## Current State

```
Commit: eb31d27
Tag: v0.5.0
Branch: main
Status: Clean working directory
```

## Next Actions

### 1. Push to GitHub
```bash
git push origin main
git push origin v0.5.0
```

### 2. Create GitHub Release
- URL: https://github.com/evoludigit/ascfix/releases/new
- Tag: v0.5.0
- Title: "v0.5.0 - Quality & Finalization Release"
- Body: Copy from `/tmp/release_notes_v0.5.0.md`

### 3. Publish to crates.io (if desired)
```bash
cargo publish
```

## Post-Release

- [ ] Verify GitHub release appears correctly
- [ ] Test installation: `cargo install ascfix`
- [ ] Update any documentation sites
- [ ] Announce release (optional)

---

**Status: READY FOR RELEASE** ✅

All verification complete. Repository is in production-ready state.
