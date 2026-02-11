# Whitespace: Various Spacing Issues
# Tests handling of different whitespace patterns

# Mixed tabs and spaces
┌─────┐    ┌─────┐
│Tab  │		│Space│
└─────┘    └─────┘

# Trailing whitespace
┌─────────────────┐
│   Normal line   │
│Trailing spaces  │
└─────────────────┘

# Leading whitespace
   ┌─────────────────┐
   │Indented diagram │
   └─────────────────┘

# Mixed indentation
- List item
  - Nested item
	- Tab indented
    - Space indented

# Table with inconsistent spacing
| Header | Header2 | Header3 |
|--------|---------|---------|
|Data1|Data2|Data3|
| Data 4 | Data 5 | Data 6 |
|Data7|  Data8  |Data9|

# Code blocks with whitespace
```bash
# Normal code
echo "hello"

	# Tab indented code
	echo "world"
```

# Mixed line endings (would be tested with actual files)
Line with CRLF
Line with LF
Line with mixed endings

# Empty lines in diagrams
┌─────┐

│ Box │

│     │

└─────┘