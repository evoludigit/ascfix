# Real World: Security Architecture
# Tests security-focused diagram patterns

┌─────────────────────────────────────────────────────────────┐
│                    SECURITY ARCHITECTURE                     │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────┐    │
│  │                EXTERNAL TRAFFIC                     │    │
│  │                                                     │    │
│  │  ┌─────────────┐    ┌─────────────┐    ┌─────────┐   │    │
│  │  │  CloudFront │    │   WAF       │    │  ELB    │   │    │
│  │  │   CDN       │    │   Rules     │    │  Load   │   │    │
│  │  │   Caching   │    │   Filtering │    │ Balancer │   │    │
│  │  └──────┬──────┘    └──────┬──────┘    └─────┬───┘   │    │
│  │         │                   │                  │       │    │
│  └─────────┼───────────────────┼──────────────────┼───────┘    │
│            │                   │                  │            │
│  ┌─────────▼───────────────────▼──────────────────▼─────────┐ │
│  │                    APPLICATION LAYER                      │ │
│  │                                                          │ │
│  │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐   │ │
│  │  │   API GW    │    │  Auth Svc   │    │   App Svc   │   │ │
│  │  │   JWT       │    │  OAuth2     │    │   Business  │   │ │
│  │  │   Throttling│    │  MFA        │    │   Logic     │   │ │
│  │  └──────┬──────┘    └──────┬──────┘    └──────┬──────┘   │ │
│  │         │                   │                   │         │ │
│  └─────────┼───────────────────┼───────────────────┼─────────┘ │
│            │                   │                   │           │
│  ┌─────────▼───────────────────▼───────────────────▼─────────┐ │
│  │                      DATABASE LAYER                        │ │
│  │                                                          │ │
│  │  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐   │ │
│  │  │ PostgreSQL  │    │   Redis     │    │   S3       │   │ │
│  │  │ Encrypted   │    │   Cache     │    │   Files    │   │ │
│  │  │ RLS         │    │   TLS       │    │   SSE       │   │ │
│  │  └─────────────┘    └─────────────┘    └─────────────┘   │ │
│  └──────────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────┐    │
│  │                MONITORING & LOGGING                 │    │
│  │                                                     │    │
│  │  ┌─────────────┐    ┌─────────────┐    ┌─────────┐   │    │
│  │  │ CloudWatch  │    │ CloudTrail  │    │  SIEM   │   │    │
│  │  │   Metrics   │    │   Audit     │    │ Security│   │    │
│  │  │   Alarms    │    │   Logs      │    │  Events │   │    │
│  │  └─────────────┘    └─────────────┘    └─────────┘   │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘

# Security Controls Matrix
| Layer | Control Type | Implementation | Status |
|-------|--------------|----------------|--------|
| Network | Firewall | AWS Security Groups | ✅ |
| Network | DDoS Protection | AWS Shield | ✅ |
| Application | Input Validation | Server-side validation | ✅ |
| Application | Authentication | JWT + MFA | ✅ |
| Application | Authorization | RBAC | ✅ |
| Data | Encryption at Rest | AES-256 | ✅ |
| Data | Encryption in Transit | TLS 1.3 | ✅ |
| Data | Data Masking | Column-level | 🚧 |