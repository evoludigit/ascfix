# Malformed: Nested Structures
# Tests deeply nested or incorrectly nested elements

# Correctly nested boxes:
┌──────────────────┐
│ ┌──────────────┐ │
│ │ ┌──────────┐ │ │
│ │ │ Inner    │ │ │
│ │ └──────────┘ │ │
│ └──────────────┘ │
└──────────────────┘

# Nested boxes with arrows:
┌──┐
│A │
└┬─┘
 │
 ▼
┌──┐
│B │
└──┘

# Properly aligned nested structure:
┌────────────────────┐
│ ┌────────────────┐ │
│ │ ┌────────────┐ │ │
│ │ │ Content    │ │ │
│ │ └────────────┘ │ │
│ └────────────────┘ │
└────────────────────┘
