# Mixed Arrow Styles

Comprehensive example using all arrow types together:

Standard vertical arrows (↓):
```
   ↓
┌─────┐
│ Box │
└─────┘
```

Double vertical arrows (⇓):
```
   ⇓
┌─────┐
│ Box │
└─────┘
```

Extended horizontal arrows (⟶):
```
┌─────┐  ⟶  ┌─────┐
│ Box │     │ Box │
└─────┘     └─────┘
```

Double horizontal arrows (⇒):
```
┌─────┐  ⇒  ┌─────┐
│ Box │     │ Box │
└─────┘     └─────┘
```

Complex workflow mixing all types:
```
Start
  ↓
┌──────────┐
│ Build    │
└──────────┘
    ⇓
┌──────────┐
│ Test     │
└──────────┘
    ↓
Deploy  ⟶  Production  ⇒  Monitor
```
