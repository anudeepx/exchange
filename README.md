<div id="logo" align="center">
  <a href="https://github.com/0x4nud33p/exchange">
    <picture>
      <img alt="Exchange" src="./docs/logo.svg" width="376" height="100" />
    </picture>
  </a>
  <br />
</div>

<div id="description" align="center">
  A fast, modular <strong>exchange</strong> backend written in Rust.
</div>

<br />

<div id="badges" align="center">

  [![Rust](https://img.shields.io/badge/Rust-stable-orange)](https://www.rust-lang.org/)
  [![GitHub stars](https://img.shields.io/github/stars/0x4nud33p/exchange?style=social)](https://github.com/0x4nud33p/exchange)
  [![Security](https://img.shields.io/badge/Security-Scanned-brightgreen)](https://github.com/0x4nud33p/exchange/security)
  [![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
</div>

---

## ❓ What's Exchange?

**Exchange** is a high-performance, modular exchange system designed for speed, security, and scalability.  
Built in **Rust**, it leverages memory safety and concurrency to deliver reliable APIs for order matching, user authentication, and state management.  

It is designed as a **pluggable architecture**:  
- Separate crates/modules for authentication, Redis integration, matching engine, and APIs.  
- Easy to extend for new assets, trading pairs, or persistence backends.  


## ❄️ Architecture

```mermaid
flowchart LR
    Client[Client]

    LB[Load Balancer]

    API1[API Service]
    API2[API Service]
    API3[API Service]

    Queue[Message Queue]

    Engine[Matching Engine]

    DB[(PostgreSQL)]
    WS[WebSocket Service]
    Backup[(Event Log Backup)]

    Client --> LB

    LB --> API1
    LB --> API2
    LB --> API3

    API1 --> Queue
    API2 --> Queue
    API3 --> Queue

    Queue --> Engine

    Engine --> DB
    Engine --> WS
    Engine --> Backup
```
