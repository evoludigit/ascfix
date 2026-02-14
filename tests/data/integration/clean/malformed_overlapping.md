# Malformed: Overlapping Elements
# Tests diagrams with conflicting or overlapping structures

┌────┐
│ A  │
└────┘
   │
   ▼
┌────┐
│ B  │
└────┘

Non-conflicting arrows:
┌────┐    ┌────┐
│ A  │───▶│ B  │
└────┘    └────┘

Separate boxes and arrow:
┌────┐
│ C  │
└────┘
   │
   ▼

Text beside diagram:
┌────────────┐
│ Short text │
└────────────┘
        ▶ Description here
