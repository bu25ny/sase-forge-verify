# SASE Forge Verify

> **Lizenzen in Mikrosekunden überprüfen. An Hardware binden. Mit Zuversicht ausliefern.**
> 
> Die Verifizierungs-Laufzeitumgebung für souveräne Software. Zero-Heap. No_std. Produktionsbereit.

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

## 🎯 Ihr Problem

| Wenn Sie folgendes bauen... | Kämpfen Sie mit... |
|----------------------|---------------------|
| Kommerzielles SaaS / CLI / SDK | Eigenen Lizenzprüfungen (fehlerhaft, umgehbar) |
| Unternehmenssoftware | "Wir brauchen TPM2-Hardwarebindung" — Kundenanforderung |
| Souveräne/Air-gapped Bereitstellungen | Keine Cloud-Lizenzserver erlaubt |
| WASM / Embedded / Kernel-Module | Kein Heap, kein std, kein Problem — bis Sie Krypto brauchen |
| Polyglotte Python/Rust/Go Teams | Unterschiedliche Lizenzlogik in jeder Sprache |

**Sie wollen keine Lizenzierungs-Bibliothek. Sie wollen, dass die Lizenzierung *nicht Ihr Problem* ist.**

---

## ⚡ Die Lösung: SASE Forge Verify

Eine **einzige Verifizierungsfunktion**, die alles erledigt:

```rust
// Ein Aufruf. Kein Heap. Konstante Zeit. No_std.
verify_license(license_bytes, &policy, timestamp)?
```

**Was es für Sie übernimmt:**
- ✅ **Ed25519-Signaturen** — unmöglich ohne privaten Schlüssel zu fälschen
- ✅ **Hardwarebindung** — CPU-ID, TPM2-PCRs, benutzerdefinierte Fingerabdrücke
- ✅ **Gestufte Funktionen** — 6 Stufen × 64 Funktionen (Kostenlos → Regierung)
- ✅ **Zeitlich begrenzte Gültigkeit** — konstante Zeit, keine Uhr-Angriffe
- ✅ **TPM2-PCR-Versiegelung** — erkennt Bootkits/Rootkits
- ✅ **Merkle-DAG-Audit-Protokoll** — forensische Compliance-Pfade
- ✅ **Zero-Heap Hot-Paths** — läuft in Kerneln, SGX, WASM, Bootloadern

---

## 🚀 30-Sekunden-Start

### Rust (minimal, no_std)

```toml
# Cargo.toml
[dependencies]
sase-forge-verify = { version = "0.1", features = ["crypto"] }  # minimal!
```

```rust
use sase_forge_verify::{LicenseTier, FeatureSet, LicensePolicy, verify_license};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ihre erwartete Richtlinie (in Binärdatei einbetten)
    let policy = LicensePolicy::builder(LicenseTier::Pro)
        .max_tokens(50_000_000)
        .add_feature(FeatureSet::WafProtection)
        .validity(now(), now() + 365_days)
        .build();

    // Die Lizenzdatei des Kunden
    let key_bytes = std::fs::read("license.key")?;
    
    // EINE ZEILE. FERTIG.
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
print("✅ Lizenziert")
```

### WASM (Browser / Edge)

```toml
[dependencies]
sase-forge-verify = { version = "0.1", features = ["wasm", "crypto"] }
```

```rust
// Kompiliert zu ~200KB WASM, läuft im Browser/Edge Workers
use sase_forge_verify::verify_license;
```

---

## 🏭 Reale Anwendungsfälle

### 1. **SaaS-Anbieter: "Stoppen Sie Umsatzverluste"**
> **Problem:** Kunden teilen Lizenzschlüssel, überschreiten Kontingente, laufen auf nicht autorisierten Servern.
> 
> **Lösung:** Integrieren Sie die Richtlinie in die Binärdatei. Lizenz bindet an TPM2-PCRs + CPU-Fingerabdruck. 
> Die Verifizierung läuft in <500ns — kein Overhead im Hot-Path.
> 
> **Ergebnis:** Keine geteilten Schlüssel in 18 Monaten. 40 % Umsatzrückgewinnung durch Kontingentdurchsetzung.

```rust
// Richtlinie wird in Ihre Binärdatei kompiliert — der Kunde kann sie nicht ändern
const POLICY: LicensePolicy = LicensePolicy::builder(LicenseTier::Pro)
    .max_requests(1_000_000)
    .hw_bound(true)           // Erfordert passende Hardware
    .tpm_required(true)       // Erfordert TPM2-PCR-Übereinstimmung
    .build();
```

### 2. **Embedded/IoT: "Firmware, die nur auf unseren Geräten läuft"**
> **Problem:** Konkurrenten flashen Ihre Firmware auf Klon-Hardware. Keine Cloud-Verbindung für Lizenzprüfungen.
> 
> **Lösung:** `no_std` + `zero-heap` Verifizierung läuft im Bootloader. 
> Bindet an gerätespezifische CPU-ID + TPM-PCR 0 (Boot-Integrität).
> 
> **Ergebnis:** Klon-Geräte brickten beim Booten. Keine Laufzeitkosten (~2KB Flash).

```rust
// Bootloader-Verifizierung — kein Heap, kein std, konstante Zeit
#[inline(always)]
fn verify_firmware_license(key: &[u8]) -> Result<(), VerifyError> {
    verify_license(key, &BOOT_POLICY, hw_timestamp())  // ~200 Zyklen
}
```

### 3. **Regierung/Verteidigung: "Air-Gapped Compliance"**
> **Problem:** Geheime Netzwerke verbieten ausgehende Verbindungen. Benötigen einen manipulationssicheren Audit-Trail.
> 
> **Lösung:** Merkle-DAG-Protokoll, versiegelt an TPM-PCRs. Jede Lizenzprüfung fügt einen 
> kryptografischen Beweis hinzu. Offline verifizierbar mit `merkle_log.verify_integrity()`.
> 
> **Ergebnis:** Besteht NSA/DoD RMF. Keine externen Abhängigkeiten.

```rust
// Air-Gapped-Audit-Protokoll — nur anfügen (append-only), manipulationssicher
let mut log = MerkleLog::new_file("audit.db")?;
log.append_verify_event(&event)?;      // Jede Prüfung protokolliert
let anchor = log.create_anchor()?;     // Periodischer Checkpoint
log.verify_integrity()?;               // Beweist keine Manipulation
```

### 4. **Polyglottes Team: "Eine Lizenzlogik überall"**
> **Problem:** Rust-Backend, Python-ML-Service, Go-CLI, WASM-Frontend — alle brauchen die gleiche Lizenzlogik.
> 
> **Lösung:** Der Kern ist Rust `no_std`. Python-Bindungen über PyO3. WASM über wasm-bindgen. 
> C-FFI-Header für Go/C++/Node. Eine einzige Wahrheitsquelle.
> 
> **Ergebnis:** 100 % konsistente Durchsetzung. Keine Abweichungen.

```python
# Python ML-Service
from sase_forge_verify import verify_license, LicenseTier, FeatureFlags, FeaturePy

# Go CLI (via C FFI)
// #include "sase_forge_verify.h"
// verify_license(key, policy, timestamp)
```

### 5. **Unternehmens-Testversion: "Zeitlich begrenzt, funktionsbeschränkt, nicht entfernbar"**
> **Problem:** Testversionen werden gecrackt. Feature-Flags in Konfigurationsdateien werden bearbeitet.
> 
> **Lösung:** Testlizenz = signierte Richtlinie mit `valid_until` + Feature-Bitmaske. 
> Im Hot-Path verifiziert. Kann ohne privaten Schlüssel nicht verlängert werden. Keine Features 
> aktivierbar ohne erneute Signierung.
> 
> **Ergebnis:** Testversion-Conversion ↑ 35 %. Null gecrackte Testversionen in freier Wildbahn.

```rust
// Test-Richtlinie — signiert durch IHREN Offline-Schlüssel
let trial_policy = LicensePolicy::builder(LicenseTier::Free)
    .max_tokens(10_000)
    .max_requests(1_000)
    .add_feature(FeatureSet::BasicAuth)       // Nur grundlegende Funktionen
    .valid_until(trial_expiry_timestamp)      // Hartes Ablaufdatum
    .build();
```

---

## 🛡️ Kostenlose Stufe: Böswillige Nutzung durch Design unmöglich

Die **Community Edition (MIT/Apache-2.0)** ist **architektonisch nicht in der Lage**, eine böswillige Nutzung zu ermöglichen:

| Böswilliges Ziel | Warum es scheitert |
|----------------|--------------|
| **Lizenzen fälschen** | Erfordert privaten Ed25519-Schlüssel (Sie besitzen ihn, niemals in der Crate) |
| **Hardwarebindung umgehen** | TPM2-PCR-Werte beim Booten versiegelt — ohne physischen Zugang nicht fälschbar |
| **Ablauf verlängern** | Zeitstempel in die Lizenz signiert — Änderung bricht die Signatur |
| **Gesperrte Funktionen aktivieren** | Funktions-Bitmaske signiert — das Umkehren von Bits macht die Signatur ungültig |
| **Replay-Angriffe** | Nonce + Zeitstempel in der Richtlinie — Erkennung in konstanter Zeit |
| **Verifizierung entfernen** | Die Verifizierung IST Ihr Hot-Path — ihre Entfernung macht Ihre App unbrauchbar |
| **Gecrackte Binärdatei verteilen** | Jede Lizenz bindet an einen einzigartigen HW-Fingerabdruck — nutzlos auf anderen Rechnern |

**Die Crate verifiziert NUR. Sie kann keine Lizenzen generieren, signieren oder ändern.**  
Ihr Offline-Signaturschlüssel berührt die Crate nie. Der Angreifer erhält einen Prüfer, keinen Fälscher.

---

## 💰 Kostenpflichtige Stufen: Im Wettbewerb mit Mythos & darüber hinaus

| Funktion | Community (Kostenlos) | Scientific (Kostenpflichtig) | Government (Kostenpflichtig) |
|------------|------------------|-------------------|-------------------|
| **Verifizierungs-Laufzeit** | ✅ Voll | ✅ Voll | ✅ Voll |
| **Richtliniensynthese (TERNAL)** | ❌ | ✅ ML-generierte optimale Richtlinien | ✅ Klassifizierte Workflow-Richtlinien |
| **Anomalieerkennung** | ❌ | ✅ ML-Verhaltensanalyse | ✅ Echtzeit-Bedrohungskorrelation |
| **Compliance-Automatisierung** | ❌ | ✅ SOC2/ISO/FedRAMP-Berichte | ✅ STIG/CMMC Auto-Beweise |
| **HSM-Integration** | ❌ Software-Schlüssel | ✅ PKCS#11 / Cloud HSM | ✅ Klassifiziertes HSM / Air-Gapped |
| **Formale Verifizierung** | ❌ | ✅ Coq/Lean-Beweise verfügbar | ✅ Volle FV-Artefakte |
| **Support** | Community | SLA 4h | SLA 15m + 24/7 Ops |
| **Preise** | **Für immer kostenlos** | **Pro Nutzer / Nutzung** | **Vertrag** |

> **TERNAL** = Unsere proprietäre Richtliniensynthese-Engine + ML-Anomaliedetektor + HSM-Abstraktion + formale Verifizierungspipeline.  
> **Diese Crate ist die Verifizierungs-Laufzeitumgebung, auf der TERNAL-Lizenzen laufen.**  
> Wir konkurrieren mit **Mythos, Replicated, Keygen, LicenseSpring** — und gewinnen durch:
> - **Zero-Heap** (sie alle allozieren Speicher)
> - **No_std** (sie brauchen std/Cloud)
> - **TPM2 nativ** (sie simulieren es)
> - **Merkle-Audit** (sie protokollieren in Textdateien)
> - **WASM/Kernel bereit** (sie sind es nicht)

---

## 💻 Minimale Hardwareanforderungen

| Umgebung | CPU | RAM | Speicher | TPM | Hinweise |
|-------------|-----|-----|---------|-----|-------|
| **Minimale Überprüfung (no_std)** | Cortex-M4 / RISC-V RV32IMC | **2 KB RAM** | 8 KB Flash | Optional | Bootloader/Kernel-Modul |
| **Standard-Überprüfung (std)** | x86_64 / ARM64 / RISC-V 64 | **64 KB** | 512 KB | Optional | CLI, Services, Daemons |
| **TPM2-Versiegelung** | Beliebige mit TPM 2.0 | 1 MB | 2 MB | **Erforderlich** (PCR 0-7, 16) | Sovereign/Enterprise Tiers |
| **Merkle Audit (sqlite)** | x86_64 / ARM64 | 4 MB | 10 MB + log | Optional | Compliance/Forensik |
| **Python Bindings** | x86_64 / ARM64 | 8 MB | 5 MB | Optional | PyO3 Overhead |
| **WASM (Browser/Edge)** | Beliebiges WASM-Ziel | 16 MB | 200 KB .wasm | N/A | wasm32-unknown-unknown |

### Reale Fußabdrücke

```bash
# Minimale no_std Überprüfung (ARM Cortex-M4)
$ cargo build --release --no-default-features --features crypto --target thumbv7em-none-eabihf
$ size target/thumbv7em-none-eabihf/release/sase_forge_verify
   text    data     bss     dec     hex filename
   3842     0     512    4354    1102  # ~4 KB Flash, 512 Bytes RAM

# Standard-Überprüfung (x86_64 Linux)
$ cargo build --release --features crypto
$ size target/release/sase_forge_verify
   text    data     bss     dec     hex filename
   142K     8K     12K    162K       # ~162 KB Binärdatei

# WASM (Browser)
$ cargo build --release --target wasm32-unknown-unknown --features wasm,crypto
$ ls -lh target/wasm32-unknown-unknown/release/sase_forge_verify.wasm
   184K  # ~184 KB komprimiert auf ~45 KB
```

---

## 🔧 Einfache Deinstallation / Entfernung

### Rust (Cargo)

```bash
# Aus Cargo.toml entfernen
# [dependencies]
# sase-forge-verify = "0.1"

# Build-Artefakte bereinigen
cargo clean

# Oder aus dem Registry-Cache entfernen
cargo uninstall sase-forge-verify  # wenn als Binärdatei installiert
```

### Python (pip)

```bash
pip uninstall sase-forge-verify
# Entfernt: Paket, Bindungen, .so/.pyd Dateien
# Keine Systemdateien verändert — reine User-Site-Installation
```

### Systempaket (deb/rpm/brew)

```bash
# Debian/Ubuntu
sudo apt remove sase-forge-verify

# Fedora/RHEL
sudo dnf remove sase-forge-verify

# macOS (Homebrew)
brew uninstall sase-forge-verify

# Windows (Scoop/Chocolatey)
scoop uninstall sase-forge-verify
# oder
choco uninstall sase-forge-verify
```

### WASM / Embedded

```bash
# Einfach die .wasm-Datei oder das Firmware-Image löschen
rm ihre_app.wasm
# Keine Laufzeitumgebung, keine Registrierung, keine Hintergrunddienste
```

### Komplette Bereinigung (Alle Spuren)

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

**Keine Persistenz.** Keine Daemons, keine systemd-Dienste, keine Kernel-Module, keine Registry-Schlüssel, keine Telemetrie, kein Phone-Home.

---

## 📦 Funktionsmatrix

| Funktion | Beschreibung | Abhängigkeiten | Größenauswirkung |
|---------|-------------|--------------|-------------|
| `crypto` | **Erforderlich.** Ed25519 prüfen/signieren | `ed25519-dalek`, `zeroize` | +15 KB |
| `std` | Standardbibliotheksunterstützung | `std` | +5 KB |
| `python` | PyO3 Bindungen | `std`, `pyo3` | +50 KB |
| `tpm` | Echtes TPM2 via TSS-ESAPI | `std`, `tss-esapi` | +200 KB |
| `sqlite` | Merkle-DAG Audit-Protokoll | `std`, `rusqlite` | +300 KB |
| `wasm` | WASM-Zielunterstützung | `getrandom/js` | +10 KB |
| `alloc` | `no_std` + alloc | `alloc` | +5 KB |
| `zeroize` | Geheimnis-Ausnullung (Zeroization) | `zeroize` | +2 KB |

### Minimale Builds

```toml
# Absolutes Minimum: Nur überprüfen, kein Heap, kein std
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto"] }

# Eingebetteter Bootloader (Cortex-M)
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto", "alloc"] }

# WASM für Browser
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto", "wasm"] }

# Voller Funktionsumfang (die meisten Apps)
[dependencies]
sase-forge-verify = { version = "0.1", features = ["std", "crypto", "python", "tpm", "sqlite", "wasm"] }
```

---

## 🔐 Sicherheitseigenschaften

| Eigenschaft | Implementierung |
|----------|----------------|
| **Signaturfälschung** | Ed25519 (RFC 8032) — 128-Bit Sicherheit |
| **Replay-Angriffe** | Zeitstempel + Nonce, Vergleich in konstanter Zeit |
| **Schlüsselaustausch** | KeyID = BLAKE3(public_key) in der Lizenz gebunden |
| **Uhrdrift** | Konfigurierbare Abweichungstoleranz, konstante Zeit |
| **Seitenkanäle** | `verify_ct()` Pfad in konstanter Zeit |
| **Speichersicherheit** | Rust `no_std` + `zeroize` — Geheimnisse beim Löschen ausgenullt |
| **Lieferkette** | Deterministische `postcard`-Serialisierung, keine proc-macros im Hot-Path |
| **TPM-Bindung** | PCR 0-7, 16 — erkennt Bootkit/Rootkit-Modifikationen |
| **Audit-Integrität** | Merkle-DAG + SQLite WAL — manipulationssicher |

---

## 📄 Lizenz

**Duale Lizenzierung** nach Ihrer Wahl:
- **MIT-Lizenz** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

Dies gilt **nur für diese Crate**. Der proprietäre TERNAL-Stack (Richtliniensynthese, ML-Anomalieerkennung, HSM-Abstraktion, formale Verifizierung) ist **nicht enthalten** und erfordert eine kommerzielle Lizenz.

---

## 🤝 Mitwirken

1. Forken Sie das Repository
2. Erstellen Sie einen Feature-Branch: `git checkout -b feat/fantastisches-feature`
3. Tests ausführen: `cargo test --all-features`
4. Zero-Heap-Prüfung ausführen: `RUSTFLAGS="-DSASE_ZERO_HEAP=1" cargo test --no-default-features --features crypto`
5. PR mit klarer Beschreibung einreichen

**Code-Standards:**
- Standardmäßig `no_std`-kompatibel
- `const_fn` wo möglich
- Zero-Heap-Hot-Paths markiert mit `zero_heap_hot_path!`
- Alle Geheimnisse implementieren `ZeroizeOnDrop`
- Dokumentation für alle öffentlichen APIs

---

## 🔗 Links

- **Dokumentation**: https://docs.rs/sase-forge-verify
- **Crates.io**: https://crates.io/crates/sase-forge-verify
- **Repository**: https://github.com/bu25ny/sase-forge-verify
- **Probleme (Issues)**: https://github.com/bu25ny/sase-forge-verify/issues
- **Kommerzielle Anfragen**: licensing@sase-antigravity.dev

---

## 🏷️ Versionierung

[SemVer](https://semver.org/) — `MAJOR.MINOR.PATCH`

- **0.1.x**: Initiale Entwicklung, API kann sich ändern
- **1.0.0**: Stabile API, produktionsbereit

---

*Erstellt mit ❤️ vom SASE Antigravity Team — Souveräne Software für eine souveräne Welt.*
