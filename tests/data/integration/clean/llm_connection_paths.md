# LLM Generated: Connection Line Issues
# Misaligned L-shaped paths and elbows

Simple  path (misaligned)
┌─────┐
│Start│─┐
└─────┘ │
         ▼
    ┌────────┐
    │  End   │
    └────────┘

Complex  branches (inconsistent)
            ┌────────┐
            │  Server
            └───┬────┘
                │
        ┌───────┼───────┐
        │       │       │
    ┌────────┌───────┌────────┐
    │ CPU  │ ││ Disk  │ Disk  │
    └────────└───────└────────┘
