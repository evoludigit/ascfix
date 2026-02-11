# Malformed: Nested Structures
# Tests deeply nested or incorrectly nested elements

# Incorrectly nested boxes:
┌─────────┐
│ ┌─────┐ │
│ │ ┌─┐ │ │
│ │ └─┘ │ │  # Missing inner box bottom
│ └─────┘ │
└─────────┘

# Broken nested arrows:
┌──┐
│A │
└┬─┘
 │ ┌──┐
 │ │B │
 └─┼──┘  # Arrow crosses box boundary
   │
   ▼

# Mixed nesting levels:
┌─────────────────┐
│ ┌───┐           │
│ │ ┌─┴──┐        │
│ │ └─┬──┘        │
│ └───┘           │
│     │           │
│     ▼           │
└─────────────────┘