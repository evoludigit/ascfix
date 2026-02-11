# Malformed: Overlapping Elements
# Tests diagrams with conflicting or overlapping structures

┌──┐
│A ├─┐  # Overlapping box corners
├──┴─┘
│B │
└──┘

Conflicting arrow directions:
┌──┐    ┌──┐
│A │←──→│B │  # Conflicting arrows
└──┘    └──┘

Box inside arrow:
┌──┐
│ ┌┴┐ │  # Box inside arrow path
└─┬┘─┘
  │
  ▼

Text overlapping diagram:
┌──┐
│Some very long text that overlaps│
└──┘───► This arrow crosses the text