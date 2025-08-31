# SolanaShredProxy

SolanaShredProxy is a Rust-based proxy for Solana ShredStream, providing ultra-low latency access to leader shreds on the Solana blockchain.

---

## Disclaimer
Use this software at your own risk. The developers do not assume responsibility for losses or network issues.  

---

## Features
- Low-latency access to leader shreds on Solana  
- Configurable target regions for optimized connectivity  
- gRPC service for downstream clients  
- Authenticated access via keypair  

---

## Installation
### Prerequisites
- Rust (latest stable recommended)  
- Linux system with GLIBC ≥ 2.35  

### Build
```bash
git clone https://github.com/xerion0712/solanashredproxy.git
cd solanashredproxy
cargo build --release
