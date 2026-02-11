# Complex: Overlapping Elements
# Tests handling of elements that visually overlap

# Partially overlapping boxes
┌─────────────┐
│   Box A     │
│             │
└─────────────┘
      ┌─────────────┐
      │   Box B     │
      │             │
      └─────────────┘

# Arrows crossing through boxes
┌─────┐    ┌─────┐
│  A  │───▶│  B  │
│     │    │     │
│  ┌──▼──┐ │     │
│  │  C  │ │     │
│  └─────┘ │     │
└─────┘    └─────┘

# Text overlapping diagram elements
┌─────────────────┐
│ Important Note: │
│ This is a test  │
│ of overlapping  │
│ text and boxes  │
└─────────────────┘
Some text that might overlap with the diagram above.

# Connection lines crossing
┌─────┐    ┌─────┐
│  X  │────│  Y  │
└─────┘    └─────┘
   │         │
   │    ┌────▼────┐
   │    │   Z     │
   │    └─────────┘
   │
   ▼
┌─────┐
│  W  │
└─────┘