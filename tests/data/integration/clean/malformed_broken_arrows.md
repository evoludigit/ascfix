# Malformed: Broken Arrows
# Tests arrows with broken connections

Proper horizontal arrow:
┌────┐    ┌────┐
│ A  │───▶│ B  │
└────┘    └────┘

Proper vertical arrow:
┌────┐
│ A  │
└────┘
   │
   ▼
┌────┐
│ B  │
└────┘

Multiple connected boxes:
┌────┐    ┌────┐    ┌────┐
│ A  │───▶│ B  │───▶│ C  │
└────┘    └────┘    └────┘

Vertical flow diagram:
┌────────┐
│ Start  │
└───┬────┘
    │
    ▼
┌────────┐
│ Middle │
└───┬────┘
    │
    ▼
┌────────┐
│ End    │
└────────┘
