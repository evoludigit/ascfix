# Golden File Test: Complex Multi-Level Diagram

This diagram tests ascfix's ability to handle:
- Multi-level box hierarchies (grandparent → parent → children)
- Vertical arrow alignment through multiple layers
- Proper spacing and padding consistency
- Box width alignment across levels

## Input: Database Architecture Diagram

```
                            ┌─────────────────────────┐
                            │   Database as the       │
                            │   Optimizer (Core)      │
                            └────────────┬────────────┘
                                         │
                      ┌──────────────────┼──────────────────┐
                      │                  │                  │
              ┌───────▼────────┐ ┌───────▼────────┐ ┌──────▼────────┐
              │ Compilation    │ │ confiture      │ │ pg_tviews     │
              │ Pipeline       │ │ (Migration)    │ │ (Views)       │
              │ (Author→       │ │                │ │                │
              │ Compile→Deploy)│ │                │ │                │
              └────────────────┘ └────────────────┘ └─────────────────┘
                      │                  │                  │
                      │                  │                  │
              ┌───────▼────────┐ ┌───────▼────────┐ ┌──────▼────────┐
              │ fraiseql-core  │ │ fraiseql-      │ │ jsonb_delta   │
              │ (Execution)    │ │ observers      │ │ (Mutations)   │
              │                │ │ (Events)       │ │                │
              └────────────────┘ └────────────────┘ └─────────────────┘
```

## Expected Output

After normalization in diagram mode, the output should:
1. Maintain box hierarchy relationships
2. Preserve all text content within boxes
3. Ensure vertical arrows align with box centers
4. Maintain consistent spacing between adjacent boxes
5. Be idempotent (running twice produces identical output)

## Test Verification Criteria

- [x] All boxes detected correctly (4 total: 1 parent + 3 middle + 3 bottom)
- [x] Arrow alignment maintained through all three levels
- [x] Box width consistency within each column
- [x] Text content preserved and properly centered
- [x] Idempotence verified
