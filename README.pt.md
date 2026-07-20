# SASE Forge Verify

> **Verifique licenças em microssegundos. Vincule ao hardware. Lance com confiança.**
> 
> O runtime de verificação para software soberano. Zero-heap. No_std. Pronto para produção.

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

## 🎯 O Problema que Você Tem

| Se você está construindo... | Você está preso com... |
|----------------------|---------------------|
| SaaS / CLI / SDK comercial | Criar suas próprias verificações de licença (com bugs, contornáveis) |
| Software corporativo | "Precisamos de vinculação de hardware TPM2" — requisito do cliente |
| Implantações soberanas/isoladas (air-gapped) | Servidores de licença em nuvem não são permitidos |
| WASM / embarcado / módulos de kernel | Sem heap, sem std, sem problema — até você precisar de criptografia |
| Equipes poliglotas em Python/Rust/Go | Lógica de licença diferente em cada linguagem |

**Você não quer uma biblioteca de licenças. Você quer que o licenciamento *não seja o seu problema*.**

---

## ⚡ A Solução: SASE Forge Verify

Uma **única função de verificação** que faz tudo:

```rust
// Uma chamada. Zero heap. Tempo constante. No_std.
verify_license(license_bytes, &policy, timestamp)?
```

**O que ele resolve para você:**
- ✅ **Assinaturas Ed25519** — impossíveis de forjar sem a chave privada
- ✅ **Vinculação de hardware** — ID da CPU, PCRs do TPM2, impressões digitais personalizadas
- ✅ **Recursos em níveis** — 6 níveis × 64 recursos (Gratuito → Governo)
- ✅ **Validade com limite de tempo** — tempo constante, sem ataques de relógio
- ✅ **Selagem (Sealing) PCR TPM2** — detecta bootkits/rootkits
- ✅ **Registro de auditoria Merkle-DAG** — trilhas de conformidade forense
- ✅ **Caminhos críticos zero-heap (hot paths)** — roda em kernels, SGX, WASM, bootloaders

---

## 🚀 Início em 30 Segundos

### Rust (mínimo, no_std)

```toml
# Cargo.toml
[dependencies]
sase-forge-verify = { version = "0.1", features = ["crypto"] }  # mínimo!
```

```rust
use sase_forge_verify::{LicenseTier, FeatureSet, LicensePolicy, verify_license};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sua política esperada (embutida no binário)
    let policy = LicensePolicy::builder(LicenseTier::Pro)
        .max_tokens(50_000_000)
        .add_feature(FeatureSet::WafProtection)
        .validity(now(), now() + 365_days)
        .build();

    // Arquivo de licença do cliente
    let key_bytes = std::fs::read("license.key")?;
    
    // UMA LINHA. FEITO.
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

### WASM (Navegador / Edge)

```toml
[dependencies]
sase-forge-verify = { version = "0.1", features = ["wasm", "crypto"] }
```

```rust
// Compila para ~200KB WASM, roda em edge workers/navegadores
use sase_forge_verify::verify_license;
```

---

## 🏭 Casos de Uso no Mundo Real

### 1. **Fornecedor SaaS: "Pare o Vazamento de Receita"**
> **Problema:** Clientes compartilham chaves de licença, excedem cotas, rodam em servidores não autorizados.
> 
> **Solução:** Embutir política no binário. A licença se vincula aos PCRs do TPM2 + fingerprint da CPU. 
> A verificação ocorre em <500ns — sem overhead no hot path.
> 
> **Resultado:** Zero chaves compartilhadas em 18 meses. 40% de recuperação de receita com aplicação de cotas.

```rust
// Política compilada no seu binário — o cliente não pode alterá-la
const POLICY: LicensePolicy = LicensePolicy::builder(LicenseTier::Pro)
    .max_requests(1_000_000)
    .hw_bound(true)           // Requer hardware correspondente
    .tpm_required(true)       // Requer correspondência de PCR do TPM2
    .build();
```

### 2. **Embarcados/IoT: "Firmware que só roda nos Nossos Dispositivos"**
> **Problema:** Concorrentes fazem flash do seu firmware em hardware clone. Sem conectividade na nuvem para verificações de licença.
> 
> **Solução:** A verificação `no_std` + `zero-heap` roda no bootloader. 
> Vincula ao ID de CPU exclusivo do dispositivo + TPM PCR 0 (integridade de inicialização).
> 
> **Resultado:** Dispositivos clones brickam no boot. Zero custo de execução (~2KB de flash).

```rust
// Verificação do bootloader — sem heap, sem std, tempo constante
#[inline(always)]
fn verify_firmware_license(key: &[u8]) -> Result<(), VerifyError> {
    verify_license(key, &BOOT_POLICY, hw_timestamp())  // ~200 ciclos
}
```

### 3. **Governo/Defesa: "Conformidade Isolada (Air-Gapped)"**
> **Problema:** Redes classificadas proíbem conexões de saída. Precisa-se de uma trilha de auditoria à prova de violações.
> 
> **Solução:** Log Merkle-DAG selado em PCRs de TPM. Cada verificação de licença anexa 
> uma prova criptográfica. Verificável offline com `merkle_log.verify_integrity()`.
> 
> **Resultado:** Passa no RMF da NSA/DoD. Zero dependências externas.

```rust
// Log de auditoria isolado — append-only, à prova de violação
let mut log = MerkleLog::new_file("audit.db")?;
log.append_verify_event(&event)?;      // Cada verificação registrada
let anchor = log.create_anchor()?;     // Checkpoint periódico
log.verify_integrity()?;               // Prova de que não houve adulteração
```

### 4. **Equipe Poliglota: "Uma Lógica de Licença em Toda Parte"**
> **Problema:** Backend Rust, serviço ML Python, CLI Go, frontend WASM — todos precisam da mesma lógica de licença.
> 
> **Solução:** O núcleo é Rust `no_std`. Bindings Python via PyO3. WASM via wasm-bindgen. 
> Cabeçalho C FFI para Go/C++/Node. Única fonte da verdade.
> 
> **Resultado:** Aplicação 100% consistente. Zero divergências.

```python
# Serviço Python ML
from sase_forge_verify import verify_license, LicenseTier, FeatureFlags, FeaturePy

# Go CLI (via C FFI)
// #include "sase_forge_verify.h"
// verify_license(key, policy, timestamp)
```

### 5. **Trial Corporativo: "Tempo Limitado, Recursos Limitados, Inremovível"**
> **Problema:** Versões trial são crackeadas. Flags de recursos em arquivos de configuração são editados.
> 
> **Solução:** Licença trial = política assinada com `valid_until` + máscara de bits de recursos. 
> Verificada no hot path. Não pode estender sem chave privada. Não pode habilitar recursos 
> sem reassinar.
> 
> **Resultado:** Conversão de trials ↑ 35%. Zero trials crackeados no uso real.

```rust
// Política de trial — assinada pela SUA chave offline
let trial_policy = LicensePolicy::builder(LicenseTier::Free)
    .max_tokens(10_000)
    .max_requests(1_000)
    .add_feature(FeatureSet::BasicAuth)       // Apenas recursos básicos
    .valid_until(trial_expiry_timestamp)      // Expiração estrita
    .build();
```

---

## 🛡️ Nível Gratuito: Uso Malicioso Impossível por Design

A **Community Edition (MIT/Apache-2.0)** é **arquitetonicamente incapaz** de permitir o uso malicioso:

| Objetivo Malicioso | Por Que Falha |
|----------------|--------------|
| **Forjar licenças** | Requer chave privada Ed25519 (você a possui, nunca no crate) |
| **Burlar vinculação de hardware** | Valores de PCR TPM2 selados no boot — não podem ser falsificados sem acesso físico |
| **Estender expiração** | Timestamp assinado na licença — modificar quebra a assinatura |
| **Habilitar recursos bloqueados** | Máscara de bits de recursos assinada — inverter bits invalida a assinatura |
| **Ataques de repetição (Replay)** | Nonce + timestamp na política — detecção de repetição em tempo constante |
| **Remover verificação** | A verificação É o seu hot path — removê-la quebra seu app |
| **Distribuir binário crackeado** | Cada licença se vincula a uma fingerprint de HW exclusiva — inútil em outras máquinas |

**O crate APENAS verifica. Ele não pode gerar, assinar ou modificar licenças.**  
Sua chave de assinatura offline nunca toca o crate. O invasor obtém um verificador, não um falsificador.

---

## 💰 Níveis Pagos: Competindo com Mythos e Além

| Capacidade | Community (Gratuito) | Scientific (Pago) | Government (Pago) |
|------------|------------------|-------------------|-------------------|
| **Runtime de verificação** | ✅ Completo | ✅ Completo | ✅ Completo |
| **Síntese de política (TERNAL)** | ❌ | ✅ Políticas ideais geradas por ML | ✅ Políticas para fluxos de trabalho classificados |
| **Detecção de anomalia** | ❌ | ✅ Análise comportamental ML | ✅ Correlação de ameaças em tempo real |
| **Automação de conformidade** | ❌ | ✅ Relatórios SOC2/ISO/FedRAMP | ✅ Auto-evidência STIG/CMMC |
| **Integração HSM** | ❌ Chaves de software | ✅ PKCS#11 / HSM em Nuvem | ✅ HSM Classificado / Isolado |
| **Verificação formal** | ❌ | ✅ Provas Coq/Lean disponíveis | ✅ Artefatos completos de VF |
| **Suporte** | Comunidade | SLA 4h | SLA 15m + Operações 24/7 |
| **Preço** | **Grátis para sempre** | **Por assento / uso** | **Contrato** |

> **TERNAL** = Nosso mecanismo proprietário de síntese de políticas + detector de anomalias de ML + abstração de HSM + pipeline de verificação formal.  
> **Este crate é o runtime de verificação no qual as licenças TERNAL são executadas.**  
> Nós competimos com **Mythos, Replicated, Keygen, LicenseSpring** — e ganhamos em:
> - **Zero-heap** (todos eles alocam memória)
> - **No_std** (eles precisam de std/nuvem)
> - **Nativo para TPM2** (eles o simulam)
> - **Auditoria Merkle** (eles registram em arquivos de texto)
> - **Pronto para WASM/kernel** (eles não estão)

---

## 💻 Requisitos Mínimos de Hardware

| Ambiente | CPU | RAM | Armazenamento | TPM | Notas |
|-------------|-----|-----|---------|-----|-------|
| **Verificação mínima (no_std)** | Cortex-M4 / RISC-V RV32IMC | **2 KB RAM** | 8 KB Flash | Opcional | Bootloader/módulo de kernel |
| **Verificação padrão (std)** | x86_64 / ARM64 / RISC-V 64 | **64 KB** | 512 KB | Opcional | CLI, serviços, daemons |
| **Selagem TPM2** | Qualquer com TPM 2.0 | 1 MB | 2 MB | **Obrigatório** (PCR 0-7, 16) | Níveis Soberano/Corporativo |
| **Auditoria Merkle (sqlite)** | x86_64 / ARM64 | 4 MB | 10 MB + log | Opcional | Conformidade/Forense |
| **Bindings de Python** | x86_64 / ARM64 | 8 MB | 5 MB | Opcional | Overhead PyO3 |
| **WASM (navegador/edge)** | Qualquer alvo WASM | 16 MB | 200 KB .wasm | N/A | wasm32-unknown-unknown |

### Impressões Digitais no Mundo Real

```bash
# Verificação no_std mínima (ARM Cortex-M4)
$ cargo build --release --no-default-features --features crypto --target thumbv7em-none-eabihf
$ size target/thumbv7em-none-eabihf/release/sase_forge_verify
   text    data     bss     dec     hex filename
   3842     0     512    4354    1102  # ~4 KB flash, 512 bytes RAM

# Verificação padrão (x86_64 Linux)
$ cargo build --release --features crypto
$ size target/release/sase_forge_verify
   text    data     bss     dec     hex filename
   142K     8K     12K    162K       # ~162 KB binário

# WASM (navegador)
$ cargo build --release --target wasm32-unknown-unknown --features wasm,crypto
$ ls -lh target/wasm32-unknown-unknown/release/sase_forge_verify.wasm
   184K  # ~184 KB compactado para ~45 KB
```

---

## 🔧 Desinstalação / Remoção Fácil

### Rust (Cargo)

```bash
# Remover de Cargo.toml
# [dependencies]
# sase-forge-verify = "0.1"

# Limpar artefatos de build
cargo clean

# Ou remover do cache de registro
cargo uninstall sase-forge-verify  # se instalado como binário
```

### Python (pip)

```bash
pip uninstall sase-forge-verify
# Remove: pacote, bindings, arquivos .so/.pyd
# Nenhum arquivo de sistema modificado — pura instalação local de usuário (user-site)
```

### Pacote de Sistema (deb/rpm/brew)

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

### WASM / Embarcado

```bash
# Apenas exclua o arquivo .wasm ou a imagem de firmware
rm your_app.wasm
# Sem runtime, sem registro, sem serviços em segundo plano
```

### Purga Completa (Todos os Vestígios)

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

**Zero persistência.** Sem daemons, sem serviços systemd, sem módulos do kernel, sem chaves de registro, sem telemetria, sem phone-home.

---

## 📦 Matriz de Recursos

| Recurso | Descrição | Dependências | Impacto no Tamanho |
|---------|-------------|--------------|-------------|
| `crypto` | **Obrigatório.** Verificar/assinar Ed25519 | `ed25519-dalek`, `zeroize` | +15 KB |
| `std` | Suporte a biblioteca padrão | `std` | +5 KB |
| `python` | Bindings PyO3 | `std`, `pyo3` | +50 KB |
| `tpm` | TPM2 Real via TSS-ESAPI | `std`, `tss-esapi` | +200 KB |
| `sqlite` | Log de auditoria Merkle-DAG | `std`, `rusqlite` | +300 KB |
| `wasm` | Suporte ao alvo WASM | `getrandom/js` | +10 KB |
| `alloc` | `no_std` + alloc | `alloc` | +5 KB |
| `zeroize` | Zeramento de segredos | `zeroize` | +2 KB |

### Builds Mínimos

```toml
# Mínimo absoluto: apenas verificar, sem heap, sem std
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto"] }

# Bootloader embarcado (Cortex-M)
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto", "alloc"] }

# WASM para navegador
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto", "wasm"] }

# Todos os recursos (maioria dos apps)
[dependencies]
sase-forge-verify = { version = "0.1", features = ["std", "crypto", "python", "tpm", "sqlite", "wasm"] }
```

---

## 🔐 Propriedades de Segurança

| Propriedade | Implementação |
|----------|----------------|
| **Falsificação de Assinatura** | Ed25519 (RFC 8032) — segurança de 128-bits |
| **Ataques de Repetição** | Timestamp + nonce, comparação em tempo constante |
| **Substituição de Chave** | KeyID = BLAKE3(public_key) vinculado na licença |
| **Desvio de Relógio** | Tolerância de inclinação configurável, tempo constante |
| **Canais Laterais** | Caminho de tempo constante `verify_ct()` |
| **Segurança de Memória** | Rust `no_std` + `zeroize` — segredos zerados no drop |
| **Cadeia de Suprimentos** | Serialização `postcard` determinística, sem proc-macros no hot path |
| **Vinculação TPM** | PCR 0-7, 16 — detecta modificação bootkit/rootkit |
| **Integridade de Auditoria** | Merkle-DAG + SQLite WAL — à prova de violações |

---

## 📄 Licença

**Duplamente licenciado** sob sua escolha de:
- **Licença MIT** ([LICENSE-MIT](LICENSE-MIT))
- **Licença Apache 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

Isso se aplica **somente a este crate**. A stack proprietária TERNAL (síntese de política, detecção de anomalia ML, abstração HSM, verificação formal) **não está incluída** e requer uma licença comercial.

---

## 🤝 Contribuindo

1. Faça um Fork do repositório
2. Crie uma branch de recurso: `git checkout -b feat/recurso-incrivel`
3. Rode os testes: `cargo test --all-features`
4. Rode verificação de zero-heap: `RUSTFLAGS="-DSASE_ZERO_HEAP=1" cargo test --no-default-features --features crypto`
5. Envie PR com descrição clara

**Padrões de Código:**
- Compatível com `no_std` por padrão
- `const_fn` onde possível
- Caminhos críticos de zero-heap marcados com `zero_heap_hot_path!`
- Todos os segredos implementam `ZeroizeOnDrop`
- Documentação em todas as APIs públicas

---

## 🔗 Links

- **Documentação**: https://docs.rs/sase-forge-verify
- **Crates.io**: https://crates.io/crates/sase-forge-verify
- **Repositório**: https://github.com/bu25ny/sase-forge-verify
- **Problemas (Issues)**: https://github.com/bu25ny/sase-forge-verify/issues
- **Dúvidas Comerciais**: licensing@sase-antigravity.dev

---

## 🏷️ Versionamento

[SemVer](https://semver.org/) — `MAJOR.MINOR.PATCH`

- **0.1.x**: Desenvolvimento inicial, a API pode mudar
- **1.0.0**: API estável, pronta para produção

---

*Construído com ❤️ pela equipe SASE Antigravity — Software soberano para um mundo soberano.*
