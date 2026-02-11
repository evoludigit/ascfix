# Complex: Mixed Content (Diagrams + Tables + Code)
# Tests handling of mixed markdown content types

# Architecture diagram
┌─────────────────┐    ┌─────────────────┐
│   Web Server    │    │   Database      │
│                 │    │                 │
│ - Nginx         │    │ - PostgreSQL    │
│ - Load balanced │    │ - Sharded       │
└─────────┬───────┘    └───────┬─────────┘
          │                    │
          └─────────┬──────────┘
                    │
          ┌─────────▼─────────┐
          │    Cache Layer    │
          │                   │
          │ - Redis Cluster   │
          │ - In-memory       │
          └───────────────────┘

# Performance metrics table
| Component | Response Time | Throughput | Availability |
|-----------|---------------|------------|--------------|
| Web Server|     45ms      |  1000 RPS  |    99.9%     |
| Database  |     12ms      |  5000 RPS  |    99.95%    |
| Cache     |      2ms      | 50000 RPS  |    99.99%    |

# Code example
```javascript
// API endpoint with caching
app.get('/api/users/:id', async (req, res) => {
  const cacheKey = `user:${req.params.id}`;

  // Check cache first
  const cached = await redis.get(cacheKey);
  if (cached) {
    return res.json(JSON.parse(cached));
  }

  // Fetch from database
  const user = await db.query('SELECT * FROM users WHERE id = $1',
                              [req.params.id]);

  // Cache result
  await redis.setex(cacheKey, 3600, JSON.stringify(user));

  res.json(user);
});
```

# Another diagram after code
┌─────────────┐
│   Client    │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ API Gateway │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Services   │
└─────────────┘