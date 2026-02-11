# Test Fixtures Directory Structure
This directory contains test fixtures for ascfix validation.

## Directory Structure
- `dirty/`: Malformed input files with formatting issues
- `clean/`: Expected clean output files after ascfix processing
- `*.md`: General fixture files (may be used as clean examples)

## Testing Approach
Tests validate that ascfix(dirty_input) == clean_expected_output
This ensures exact correctness of transformations.
