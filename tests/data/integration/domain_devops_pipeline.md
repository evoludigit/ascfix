# Domain: DevOps CI/CD Pipeline
# Tests complex workflow diagrams common in DevOps documentation

┌─────────────────┐
│   Code Push     │
└─────────┬───────┘
          │
          ▼
┌─────────────────┐    ┌─────────────────┐
│  Git Hooks      │    │   Pre-commit    │
│  Validation     │    │   Checks        │
└─────────┬───────┘    └─────────┬───────┘
          │                      │
          └─────────┬────────────┘
                    │
                    ▼
          ┌─────────────────┐
          │   CI Pipeline   │
          └─────────┬───────┘
                    │
          ┌─────────┼─────────┐
          │         │         │
    ┌─────▼────┐ ┌──▼──┐ ┌───▼────┐
    │   Lint   │ │Test │ │ Build  │
    │           │ │     │ │        │
    └─────┬────┘ └─────┘ └────┬───┘
          │                   │
          └─────┬─────────────┘
                │
                ▼
          ┌─────────────────┐
          │   Artifacts     │
          └─────────┬───────┘
                    │
          ┌─────────┼─────────┐
          │         │         │
    ┌─────▼────┐ ┌──▼──┐ ┌───▼────┐
    │Deploy to │ │     │ │Deploy to│
    │  Staging │ │ UAT │ │Production│
    └──────────┘ └─────┘ └─────────┘

# Pipeline stages with labels
Build Stage: Compile and package application
Test Stage: Run unit, integration, and E2E tests
Deploy Stage: Automated deployment with rollback capability

# Configuration as code
```yaml
# .github/workflows/ci.yml
name: CI/CD Pipeline
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
      - name: Install dependencies
        run: npm ci
      - name: Run tests
        run: npm test
```