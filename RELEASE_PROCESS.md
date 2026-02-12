# v0.5.0 Release Process (Automated)

## 🤖 Automated GitHub Actions Workflow

The repository has automated publishing via `.github/workflows/publish.yml`!

### What Happens Automatically

When you create a GitHub Release, the workflow will:
1. ✅ Verify version in `Cargo.toml` matches the tag
2. ✅ Run full test suite (`cargo test --release`)
3. ✅ Run clippy with warnings as errors
4. ✅ Verify the package can be published (`cargo publish --dry-run`)
5. ✅ Publish to crates.io automatically (using `CARGO_REGISTRY_TOKEN` secret)

### Prerequisites

Ensure the GitHub repository has:
- [x] `CARGO_REGISTRY_TOKEN` secret configured (for crates.io publishing)
- [x] Tag pushed to GitHub: `v0.5.0`

## 📋 Release Steps

### Step 1: Push to GitHub

```bash
# Push the commit
git push origin main

# Push the tag (triggers automation once release is created)
git push origin v0.5.0
```

### Step 2: Create GitHub Release

1. Go to: https://github.com/evoludigit/ascfix/releases/new
2. **Choose tag**: Select `v0.5.0` from dropdown
3. **Release title**: `v0.5.0 - Quality & Finalization Release`
4. **Description**: Copy content from `/tmp/release_notes_v0.5.0.md`
5. Click **"Publish release"**

### Step 3: Automated Publishing 🚀

Once you publish the release:
- GitHub Actions automatically triggers
- Tests run in CI
- If all tests pass, publishes to crates.io
- View progress: https://github.com/evoludigit/ascfix/actions

### Step 4: Verify Publication

After ~5 minutes, verify:
- ✅ Crates.io: https://crates.io/crates/ascfix
- ✅ Docs.rs: https://docs.rs/ascfix/0.5.0
- ✅ GitHub Actions passed

## 🔍 Workflow Details

### Full Automation Chain

```
Create GitHub Release
        ↓
   Tag: v0.5.0
        ↓
Workflow Triggered (.github/workflows/publish.yml)
        ↓
    [Check version matches tag]
        ↓
    [Run test suite]
        ↓
    [Run clippy]
        ↓
    [Verify publish]
        ↓
    [Publish to crates.io] ← Uses CARGO_REGISTRY_TOKEN
        ↓
   ✅ Published!
```

### Benefits of Automation

✅ **Consistency**: Same checks every time
✅ **Safety**: Tests must pass before publishing
✅ **Traceability**: Full CI logs for every release
✅ **No manual cargo publish**: Less room for error

## 🛡️ Safety Checks

The workflow ensures:
- Version consistency (tag matches Cargo.toml)
- All tests passing
- Zero clippy warnings
- Package builds correctly
- Dry-run succeeds before actual publish

## 📝 Manual Override (Not Recommended)

If you need to publish manually:
```bash
cargo publish
```

But the automated workflow is preferred for consistency and safety.

## 🎯 Current Status

- [x] Commit created: `eb31d27`
- [x] Tag created: `v0.5.0`
- [x] All tests passing locally
- [x] Ready to push
- [ ] Push to GitHub
- [ ] Create GitHub Release
- [ ] Wait for automated publishing

---

**Next Action:** Push to GitHub and create the release!

```bash
git push origin main
git push origin v0.5.0
```

Then create the GitHub Release at:
https://github.com/evoludigit/ascfix/releases/new
