# Error Recovery: Corrupted Diagrams
# Tests how ascfix handles various types of corruption gracefully

# Incomplete box (missing bottom)
┌─────────────────┐
│   Incomplete    │
│     Box

# Box with invalid characters in borders
┌───Invalid─┐
│   Mixed    │
├──✗Invalid─┤
│  Borders  │
└───Invalid─┘

# Arrow pointing to nowhere
┌─────┐
│  A  │ ───▶ [nowhere]
└─────┘

# Table with completely broken structure
| Header 1 | Header 2
|----------|
Data 1 | Data 2 | Extra |
| Missing | Pipes |
Trailing | data | here |
| Too | Many | Columns | Extra | Data |

# Nested quotes and escapes
```markdown
# This is code
| Table | In | Code |
|-------|----|------|
| Should | Be | Untouched |
```

> Quote with diagram
> ┌─────┐
> │ Box │
> └─────┘

# Mixed quote levels
> Level 1
>> Level 2
>>> Level 3
> Back to level 1

# Escaped characters in tables
| Character | Escaped | Description |
|-----------|---------|-------------|
| \| | \\| | Pipe symbol |
| \* | \\* | Asterisk |
| \` | \\` | Backtick |
| \[ | \\[ | Square bracket |
| \_ | \\_ | Underscore |

# Malformed list items
- Valid list item
-Another valid item
  - Nested item
    - Deeply nested
- Item with [link](url)
- Item with `code`
- Item with **bold** and *italic*

1. Numbered list
2. Another item
3. Item with code: `let x = 1;`
4. Item with [link](https://example.com)

# Headers with special characters
# Normal Header
## Header with `code`
### Header with [link](url)
#### Header with **bold**
##### Header with *italic*
###### Header with ~~strikethrough~~