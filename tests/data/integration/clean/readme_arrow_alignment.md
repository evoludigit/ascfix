# README Arrow Alignment Test

Example from documentation with misaligned arrows:

```markdown
Source Code
       ↓
┌──────────────────┐
│ Build & Test     │
└──────────────────┘
       ↓
┌────────────────┐
│ Deploy         │
└────────────────┘
```

And another example with inconsistent arrow positions:

```markdown
┌─────────┐    ┌──────────┐
│ Process │    │ Database │
└─────────┘    └──────────┘
     │             │
┌────────────────────────┐
│ ResultProcessor        │
└────────────────────────┘
```

Complex workflow with single arrows misaligned:

```markdown
Source Code
       ↓
┌──────────────────┐
│ Build Job        │
└──────────────────┘
       ↓
┌──────────────────┐
│ Test Suite       │
└──────────────────┘
       ↓
┌──────────────────┐
│ Deploy           │
└──────────────────┘
```
