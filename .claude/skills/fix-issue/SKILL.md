---
name: fix-issue
description: Fix a GitHub issue by number. Enforces RED→GREEN→CLEANUP TDD discipline and requires every reproduction case from the issue to have a passing test before any code ships.
argument-hint: [issue-number]
allowed-tools: Read, Edit, Write, Bash, Glob, Grep
---

# Fix GitHub Issue #$ARGUMENTS

## Step 0 — Read the issue in full

Fetch the issue and all its comments. Read everything before touching any code.

```
gh issue view $ARGUMENTS --comments
```

## Step 1 — Extract every reproduction case

From the issue body **and every comment**, extract all concrete input/output examples:
- Exact Markdown input snippets
- Exact diffs showing before/after
- Any edge cases mentioned in follow-up comments

List them explicitly before continuing. Do not skip cases that appear only in comments — those are often the ones that reveal a more complete failure mode.

**STOP HERE** if any reproduction case is ambiguous. Ask for clarification before writing any code.

## Step 2 — RED: write failing tests for every reproduction case

For each case identified in Step 1, write a test that:
1. Uses the **exact** input from the issue (copy verbatim, do not paraphrase)
2. Asserts the **exact** expected output described in the report
3. Is named after the scenario (e.g. `ordered_list_child_after_blank_line_preserves_indent`)

Run the tests. **Every new test must fail before you write any fix.** If a test passes already, the case is either already handled or the test is wrong — investigate before continuing.

```
cargo test [new_test_name] -- --nocapture
```

Do not move to Step 3 until all new tests are RED for the right reason.

## Step 3 — GREEN: implement the minimal fix

Write the smallest code change that makes all RED tests pass. No refactoring, no extra features.

```
cargo test
```

All tests (old and new) must be GREEN before continuing.

## Step 4 — CLEANUP: lint and format

```
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

Fix every warning. Zero tolerance.

## Step 5 — Verify idempotency

Run the fix on each reproduction case twice. The second run must produce no diff (idempotent output). Add a test if it isn't.

## Step 6 — Prepare the release

1. Determine the version bump (bug fix → patch)
2. Update `Cargo.toml` version
3. Run `cargo build --release` to update `Cargo.lock`
4. Add a `CHANGELOG.md` entry that references the issue number and describes what was broken and what was fixed
5. Update `aur/PKGBUILD` and `aur/.SRCINFO` version (`pkgrel` reset to `1`, sha256 marked `SKIP`)
6. Commit all changes
7. Tag the release: `git tag vX.Y.Z`
8. Push commit and tag: `git push && git push --tags`
9. Create the GitHub release to trigger the publish workflow: `gh release create vX.Y.Z`
   - The release notes must close the issue: include "Fixes #$ARGUMENTS"

## Hard rules

- **Never write a fix before writing a failing test for every case in the issue, including comments.**
- **Never release without running the full test suite.**
- **Never skip the idempotency check.**
- If a comment on the issue reveals that a previous fix did not work, re-read all comments as new reproduction cases and restart from Step 1.
