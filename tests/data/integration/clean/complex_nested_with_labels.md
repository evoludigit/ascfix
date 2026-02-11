# Complex: Nested Boxes with Labels
# Tests nested box hierarchies with attached labels

┌─────────────────────────────────────────────────┐
│                E-COMMERCE PLATFORM              │
│                                                 │
│  ┌─────────────────────────────────────────┐    │
│  │           FRONTEND LAYER               │    │
│  │                                         │    │
│  │    ┌─────────────┐    ┌─────────────┐   │    │
│  │    │   React     │    │   Vue.js    │   │    │
│  │    │ Components  │    │ Framework   │   │    │
│  │    └─────────────┘    └─────────────┘   │    │
│  │                                         │    │
│  └─────────────────────────────────────────┘    │
│                                                 │
│  ┌─────────────────────────────────────────┐    │
│  │           BACKEND SERVICES              │    │
│  │                                         │    │
│  │    ┌─────────────┐    ┌─────────────┐   │    │
│  │    │   Node.js   │    │   Python    │   │    │
│  │    │   Express   │    │   Django    │   │    │
│  │    └─────────────┘    └─────────────┘   │    │
│  │                                         │    │
│  └─────────────────────────────────────────┘    │
│                                                 │
└─────────────────────────────────────────────────┘

# Labels for the boxes above
Frontend
    ↓
Backend

# Connection lines with labels
┌──────┐     ┌──────┐
│ API  │────▶│ DB   │
└──────┘     └──────┘
   ▲            │
   │            ▼
"REST calls"  "Queries"
