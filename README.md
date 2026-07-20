# SASE License Core

> **Sovereign Application Security Engine** — A `no_std` Rust licensing engine for zero-trust software distribution with cryptographic verification, hardware binding, and government-grade compliance.

[![Crates.io](https://img.shields.io/crates/v/sase-license-core.svg)](https://crates.io/crates/sase-license-core)
[![Documentation](https://docs.rs/sase-license-core/badge.svg)](https://docs.rs/sase-license-core)
[![License](https://img.shields.io/crates/l/sase-license-core.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://blog.rust-lang.org/2023/12/21/Rust-1.75.0.html)
[![Zero-Heap](https://img.shields.io/badge/zero--heap-compliant-brightgreen.svg)](#zero-heap-compliance)

A production-ready licensing engine designed for **SASE (Secure Access Service Edge)** platforms, **sovereign cloud providers**, and **government infrastructure** requiring tamper-proof license enforcement without runtime dependencies.

---

## 🎯 What This Software Does

**SASE License Core** is the **verification and enforcement layer** for software licensing. It answers one critical question at runtime:

> *"Is this software authorized to run on this hardware, at this time, with these features?"*

It does this through:

| Capability | Description |
|------------|-------------|
| **Cryptographic Verification** | Ed25519 signatures over license policies — impossible to forge without the private key |
| **Hardware Binding** | Licenses cryptographically bound to device fingerprints (TPM PCRs, CPU IDs, custom HW IDs) |
| **Tiered Feature Gates** | 6 license tiers × 64 features across 8 categories — enforce what each customer pays for |
| **Time-Bounded Validity** | Unix timestamp windows with constant-time validation — no clock manipulation attacks |
| **TPM2 PCR Sealing** | Policies sealed to platform configuration registers — detects boot/rootkit tampering |
| **Merkle-DAG Audit Log** | Append-only tamper-evident log with SQLite WAL — forensic-grade compliance trails |
| **Zero-Heap Hot Paths** | Verification runs in `no_std` + `const_fn` contexts — suitable for kernels, bootloaders, WASM, enclaves |

---

## 🏗️ How It Works (Architecture)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         SASE LICENSE CORE ARCHITECTURE                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌──────────────┐    ┌──────────────────┐    ┌────────────────────────┐   │
│   │  LICENSE     │    │  VERIFICATION    │    │  ENFORCEMENT           │   │
│   │  ISSUANCE    │───▶│  ENGINE (THIS    │───▶│  (YOUR APPLICATION)    │   │
│   │  (OFFLINE)   │    │  CRATE)          │    │                        │   │
│   └──────────────┘    └──────────────────┘    └────────────────────────┘   │
│                              │                       ▲                      │
│                              ▼                       │                      │
│                     ┌──────────────────┐            │                      │
│                     │  HARDWARE ROOT   │            │                      │
│                     │  OF TRUST        │            │                      │
│                     │  (TPM2 / Secure  │            │                      │
│                     │   Enclave / CPU  │────────────┘                      │
│                     │   ID)            │                                   │
│                     └──────────────────┘                                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Core Data Flow

1. **Offline Issuance** (your build/CI pipeline):
   ```
   Policy (tier + features + quotas + validity) 
       │
       ▼
   Serialize → Sign with Ed25519 Private Key → LicenseKey (postcard/JSON)
   ```

2. **Runtime Verification** (in your application, zero-heap):
   ```
   LicenseKey + Policy + Timestamp + HW Fingerprint
       │
       ▼
   verify_license() → Result<(), VerifyError>  // Constant-time, no allocations
   ```

3. **Enforcement** (your code):
   ```rust
   match verify_license(key_bytes, &policy, now, hw_id) {
       Ok(()) => enable_features(policy.features()),
       Err(e) => deny_access(e),
   }
   ```

### Verification Internals (Zero-Heap Path)

```rust
// Hot path — no allocations, constant-time, no_std compatible
pub fn verify_license(
    key_bytes: &[u8],           // postcard-encoded LicenseKey
    policy: &LicensePolicy,     // Expected policy (from your config)
    ctx: VerificationContext    // timestamp + optional hw_fingerprint + pcr_values
) -> Result<(), VerifyError> {
    // 1. Decode (postcard, zero-copy where possible)
    let key = LicenseKey::from_postcard(key_bytes)?;
    
    // 2. Verify Ed25519 signature over policy (constant-time)
    Ed25519Verifier::verify_ct(&key.policy_bytes, &key.signature, &key.public_key)?;
    
    // 3. Verify key ID = BLAKE3(public_key) (anti-substitution)
    assert_eq!(key.key_id, KeyId::from_public_key(&key.public_key));
    
    // 4. Validate policy against expected (tiers, features, quotas)
    policy.validate(ctx.current_time)?;
    
    // 5. Hardware binding check (if required by tier)
    if policy.requires_hw_binding() {
        verify_hw_binding(&key, ctx.hw_fingerprint)?;
    }
    
    // 6. TPM PCR binding check (if sealed)
    if policy.requires_tpm() {
        verify_tpm_pcr_binding(&key, ctx.pcr_values)?;
    }
    
    Ok(())
}
```

---

## 💰 Business Model & Licensing Tiers

This crate is released under **dual license: MIT OR Apache-2.0** — free for all use.

### Our Commercial Offering (Not in This Repo)

| Tier | Target | What You Get |
|------|--------|--------------|
| **Community (Free)** | Open source, hobbyists, startups | This crate + docs + community support |
| **Scientific / Research** | Universities, labs, R&D teams | **TERNAL SDK** — advanced policy synthesis, ML-based anomaly detection, automated compliance reporting, priority support |
| **Government / Sovereign** | Defense, intelligence, critical infrastructure | **TERNAL Supervisor** — air-gapped deployment, classified policy workflows, HSM integration, formal verification artifacts, 24/7 operational support |

> **TERNAL** is our proprietary technology stack (policy synthesis engine, ML anomaly detector, HSM abstraction layer, formal verification pipeline). It is **not open source** and not included in this repository. This crate provides the **verification runtime** that TERNAL-produced licenses run on.

---

## 🚀 Quick Start

### Rust (no_std, minimal)

```toml
# Cargo.toml
[dependencies]
sase-license-core = { version = "0.1", default-features = false, features = [] }
```

```rust
use sase_license_core::{LicenseTier, FeatureSet, LicensePolicy, verify_license};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define expected policy (embed in your binary)
    let policy = LicensePolicy::builder(LicenseTier::Pro)
        .max_tokens(50_000_000)
        .max_requests(5_000_000)
        .add_feature(FeatureSet::WafProtection)
        .add_feature(FeatureSet::DdosProtection)
        .validity(1_700_000_000, 1_800_000_000)
        .build();

    // Load license from file/device
    let key_bytes = std::fs::read("license.key")?;
    let timestamp = current_unix_timestamp(); // Your time source
    
    // Zero-heap verification
    verify_license(&key_bytes, &policy, timestamp)?;
    println!("✅ License valid — enabling Pro features");
    Ok(())
}
```

### Rust (std, all features)

```toml
[dependencies]
sase-license-core = { version = "0.1", features = ["std", "crypto", "tpm", "sqlite", "wasm"] }
```

### Python Bindings

```bash
pip install sase-license-core
```

```python
from sase_license_core import (
    LicenseTier, FeatureFlags, FeaturePy,
    LicensePolicy, verify_license
)
import time

policy = LicensePolicy(
    tier=LicenseTier.Pro,
    features=FeatureFlags(FeaturePy.WafProtection | FeaturePy.DdosProtection),
    max_tokens=50_000_000,
    max_requests=5_000_000,
    max_compute=16,
    valid_from=1_700_000_000,
    valid_until=1_800_000_000,
)

with open("license.key", "rb") as f:
    key_bytes = f.read()

verify_license(key_bytes, policy, int(time.time()))
print("✅ License valid")
```

---

## 📦 Feature Matrix

| Feature | Description | Dependencies | Use Case |
|---------|-------------|--------------|----------|
| `std` | Standard library support (default) | — | Most applications |
| `crypto` | Ed25519 signing/verification | `ed25519-dalek`, `signature` | **Required for verification** |
| `python` | PyO3 bindings | `std`, `pyo3` | Python applications |
| `tpm` | Real TPM2 via TSS-ESAPI | `std`, system `tss2` | Hardware-rooted sealing |
| `sqlite` | Merkle-DAG audit log | `std`, `rusqlite` | Compliance/forensics |
| `wasm` | WASM target support | `std` or `no_std` | Browser/edge workers |
| `zeroize` | Secret zeroization on drop | `zeroize` | High-security environments |

### Minimal Verification-Only Build

```toml
[dependencies]
sase-license-core = { version = "0.1", default-features = false, features = ["crypto"] }
```

---

## 🔐 License Tiers (Built-In)

| Tier | Max Tokens | Max Requests | Max Compute | HW Binding | TPM Required |
|------|------------|--------------|-------------|------------|--------------|
| **Free** | 10,000 | 1,000 | 1 | No | No |
| **Base** | 1M | 100K | 4 | No | No |
| **Pro** | 100M | 10M | 32 | No | No |
| **Engineering** | 1B | 1B | 256 | No | No |
| **Sovereign** | ∞ | ∞ | ∞ | **Yes** | **Yes** |
| **Government** | ∞ | ∞ | ∞ | **Yes** | **Yes** |

Tiers are **extensible** — define your own via `LicenseTier::Custom(u8)`.

---

## ⚙️ Feature Flags (64 Features, 8 Categories)

```
Core (8)          → BasicAuth, RateLimiting, RequestTransform, ResponseCaching,
                    RequestBuffering, ConnectionPooling, CircuitBreaker, HealthChecks

Network (8)       → TlsTermination, Mtls, LoadBalancing, AdvancedRouting,
                    ServiceMesh, TrafficSplitting, GrpcTranscoding, WebsocketSupport

Security (8)      → WafProtection, DdosProtection, BotDetection, IpReputation,
                    GeoBlocking, RequestValidation, ResponseSanitization, CertificateManagement

Observability (8) → BasicLogging, AdvancedLogging, MetricsExport, DistributedTracing,
                    RealtimeAnalytics, AuditLogging, CustomDashboards, Alerting

Policy (8)        → CustomPolicies, Rbac, Abac, PolicyAsCode, PolicySimulation,
                    CompliancePolicies, Dlp, SecretsManagement

Advanced (8)      → MultiTenancy, Federation, AirGap, SovereignCloud,
                    HardwareBinding, TpmSealing, SecureEnclave, QuantumResistant

Government (8)    → ClassifiedData, CrossDomain, MandatoryAccessControl, TypeEnforcement,
                    MultiLevelSecurity, CompartmentedMode, TrustedPath, CovertChannelAnalysis
```

Use `FeatureSet` (bitflags) for efficient storage and runtime checks.

---

## 🔐 Zero-Heap Compliance

Enforce **zero heap allocations** in verification hot paths at compile time:

```bash
# Fails compilation if ANY heap allocation in #[inline(always)] hot paths
RUSTFLAGS="-DSASE_ZERO_HEAP=1" cargo build --no-default-features --target wasm32-unknown-unknown
```

This uses a custom `zero_heap_hot_path!` macro that expands to a `const` assertion checking `std::alloc::GlobalAlloc` is never invoked. Ideal for:
- Kernel modules / bootloaders
- SGX/TEE enclaves
- WASM runtimes with no allocator
- Safety-critical real-time systems

---

## 🏛️ TPM2 Integration

### Mock TPM (Testing, CI, `no_std`)

```rust
use sase_license_core::{MockTpmSealer, PcrSelection, LicensePolicy, LicenseTier};

let mut tpm = MockTpmSealer::new();
tpm.init()?;
tpm.set_pcr(0, [0x01; 32]);
tpm.set_pcr(7, [0x02; 32]);

let policy = LicensePolicy::builder(LicenseTier::Sovereign)
    .hw_id([0x42; 32])
    .build();

let sealed = tpm.seal_to_pcr(&policy, &PcrSelection::standard())?;
let unsealed = tpm.unseal(&sealed)?;
```

### Real TPM2 (Feature `tpm`)

```rust
#[cfg(feature = "tpm")]
use sase_license_core::tpm2::Tpm2Sealer;

#[cfg(feature = "tpm")]
let tpm = Tpm2Sealer::new()?; // Uses TCTI from TSS2_TCTI env var
```

PCRs 0-7 + 16 sealed by default (boot integrity + custom).

---

## 📜 Merkle-DAG Audit Log (Feature `sqlite`)

```rust
#[cfg(all(feature = "std", feature = "sqlite"))]
use sase_license_core::MerkleLog;

let mut log = MerkleLog::new_file("audit.db")?;

log.append_policy(&policy)?;
log.append_key(&key)?;

let anchor = log.create_anchor()?;  // Periodic checkpoint
log.verify_integrity()?;            // Tamper detection
```

- SQLite WAL mode for crash consistency
- Merkle proofs for inclusion/exclusion
- Anchors for timestamping / transparency logs

---

## 🛡️ Security Properties

| Property | Implementation |
|----------|----------------|
| **Signature Forgery** | Ed25519 (RFC 8032) — 128-bit security |
| **Replay Attacks** | Timestamp validation + nonce in policy |
| **Key Substitution** | KeyID = BLAKE3(public_key) bound in license |
| **Clock Drift** | Constant-time comparison, configurable skew tolerance |
| **Side Channels** | `verify_ct()` constant-time verification path |
| **Memory Safety** | Rust `no_std` + `zeroize` — secrets zeroed on drop |
| **Supply Chain** | `postcard` deterministic serialization, no proc-macros in hot path |
| **TPM Binding** | PCR 0-7,16 — detects bootkit/rootkit modification |

---

## 🧪 Testing

```bash
# All features, std
cargo test --features "std,crypto,tpm,sqlite,wasm"

# Zero-heap verification (no_std)
RUSTFLAGS="-DSASE_ZERO_HEAP=1" cargo test --no-default-features --features crypto

# Python bindings
maturin develop --features python && pytest tests/python/

# WASM
cargo test --target wasm32-unknown-unknown --features wasm
```

---

## 📋 Requirements

| Component | Minimum Version |
|-----------|-----------------|
| Rust | 1.75+ (for `const_fn` traits) |
| Python | 3.8+ (for PyO3 bindings) |
| TPM2 + TSS2 | For `tpm` feature |
| SQLite3 | For `sqlite` feature |

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/amazing-feature`
3. Run tests: `cargo test --all-features`
4. Run zero-heap check: `RUSTFLAGS="-DSASE_ZERO_HEAP=1" cargo test --no-default-features --features crypto`
5. Submit PR with clear description

**Code Standards:**
- `no_std` compatible by default
- `const_fn` where possible
- Zero-heap hot paths marked with `zero_heap_hot_path!`
- All secrets implement `ZeroizeOnDrop`
- Documentation on all public APIs

---

## 📄 License

**Dual-licensed** under your choice of:
- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

This applies to **this crate only**. The TERNAL proprietary stack (policy synthesis, ML anomaly detection, HSM abstraction, formal verification) is **not included** and requires a commercial license.

---

## 🔗 Links

- **Documentation**: https://docs.rs/sase-license-core
- **Crates.io**: https://crates.io/crates/sase-license-core
- **Repository**: https://github.com/sase-antigravity/sase-license-core
- **Issues**: https://github.com/sase-antigravity/sase-license-core/issues
- **Commercial Inquiries**: licensing@sase-antigravity.dev

---

## 🏷️ Versioning

[SemVer](https://semver.org/) — `MAJOR.MINOR.PATCH`

- **0.1.x**: Initial development, API may change
- **1.0.0**: Stable API, production-ready

---

*Built with ❤️ by the SASE Antigravity team — Sovereign software for a sovereign world.*
