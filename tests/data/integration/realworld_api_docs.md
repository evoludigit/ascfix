# Real World: API Documentation Pattern
# Tests common API documentation diagram patterns

┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Client App    │    │   API Gateway   │    │  Microservice   │
│                 │    │                 │    │                 │
│ - Mobile App    │    │ - Authentication │    │ - Business     │
│ - Web App       │    │ - Rate Limiting  │    │   Logic        │
│ - Desktop App   │    │ - Routing       │    │ - Data Access   │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          │                      │                      │
          ▼                      ▼                      ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   HTTP/HTTPS    │    │   REST/GraphQL  │    │   Database      │
│   Transport     │    │   Protocol      │    │   Layer         │
│                 │    │                 │    │                 │
│ - SSL/TLS       │    │ - JSON/XML      │    │ - PostgreSQL    │
│ - Compression   │    │ - OpenAPI Spec  │    │ - Redis Cache   │
│ - Headers       │    │ - Versioning    │    │ - Connection    │
└─────────────────┘    └─────────────────┘    └─────────────────┘

# API Response Codes
| Method | Endpoint | Success | Error Codes |
|--------|----------|---------|-------------|
| GET | /api/users | 200 OK | 401, 403, 404 |
| POST | /api/users | 201 Created | 400, 401, 422 |
| PUT | /api/users/{id} | 200 OK | 400, 401, 404 |
| DELETE | /api/users/{id} | 204 No Content | 401, 403, 404 |

# Authentication Flow
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Login     │    │  Validate   │    │   JWT       │
│  Request    │───▶│  Credentials│───▶│   Token     │
│             │    │             │    │             │
│ - Username  │    │ - Check DB  │    │ - Sign      │
│ - Password  │    │ - Hash      │    │ - Expire    │
└─────────────┘    └─────────────┘    └─────────────┘
       ▲                   │                   │
       │                   │                   ▼
       └───────────────────┼───────────────────┐
                           ▼                   │
                    ┌─────────────┐    ┌─────────────┐
                    │  401 Error  │    │  200 OK     │
                    │             │    │             │
                    │ - Invalid   │    │ - Token     │
                    │   Creds     │    │ - User Data │
                    └─────────────┘    └─────────────┘