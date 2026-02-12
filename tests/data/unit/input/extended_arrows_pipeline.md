# Extended Arrow Pipeline

Using extended arrows (⟶ ⟹) for data flow diagrams:

```
┌──────────┐
│ Source   │
└──────────┘
   ⟶
┌──────────┐
│ Pipeline │
└──────────┘
   ⟶
┌──────────┐
│ Sink     │
└──────────┘
```

Multiple extended arrows in sequence:

```
Input  ⟹  Process  ⟹  Transform  ⟹  Output
```

Mixed styles - extended with standard:

```
┌──────────┐     ┌──────────┐
│ API Call │  ⟶  │ Database │
└──────────┘     └──────────┘
   ↓                  ↓
Success        Cache Hit
```
