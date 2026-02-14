# LLM Generated: Tables and Diagrams Mixed
# Real-world scenario: documentation with both tables and diagrams

System Configuration:

| Component | Version | Status |
|-----------|---------|--------|
|Frontend| 1.0.0 |Active|
|Backend | 2.1.5|  Active |
|Database  |5.7  | Active |

Architecture flow:
┌─────────┐
│ Request │
└────┬────┘
     ▼
┌────────────┐    ┌──────────┐
│  Router   │───▶│ Handler  │
└────────────┘    └────┬─────┘
                       ▼
                  ┌─────────┐
                  │Database │
                  └─────────┘

Performance metrics:

| Metric | Value | Unit |
|--------|-------|------|
|Latency| 50 | ms |
|Throughput|10000|req/s|
|Errors|0.1|%|
