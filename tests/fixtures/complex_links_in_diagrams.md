# Complex: Links in Diagrams
# Tests markdown links within ASCII diagram contexts

# Links in box content
┌─────────────────────────────────┐
│ [Frontend](https://frontend.com) │
│                                 │
│ Uses [React](https://reactjs.org)│
│ and [TypeScript](https://typescriptlang.org) │
└─────────────────────────────────┘

# Links with parentheses in URLs
┌─────────────────────────────┐
│ API: [Documentation](https://api.example.com/v1/docs) │
│                                │
│ [Advanced Features](https://example.com/features(v2)) │
└─────────────────────────────┘

# Reference-style links
┌─────────────────┐    ┌─────────────────┐
│   Client    [1] │───▶│   Server    [2] │
└─────────────────┘    └─────────────────┘

[1]: https://client.example.com
[2]: https://server.example.com/api(v2)

# Links in complex diagrams
┌─────────────────┐
│ Web Application │
│                 │
│ [React SPA][spa]│
│ [API Docs][api] │
└─────────┬───────┘
          │
          ▼
┌─────────▼───────┐
│   API Gateway   │
│                 │
│ [Kong][kong]    │
│ [Rate Limiting][rate] │
└─────────────────┘

[spa]: https://reactjs.org
[api]: https://api.example.com/docs
[kong]: https://konghq.com
[rate]: https://example.com/rate-limiting

# Links in table cells (mixed content)
| Component | Link | Description |
|-----------|------|-------------|
| Frontend | [React](https://reactjs.org) | UI Framework |
| Backend | [Node.js](https://nodejs.org) | Runtime |
| Database | [PostgreSQL](https://postgresql.org) | RDBMS |

```javascript
// Code with links
const api = require('[axios](https://axios-http.com)');
```