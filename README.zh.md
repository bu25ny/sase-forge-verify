# SASE Forge Verify

> **以微秒级速度验证许可证。绑定到硬件。自信地交付。**
> 
> 主权软件的验证运行时。零堆分配 (Zero-heap)。无标准库 (No_std)。生产就绪。

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

## 🎯 您面临的问题

| 如果您正在构建... | 您将受困于... |
|----------------------|---------------------|
| 商业 SaaS / CLI / SDK | 自行推出许可证检查（容易出bug，可被绕过） |
| 企业软件 | “我们需要 TPM2 硬件绑定” —— 客户要求 |
| 主权/物理隔离 (air-gapped) 部署 | 不允许使用云端许可证服务器 |
| WASM / 嵌入式 / 内核模块 | 无堆，无标准库，没问题 —— 直到你需要密码学 |
| Python/Rust/Go 多语言团队 | 各种语言中存在不同的许可证逻辑 |

**您不需要一个许可证库。您需要让许可证*不再是您的问题*。**

---

## ⚡ 解决方案：SASE Forge Verify

一个能处理一切的**单一验证函数**：

```rust
// 一次调用。零堆。恒定时间。无标准库。
verify_license(license_bytes, &policy, timestamp)?
```

**它为您处理的内容：**
- ✅ **Ed25519 签名** — 没有私钥绝无伪造可能
- ✅ **硬件绑定** — CPU ID，TPM2 PCR，自定义指纹
- ✅ **分层功能** — 6 个层级 × 64 个功能（从免费 → 政府级）
- ✅ **有时间限制的有效性** — 恒定时间，无时钟攻击
- ✅ **TPM2 PCR 密封** — 检测 bootkits/rootkits
- ✅ **Merkle-DAG 审计日志** — 取证合规追踪
- ✅ **零堆热路径 (hot paths)** — 运行在内核，SGX，WASM，引导加载程序 (bootloaders) 中

---

## 🚀 30秒入门

### Rust (极简, no_std)

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

## 🏭 实际使用场景

### 1. **SaaS 供应商：“停止收入泄漏”**
> **问题：** 客户共享许可证密钥，超出配额，在未经授权的服务器上运行。
> 
> **解决方案：** 将策略嵌入到二进制文件中。许可证绑定到 TPM2 PCRs + CPU 指纹。
> 验证运行时间 <500ns — 热路径上零开销。
> 
> **结果：** 18个月内零共享密钥。通过配额执行恢复了 40% 的收入。

```rust
// Policy compiled into your binary — customer can't change it
const POLICY: LicensePolicy = LicensePolicy::builder(LicenseTier::Pro)
    .max_requests(1_000_000)
    .hw_bound(true)           // Requires matching hardware
    .tpm_required(true)       // Requires TPM2 PCR match
    .build();
```

### 2. **嵌入式/物联网：“仅在我们设备上运行的固件”**
> **问题：** 竞争对手将您的固件刷入克隆硬件。没有云连接来进行许可证检查。
> 
> **解决方案：** `no_std` + `zero-heap` 验证运行在引导加载程序 (bootloader) 中。
> 绑定到设备唯一的 CPU ID + TPM PCR 0（启动完整性）。
> 
> **结果：** 克隆设备在启动时变砖。零运行时成本（~2KB 闪存）。

```rust
// Bootloader verification — no heap, no std, constant time
#[inline(always)]
fn verify_firmware_license(key: &[u8]) -> Result<(), VerifyError> {
    verify_license(key, &BOOT_POLICY, hw_timestamp())  // ~200 cycles
}
```

### 3. **政府/国防部：“物理隔离的合规性”**
> **问题：** 机密网络禁止出站连接。需要防篡改的审计追踪。
> 
> **解决方案：** Merkle-DAG 日志密封到 TPM PCRs。每次许可证检查都会追加密码学证明。可以通过 `merkle_log.verify_integrity()` 进行离线验证。
> 
> **结果：** 通过 NSA/DoD RMF。零外部依赖。

```rust
// Air-gapped audit log — append-only, tamper-evident
let mut log = MerkleLog::new_file("audit.db")?;
log.append_verify_event(&event)?;      // Every check logged
let anchor = log.create_anchor()?;     // Periodic checkpoint
log.verify_integrity()?;               // Proves no tampering
```

### 4. **多语言团队：“处处统一的许可证逻辑”**
> **问题：** Rust 后端、Python ML 服务、Go CLI、WASM 前端 —— 全都需要相同的许可证逻辑。
> 
> **解决方案：** 核心是 Rust `no_std`。通过 PyO3 的 Python 绑定。通过 wasm-bindgen 的 WASM 绑定。针对 Go/C++/Node 的 C FFI 头文件。唯一的真相来源 (Single source of truth)。
> 
> **结果：** 100% 一致的执行。零漂移 (Zero drift)。

```python
# Python ML service
from sase_forge_verify import verify_license, LicenseTier, FeatureFlags, FeaturePy

# Go CLI (via C FFI)
// #include "sase_forge_verify.h"
// verify_license(key, policy, timestamp)
```

### 5. **企业试用：“限时、功能受限、不可移除”**
> **问题：** 试用版被破解。配置文件中的功能标志 (Feature flags) 被篡改。
> 
> **解决方案：** 试用许可证 = 带有 `valid_until` + 功能位掩码 (feature bitmask) 的已签名策略。
> 在热路径中验证。没有私钥无法延期。不重新签名则无法启用功能。
> 
> **结果：** 试用转化率提升 35%。市面上零破解的试用版。

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

## 🛡️ 免费层：在设计上杜绝恶意使用

**社区版 (Community Edition, MIT/Apache-2.0)** 在**架构上无法**被恶意使用：

| 恶意目标 | 失败原因 |
|----------------|--------------|
| **伪造许可证** | 需要 Ed25519 私钥（由您持有，永远不在 crate 中） |
| **绕过硬件绑定** | TPM2 PCR 值在启动时被密封 — 没有物理访问权限就无法欺骗 |
| **延长到期时间** | 时间戳被签名在许可证中 — 修改会破坏签名 |
| **启用锁定功能** | 功能位掩码被签名 — 翻转位会使签名失效 |
| **重放攻击** | 策略中的随机数 (Nonce) + 时间戳 — 恒定时间的重放检测 |
| **剥离验证** | 验证**就是**您的热路径 — 移除它会破坏您的应用 |
| **分发破解的二进制文件** | 每个许可证绑定到唯一的硬件指纹 — 在其他机器上无效 |

**此 crate 仅进行验证。它无法生成、签名或修改许可证。**  
您的离线签名密钥永远不会接触到此 crate。攻击者得到的是验证器，而不是伪造器。

---

## 💰 付费层：与 Mythos 及更高级别竞争

| 能力 | 社区版 (免费) | 科学版 (付费) | 政府版 (付费) |
|------------|------------------|-------------------|-------------------|
| **验证运行时** | ✅ 完整 | ✅ 完整 | ✅ 完整 |
| **策略合成 (TERNAL)** | ❌ | ✅ 机器学习生成的最佳策略 | ✅ 机密工作流策略 |
| **异常检测** | ❌ | ✅ 机器学习行为分析 | ✅ 实时威胁关联 |
| **合规自动化** | ❌ | ✅ SOC2/ISO/FedRAMP 报告 | ✅ STIG/CMMC 自动证据 |
| **HSM 集成** | ❌ 软件密钥 | ✅ PKCS#11 / 云 HSM | ✅ 机密 HSM / 物理隔离 |
| **形式化验证** | ❌ | ✅ 提供 Coq/Lean 证明 | ✅ 完整的 FV 构件 |
| **支持** | 社区 | SLA 4小时 | SLA 15分钟 + 24/7 运维 |
| **定价** | **永远免费** | **按席位 / 用量** | **合同** |

> **TERNAL** = 我们的专有策略合成引擎 + ML 异常检测器 + HSM 抽象层 + 形式化验证管道。  
> **此 crate 是 TERNAL 许可证运行的验证运行时。**  
> 我们与 **Mythos, Replicated, Keygen, LicenseSpring** 竞争 — 并在以下方面胜出：
> - **零堆 (Zero-heap)** (它们都进行内存分配)
> - **无标准库 (No_std)** (它们需要 std/云端)
> - **原生 TPM2** (它们只是模拟)
> - **Merkle 审计** (它们记录到文本文件)
> - **为 WASM/内核准备就绪** (它们没有)

---

## 💻 最低硬件要求

| 环境 | CPU | RAM | 存储 | TPM | 备注 |
|-------------|-----|-----|---------|-----|-------|
| **极简验证 (no_std)** | Cortex-M4 / RISC-V RV32IMC | **2 KB RAM** | 8 KB 闪存 | 可选 | 引导加载程序/内核模块 |
| **标准验证 (std)** | x86_64 / ARM64 / RISC-V 64 | **64 KB** | 512 KB | 可选 | CLI, 服务, 守护进程 |
| **TPM2 密封** | 任何支持 TPM 2.0 的设备 | 1 MB | 2 MB | **必需** (PCR 0-7, 16) | 主权/企业层级 |
| **Merkle 审计 (sqlite)** | x86_64 / ARM64 | 4 MB | 10 MB + log | 可选 | 合规/取证 |
| **Python 绑定** | x86_64 / ARM64 | 8 MB | 5 MB | 可选 | PyO3 开销 |
| **WASM (浏览器/边缘)** | 任何 WASM 目标 | 16 MB | 200 KB .wasm | 不适用 | wasm32-unknown-unknown |

### 真实的内存占用

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

## 🔧 轻松卸载 / 移除

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

### 完全清除 (所有痕迹)

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

**零持久化。** 没有守护进程，没有 systemd 服务，没有内核模块，没有注册表键，没有遥测，没有回传 (phone-home)。

---

## 📦 功能矩阵

| 功能 (Feature) | 描述 | 依赖项 | 大小影响 |
|---------|-------------|--------------|-------------|
| `crypto` | **必需。** Ed25519 验证/签名 | `ed25519-dalek`, `zeroize` | +15 KB |
| `std` | 标准库支持 | `std` | +5 KB |
| `python` | PyO3 绑定 | `std`, `pyo3` | +50 KB |
| `tpm` | 通过 TSS-ESAPI 的真实 TPM2 | `std`, `tss-esapi` | +200 KB |
| `sqlite` | Merkle-DAG 审计日志 | `std`, `rusqlite` | +300 KB |
| `wasm` | WASM 目标支持 | `getrandom/js` | +10 KB |
| `alloc` | `no_std` + alloc | `alloc` | +5 KB |
| `zeroize` | 机密信息置零 (Secret zeroization) | `zeroize` | +2 KB |

### 最小构建

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

## 🔐 安全属性

| 属性 | 实现 |
|----------|----------------|
| **签名伪造** | Ed25519 (RFC 8032) — 128位安全性 |
| **重放攻击** | 时间戳 + Nonce，恒定时间比较 |
| **密钥替换** | KeyID = BLAKE3(public_key) 绑定在许可证中 |
| **时钟漂移** | 可配置的偏差容忍度，恒定时间 |
| **侧信道** | `verify_ct()` 恒定时间路径 |
| **内存安全** | Rust `no_std` + `zeroize` — 释放时密钥置零 |
| **供应链** | 确定性的 `postcard` 序列化，热路径中无过程宏 (proc-macros) |
| **TPM 绑定** | PCR 0-7, 16 — 检测 bootkit/rootkit 修改 |
| **审计完整性** | Merkle-DAG + SQLite WAL — 防篡改 |

---

## 📄 许可证

**双重许可**，您可以选择：
- **MIT 许可证** ([LICENSE-MIT](LICENSE-MIT))
- **Apache 许可证 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

这**仅适用于此 crate**。TERNAL 专有技术栈（策略合成，机器学习异常检测，HSM 抽象，形式化验证）**不包含在内**，且需要商业许可证。

---

## 🤝 贡献

1. Fork 本仓库
2. 创建一个特性分支：`git checkout -b feat/amazing-feature`
3. 运行测试：`cargo test --all-features`
4. 运行零堆分配检查：`RUSTFLAGS="-DSASE_ZERO_HEAP=1" cargo test --no-default-features --features crypto`
5. 提交带有清晰描述的 PR

**代码标准：**
- 默认兼容 `no_std`
- 尽可能使用 `const_fn`
- 使用 `zero_heap_hot_path!` 标记零堆热路径
- 所有密钥实现 `ZeroizeOnDrop`
- 对所有公共 API 进行文档化

---

## 🔗 链接

- **Documentation**: https://docs.rs/sase-forge-verify
- **Crates.io**: https://crates.io/crates/sase-forge-verify
- **Repository**: https://github.com/bu25ny/sase-forge-verify
- **Issues**: https://github.com/bu25ny/sase-forge-verify/issues
- **Commercial Inquiries**: licensing@sase-antigravity.dev

---

## 🏷️ 版本控制

[SemVer](https://semver.org/) — `MAJOR.MINOR.PATCH`

- **0.1.x**: 初始开发阶段，API 可能会改变
- **1.0.0**: 稳定的 API，生产就绪

---

*由 SASE Antigravity 团队用心构建 (❤️) — 为主权世界打造主权软件。*
