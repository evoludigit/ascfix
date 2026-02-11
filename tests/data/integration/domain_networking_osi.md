# Domain: Networking - OSI Model
# Tests layered network architecture diagrams

┌─────────────────────────────────────────────────────────┐
│                    OSI Model Layers                     │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────┐    │
│  │                Layer 7: Application           │    │
│  │                                                 │    │
│  │  HTTP, FTP, SMTP, DNS, DHCP, Telnet, SSH      │    │
│  └─────────────────────────────────────────────────┘    │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────┐    │
│  │               Layer 6: Presentation            │    │
│  │                                                 │    │
│  │  Data translation, encryption, compression     │    │
│  └─────────────────────────────────────────────────┘    │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────┐    │
│  │                Layer 5: Session                │    │
│  │                                                 │    │
│  │  Session establishment, maintenance, termination│    │
│  └─────────────────────────────────────────────────┘    │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────┐    │
│  │               Layer 4: Transport                │    │
│  │                                                 │    │
│  │  TCP, UDP - reliable/unreliable delivery        │    │
│  └─────────────────────────────────────────────────┘    │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────┐    │
│  │                Layer 3: Network                 │    │
│  │                                                 │    │
│  │  IP, ICMP, ARP - routing and addressing         │    │
│  └─────────────────────────────────────────────────┘    │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────┐    │
│  │               Layer 2: Data Link                │    │
│  │                                                 │    │
│  │  Ethernet, WiFi, PPP - framing, MAC addresses  │    │
│  └─────────────────────────────────────────────────┘    │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────┐    │
│  │               Layer 1: Physical                 │    │
│  │                                                 │    │
│  │  Cables, hubs, repeaters - electrical signals  │    │
│  └─────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘

# TCP/IP Model Comparison
| OSI Layer | TCP/IP Layer | Protocols | PDU |
|-----------|--------------|-----------|-----|
| 7 Application | Application | HTTP, FTP, SMTP | Data |
| 6 Presentation | Application | SSL/TLS, MIME | Data |
| 5 Session | Application | NetBIOS, RPC | Data |
| 4 Transport | Transport | TCP, UDP | Segment |
| 3 Network | Internet | IP, ICMP | Packet |
| 2 Data Link | Network Access | Ethernet | Frame |
| 1 Physical | Network Access | Cables | Bits |