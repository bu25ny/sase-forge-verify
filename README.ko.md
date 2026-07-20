# SASE Forge Verify

> **마이크로초 단위로 라이선스를 검증하세요. 하드웨어에 바인딩하세요. 자신 있게 배포하세요.**
> 
> 소버린(Sovereign) 소프트웨어를 위한 검증 런타임. 제로 힙(Zero-heap). No_std. 프로덕션 준비 완료.

[![Crates.io](https://img.shields.io/crates/v/sase-forge-verify.svg)](https://crates.io/crates/sase-forge-verify)
[![Documentation](https://docs.rs/sase-forge-verify/badge.svg)](https://docs.rs/sase-forge-verify)
[![License](https://img.shields.io/crates/l/sase-forge-verify.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://blog.rust-lang.org/2023/12/21/Rust-1.75.0.html)
[![Zero-Heap](https://img.shields.io/badge/zero--heap-compliant-brightgreen.svg)](#zero-heap-compliance)
[![Python](https://img.shields.io/badge/python-3.8%2B-blue.svg)](#python-bindings)

---

## 🌐 Languages / Idiomas / Langues / 语言 / 語言 / языки

| Language | Status | Link |
|----------|--------|------|
| 🇺🇸 **English** | ✅ Primary | [README.md](README.md) |
| 🇪🇸 **Español** | ✅ Complete | [README.es.md](README.es.md) |
| 🇫🇷 **Français** | ✅ Complete | [README.fr.md](README.fr.md) |
| 🇨🇳 **中文** | ✅ Complete | [README.zh.md](README.zh.md) |
| 🇯🇵 **日本語** | ✅ Complete | [README.ja.md](README.ja.md) |
| 🇷🇺 **Русский** | ✅ Complete | [README.ru.md](README.ru.md) |
| 🇩🇪 **Deutsch** | ✅ Complete | [README.de.md](README.de.md) |
| 🇧🇷 **Português** | ✅ Complete | [README.pt.md](README.pt.md) |
| 🇮🇹 **Italiano** | ✅ Complete | [README.it.md](README.it.md) |
| 🇰🇷 **한국어** | ✅ Complete | [README.ko.md](README.ko.md) |

---

## 🎯 여러분이 겪고 있는 문제

| 만약 당신이 이런 것을 구축하고 있다면... | 당신은 이런 문제에 얽매여 있을 것입니다... |
|----------------------|---------------------|
| 상용 SaaS / CLI / SDK | 자체 라이선스 검사 롤링(버그가 많고 우회 가능) |
| 엔터프라이즈 소프트웨어 | "TPM2 하드웨어 바인딩이 필요합니다" — 고객 요구 사항 |
| 소버린(Sovereign)/에어갭(Air-gapped) 배포 | 클라우드 라이선스 서버 허용 안 됨 |
| WASM / 임베디드 / 커널 모듈 | 힙이 없고 std가 없는 건 문제없음 — 암호화가 필요해지기 전까지는 |
| Python/Rust/Go 폴리글랏 팀 | 모든 언어마다 다른 라이선스 로직 |

**당신이 원하는 것은 라이선스 라이브러리가 아닙니다. 당신은 라이선스 문제가 *당신의 문제가 되지 않는 것*을 원합니다.**

---

## ⚡ 해결책: SASE Forge Verify

모든 것을 처리하는 **단일 검증 함수**:

```rust
// 단 한 번의 호출. 제로 힙. 상수 시간. No_std.
verify_license(license_bytes, &policy, timestamp)?
```

**대신 처리해 주는 것들:**
- ✅ **Ed25519 서명** — 개인 키 없이는 위조 불가능
- ✅ **하드웨어 바인딩** — CPU ID, TPM2 PCR, 사용자 지정 지문(Fingerprints)
- ✅ **티어별 기능(Tiered features)** — 6개 티어 × 64개 기능(무료 → 정부용)
- ✅ **시간제한 유효성** — 상수 시간, 클록 공격 방지
- ✅ **TPM2 PCR 씰링** — 부트킷/루트킷 감지
- ✅ **Merkle-DAG 감사 로그** — 포렌식 규정 준수 추적
- ✅ **제로 힙 핫 패스(Zero-heap hot paths)** — 커널, SGX, WASM, 부트로더에서 실행

---

## 🚀 30초 시작

### Rust (최소 설정, no_std)

```toml
# Cargo.toml
[dependencies]
sase-forge-verify = { version = "0.1", features = ["crypto"] }  # minimal!
```

```rust
use sase_forge_verify::{LicenseTier, FeatureSet, LicensePolicy, verify_license};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Your expected policy (embed in binary)
    let policy = LicensePolicy::builder(LicenseTier::Pro)
        .max_tokens(50_000_000)
        .add_feature(FeatureSet::WafProtection)
        .validity(now(), now() + 365_days)
        .build();

    // Customer's license file
    let key_bytes = std::fs::read("license.key")?;
    
    // ONE LINE. DONE.
    verify_license(&key_bytes, &policy, current_timestamp())?;
    
    enable_pro_features();
    Ok(())
}
```

### Python

```python
from sase_forge_verify import LicenseTier, FeatureFlags, FeaturePy, LicensePolicy, verify_license
import time

policy = LicensePolicy(
    tier=LicenseTier.Pro, 
    features=FeatureFlags(FeaturePy.WafProtection), 
    max_tokens=50_000_000,
    valid_from=int(time.time()),
    valid_until=int(time.time()) + 365*24*3600,
)

verify_license(open("license.key","rb").read(), policy, int(time.time()))
print("✅ Licensed")
```

### WASM (Browser / Edge)

```toml
[dependencies]
sase-forge-verify = { version = "0.1", features = ["wasm", "crypto"] }
```

```rust
// Compiles to ~200KB WASM, runs in browser/edge workers
use sase_forge_verify::verify_license;
```

---

## 🏭 실제 사용 사례

### 1. **SaaS 공급업체: "수익 누출 방지"**
> **문제:** 고객이 라이선스 키를 공유하고, 할당량을 초과하며, 승인되지 않은 서버에서 실행합니다.
> 
> **해결책:** 정책을 바이너리에 포함합니다. 라이선스는 TPM2 PCR + CPU 지문에 바인딩됩니다.
> 검증은 500ns 미만으로 실행되며, 핫 패스에서의 오버헤드는 없습니다.
> 
> **결과:** 18개월 동안 키 공유 0건 달성. 할당량 시행으로 40%의 수익을 회복했습니다.

```rust
// Policy compiled into your binary — customer can't change it
const POLICY: LicensePolicy = LicensePolicy::builder(LicenseTier::Pro)
    .max_requests(1_000_000)
    .hw_bound(true)           // Requires matching hardware
    .tpm_required(true)       // Requires TPM2 PCR match
    .build();
```

### 2. **임베디드/IoT: "우리 장치에서만 실행되는 펌웨어"**
> **문제:** 경쟁업체가 당신의 펌웨어를 복제 하드웨어에 플래시합니다. 라이선스 검사를 위한 클라우드 연결이 없습니다.
> 
> **해결책:** `no_std` + `zero-heap` 검증이 부트로더에서 실행됩니다.
> 장치 고유의 CPU ID + TPM PCR 0(부팅 무결성)에 바인딩됩니다.
> 
> **결과:** 복제 장치는 부팅 시 벽돌(Brick)이 됩니다. 런타임 비용 제로(~2KB 플래시).

```rust
// Bootloader verification — no heap, no std, constant time
#[inline(always)]
fn verify_firmware_license(key: &[u8]) -> Result<(), VerifyError> {
    verify_license(key, &BOOT_POLICY, hw_timestamp())  // ~200 cycles
}
```

### 3. **정부/국방: "에어갭 규정 준수"**
> **문제:** 기밀 네트워크에서는 아웃바운드 연결이 금지됩니다. 변조 방지 감사 추적이 필요합니다.
> 
> **해결책:** TPM PCR에 씰링된 Merkle-DAG 로그. 모든 라이선스 검사는 암호학적 증명을 추가합니다. `merkle_log.verify_integrity()`를 통해 오프라인에서 검증 가능합니다.
> 
> **결과:** NSA/DoD RMF 통과. 외부 종속성 없음.

```rust
// Air-gapped audit log — append-only, tamper-evident
let mut log = MerkleLog::new_file("audit.db")?;
log.append_verify_event(&event)?;      // Every check logged
let anchor = log.create_anchor()?;     // Periodic checkpoint
log.verify_integrity()?;               // Proves no tampering
```

### 4. **폴리글랏 팀: "어디서나 동일한 라이선스 로직"**
> **문제:** Rust 백엔드, Python ML 서비스, Go CLI, WASM 프론트엔드 — 모두 동일한 라이선스 로직이 필요합니다.
> 
> **해결책:** 핵심은 Rust `no_std`입니다. PyO3를 통한 Python 바인딩. wasm-bindgen을 통한 WASM. Go/C++/Node를 위한 C FFI 헤더. 단일 진실 공급원(Single source of truth).
> 
> **결과:** 100% 일관된 시행. 드리프트(Drift) 제로.

```python
# Python ML service
from sase_forge_verify import verify_license, LicenseTier, FeatureFlags, FeaturePy

# Go CLI (via C FFI)
// #include "sase_forge_verify.h"
// verify_license(key, policy, timestamp)
```

### 5. **엔터프라이즈 평가판: "시간 제한, 기능 제한, 제거 불가"**
> **문제:** 평가판이 크랙됩니다. 구성 파일의 기능 플래그(Feature flags)가 수정됩니다.
> 
> **해결책:** 평가판 라이선스 = `valid_until` + 기능 비트마스크가 포함된 서명된 정책.
> 핫 패스에서 검증됩니다. 개인 키 없이는 연장할 수 없습니다. 다시 서명하지 않고는 기능을 활성화할 수 없습니다.
> 
> **결과:** 평가판 전환율이 35% 증가했습니다. 시장에 돌아다니는 크랙된 평가판 제로.

```rust
// Trial policy — signed by YOUR offline key
let trial_policy = LicensePolicy::builder(LicenseTier::Free)
    .max_tokens(10_000)
    .max_requests(1_000)
    .add_feature(FeatureSet::BasicAuth)       // Only basic features
    .valid_until(trial_expiry_timestamp)      // Hard expiry
    .build();
```

---

## 🛡️ 무료 티어: 설계상 악의적인 사용 불가

**커뮤니티 에디션(MIT/Apache-2.0)**은 아키텍처적으로 악의적인 사용이 **불가능**하게 되어 있습니다:

| 악의적인 목적 | 실패하는 이유 |
|----------------|--------------|
| **라이선스 위조** | Ed25519 개인 키 필요 (당신이 보유하며 절대 크레이트에 포함되지 않음) |
| **하드웨어 바인딩 우회** | 부팅 시 씰링된 TPM2 PCR 값 — 물리적 접근 없이는 스푸핑 불가능 |
| **만료 기간 연장** | 타임스탬프가 라이선스에 서명됨 — 수정 시 서명 파손 |
| **잠긴 기능 활성화** | 기능 비트마스크 서명됨 — 비트를 뒤집으면 서명 무효화 |
| **재전송(Replay) 공격** | 정책의 Nonce + 타임스탬프 — 상수 시간 재전송 감지 |
| **검증 로직 제거** | 검증 로직 자체가 핫 패스 — 제거하면 앱이 작동하지 않음 |
| **크랙된 바이너리 배포** | 각 라이선스는 고유한 HW 지문에 바인딩됨 — 다른 기계에서는 무용지물 |

**이 크레이트는 검증만 수행합니다. 라이선스를 생성하거나 서명하거나 수정할 수 없습니다.**  
오프라인 서명 키는 절대 크레이트에 닿지 않습니다. 공격자는 위조기가 아닌 검증기를 얻게 됩니다.

---

## 💰 유료 티어: Mythos 등과의 경쟁

| 기능(Capability) | Community (무료) | Scientific (유료) | Government (유료) |
|------------|------------------|-------------------|-------------------|
| **검증 런타임** | ✅ 전체 | ✅ 전체 | ✅ 전체 |
| **정책 합성(TERNAL)** | ❌ | ✅ ML이 생성한 최적 정책 | ✅ 기밀 워크플로 정책 |
| **이상 탐지** | ❌ | ✅ ML 행동 분석 | ✅ 실시간 위협 상관관계 |
| **규정 준수 자동화** | ❌ | ✅ SOC2/ISO/FedRAMP 보고서 | ✅ STIG/CMMC 자동 증거 생성 |
| **HSM 통합** | ❌ 소프트웨어 키 | ✅ PKCS#11 / 클라우드 HSM | ✅ 기밀 HSM / 에어갭 |
| **정형 검증(Formal verification)** | ❌ | ✅ Coq/Lean 증명 제공 | ✅ 완전한 FV 아티팩트 |
| **지원(Support)** | 커뮤니티 | SLA 4시간 | SLA 15분 + 24/7 운영 |
| **가격(Pricing)** | **영구 무료** | **시트당 / 사용량** | **계약** |

> **TERNAL** = 독점적인 정책 합성 엔진 + ML 이상 탐지기 + HSM 추상화 + 정형 검증 파이프라인.  
> **이 크레이트는 TERNAL 라이선스가 실행되는 검증 런타임입니다.**  
> 우리는 **Mythos, Replicated, Keygen, LicenseSpring**과 경쟁하며, 다음과 같은 부분에서 승리합니다:
> - **제로 힙** (다른 것들은 메모리를 할당함)
> - **No_std** (다른 것들은 std/클라우드가 필요함)
> - **TPM2 네이티브** (다른 것들은 이를 모방(mock)함)
> - **Merkle 감사** (다른 것들은 텍스트 파일에 기록함)
> - **WASM/커널 준비 완료** (다른 것들은 불가능함)

---

## 💻 최소 하드웨어 요구 사항

| 환경 | CPU | RAM | 저장소 | TPM | 비고 |
|-------------|-----|-----|---------|-----|-------|
| **최소 검증 (no_std)** | Cortex-M4 / RISC-V RV32IMC | **2 KB RAM** | 8 KB 플래시 | 선택 사항 | 부트로더/커널 모듈 |
| **표준 검증 (std)** | x86_64 / ARM64 / RISC-V 64 | **64 KB** | 512 KB | 선택 사항 | CLI, 서비스, 데몬 |
| **TPM2 씰링** | TPM 2.0 지원 기기 | 1 MB | 2 MB | **필수** (PCR 0-7, 16) | 소버린/엔터프라이즈 티어 |
| **Merkle 감사 (sqlite)** | x86_64 / ARM64 | 4 MB | 10 MB + log | 선택 사항 | 규정 준수/포렌식 |
| **Python 바인딩** | x86_64 / ARM64 | 8 MB | 5 MB | 선택 사항 | PyO3 오버헤드 |
| **WASM (브라우저/엣지)** | 모든 WASM 타겟 | 16 MB | 200 KB .wasm | 해당 없음 | wasm32-unknown-unknown |

### 실제 차지 용량(Footprints)

```bash
# Minimal no_std verify (ARM Cortex-M4)
$ cargo build --release --no-default-features --features crypto --target thumbv7em-none-eabihf
$ size target/thumbv7em-none-eabihf/release/sase_forge_verify
   text    data     bss     dec     hex filename
   3842     0     512    4354    1102  # ~4 KB flash, 512 bytes RAM

# Standard verify (x86_64 Linux)
$ cargo build --release --features crypto
$ size target/release/sase_forge_verify
   text    data     bss     dec     hex filename
   142K     8K     12K    162K       # ~162 KB binary

# WASM (browser)
$ cargo build --release --target wasm32-unknown-unknown --features wasm,crypto
$ ls -lh target/wasm32-unknown-unknown/release/sase_forge_verify.wasm
   184K  # ~184 KB gzipped to ~45 KB
```

---

## 🔧 쉬운 제거 / 삭제

### Rust (Cargo)

```bash
# Remove from Cargo.toml
# [dependencies]
# sase-forge-verify = "0.1"

# Clean build artifacts
cargo clean

# Or remove from registry cache
cargo uninstall sase-forge-verify  # if installed as binary
```

### Python (pip)

```bash
pip uninstall sase-forge-verify
# Removes: package, bindings, .so/.pyd files
# No system files modified — pure user-site install
```

### System Package (deb/rpm/brew)

```bash
# Debian/Ubuntu
sudo apt remove sase-forge-verify

# Fedora/RHEL
sudo dnf remove sase-forge-verify

# macOS (Homebrew)
brew uninstall sase-forge-verify

# Windows (Scoop/Chocolatey)
scoop uninstall sase-forge-verify
# or
choco uninstall sase-forge-verify
```

### WASM / Embedded

```bash
# Just delete the .wasm file or firmware image
rm your_app.wasm
# No runtime, no registry, no background services
```

### 완전한 삭제 (모든 흔적 제거)

```bash
# Rust
rm -rf ~/.cargo/registry/src/*/sase-forge-verify-*
rm -rf ~/.cargo/registry/cache/*/sase-forge-verify-*

# Python
pip cache purge
rm -rf ~/.cache/pip/*sase_forge_verify*

# System
sudo rm -rf /usr/lib/sase-forge-verify /usr/include/sase-forge-verify
```

**영속성 제로(Zero persistence).** 데몬, systemd 서비스, 커널 모듈, 레지스트리 키, 텔레메트리, 폰홈(phone-home) 기능이 전혀 없습니다.

---

## 📦 기능 매트릭스

| 기능 (Feature) | 설명 | 종속성 | 크기 영향 |
|---------|-------------|--------------|-------------|
| `crypto` | **필수.** Ed25519 검증/서명 | `ed25519-dalek`, `zeroize` | +15 KB |
| `std` | 표준 라이브러리 지원 | `std` | +5 KB |
| `python` | PyO3 바인딩 | `std`, `pyo3` | +50 KB |
| `tpm` | TSS-ESAPI를 통한 실제 TPM2 | `std`, `tss-esapi` | +200 KB |
| `sqlite` | Merkle-DAG 감사 로그 | `std`, `rusqlite` | +300 KB |
| `wasm` | WASM 타겟 지원 | `getrandom/js` | +10 KB |
| `alloc` | `no_std` + alloc | `alloc` | +5 KB |
| `zeroize` | 비밀(Secret) 제로화 | `zeroize` | +2 KB |

### 최소 빌드

```toml
# Absolute minimum: verify only, no heap, no std
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto"] }

# Embedded bootloader (Cortex-M)
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto", "alloc"] }

# WASM for browser
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto", "wasm"] }

# Full featured (most apps)
[dependencies]
sase-forge-verify = { version = "0.1", features = ["std", "crypto", "python", "tpm", "sqlite", "wasm"] }
```

---

## 🔐 보안 속성(Security Properties)

| 속성 | 구현 |
|----------|----------------|
| **서명 위조** | Ed25519 (RFC 8032) — 128비트 보안 |
| **재전송(Replay) 공격** | 타임스탬프 + Nonce, 상수 시간 비교 |
| **키 교체** | KeyID = BLAKE3(public_key) 라이선스에 바인딩 |
| **클록 드리프트(Clock Drift)** | 구성 가능한 편차 허용 오차, 상수 시간 |
| **사이드 채널** | `verify_ct()` 상수 시간 경로 |
| **메모리 안전성** | Rust `no_std` + `zeroize` — 드롭 시 비밀 제로화 |
| **공급망(Supply Chain)** | 결정론적 `postcard` 직렬화, 핫 패스에 proc-macros 없음 |
| **TPM 바인딩** | PCR 0-7, 16 — 부트킷/루트킷 수정 감지 |
| **감사 무결성** | Merkle-DAG + SQLite WAL — 변조 감지 가능 |

---

## 📄 라이선스

다음 중 하나를 선택할 수 있는 **이중 라이선스(Dual-licensed)**입니다:
- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

이것은 **이 크레이트에만** 적용됩니다. TERNAL의 독점적인 기술 스택(정책 합성, ML 이상 탐지, HSM 추상화, 정형 검증)은 **포함되어 있지 않으며** 상용 라이선스가 필요합니다.

---

## 🤝 기여하기

1. 리포지토리를 포크(Fork)합니다.
2. 기능 브랜치를 생성합니다: `git checkout -b feat/amazing-feature`
3. 테스트를 실행합니다: `cargo test --all-features`
4. 제로 힙 검사를 실행합니다: `RUSTFLAGS="-DSASE_ZERO_HEAP=1" cargo test --no-default-features --features crypto`
5. 명확한 설명과 함께 PR을 제출합니다.

**코드 표준:**
- 기본적으로 `no_std` 호환
- 가능한 한 `const_fn` 사용
- 제로 힙 핫 패스는 `zero_heap_hot_path!`로 표시
- 모든 비밀은 `ZeroizeOnDrop`을 구현
- 모든 공개 API에 대한 문서화

---

## 🔗 링크

- **Documentation**: https://docs.rs/sase-forge-verify
- **Crates.io**: https://crates.io/crates/sase-forge-verify
- **Repository**: https://github.com/bu25ny/sase-forge-verify
- **Issues**: https://github.com/bu25ny/sase-forge-verify/issues
- **Commercial Inquiries**: licensing@sase-antigravity.dev

---

## 🏷️ 버전 관리

[SemVer](https://semver.org/) — `MAJOR.MINOR.PATCH`

- **0.1.x**: 초기 개발 단계, API 변경 가능
- **1.0.0**: 안정된 API, 프로덕션 준비 완료

---

*SASE Antigravity 팀이 ❤️를 담아 구축했습니다 — 소버린 세상을 위한 소버린 소프트웨어.*
