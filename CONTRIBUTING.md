# Contributing to ascfix

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing.

## Development Setup

1. Clone the repository:
```bash
git clone https://github.com/evoludigit/ascfix.git
cd ascfix
```

2. Install Rust (if not already installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Build and test:
```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

## Code Quality Standards

This project maintains strict quality standards:

- **Tests**: All code changes must include tests
- **Linting**: Zero Clippy warnings (all, pedantic, cargo)
- **Formatting**: Code must pass `cargo fmt`
- **Documentation**: Public items must have documentation comments
- **Safety**: No `unsafe` code unless absolutely justified

## Making Changes

1. Create a feature branch:
```bash
git checkout -b feature/description
```

2. Make your changes and write tests

3. Verify quality:
```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt
```

4. Commit with a clear message:
```bash
git commit -m "feat: clear description of changes"
```

5. Push and open a pull request

## Commit Message Format

Follow semantic commit conventions:

- `feat:` - New feature
- `fix:` - Bug fix
- `refactor:` - Code restructuring (no behavior change)
- `test:` - Test additions or fixes
- `docs:` - Documentation changes
- `chore:` - Maintenance tasks

## Semantic Versioning

This project follows [Semantic Versioning](https://semver.org/):

- `MAJOR.MINOR.PATCH`
- `MAJOR`: Breaking changes
- `MINOR`: New features (backwards compatible)
- `PATCH`: Bug fixes (backwards compatible)

Update `Cargo.toml` version and `CHANGELOG.md` when preparing releases.

## Pull Request Process

1. Ensure all tests pass: `cargo test --release`
2. Ensure no Clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`
3. Ensure formatting: `cargo fmt`
4. Update CHANGELOG.md if user-facing
5. Write a clear PR description

## Testing

We maintain comprehensive test coverage:

- **Unit tests**: Test individual functions and modules
- **Integration tests**: Test feature combinations
- **Golden file tests**: Test complete workflows with real examples

Run all tests:
```bash
cargo test --release
```

## Questions?

Feel free to open an issue or discussion. We're happy to help!
