# Error Recovery: Corrupted Diagrams
# Tests how ascfix handles various types of corruption gracefully

# Complete box
┌─────────────────┐
│   Complete      │
│     Box         │
└─────────────────┘

# Box with proper borders
┌────────────┐
│   Mixed    │
├────────────┤
│  Borders   │
└────────────┘

# Arrow with proper target
┌─────┐
│  A  │───▶ Target
└─────┘

# Well-formed table structure
| Header 1 | Header 2 |
|----------|----------|
| Data 1   | Data 2   |
| Missing  | Pipes    |

# Code block with diagrams
```markdown
# This is code
| Table | In | Code |
|-------|-------|------|
| Should | Be | Untouched |
```

# Quote with diagram
> Quoted content
> ┌─────┐
> │ Box │
> └─────┘
