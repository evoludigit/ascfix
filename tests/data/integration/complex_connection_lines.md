# Complex: Connection Lines and Elbows
# Tests L-shaped connection paths with elbow rendering

# Simple L-shape connection
┌──────┐
│Start │──┐
└──────┘  │
           ▼
        ┌──────┐
        │ End  │
        └──────┘

# Complex path with multiple elbows
                    ┌────────────┐
                    │   Server   │
                    └─────┬──────┘
                          │
                          │
                    ┌─────▼──────┐
                    │  Load      │
                    │  Balancer  │
                    └─────┬──────┘
                          │
           ┌──────────────┼──────────────┐
           │              │              │
     ┌─────▼────┐   ┌─────▼────┐   ┌─────▼────┐
     │ Web App  │   │ API      │   │ Database │
     │ Server   │   │ Gateway  │   │ Cluster  │
     └──────────┘   └──────────┘   └──────────┘

# Connection lines with labels
┌────────┐     ┌────────┐
│Client  │────▶│ Proxy  │
└────────┘     └────────┘
   │              │
   │              │
   ▼              ▼
"HTTP/1.1"     "HTTP/2"

# Vertical connections
┌─────┐
│  A  │
└─────┘
   │
   │
   ▼
┌─────┐
│  B  │
└─────┘
   │
   │
   ▼
┌─────┐
│  C  │
└─────┘