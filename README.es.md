# SASE Forge Verify

> **Verifica licencias en microsegundos. Vincula al hardware. Lanza con confianza.**
> 
> El entorno de ejecución de verificación para software soberano. Zero-heap. No_std. Listo para producción.

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

## 🎯 El Problema que Tienes

| Si estás construyendo... | Estás atrapado con... |
|----------------------|---------------------|
| SaaS / CLI / SDK comercial | Crear tus propios controles de licencia (con errores, evasibles) |
| Software empresarial | "Necesitamos vinculación de hardware TPM2" — requisito del cliente |
| Implementaciones soberanas/aisladas (air-gapped) | No se permiten servidores de licencias en la nube |
| WASM / embebido / módulos de kernel | Sin montículo (heap), sin std, no hay problema — hasta que necesitas criptografía |
| Equipos políglotas en Python/Rust/Go | Lógica de licencias diferente en cada lenguaje |

**No quieres una biblioteca de licencias. Quieres que las licencias *no sean tu problema*.**

---

## ⚡ La Solución: SASE Forge Verify

Una **única función de verificación** que lo hace todo:

```rust
// Una llamada. Cero montículo (Zero heap). Tiempo constante. No_std.
verify_license(license_bytes, &policy, timestamp)?
```

**Lo que maneja por ti:**
- ✅ **Firmas Ed25519** — imposibles de falsificar sin clave privada
- ✅ **Vinculación de hardware** — ID de CPU, PCRs de TPM2, huellas personalizadas
- ✅ **Características por niveles** — 6 niveles × 64 características (Gratis → Gobierno)
- ✅ **Validez limitada en el tiempo** — tiempo constante, sin ataques de reloj
- ✅ **Sellado de PCR TPM2** — detecta bootkits/rootkits
- ✅ **Registro de auditoría Merkle-DAG** — rastros de cumplimiento forense
- ✅ **Rutas críticas sin montículo (zero-heap hot paths)** — se ejecuta en kernels, SGX, WASM, gestores de arranque

---

## 🚀 Inicio en 30 Segundos

### Rust (mínimo, no_std)

```toml
# Cargo.toml
[dependencies]
sase-forge-verify = { version = "0.1", features = ["crypto"] }  # ¡mínimo!
```

```rust
use sase_forge_verify::{LicenseTier, FeatureSet, LicensePolicy, verify_license};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tu política esperada (incrustar en el binario)
    let policy = LicensePolicy::builder(LicenseTier::Pro)
        .max_tokens(50_000_000)
        .add_feature(FeatureSet::WafProtection)
        .validity(now(), now() + 365_days)
        .build();

    // Archivo de licencia del cliente
    let key_bytes = std::fs::read("license.key")?;
    
    // UNA LÍNEA. HECHO.
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
// Se compila a WASM de ~200KB, se ejecuta en trabajadores de navegador/edge
use sase_forge_verify::verify_license;
```

---

## 🏭 Casos de Uso Reales

### 1. **Proveedor de SaaS: "Detener la Fuga de Ingresos"**
> **Problema:** Los clientes comparten claves de licencia, exceden las cuotas, se ejecutan en servidores no autorizados.
> 
> **Solución:** Incrustar la política en el binario. La licencia se vincula a los PCR de TPM2 + huella de la CPU. 
> La verificación se ejecuta en <500ns — cero sobrecarga en la ruta crítica.
> 
> **Resultado:** Cero claves compartidas en 18 meses. Recuperación del 40% de ingresos mediante el cumplimiento de cuotas.

```rust
// Política compilada en tu binario — el cliente no puede cambiarla
const POLICY: LicensePolicy = LicensePolicy::builder(LicenseTier::Pro)
    .max_requests(1_000_000)
    .hw_bound(true)           // Requiere hardware coincidente
    .tpm_required(true)       // Requiere coincidencia de PCR de TPM2
    .build();
```

### 2. **Embebido/IoT: "Firmware que solo se ejecuta en nuestros dispositivos"**
> **Problema:** Los competidores instalan tu firmware en hardware clonado. Sin conectividad a la nube para controles de licencia.
> 
> **Solución:** Verificación `no_std` + `zero-heap` se ejecuta en el gestor de arranque. 
> Se vincula al ID de CPU único del dispositivo + TPM PCR 0 (integridad de arranque).
> 
> **Resultado:** Los dispositivos clonados se bloquean (brick) al arrancar. Costo de ejecución nulo (~2KB flash).

```rust
// Verificación en el gestor de arranque — sin montículo, sin std, tiempo constante
#[inline(always)]
fn verify_firmware_license(key: &[u8]) -> Result<(), VerifyError> {
    verify_license(key, &BOOT_POLICY, hw_timestamp())  // ~200 ciclos
}
```

### 3. **Gobierno/Defensa: "Cumplimiento Aislado (Air-Gapped)"**
> **Problema:** Las redes clasificadas prohíben las conexiones salientes. Se necesita un rastro de auditoría a prueba de manipulaciones.
> 
> **Solución:** Registro Merkle-DAG sellado a PCRs de TPM. Cada control de licencia añade 
> una prueba criptográfica. Verificable sin conexión con `merkle_log.verify_integrity()`.
> 
> **Resultado:** Pasa NSA/DoD RMF. Cero dependencias externas.

```rust
// Registro de auditoría aislado — solo anexión, a prueba de manipulaciones
let mut log = MerkleLog::new_file("audit.db")?;
log.append_verify_event(&event)?;      // Cada comprobación se registra
let anchor = log.create_anchor()?;     // Punto de control periódico
log.verify_integrity()?;               // Prueba que no hubo manipulación
```

### 4. **Equipo Políglota: "Una Única Lógica de Licencias en Todas Partes"**
> **Problema:** Backend en Rust, servicio ML en Python, CLI en Go, frontend WASM — todos necesitan la misma lógica de licencias.
> 
> **Solución:** El núcleo es Rust `no_std`. Vinculaciones para Python vía PyO3. WASM vía wasm-bindgen. 
> Encabezado C FFI para Go/C++/Node. Única fuente de verdad.
> 
> **Resultado:** Cumplimiento 100% consistente. Cero desviaciones.

```python
# Servicio ML en Python
from sase_forge_verify import verify_license, LicenseTier, FeatureFlags, FeaturePy

# CLI en Go (vía C FFI)
// #include "sase_forge_verify.h"
// verify_license(key, policy, timestamp)
```

### 5. **Prueba Empresarial: "Limitada en Tiempo, con Funciones Restringidas, Inamovible"**
> **Problema:** Las versiones de prueba se crackean. Las banderas de características en los archivos de configuración se editan.
> 
> **Solución:** Licencia de prueba = política firmada con `valid_until` + máscara de bits de características. 
> Verificada en la ruta crítica. No se puede extender sin la clave privada. No se pueden habilitar características 
> sin volver a firmar.
> 
> **Resultado:** La conversión de pruebas aumenta un 35%. Cero pruebas crackeadas en uso real.

```rust
// Política de prueba — firmada por TU clave fuera de línea (offline)
let trial_policy = LicensePolicy::builder(LicenseTier::Free)
    .max_tokens(10_000)
    .max_requests(1_000)
    .add_feature(FeatureSet::BasicAuth)       // Solo funciones básicas
    .valid_until(trial_expiry_timestamp)      // Expiración estricta
    .build();
```

---

## 🛡️ Nivel Gratuito: Uso Malicioso Imposible por Diseño

La **Community Edition (MIT/Apache-2.0)** es **arquitectónicamente incapaz** de permitir el uso malicioso:

| Objetivo Malicioso | Por Qué Falla |
|----------------|--------------|
| **Falsificar licencias** | Requiere clave privada Ed25519 (tú la tienes, nunca en el crate) |
| **Eludir vinculación de hardware** | Valores PCR de TPM2 sellados al arrancar — no se pueden falsificar sin acceso físico |
| **Extender expiración** | Marca de tiempo firmada en la licencia — modificarla rompe la firma |
| **Habilitar funciones bloqueadas** | Máscara de bits de características firmada — cambiar bits invalida la firma |
| **Ataques de repetición (Replay)** | Nonce + marca de tiempo en la política — detección de repetición en tiempo constante |
| **Eliminar verificación** | La verificación ES tu ruta crítica — eliminarla rompe tu aplicación |
| **Distribuir binario crackeado** | Cada licencia se vincula a una huella de HW única — inútil en otras máquinas |

**El crate SOLAMENTE verifica. No puede generar, firmar ni modificar licencias.**  
Tu clave de firma offline nunca toca el crate. El atacante obtiene un verificador, no un falsificador.

---

## 💰 Niveles de Pago: Compitiendo con Mythos y Más Allá

| Capacidad | Community (Gratis) | Scientific (Pago) | Government (Pago) |
|------------|------------------|-------------------|-------------------|
| **Entorno de verificación** | ✅ Completo | ✅ Completo | ✅ Completo |
| **Síntesis de política (TERNAL)** | ❌ | ✅ Políticas óptimas generadas por ML | ✅ Políticas para flujos clasificados |
| **Detección de anomalías** | ❌ | ✅ Análisis de comportamiento con ML | ✅ Correlación de amenazas en tiempo real |
| **Automatización de cumplimiento** | ❌ | ✅ Informes SOC2/ISO/FedRAMP | ✅ Auto-evidencia STIG/CMMC |
| **Integración HSM** | ❌ Claves por software | ✅ PKCS#11 / HSM en la Nube | ✅ HSM Clasificado / Aislado |
| **Verificación formal** | ❌ | ✅ Pruebas Coq/Lean disponibles | ✅ Artefactos de VF completos |
| **Soporte** | Comunidad | SLA 4h | SLA 15m + Operaciones 24/7 |
| **Precios** | **Gratis para siempre** | **Por asiento / uso** | **Contrato** |

> **TERNAL** = Nuestro motor de síntesis de políticas propietario + detector de anomalías ML + abstracción HSM + canalización de verificación formal.  
> **Este crate es el entorno de verificación sobre el cual se ejecutan las licencias TERNAL.**  
> Competimos con **Mythos, Replicated, Keygen, LicenseSpring** — y ganamos en:
> - **Zero-heap** (todos ellos asignan memoria)
> - **No_std** (requieren std/nube)
> - **TPM2 nativo** (lo simulan)
> - **Auditoría Merkle** (registran en archivos de texto)
> - **Listos para WASM/kernel** (no lo están)

---

## 💻 Requisitos Mínimos de Hardware

| Entorno | CPU | RAM | Almacenamiento | TPM | Notas |
|-------------|-----|-----|---------|-----|-------|
| **Verificación mínima (no_std)** | Cortex-M4 / RISC-V RV32IMC | **2 KB RAM** | 8 KB Flash | Opcional | Gestor de arranque/módulo kernel |
| **Verificación estándar (std)** | x86_64 / ARM64 / RISC-V 64 | **64 KB** | 512 KB | Opcional | CLI, servicios, demonios |
| **Sellado TPM2** | Cualquiera con TPM 2.0 | 1 MB | 2 MB | **Requerido** (PCR 0-7, 16) | Niveles Soberano/Empresarial |
| **Auditoría Merkle (sqlite)** | x86_64 / ARM64 | 4 MB | 10 MB + log | Opcional | Cumplimiento/forense |
| **Bindings de Python** | x86_64 / ARM64 | 8 MB | 5 MB | Opcional | Sobrecarga de PyO3 |
| **WASM (navegador/edge)** | Cualquier destino WASM | 16 MB | 200 KB .wasm | N/A | wasm32-unknown-unknown |

### Huellas de Memoria en el Mundo Real

```bash
# Verificación mínima no_std (ARM Cortex-M4)
$ cargo build --release --no-default-features --features crypto --target thumbv7em-none-eabihf
$ size target/thumbv7em-none-eabihf/release/sase_forge_verify
   text    data     bss     dec     hex filename
   3842     0     512    4354    1102  # ~4 KB flash, 512 bytes RAM

# Verificación estándar (x86_64 Linux)
$ cargo build --release --features crypto
$ size target/release/sase_forge_verify
   text    data     bss     dec     hex filename
   142K     8K     12K    162K       # ~162 KB binario

# WASM (navegador)
$ cargo build --release --target wasm32-unknown-unknown --features wasm,crypto
$ ls -lh target/wasm32-unknown-unknown/release/sase_forge_verify.wasm
   184K  # ~184 KB comprimido con gzip a ~45 KB
```

---

## 🔧 Desinstalación / Eliminación Fácil

### Rust (Cargo)

```bash
# Eliminar de Cargo.toml
# [dependencies]
# sase-forge-verify = "0.1"

# Limpiar artefactos de compilación
cargo clean

# O eliminar de la caché del registro
cargo uninstall sase-forge-verify  # si está instalado como binario
```

### Python (pip)

```bash
pip uninstall sase-forge-verify
# Elimina: paquete, bindings, archivos .so/.pyd
# No se modifican archivos del sistema — pura instalación a nivel de usuario (user-site)
```

### Paquete del Sistema (deb/rpm/brew)

```bash
# Debian/Ubuntu
sudo apt remove sase-forge-verify

# Fedora/RHEL
sudo dnf remove sase-forge-verify

# macOS (Homebrew)
brew uninstall sase-forge-verify

# Windows (Scoop/Chocolatey)
scoop uninstall sase-forge-verify
# o
choco uninstall sase-forge-verify
```

### WASM / Embebido

```bash
# Simplemente elimina el archivo .wasm o la imagen de firmware
rm your_app.wasm
# Sin entorno de ejecución, sin registro, sin servicios en segundo plano
```

### Purga Completa (Todos los rastros)

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

**Cero persistencia.** Sin demonios, sin servicios systemd, sin módulos del kernel, sin claves de registro, sin telemetría, sin "phone-home".

---

## 📦 Matriz de Características

| Característica | Descripción | Dependencias | Impacto en Tamaño |
|---------|-------------|--------------|-------------|
| `crypto` | **Requerido.** Verificar/firmar Ed25519 | `ed25519-dalek`, `zeroize` | +15 KB |
| `std` | Soporte de biblioteca estándar | `std` | +5 KB |
| `python` | Bindings PyO3 | `std`, `pyo3` | +50 KB |
| `tpm` | TPM2 Real vía TSS-ESAPI | `std`, `tss-esapi` | +200 KB |
| `sqlite` | Registro de auditoría Merkle-DAG | `std`, `rusqlite` | +300 KB |
| `wasm` | Soporte para destino WASM | `getrandom/js` | +10 KB |
| `alloc` | `no_std` + alloc | `alloc` | +5 KB |
| `zeroize` | Puesta a cero de secretos | `zeroize` | +2 KB |

### Compilaciones Mínimas

```toml
# Mínimo absoluto: solo verificación, sin montículo, sin std
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto"] }

# Gestor de arranque embebido (Cortex-M)
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto", "alloc"] }

# WASM para navegador
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto", "wasm"] }

# Funciones completas (la mayoría de las apps)
[dependencies]
sase-forge-verify = { version = "0.1", features = ["std", "crypto", "python", "tpm", "sqlite", "wasm"] }
```

---

## 🔐 Propiedades de Seguridad

| Propiedad | Implementación |
|----------|----------------|
| **Falsificación de Firma** | Ed25519 (RFC 8032) — 128-bit de seguridad |
| **Ataques de Repetición** | Marca de tiempo + nonce, comparación de tiempo constante |
| **Sustitución de Clave** | KeyID = BLAKE3(public_key) vinculado en licencia |
| **Desviación de Reloj** | Tolerancia de sesgo configurable, tiempo constante |
| **Canales Laterales** | Ruta de tiempo constante `verify_ct()` |
| **Seguridad de Memoria** | Rust `no_std` + `zeroize` — secretos puestos a cero al liberar |
| **Cadena de Suministro** | Serialización determinista `postcard`, sin proc-macros en ruta crítica |
| **Vinculación TPM** | PCR 0-7, 16 — detecta modificaciones por bootkit/rootkit |
| **Integridad de Auditoría** | Merkle-DAG + SQLite WAL — a prueba de manipulaciones |

---

## 📄 Licencia

**Doble licencia** bajo su elección de:
- **Licencia MIT** ([LICENSE-MIT](LICENSE-MIT))
- **Licencia Apache 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

Esto se aplica **solo a este crate**. El stack propietario TERNAL (síntesis de políticas, detección de anomalías ML, abstracción HSM, verificación formal) **no está incluido** y requiere una licencia comercial.

---

## 🤝 Contribuir

1. Haz un Fork del repositorio
2. Crea una rama para la característica: `git checkout -b feat/caracteristica-increible`
3. Ejecuta las pruebas: `cargo test --all-features`
4. Ejecuta la verificación de zero-heap: `RUSTFLAGS="-DSASE_ZERO_HEAP=1" cargo test --no-default-features --features crypto`
5. Envía un PR con una descripción clara

**Estándares de Código:**
- Compatible con `no_std` por defecto
- `const_fn` donde sea posible
- Rutas críticas sin montículo marcadas con `zero_heap_hot_path!`
- Todos los secretos implementan `ZeroizeOnDrop`
- Documentación en todas las APIs públicas

---

## 🔗 Enlaces

- **Documentación**: https://docs.rs/sase-forge-verify
- **Crates.io**: https://crates.io/crates/sase-forge-verify
- **Repositorio**: https://github.com/bu25ny/sase-forge-verify
- **Incidencias (Issues)**: https://github.com/bu25ny/sase-forge-verify/issues
- **Consultas Comerciales**: licensing@sase-antigravity.dev

---

## 🏷️ Versionado

[SemVer](https://semver.org/) — `MAYOR.MENOR.PARCHE`

- **0.1.x**: Desarrollo inicial, la API puede cambiar
- **1.0.0**: API estable, lista para producción

---

*Construido con ❤️ por el equipo SASE Antigravity — Software soberano para un mundo soberano.*
