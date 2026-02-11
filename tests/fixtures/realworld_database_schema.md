# Real World: Database Schema
# Tests common database schema visualization patterns

                    ┌─────────────────┐
                    │   users         │
                    ├─────────────────┤
                    │ id (PK)         │
                    │ email (UQ)      │
                    │ password_hash   │
                    │ created_at      │
                    │ updated_at      │
                    └─────────┬───────┘
                              │
                              │ 1:N
                              ▼
┌─────────────────┐    ┌─────────────────┐
│   orders        │    │   products      │
├─────────────────┤    ├─────────────────┤
│ id (PK)         │    │ id (PK)         │
│ user_id (FK)    │    │ name            │
│ total_amount    │    │ price           │
│ status          │    │ description     │
│ created_at      │    │ category_id (FK)│
└─────────┬───────┘    └─────────┬───────┘
          │                            │
          │ N:M                        │ N:1
          ▼                            ▼
┌─────────────────┐          ┌─────────────────┐
│ order_items     │          │   categories    │
├─────────────────┤          ├─────────────────┤
│ id (PK)         │          │ id (PK)         │
│ order_id (FK)   │          │ name            │
│ product_id (FK) │          │ description     │
│ quantity        │          │ parent_id (FK)  │
│ unit_price      │          └─────────────────┘
└─────────────────┘
          │
          │ N:1
          ▼
┌─────────────────┐
│   inventory     │
├─────────────────┤
│ id (PK)         │
│ product_id (FK) │
│ quantity        │
│ location        │
│ last_updated    │
└─────────────────┘

# Relationships Legend
PK = Primary Key
FK = Foreign Key
UQ = Unique Constraint
1:1 = One to One
1:N = One to Many
N:M = Many to Many