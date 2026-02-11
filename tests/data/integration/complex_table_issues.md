# Complex: Table Issues
# Tests various table formatting problems

# Hard-wrapped table (80 columns)
| Component | Description | Status | Notes |
|-----------|-------------|--------|-------|
| Frontend | React application with TypeScript support and modern build tools | Active | Currently being developed |
| Backend | Node.js API server with Express framework and PostgreSQL database | Active | RESTful API design |
| Database | PostgreSQL with connection pooling and migration scripts | Active | Data modeling complete |
| Testing | Jest test suite with 95% coverage and CI/CD integration | Active | TDD approach used |
| Deployment | Docker containers with Kubernetes orchestration and monitoring | Planned | Infrastructure setup |

# Misaligned table
| Name | Age | City |
|------|-----|------|
| Alice | 30 | New York |
| Bob | 25 | Boston |
| Charlie | 35 | Chicago |

# Table with missing separators
| Header 1 | Header 2 | Header 3 |
|----------|----------|----------|
| Data 1 | Data 2 | Data 3 |
| More data | Additional | Content |

# Table with inconsistent column widths
| Short | This is a very long column header that should cause alignment issues |
|-------|---------------------------------------------------------------------|
| A | Short content |
| B | This is much longer content that will cause the table to be misaligned |

# Empty table
| Column 1 | Column 2 |
|----------|----------|
|          |          |
|          |          |

# Table with special characters
| Symbol | Name | Unicode |
|--------|------|---------|
| π | Pi | U+03C0 |
| ∑ | Sum | U+2211 |
| ∞ | Infinity | U+221E |