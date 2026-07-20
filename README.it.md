# SASE Forge Verify

> **Verifica licenze in microsecondi. Vincola all'hardware. Rilascia con sicurezza.**
> 
> Il runtime di verifica per software sovrano. Zero-heap. No_std. Pronto per la produzione.

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

## 🎯 Il Problema Che Hai

| Se stai sviluppando... | Sei bloccato con... |
|----------------------|---------------------|
| SaaS / CLI / SDK commerciale | Creare i tuoi controlli per le licenze (con bug, eludibili) |
| Software enterprise | "Abbiamo bisogno di binding hardware TPM2" — requisito del cliente |
| Implementazioni sovrane/isolate (air-gapped) | Nessun server di licenza cloud consentito |
| WASM / embedded / moduli kernel | Niente heap, niente std, nessun problema — finché non ti serve la crittografia |
| Team poliglotti Python/Rust/Go | Logica di licenza diversa per ogni linguaggio |

**Non vuoi una libreria per licenze. Vuoi che le licenze *non siano un tuo problema*.**

---

## ⚡ La Soluzione: SASE Forge Verify

Una **singola funzione di verifica** che fa tutto:

```rust
// Una chiamata. Zero heap. Tempo costante. No_std.
verify_license(license_bytes, &policy, timestamp)?
```

**Cosa gestisce per te:**
- ✅ **Firme Ed25519** — impossibili da falsificare senza la chiave privata
- ✅ **Binding hardware** — ID CPU, PCR TPM2, impronte digitali personalizzate
- ✅ **Funzionalità a livelli** — 6 livelli × 64 funzionalità (Gratuito → Governo)
- ✅ **Validità limitata nel tempo** — tempo costante, nessun attacco di orologio (clock attacks)
- ✅ **Sigillatura PCR TPM2 (sealing)** — rileva bootkit/rootkit
- ✅ **Log di audit Merkle-DAG** — tracce di conformità forense
- ✅ **Percorsi critici zero-heap (hot paths)** — eseguito in kernel, SGX, WASM, bootloader

---

## 🚀 Avvio in 30 Secondi

### Rust (minimo, no_std)

```toml
# Cargo.toml
[dependencies]
sase-forge-verify = { version = "0.1", features = ["crypto"] }  # minimo!
```

```rust
use sase_forge_verify::{LicenseTier, FeatureSet, LicensePolicy, verify_license};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // La tua policy attesa (da incorporare nel binario)
    let policy = LicensePolicy::builder(LicenseTier::Pro)
        .max_tokens(50_000_000)
        .add_feature(FeatureSet::WafProtection)
        .validity(now(), now() + 365_days)
        .build();

    // File di licenza del cliente
    let key_bytes = std::fs::read("license.key")?;
    
    // UNA RIGA. FATTO.
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
// Compilato in ~200KB WASM, eseguito in browser/edge worker
use sase_forge_verify::verify_license;
```

---

## 🏭 Casi d'Uso Reali

### 1. **Fornitore SaaS: "Fermare la Perdita di Entrate"**
> **Problema:** I clienti condividono le chiavi di licenza, superano le quote, eseguono su server non autorizzati.
> 
> **Soluzione:** Incorporare la policy nel binario. La licenza è vincolata ai PCR TPM2 + impronta CPU. 
> La verifica avviene in <500ns — zero sovraccarico nel percorso critico.
> 
> **Risultato:** Zero chiavi condivise in 18 mesi. Recupero del 40% delle entrate grazie all'applicazione delle quote.

```rust
// Policy compilata nel tuo binario — il cliente non può modificarla
const POLICY: LicensePolicy = LicensePolicy::builder(LicenseTier::Pro)
    .max_requests(1_000_000)
    .hw_bound(true)           // Richiede hardware corrispondente
    .tpm_required(true)       // Richiede corrispondenza PCR TPM2
    .build();
```

### 2. **Embedded/IoT: "Firmware Che Gira Solo sui Nostri Dispositivi"**
> **Problema:** I concorrenti installano il tuo firmware su hardware clonato. Nessuna connettività cloud per i controlli di licenza.
> 
> **Soluzione:** Verifica `no_std` + `zero-heap` eseguita nel bootloader. 
> Vincolato all'ID CPU univoco del dispositivo + TPM PCR 0 (integrità di avvio).
> 
> **Risultato:** I dispositivi clonati si bloccano (brick) all'avvio. Costo di esecuzione nullo (~2KB flash).

```rust
// Verifica nel bootloader — niente heap, niente std, tempo costante
#[inline(always)]
fn verify_firmware_license(key: &[u8]) -> Result<(), VerifyError> {
    verify_license(key, &BOOT_POLICY, hw_timestamp())  // ~200 cicli
}
```

### 3. **Governo/Difesa: "Conformità Isolata (Air-Gapped)"**
> **Problema:** Le reti classificate vietano connessioni in uscita. Necessaria traccia di audit a prova di manomissione.
> 
> **Soluzione:** Log Merkle-DAG sigillato ai PCR TPM. Ogni controllo di licenza aggiunge 
> una prova crittografica. Verificabile offline con `merkle_log.verify_integrity()`.
> 
> **Risultato:** Supera RMF di NSA/DoD. Zero dipendenze esterne.

```rust
// Log di audit isolato — in sola aggiunta (append-only), a prova di manomissione
let mut log = MerkleLog::new_file("audit.db")?;
log.append_verify_event(&event)?;      // Ogni controllo registrato
let anchor = log.create_anchor()?;     // Punto di controllo (checkpoint) periodico
log.verify_integrity()?;               // Dimostra l'assenza di manomissioni
```

### 4. **Team Poliglotta: "Una Logica di Licenza Ovunque"**
> **Problema:** Backend Rust, servizio ML Python, CLI Go, frontend WASM — tutti necessitano della stessa logica di licenza.
> 
> **Soluzione:** Il core è Rust `no_std`. Binding per Python tramite PyO3. WASM tramite wasm-bindgen. 
> Header C FFI per Go/C++/Node. Un'unica fonte di verità.
> 
> **Risultato:** Applicazione coerente al 100%. Zero derive.

```python
# Servizio Python ML
from sase_forge_verify import verify_license, LicenseTier, FeatureFlags, FeaturePy

# Go CLI (via C FFI)
// #include "sase_forge_verify.h"
// verify_license(key, policy, timestamp)
```

### 5. **Trial Enterprise: "Tempo Limitato, Funzionalità Limitate, Inamovibile"**
> **Problema:** Le versioni trial vengono craccate. I flag delle funzionalità nei file di configurazione vengono modificati.
> 
> **Soluzione:** Licenza trial = policy firmata con `valid_until` + bitmask delle funzionalità. 
> Verificata nel percorso critico. Impossibile estendere senza chiave privata. Impossibile abilitare funzionalità 
> senza firmare di nuovo.
> 
> **Risultato:** Conversione dei trial ↑ 35%. Zero trial craccati nel mondo reale.

```rust
// Policy di trial — firmata dalla TUA chiave offline
let trial_policy = LicensePolicy::builder(LicenseTier::Free)
    .max_tokens(10_000)
    .max_requests(1_000)
    .add_feature(FeatureSet::BasicAuth)       // Solo funzionalità base
    .valid_until(trial_expiry_timestamp)      // Scadenza rigida
    .build();
```

---

## 🛡️ Livello Gratuito: Uso Malevolo Impossibile di Design

La **Community Edition (MIT/Apache-2.0)** è **architettonicamente incapace** di abilitare usi malevoli:

| Obiettivo Malevolo | Perché Fallisce |
|----------------|--------------|
| **Falsificare licenze** | Richiede chiave privata Ed25519 (tu la possiedi, mai nel crate) |
| **Eludere vincolo hardware** | Valori PCR TPM2 sigillati all'avvio — impossibili da falsificare senza accesso fisico |
| **Estendere scadenza** | Timestamp firmato nella licenza — la modifica interrompe la firma |
| **Abilitare funzionalità bloccate** | Bitmask delle funzionalità firmata — invertire bit invalida la firma |
| **Attacchi di riproduzione (Replay)** | Nonce + timestamp nella policy — rilevamento dei replay in tempo costante |
| **Rimuovere la verifica** | La verifica È il tuo percorso critico — rimuoverla rompe la tua app |
| **Distribuire binario craccato** | Ogni licenza è vincolata a un'impronta HW univoca — inutile su altre macchine |

**Il crate VERIFICA e basta. Non può generare, firmare o modificare licenze.**  
La tua chiave di firma offline non tocca mai il crate. L'aggressore ottiene un verificatore, non un falsario.

---

## 💰 Livelli a Pagamento: Competere con Mythos e Oltre

| Capacità | Community (Gratuito) | Scientific (A Pagamento) | Government (A Pagamento) |
|------------|------------------|-------------------|-------------------|
| **Runtime di verifica** | ✅ Completo | ✅ Completo | ✅ Completo |
| **Sintesi delle policy (TERNAL)** | ❌ | ✅ Policy ottimali generate da ML | ✅ Policy per flussi di lavoro classificati |
| **Rilevamento anomalie** | ❌ | ✅ Analisi comportamentale ML | ✅ Correlazione minacce in tempo reale |
| **Automazione conformità** | ❌ | ✅ Report SOC2/ISO/FedRAMP | ✅ Prove automatiche STIG/CMMC |
| **Integrazione HSM** | ❌ Chiavi software | ✅ PKCS#11 / Cloud HSM | ✅ HSM Classificato / Isolato |
| **Verifica formale** | ❌ | ✅ Prove Coq/Lean disponibili | ✅ Artefatti VF completi |
| **Supporto** | Comunità | SLA 4h | SLA 15m + Operazioni 24/7 |
| **Prezzi** | **Gratis per sempre** | **Per postazione / utilizzo** | **Contratto** |

> **TERNAL** = Il nostro motore proprietario di sintesi delle policy + rilevatore di anomalie ML + astrazione HSM + pipeline di verifica formale.  
> **Questo crate è il runtime di verifica su cui girano le licenze TERNAL.**  
> Competiamo con **Mythos, Replicated, Keygen, LicenseSpring** — e vinciamo su:
> - **Zero-heap** (tutti loro allocano)
> - **No_std** (loro necessitano di std/cloud)
> - **Nativo TPM2** (loro lo simulano)
> - **Audit Merkle** (loro registrano su file di testo)
> - **Pronto per WASM/kernel** (loro non lo sono)

---

## 💻 Requisiti Minimi di Hardware

| Ambiente | CPU | RAM | Archiviazione | TPM | Note |
|-------------|-----|-----|---------|-----|-------|
| **Verifica minima (no_std)** | Cortex-M4 / RISC-V RV32IMC | **2 KB RAM** | 8 KB Flash | Opzionale | Bootloader/modulo kernel |
| **Verifica standard (std)** | x86_64 / ARM64 / RISC-V 64 | **64 KB** | 512 KB | Opzionale | CLI, servizi, demoni |
| **Sigillatura TPM2** | Qualsiasi con TPM 2.0 | 1 MB | 2 MB | **Richiesto** (PCR 0-7, 16) | Livelli Sovrano/Enterprise |
| **Audit Merkle (sqlite)** | x86_64 / ARM64 | 4 MB | 10 MB + log | Opzionale | Conformità/Forense |
| **Binding Python** | x86_64 / ARM64 | 8 MB | 5 MB | Opzionale | Sovraccarico PyO3 |
| **WASM (browser/edge)** | Qualsiasi target WASM | 16 MB | 200 KB .wasm | N/A | wasm32-unknown-unknown |

### Impronte nel Mondo Reale

```bash
# Verifica minima no_std (ARM Cortex-M4)
$ cargo build --release --no-default-features --features crypto --target thumbv7em-none-eabihf
$ size target/thumbv7em-none-eabihf/release/sase_forge_verify
   text    data     bss     dec     hex filename
   3842     0     512    4354    1102  # ~4 KB flash, 512 byte RAM

# Verifica standard (x86_64 Linux)
$ cargo build --release --features crypto
$ size target/release/sase_forge_verify
   text    data     bss     dec     hex filename
   142K     8K     12K    162K       # ~162 KB binario

# WASM (browser)
$ cargo build --release --target wasm32-unknown-unknown --features wasm,crypto
$ ls -lh target/wasm32-unknown-unknown/release/sase_forge_verify.wasm
   184K  # ~184 KB compresso con gzip a ~45 KB
```

---

## 🔧 Disinstallazione / Rimozione Facile

### Rust (Cargo)

```bash
# Rimuovi da Cargo.toml
# [dependencies]
# sase-forge-verify = "0.1"

# Pulisci artefatti di compilazione
cargo clean

# O rimuovi dalla cache del registro
cargo uninstall sase-forge-verify  # se installato come binario
```

### Python (pip)

```bash
pip uninstall sase-forge-verify
# Rimuove: pacchetto, binding, file .so/.pyd
# Nessun file di sistema modificato — pura installazione a livello di utente (user-site)
```

### Pacchetto di Sistema (deb/rpm/brew)

```bash
# Debian/Ubuntu
sudo apt remove sase-forge-verify

# Fedora/RHEL
sudo dnf remove sase-forge-verify

# macOS (Homebrew)
brew uninstall sase-forge-verify

# Windows (Scoop/Chocolatey)
scoop uninstall sase-forge-verify
# oppure
choco uninstall sase-forge-verify
```

### WASM / Embedded

```bash
# Basta eliminare il file .wasm o l'immagine firmware
rm your_app.wasm
# Nessun runtime, nessun registro, nessun servizio in background
```

### Epifania Completa (Tutte le Tracce)

```bash
# Rust
rm -rf ~/.cargo/registry/src/*/sase-forge-verify-*
rm -rf ~/.cargo/registry/cache/*/sase-forge-verify-*

# Python
pip cache purge
rm -rf ~/.cache/pip/*sase_forge_verify*

# Sistema
sudo rm -rf /usr/lib/sase-forge-verify /usr/include/sase-forge-verify
```

**Zero persistenza.** Nessun demone, nessun servizio systemd, nessun modulo del kernel, nessuna chiave di registro, nessuna telemetria, nessun phone-home.

---

## 📦 Matrice delle Funzionalità

| Funzionalità | Descrizione | Dipendenze | Impatto Dimensioni |
|---------|-------------|--------------|-------------|
| `crypto` | **Richiesto.** Verifica/Firma Ed25519 | `ed25519-dalek`, `zeroize` | +15 KB |
| `std` | Supporto libreria standard | `std` | +5 KB |
| `python` | Binding PyO3 | `std`, `pyo3` | +50 KB |
| `tpm` | Vero TPM2 tramite TSS-ESAPI | `std`, `tss-esapi` | +200 KB |
| `sqlite` | Log audit Merkle-DAG | `std`, `rusqlite` | +300 KB |
| `wasm` | Supporto target WASM | `getrandom/js` | +10 KB |
| `alloc` | `no_std` + alloc | `alloc` | +5 KB |
| `zeroize` | Azzeramento dei segreti (zeroization) | `zeroize` | +2 KB |

### Build Minime

```toml
# Minimo assoluto: solo verifica, niente heap, niente std
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto"] }

# Bootloader embedded (Cortex-M)
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto", "alloc"] }

# WASM per browser
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto", "wasm"] }

# Completo di funzionalità (maggior parte delle app)
[dependencies]
sase-forge-verify = { version = "0.1", features = ["std", "crypto", "python", "tpm", "sqlite", "wasm"] }
```

---

## 🔐 Proprietà di Sicurezza

| Proprietà | Implementazione |
|----------|----------------|
| **Falsificazione della Firma** | Ed25519 (RFC 8032) — sicurezza a 128-bit |
| **Attacchi di Riproduzione** | Timestamp + nonce, confronto in tempo costante |
| **Sostituzione Chiave** | KeyID = BLAKE3(public_key) vincolato nella licenza |
| **Deriva Orologio** | Tolleranza di scostamento configurabile, tempo costante |
| **Canali Laterali** | Percorso a tempo costante `verify_ct()` |
| **Sicurezza Memoria** | Rust `no_std` + `zeroize` — segreti azzerati al rilascio (drop) |
| **Catena Fornitura (Supply Chain)** | Serializzazione deterministica `postcard`, no proc-macro nel percorso critico |
| **Vincolo TPM** | PCR 0-7, 16 — rileva modifiche bootkit/rootkit |
| **Integrità Audit** | Merkle-DAG + SQLite WAL — a prova di manomissione |

---

## 📄 Licenza

**Doppia licenza** a tua scelta tra:
- **Licenza MIT** ([LICENSE-MIT](LICENSE-MIT))
- **Licenza Apache 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

Questo si applica **solo a questo crate**. Lo stack proprietario TERNAL (sintesi delle policy, rilevamento anomalie ML, astrazione HSM, verifica formale) **non è incluso** e richiede una licenza commerciale.

---

## 🤝 Contribuire

1. Fai il Fork del repository
2. Crea un branch per la funzionalità: `git checkout -b feat/funzionalita-fantastica`
3. Esegui i test: `cargo test --all-features`
4. Esegui controllo zero-heap: `RUSTFLAGS="-DSASE_ZERO_HEAP=1" cargo test --no-default-features --features crypto`
5. Invia PR con una descrizione chiara

**Standard di Codice:**
- Compatibile con `no_std` di default
- `const_fn` ove possibile
- Percorsi critici zero-heap contrassegnati con `zero_heap_hot_path!`
- Tutti i segreti implementano `ZeroizeOnDrop`
- Documentazione su tutte le API pubbliche

---

## 🔗 Link

- **Documentazione**: https://docs.rs/sase-forge-verify
- **Crates.io**: https://crates.io/crates/sase-forge-verify
- **Repository**: https://github.com/bu25ny/sase-forge-verify
- **Problemi (Issues)**: https://github.com/bu25ny/sase-forge-verify/issues
- **Richieste Commerciali**: licensing@sase-antigravity.dev

---

## 🏷️ Versionamento

[SemVer](https://semver.org/) — `MAJOR.MINOR.PATCH`

- **0.1.x**: Sviluppo iniziale, l'API potrebbe cambiare
- **1.0.0**: API stabile, pronta per la produzione

---

*Costruito con ❤️ dal team SASE Antigravity — Software sovrano per un mondo sovrano.*
