# Test Data Organization

This directory contains all test data and fixtures used by ascfix's test suite.

## Directory Structure

### `data/unit/`
Contains golden file tests for unit testing of specific diagram transformations.
- **`input/`**: Source files containing diagrams to be processed
- **`expected/`**: Expected output files after ascfix processing
- **Purpose**: Fast, focused unit tests for individual features
- **Used by**: `golden_file_tests.rs`

### `data/integration/`
Contains integration test fixtures for complex, real-world scenarios.
- **`input/`**: Input files with formatting issues, malformed content, etc.
- **`expected/`**: Expected output files after ascfix processing
- **Purpose**: Comprehensive integration tests for end-to-end correctness
- **Used by**: `malformed_fixture_tests.rs`

## Adding New Test Data

### For Unit Tests (Recommended: `data/unit/`)
1. Add input file to `data/unit/input/`
2. Generate expected output: `ascfix --mode diagram data/unit/input/your_file.txt`
3. Save result to `data/unit/expected/your_file.txt`
4. Add test case to `golden_file_tests.rs`

### For Integration Tests (Recommended: `data/integration/`)
1. Add malformed input to `data/integration/input/`
2. Generate expected output with appropriate mode
3. Save result to `data/integration/expected/`
4. Add test case to `malformed_fixture_tests.rs`



## Test Data Guidelines

- **Fixtures**: Test complex, real-world scenarios and edge cases
- **Golden Files**: Test specific transformation behaviors with minimal examples
- Keep file names descriptive of what they test
- Update both input and expected files when behavior changes intentionally