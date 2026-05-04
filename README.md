# JurisTrust
**Smart Multi-Signature Legal Escrow**

### Problem
Traditional legal trust accounts are slow, manual, and expensive to manage, requiring constant bank reconciliations and administrative oversight.

### Solution
JurisTrust automates the IOLTA (trust account) process. Funds are locked on-chain and only released to the intended recipient once all authorized legal counsel have provided a cryptographic signature, ensuring compliance and speed.

### Timeline
- **Build:** 2 Days
- **Test:** 1 Day
- **UI Mockup:** 1 Day

### Stellar Features
- **Soroban Multi-Auth:** Requires explicit `require_auth()` from multiple attorney addresses.
- **Atomic Settlement:** Instantly moves USDC upon final approval.

### Prerequisites
- Rust & Cargo
- Soroban CLI

### Build & Test
```bash
soroban contract build
cargo test

##contract
CDTHXZP2J4KE2CAL3QYYDI5IW4ZSY2Y3WWZO6UPYATHUGYTTF3S45AGF
CDTHXZP2J4KE2CAL3QYYDI5IW4ZSY2Y3WWZO6UPYATHUGYTTF3S45AGF