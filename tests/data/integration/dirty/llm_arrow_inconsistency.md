# LLM Generated: Arrow Alignment Issues
# Common problem: LLM generates arrows at inconsistent column positions

Pipeline with misaligned arrows:
┌──────────┐
│ Start    │
└──────────┘
    ↓
┌──────────┐
│ Process  │
└──────────┘
     ↓
┌──────────┐
│ Output   │
└──────────┘

Side-by-side boxes with arrows:
┌────────┐    ┌────────┐
│ Input  │───▶│ Logic  │
└────────┘    └────────┘
  ↓              ↓
┌────────┐    ┌────────┐
│Result1 │    │Result2 │
└────────┘    └────────┘
