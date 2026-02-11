# Complex: Mixed Box Styles
# Tests different box drawing character sets

# Single-line boxes
┌─────────────┐
│ Single Line │
└─────────────┘

# Double-line boxes
╔═════════════╗
║ Double Line ║
╚═════════════╝

# Rounded corner boxes
╭─────────────╮
│ Rounded     │
╰─────────────╯

# Mixed styles in same diagram
┌─────────────────┐
│   Single Line   │
│  ┏━━━━━━━━━━━┓  │
│  ┃ Double     ┃  │
│  ┃   ┌─────┐  ┃  │
│  ┃   │Round│  ┃  │
│  ┃   └─────┘  ┃  │
│  ┗━━━━━━━━━━━┛  │
└─────────────────┘

# Box style transitions
┌─────────────┐
│ Starting    │
│ Single      │
├─────────────┤  # Mixed border styles
│ Now Double  │
│ Line Below  │
╚═════════════╝

# Very thick borders (if supported)
┌─────────────┐
│ Normal      │
│ Borders     │
└─────────────┘