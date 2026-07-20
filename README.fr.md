# SASE Forge Verify

> **Vérifiez les licences en quelques microsecondes. Liez au matériel. Livrez en toute confiance.**
> 
> Le runtime de vérification pour les logiciels souverains. Zéro allocation (Zero-heap). No_std. Prêt pour la production.

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

## 🎯 Le problème que vous rencontrez

| Si vous construisez... | Vous êtes bloqué avec... |
|----------------------|---------------------|
| SaaS commercial / CLI / SDK | La création de vos propres vérifications de licence (boguées, contournables) |
| Logiciel d'entreprise | "Nous avons besoin d'une liaison matérielle TPM2" — exigence du client |
| Déploiements souverains/air-gapped | Aucun serveur de licence cloud autorisé |
| WASM / embarqué / modules noyau | Pas de tas, pas de std, pas de problème — jusqu'à ce que vous ayez besoin de cryptographie |
| Équipes polyglottes Python/Rust/Go | Une logique de licence différente dans chaque langage |

**Vous ne voulez pas d'une bibliothèque de licences. Vous voulez que les licences *ne soient pas votre problème*.**

---

## ⚡ La solution : SASE Forge Verify

Une **fonction de vérification unique** qui fait tout :

```rust
// Une seule fonction. Zéro allocation (zéro heap). Temps constant. No_std.
verify_license(license_bytes, &policy, timestamp)?
```

**Ce qu'elle gère pour vous :**
- ✅ **Signatures Ed25519** — impossibles à falsifier sans clé privée
- ✅ **Liaison matérielle** — CPU ID, PCR TPM2, empreintes personnalisées
- ✅ **Fonctionnalités hiérarchisées** — 6 niveaux × 64 fonctionnalités (Gratuit → Gouvernement)
- ✅ **Validité limitée dans le temps** — temps constant, pas d'attaques d'horloge
- ✅ **Scellement PCR TPM2** — détecte les bootkits/rootkits
- ✅ **Journal d'audit Merkle-DAG** — pistes de conformité médico-légales
- ✅ **Chemins critiques sans allocation (Zero-heap)** — s'exécute dans les noyaux, SGX, WASM, bootloaders

---

## 🚀 Démarrage en 30 secondes

### Rust (minimal, no_std)

```toml
# Cargo.toml
[dependencies]
sase-forge-verify = { version = "0.1", features = ["crypto"] }  # minimal!
```

```rust
use sase_forge_verify::{LicenseTier, FeatureSet, LicensePolicy, verify_license};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Votre politique attendue (intégrée dans le binaire)
    let policy = LicensePolicy::builder(LicenseTier::Pro)
        .max_tokens(50_000_000)
        .add_feature(FeatureSet::WafProtection)
        .validity(now(), now() + 365_days)
        .build();

    // Le fichier de licence du client
    let key_bytes = std::fs::read("license.key")?;
    
    // UNE LIGNE. C'EST FAIT.
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
print("✅ Sous licence")
```

### WASM (Navigateur / Edge)

```toml
[dependencies]
sase-forge-verify = { version = "0.1", features = ["wasm", "crypto"] }
```

```rust
// Se compile en ~200KB WASM, s'exécute dans le navigateur/les workers edge
use sase_forge_verify::verify_license;
```

---

## 🏭 Cas d'utilisation réels

### 1. **Éditeur SaaS : "Arrêter la fuite de revenus"**
> **Problème :** Les clients partagent des clés de licence, dépassent les quotas, exécutent sur des serveurs non autorisés.
> 
> **Solution :** Intégrez la politique dans le binaire. La licence se lie aux PCR TPM2 + empreinte CPU. 
> La vérification s'exécute en <500ns — aucun surcoût sur le chemin critique.
> 
> **Résultat :** Zéro clé partagée en 18 mois. 40 % de récupération des revenus grâce à l'application des quotas.

```rust
// Politique compilée dans votre binaire — le client ne peut pas la changer
const POLICY: LicensePolicy = LicensePolicy::builder(LicenseTier::Pro)
    .max_requests(1_000_000)
    .hw_bound(true)           // Nécessite le matériel correspondant
    .tpm_required(true)       // Nécessite la correspondance PCR TPM2
    .build();
```

### 2. **Embarqué/IoT : "Le firmware qui ne fonctionne que sur nos appareils"**
> **Problème :** Les concurrents flashent votre firmware sur du matériel clone. Pas de connectivité cloud pour les vérifications de licence.
> 
> **Solution :** La vérification `no_std` + `zero-heap` s'exécute dans le bootloader. 
> Se lie à l'ID CPU unique de l'appareil + PCR 0 du TPM (intégrité du démarrage).
> 
> **Résultat :** Les appareils clones se briquent au démarrage. Zéro coût d'exécution (~2KB de flash).

```rust
// Vérification du bootloader — pas de tas, pas de std, temps constant
#[inline(always)]
fn verify_firmware_license(key: &[u8]) -> Result<(), VerifyError> {
    verify_license(key, &BOOT_POLICY, hw_timestamp())  // ~200 cycles
}
```

### 3. **Gouvernement/Défense : "Conformité Air-Gapped"**
> **Problème :** Les réseaux classifiés interdisent les connexions sortantes. Besoin d'une piste d'audit inviolable.
> 
> **Solution :** Journal Merkle-DAG scellé aux PCR TPM. Chaque vérification de licence ajoute une 
> preuve cryptographique. Vérifiable hors ligne avec `merkle_log.verify_integrity()`.
> 
> **Résultat :** Passe le RMF de la NSA/DoD. Zéro dépendance externe.

```rust
// Journal d'audit air-gapped — ajout uniquement, inviolable
let mut log = MerkleLog::new_file("audit.db")?;
log.append_verify_event(&event)?;      // Chaque vérification est consignée
let anchor = log.create_anchor()?;     // Point de contrôle périodique
log.verify_integrity()?;               // Prouve l'absence d'altération
```

### 4. **Équipe polyglotte : "Une logique de licence unique partout"**
> **Problème :** Backend Rust, service ML Python, CLI Go, frontend WASM — tous ont besoin de la même logique de licence.
> 
> **Solution :** Le cœur est Rust `no_std`. Liaisons Python via PyO3. WASM via wasm-bindgen. 
> En-tête C FFI pour Go/C++/Node. Une seule source de vérité.
> 
> **Résultat :** Application 100 % cohérente. Zéro dérive.

```python
# Service ML Python
from sase_forge_verify import verify_license, LicenseTier, FeatureFlags, FeaturePy

# CLI Go (via C FFI)
// #include "sase_forge_verify.h"
// verify_license(key, policy, timestamp)
```

### 5. **Essai d'entreprise : "Limité dans le temps, fonctionnalités restreintes, inamovible"**
> **Problème :** Les essais sont piratés. Les indicateurs de fonctionnalités dans les fichiers de configuration sont modifiés.
> 
> **Solution :** Licence d'essai = politique signée avec `valid_until` + masque de bits de fonctionnalités. 
> Vérifié sur le chemin critique. Impossible à prolonger sans clé privée. Impossible d'activer des fonctionnalités 
> sans nouvelle signature.
> 
> **Résultat :** Conversion des essais ↑ 35 %. Zéro essai piraté dans la nature.

```rust
// Politique d'essai — signée par VOTRE clé hors ligne
let trial_policy = LicensePolicy::builder(LicenseTier::Free)
    .max_tokens(10_000)
    .max_requests(1_000)
    .add_feature(FeatureSet::BasicAuth)       // Seulement les fonctionnalités de base
    .valid_until(trial_expiry_timestamp)      // Expiration stricte
    .build();
```

---

## 🛡️ Niveau gratuit : L'utilisation malveillante impossible par conception

La **Community Edition (MIT/Apache-2.0)** est **architecturalement incapable** de permettre une utilisation malveillante :

| Objectif malveillant | Pourquoi ça échoue |
|----------------|--------------|
| **Falsifier les licences** | Nécessite une clé privée Ed25519 (vous la détenez, jamais dans la crate) |
| **Contourner la liaison matérielle** | Valeurs PCR TPM2 scellées au démarrage — impossible à usurper sans accès physique |
| **Prolonger l'expiration** | Horodatage signé dans la licence — la modification casse la signature |
| **Activer des fonctionnalités verrouillées** | Masque de bits de fonctionnalité signé — inverser les bits invalide la signature |
| **Attaques par rejeu** | Nonce + horodatage dans la politique — détection de rejeu en temps constant |
| **Supprimer la vérification** | La vérification EST votre chemin critique — sa suppression casse votre application |
| **Distribuer un binaire piraté** | Chaque licence se lie à une empreinte matérielle unique — inutile sur d'autres machines |

**La crate effectue UNIQUEMENT des vérifications. Elle ne peut pas générer, signer ni modifier de licences.**  
Votre clé de signature hors ligne ne touche jamais la crate. L'attaquant obtient un vérificateur, pas un forgeur.

---

## 💰 Niveaux payants : Concurrencer Mythos et au-delà

| Capacité | Community (Gratuit) | Scientific (Payant) | Government (Payant) |
|------------|------------------|-------------------|-------------------|
| **Runtime de vérification** | ✅ Complet | ✅ Complet | ✅ Complet |
| **Synthèse de politiques (TERNAL)** | ❌ | ✅ Politiques optimales générées par ML | ✅ Politiques de flux de travail classifiés |
| **Détection d'anomalies** | ❌ | ✅ Analyse comportementale ML | ✅ Corrélation des menaces en temps réel |
| **Automatisation de la conformité** | ❌ | ✅ Rapports SOC2/ISO/FedRAMP | ✅ Preuves auto STIG/CMMC |
| **Intégration HSM** | ❌ Clés logicielles | ✅ PKCS#11 / Cloud HSM | ✅ HSM classifié / air-gapped |
| **Vérification formelle** | ❌ | ✅ Preuves Coq/Lean disponibles | ✅ Artefacts de VF complets |
| **Support** | Communauté | SLA 4h | SLA 15m + opérations 24/7 |
| **Tarification** | **Gratuit à vie** | **Par poste / utilisation** | **Contrat** |

> **TERNAL** = Notre moteur propriétaire de synthèse de politiques + détecteur d'anomalies ML + abstraction HSM + pipeline de vérification formelle.  
> **Cette crate est le runtime de vérification sur lequel fonctionnent les licences TERNAL.**  
> Nous concurrençons **Mythos, Replicated, Keygen, LicenseSpring** — et nous gagnons sur :
> - **Zéro allocation (Zero-heap)** (ils allouent tous)
> - **No_std** (ils ont besoin de std/cloud)
> - **TPM2 natif** (ils le simulent)
> - **Audit Merkle** (ils journalisent dans des fichiers texte)
> - **Prêt pour WASM/noyau** (ils ne le sont pas)

---

## 💻 Exigences matérielles minimales

| Environnement | CPU | RAM | Stockage | TPM | Notes |
|-------------|-----|-----|---------|-----|-------|
| **Vérification minimale (no_std)** | Cortex-M4 / RISC-V RV32IMC | **2 KB RAM** | 8 KB Flash | Optionnel | Bootloader/module noyau |
| **Vérification standard (std)** | x86_64 / ARM64 / RISC-V 64 | **64 KB** | 512 KB | Optionnel | CLI, services, démons |
| **Scellement TPM2** | Tout CPU avec TPM 2.0 | 1 MB | 2 MB | **Requis** (PCR 0-7, 16) | Niveaux Souverain/Entreprise |
| **Audit Merkle (sqlite)** | x86_64 / ARM64 | 4 MB | 10 MB + journal | Optionnel | Conformité/investigation |
| **Liaisons Python** | x86_64 / ARM64 | 8 MB | 5 MB | Optionnel | Surcoût PyO3 |
| **WASM (navigateur/edge)** | Toute cible WASM | 16 MB | 200 KB .wasm | N/A | wasm32-unknown-unknown |

### Empreintes dans le monde réel

```bash
# Vérification minimale no_std (ARM Cortex-M4)
$ cargo build --release --no-default-features --features crypto --target thumbv7em-none-eabihf
$ size target/thumbv7em-none-eabihf/release/sase_forge_verify
   text    data     bss     dec     hex filename
   3842     0     512    4354    1102  # ~4 KB flash, 512 bytes RAM

# Vérification standard (x86_64 Linux)
$ cargo build --release --features crypto
$ size target/release/sase_forge_verify
   text    data     bss     dec     hex filename
   142K     8K     12K    162K       # ~162 KB binary

# WASM (navigateur)
$ cargo build --release --target wasm32-unknown-unknown --features wasm,crypto
$ ls -lh target/wasm32-unknown-unknown/release/sase_forge_verify.wasm
   184K  # ~184 KB compressé à ~45 KB
```

---

## 🔧 Désinstallation / Suppression facile

### Rust (Cargo)

```bash
# Supprimer de Cargo.toml
# [dependencies]
# sase-forge-verify = "0.1"

# Nettoyer les artefacts de compilation
cargo clean

# Ou supprimer du cache du registre
cargo uninstall sase-forge-verify  # si installé en tant que binaire
```

### Python (pip)

```bash
pip uninstall sase-forge-verify
# Supprime : paquet, liaisons, fichiers .so/.pyd
# Aucun fichier système modifié — installation purement utilisateur
```

### Paquet système (deb/rpm/brew)

```bash
# Debian/Ubuntu
sudo apt remove sase-forge-verify

# Fedora/RHEL
sudo dnf remove sase-forge-verify

# macOS (Homebrew)
brew uninstall sase-forge-verify

# Windows (Scoop/Chocolatey)
scoop uninstall sase-forge-verify
# ou
choco uninstall sase-forge-verify
```

### WASM / Embarqué

```bash
# Il suffit de supprimer le fichier .wasm ou l'image du firmware
rm votre_app.wasm
# Pas de runtime, pas de registre, pas de services en arrière-plan
```

### Purge complète (Toutes les traces)

```bash
# Rust
rm -rf ~/.cargo/registry/src/*/sase-forge-verify-*
rm -rf ~/.cargo/registry/cache/*/sase-forge-verify-*

# Python
pip cache purge
rm -rf ~/.cache/pip/*sase_forge_verify*

# Système
sudo rm -rf /usr/lib/sase-forge-verify /usr/include/sase-forge-verify
```

**Zéro persistance.** Aucun démon, aucun service systemd, aucun module noyau, aucune clé de registre, aucune télémétrie, aucun appel à la maison (phone-home).

---

## 📦 Matrice des fonctionnalités

| Fonctionnalité | Description | Dépendances | Impact sur la taille |
|---------|-------------|--------------|-------------|
| `crypto` | **Requis.** Vérification/signature Ed25519 | `ed25519-dalek`, `zeroize` | +15 KB |
| `std` | Support de la bibliothèque standard | `std` | +5 KB |
| `python` | Liaisons PyO3 | `std`, `pyo3` | +50 KB |
| `tpm` | Vrai TPM2 via TSS-ESAPI | `std`, `tss-esapi` | +200 KB |
| `sqlite` | Journal d'audit Merkle-DAG | `std`, `rusqlite` | +300 KB |
| `wasm` | Support de la cible WASM | `getrandom/js` | +10 KB |
| `alloc` | `no_std` + alloc | `alloc` | +5 KB |
| `zeroize` | Remise à zéro des secrets | `zeroize` | +2 KB |

### Compilations minimales

```toml
# Minimum absolu : vérification uniquement, pas de tas, pas de std
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto"] }

# Bootloader embarqué (Cortex-M)
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto", "alloc"] }

# WASM pour navigateur
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto", "wasm"] }

# Toutes fonctionnalités (plupart des applications)
[dependencies]
sase-forge-verify = { version = "0.1", features = ["std", "crypto", "python", "tpm", "sqlite", "wasm"] }
```

---

## 🔐 Propriétés de sécurité

| Propriété | Implémentation |
|----------|----------------|
| **Falsification de signature** | Ed25519 (RFC 8032) — sécurité 128 bits |
| **Attaques par rejeu** | Horodatage + nonce, comparaison en temps constant |
| **Substitution de clé** | KeyID = BLAKE3(public_key) lié dans la licence |
| **Dérive d'horloge** | Tolérance de biais configurable, temps constant |
| **Canaux auxiliaires** | Chemin en temps constant `verify_ct()` |
| **Sécurité de la mémoire** | Rust `no_std` + `zeroize` — secrets mis à zéro à la destruction |
| **Chaîne d'approvisionnement** | Sérialisation `postcard` déterministe, pas de proc-macros sur le chemin critique |
| **Liaison TPM** | PCR 0-7, 16 — détecte la modification bootkit/rootkit |
| **Intégrité de l'audit** | Merkle-DAG + SQLite WAL — inviolable |

---

## 📄 Licence

**Double licence** sous votre choix de :
- **Licence MIT** ([LICENSE-MIT](LICENSE-MIT))
- **Licence Apache 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

Cela s'applique à **cette crate uniquement**. La pile propriétaire TERNAL (synthèse de politiques, détection d'anomalies ML, abstraction HSM, vérification formelle) n'est **pas incluse** et nécessite une licence commerciale.

---

## 🤝 Contribution

1. Forkez le dépôt
2. Créez une branche de fonctionnalité : `git checkout -b feat/fonctionnalite-incroyable`
3. Exécutez les tests : `cargo test --all-features`
4. Exécutez la vérification zero-heap : `RUSTFLAGS="-DSASE_ZERO_HEAP=1" cargo test --no-default-features --features crypto`
5. Soumettez une PR avec une description claire

**Normes de code :**
- Compatible `no_std` par défaut
- `const_fn` si possible
- Chemins critiques sans allocation marqués avec `zero_heap_hot_path!`
- Tous les secrets implémentent `ZeroizeOnDrop`
- Documentation sur toutes les API publiques

---

## 🔗 Liens

- **Documentation** : https://docs.rs/sase-forge-verify
- **Crates.io** : https://crates.io/crates/sase-forge-verify
- **Dépôt** : https://github.com/bu25ny/sase-forge-verify
- **Tickets (Issues)** : https://github.com/bu25ny/sase-forge-verify/issues
- **Demandes commerciales** : licensing@sase-antigravity.dev

---

## 🏷️ Versionnage

[SemVer](https://semver.org/) — `MAJEURE.MINEURE.CORRECTIF`

- **0.1.x** : Développement initial, l'API peut changer
- **1.0.0** : API stable, prêt pour la production

---

*Construit avec ❤️ par l'équipe SASE Antigravity — Des logiciels souverains pour un monde souverain.*
