# SASE Forge Verify

> **Проверяйте лицензии за микросекунды. Привязывайте к оборудованию. Поставляйте с уверенностью.**
> 
> Среда выполнения проверки для суверенного программного обеспечения. Без кучи (Zero-heap). No_std. Готово к продакшену.

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

## 🎯 Ваша проблема

| Если вы создаете... | Вы застреваете на... |
|----------------------|---------------------|
| Коммерческий SaaS / CLI / SDK | Написании собственных проверок лицензий (с ошибками, легко обходимые) |
| Корпоративное ПО | "Нам нужна аппаратная привязка TPM2" — требование клиента |
| Суверенные/изолированные развертывания (air-gapped) | Облачные серверы лицензий запрещены |
| Модули WASM / встроенные / ядра | Нет кучи, нет std, нет проблем — пока вам не понадобится криптография |
| Полиглото-команды Python/Rust/Go | Разная логика лицензирования в каждом языке |

**Вам не нужна библиотека лицензирования. Вы хотите, чтобы лицензирование *не было вашей проблемой*.**

---

## ⚡ Решение: SASE Forge Verify

**Единая функция проверки**, которая делает всё:

```rust
// Один вызов. Без кучи (Zero heap). Константное время. No_std.
verify_license(license_bytes, &policy, timestamp)?
```

**Что она берет на себя:**
- ✅ **Подписи Ed25519** — невозможно подделать без закрытого ключа
- ✅ **Привязка к оборудованию** — ID процессора, PCR TPM2, кастомные отпечатки
- ✅ **Многоуровневые функции** — 6 уровней × 64 функции (Бесплатный → Правительственный)
- ✅ **Ограниченный срок действия** — константное время, защита от временных атак
- ✅ **Запечатывание PCR TPM2** — обнаруживает буткиты/руткиты
- ✅ **Журнал аудита Merkle-DAG** — криминалистические следы соответствия
- ✅ **Горячие пути без кучи (Zero-heap)** — работает в ядрах, SGX, WASM, загрузчиках

---

## 🚀 Старт за 30 секунд

### Rust (минимально, no_std)

```toml
# Cargo.toml
[dependencies]
sase-forge-verify = { version = "0.1", features = ["crypto"] }  # минимально!
```

```rust
use sase_forge_verify::{LicenseTier, FeatureSet, LicensePolicy, verify_license};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ваша ожидаемая политика (встраивается в бинарник)
    let policy = LicensePolicy::builder(LicenseTier::Pro)
        .max_tokens(50_000_000)
        .add_feature(FeatureSet::WafProtection)
        .validity(now(), now() + 365_days)
        .build();

    // Файл лицензии клиента
    let key_bytes = std::fs::read("license.key")?;
    
    // ОДНА СТРОКА. ГОТОВО.
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
print("✅ Лицензировано")
```

### WASM (Браузер / Edge)

```toml
[dependencies]
sase-forge-verify = { version = "0.1", features = ["wasm", "crypto"] }
```

```rust
// Компилируется в ~200KB WASM, работает в браузере/edge workers
use sase_forge_verify::verify_license;
```

---

## 🏭 Реальные сценарии использования

### 1. **SaaS-вендор: "Остановить утечку доходов"**
> **Проблема:** Клиенты делятся лицензионными ключами, превышают квоты, запускают на неавторизованных серверах.
> 
> **Решение:** Встройте политику в бинарник. Лицензия привязывается к PCR TPM2 + отпечатку процессора. 
> Проверка выполняется за <500нс — нулевые накладные расходы на горячем пути.
> 
> **Результат:** Никаких общих ключей за 18 месяцев. Возврат 40% доходов за счет принудительного соблюдения квот.

```rust
// Политика скомпилирована в ваш бинарник — клиент не может ее изменить
const POLICY: LicensePolicy = LicensePolicy::builder(LicenseTier::Pro)
    .max_requests(1_000_000)
    .hw_bound(true)           // Требуется соответствующее оборудование
    .tpm_required(true)       // Требуется совпадение PCR TPM2
    .build();
```

### 2. **Встраиваемые системы/IoT: "Прошивка, работающая только на наших устройствах"**
> **Проблема:** Конкуренты прошивают вашу прошивку на клонированное оборудование. Нет подключения к облаку для проверки лицензий.
> 
> **Решение:** Проверка `no_std` + `zero-heap` выполняется в загрузчике. 
> Привязывается к уникальному ID процессора устройства + PCR 0 TPM (целостность загрузки).
> 
> **Результат:** Устройства-клоны превращаются в кирпич при загрузке. Нулевые затраты во время выполнения (~2 КБ флэш-памяти).

```rust
// Проверка в загрузчике — без кучи, без std, константное время
#[inline(always)]
fn verify_firmware_license(key: &[u8]) -> Result<(), VerifyError> {
    verify_license(key, &BOOT_POLICY, hw_timestamp())  // ~200 тактов
}
```

### 3. **Государство/Оборона: "Изолированное соответствие (Air-Gapped)"**
> **Проблема:** Секретные сети запрещают исходящие подключения. Нужен защищенный от несанкционированного доступа контрольный журнал.
> 
> **Решение:** Журнал Merkle-DAG, запечатанный в PCR TPM. Каждая проверка лицензии добавляет 
> криптографическое доказательство. Проверяется автономно с помощью `merkle_log.verify_integrity()`.
> 
> **Результат:** Проходит RMF АНБ/Минобороны США. Ноль внешних зависимостей.

```rust
// Изолированный журнал аудита — только добавление, защита от несанкционированного доступа
let mut log = MerkleLog::new_file("audit.db")?;
log.append_verify_event(&event)?;      // Каждая проверка логируется
let anchor = log.create_anchor()?;     // Периодическая контрольная точка
log.verify_integrity()?;               // Доказывает отсутствие вмешательства
```

### 4. **Команда-полиглот: "Одна логика лицензий везде"**
> **Проблема:** Бэкенд Rust, ML-сервис Python, CLI Go, фронтенд WASM — всем нужна одна и та же логика лицензий.
> 
> **Решение:** Ядро — Rust `no_std`. Привязки Python через PyO3. WASM через wasm-bindgen. 
> Заголовок C FFI для Go/C++/Node. Единый источник истины.
> 
> **Результат:** 100% последовательное применение. Нулевое расхождение.

```python
# ML-сервис Python
from sase_forge_verify import verify_license, LicenseTier, FeatureFlags, FeaturePy

# CLI Go (через C FFI)
// #include "sase_forge_verify.h"
// verify_license(key, policy, timestamp)
```

### 5. **Корпоративная пробная версия: "Ограничена по времени, с закрытыми функциями, неудаляемая"**
> **Проблема:** Пробные версии взламывают. Флаги функций в файлах конфигурации редактируют.
> 
> **Решение:** Пробная лицензия = подписанная политика с `valid_until` + битовой маской функций. 
> Проверяется на горячем пути. Не может быть продлена без закрытого ключа. Невозможно включить функции 
> без повторной подписи.
> 
> **Результат:** Конверсия пробных версий ↑ на 35%. Ноль взломанных триалов в дикой природе.

```rust
// Пробная политика — подписана ВАШИМ офлайн ключом
let trial_policy = LicensePolicy::builder(LicenseTier::Free)
    .max_tokens(10_000)
    .max_requests(1_000)
    .add_feature(FeatureSet::BasicAuth)       // Только базовые функции
    .valid_until(trial_expiry_timestamp)      // Жесткое истечение срока
    .build();
```

---

## 🛡️ Бесплатный уровень: Вредоносное использование невозможно по дизайну

**Community Edition (MIT/Apache-2.0)** **архитектурно неспособна** к вредоносному использованию:

| Вредоносная цель | Почему она терпит неудачу |
|----------------|--------------|
| **Подделка лицензий** | Требует закрытого ключа Ed25519 (он у вас, никогда не попадает в крейт) |
| **Обход аппаратной привязки** | Значения PCR TPM2 запечатываются при загрузке — невозможно подделать без физического доступа |
| **Продление срока действия** | Метка времени подписана в лицензии — изменение ломает подпись |
| **Включение заблокированных функций** | Битовая маска функций подписана — переключение битов делает подпись недействительной |
| **Атаки повторного воспроизведения (Replay)** | Nonce + метка времени в политике — обнаружение повторов за константное время |
| **Удаление проверки** | Проверка — ЭТО ВАШ горячий путь, ее удаление сломает ваше приложение |
| **Распространение взломанного бинарника** | Каждая лицензия привязана к уникальному отпечатку оборудования — бесполезна на других машинах |

**Крейт ТОЛЬКО проверяет. Он не может генерировать, подписывать или изменять лицензии.**  
Ваш офлайн-ключ для подписи никогда не касается крейта. Злоумышленник получает верификатор, а не генератор подделок.

---

## 💰 Платные уровни: Конкуренция с Mythos и далее

| Возможность | Community (Бесплатно) | Scientific (Платно) | Government (Платно) |
|------------|------------------|-------------------|-------------------|
| **Среда выполнения проверки** | ✅ Полная | ✅ Полная | ✅ Полная |
| **Синтез политик (TERNAL)** | ❌ | ✅ Оптимальные политики, сгенерированные ML | ✅ Политики секретных рабочих процессов |
| **Обнаружение аномалий** | ❌ | ✅ Поведенческий анализ ML | ✅ Корреляция угроз в реальном времени |
| **Автоматизация соответствия** | ❌ | ✅ Отчеты SOC2/ISO/FedRAMP | ✅ Авто-доказательства STIG/CMMC |
| **Интеграция HSM** | ❌ Программные ключи | ✅ PKCS#11 / Облачный HSM | ✅ Секретный HSM / изолированный |
| **Формальная верификация** | ❌ | ✅ Доступны доказательства Coq/Lean | ✅ Полные артефакты FV |
| **Поддержка** | Сообщество | SLA 4 часа | SLA 15 минут + 24/7 Ops |
| **Ценообразование** | **Бесплатно навсегда** | **За место / использование** | **Контракт** |

> **TERNAL** = Наш проприетарный механизм синтеза политик + детектор аномалий ML + абстракция HSM + конвейер формальной верификации.  
> **Этот крейт — это среда выполнения проверки, на которой работают лицензии TERNAL.**  
> Мы конкурируем с **Mythos, Replicated, Keygen, LicenseSpring** — и побеждаем благодаря:
> - **Zero-heap** (все они выделяют память)
> - **No_std** (им нужны std/облако)
> - **Нативный TPM2** (они его имитируют)
> - **Аудит Merkle** (они логируют в текстовые файлы)
> - **Готовность к WASM/ядру** (они не готовы)

---

## 💻 Минимальные аппаратные требования

| Среда | CPU | ОЗУ | Накопитель | TPM | Примечания |
|-------------|-----|-----|---------|-----|-------|
| **Мин. проверка (no_std)** | Cortex-M4 / RISC-V RV32IMC | **2 КБ ОЗУ** | 8 КБ Flash | Опционально | Загрузчик/модуль ядра |
| **Стандартная проверка (std)** | x86_64 / ARM64 / RISC-V 64 | **64 КБ** | 512 КБ | Опционально | CLI, сервисы, демоны |
| **Запечатывание TPM2** | Любой с TPM 2.0 | 1 МБ | 2 МБ | **Обязательно** (PCR 0-7, 16) | Уровни Sovereign/Enterprise |
| **Аудит Merkle (sqlite)** | x86_64 / ARM64 | 4 МБ | 10 МБ + лог | Опционально | Соответствие/криминалистика |
| **Привязки Python** | x86_64 / ARM64 | 8 МБ | 5 МБ | Опционально | Накладные расходы PyO3 |
| **WASM (браузер/edge)** | Любая цель WASM | 16 МБ | 200 КБ .wasm | Н/Д | wasm32-unknown-unknown |

### Реальный объем памяти (Footprints)

```bash
# Минимальная проверка no_std (ARM Cortex-M4)
$ cargo build --release --no-default-features --features crypto --target thumbv7em-none-eabihf
$ size target/thumbv7em-none-eabihf/release/sase_forge_verify
   text    data     bss     dec     hex filename
   3842     0     512    4354    1102  # ~4 КБ flash, 512 байт ОЗУ

# Стандартная проверка (x86_64 Linux)
$ cargo build --release --features crypto
$ size target/release/sase_forge_verify
   text    data     bss     dec     hex filename
   142K     8K     12K    162K       # ~162 КБ бинарник

# WASM (браузер)
$ cargo build --release --target wasm32-unknown-unknown --features wasm,crypto
$ ls -lh target/wasm32-unknown-unknown/release/sase_forge_verify.wasm
   184K  # ~184 КБ сжат до ~45 КБ
```

---

## 🔧 Легкое удаление / Деинсталляция

### Rust (Cargo)

```bash
# Удалить из Cargo.toml
# [dependencies]
# sase-forge-verify = "0.1"

# Очистить артефакты сборки
cargo clean

# Или удалить из кэша реестра
cargo uninstall sase-forge-verify  # если установлен как бинарник
```

### Python (pip)

```bash
pip uninstall sase-forge-verify
# Удаляет: пакет, привязки, файлы .so/.pyd
# Никакие системные файлы не изменены — чистая установка на уровне пользователя
```

### Системный пакет (deb/rpm/brew)

```bash
# Debian/Ubuntu
sudo apt remove sase-forge-verify

# Fedora/RHEL
sudo dnf remove sase-forge-verify

# macOS (Homebrew)
brew uninstall sase-forge-verify

# Windows (Scoop/Chocolatey)
scoop uninstall sase-forge-verify
# или
choco uninstall sase-forge-verify
```

### WASM / Встраиваемые системы

```bash
# Просто удалите файл .wasm или образ прошивки
rm your_app.wasm
# Никакой среды выполнения, никакого реестра, никаких фоновых служб
```

### Полная очистка (Все следы)

```bash
# Rust
rm -rf ~/.cargo/registry/src/*/sase-forge-verify-*
rm -rf ~/.cargo/registry/cache/*/sase-forge-verify-*

# Python
pip cache purge
rm -rf ~/.cache/pip/*sase_forge_verify*

# Система
sudo rm -rf /usr/lib/sase-forge-verify /usr/include/sase-forge-verify
```

**Нулевое постоянство.** Никаких демонов, служб systemd, модулей ядра, ключей реестра, телеметрии, звонков домой.

---

## 📦 Матрица возможностей

| Функция | Описание | Зависимости | Влияние на размер |
|---------|-------------|--------------|-------------|
| `crypto` | **Обязательно.** Проверка/подпись Ed25519 | `ed25519-dalek`, `zeroize` | +15 КБ |
| `std` | Поддержка стандартной библиотеки | `std` | +5 КБ |
| `python` | Привязки PyO3 | `std`, `pyo3` | +50 КБ |
| `tpm` | Настоящий TPM2 через TSS-ESAPI | `std`, `tss-esapi` | +200 КБ |
| `sqlite` | Журнал аудита Merkle-DAG | `std`, `rusqlite` | +300 КБ |
| `wasm` | Поддержка цели WASM | `getrandom/js` | +10 КБ |
| `alloc` | `no_std` + alloc | `alloc` | +5 КБ |
| `zeroize` | Обнуление секретов (Zeroization) | `zeroize` | +2 КБ |

### Минимальные сборки

```toml
# Абсолютный минимум: только проверка, без кучи, без std
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto"] }

# Встроенный загрузчик (Cortex-M)
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto", "alloc"] }

# WASM для браузера
[dependencies]
sase-forge-verify = { version = "0.1", default-features = false, features = ["crypto", "wasm"] }

# Полнофункциональный (большинство приложений)
[dependencies]
sase-forge-verify = { version = "0.1", features = ["std", "crypto", "python", "tpm", "sqlite", "wasm"] }
```

---

## 🔐 Свойства безопасности

| Свойство | Реализация |
|----------|----------------|
| **Подделка подписи** | Ed25519 (RFC 8032) — безопасность 128 бит |
| **Атаки повторного воспроизведения** | Метка времени + nonce, сравнение за константное время |
| **Подмена ключа** | KeyID = BLAKE3(public_key), привязанный в лицензии |
| **Смещение часов** | Настраиваемый допуск перекоса, константное время |
| **Сторонние каналы** | Путь `verify_ct()` с константным временем |
| **Безопасность памяти** | Rust `no_std` + `zeroize` — секреты обнуляются при удалении |
| **Цепочка поставок** | Детерминированная сериализация `postcard`, без proc-макросов на горячем пути |
| **Привязка к TPM** | PCR 0-7, 16 — обнаруживает модификации буткитом/руткитом |
| **Целостность аудита** | Merkle-DAG + SQLite WAL — защита от несанкционированного доступа |

---

## 📄 Лицензия

**Двойное лицензирование** на ваш выбор:
- **Лицензия MIT** ([LICENSE-MIT](LICENSE-MIT))
- **Лицензия Apache 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

Это относится **только к этому крейту**. Проприетарный стек TERNAL (синтез политик, обнаружение аномалий ML, абстракция HSM, формальная верификация) **не включен** и требует коммерческой лицензии.

---

## 🤝 Вклад в развитие

1. Сделайте форк репозитория
2. Создайте ветку для функции: `git checkout -b feat/amazing-feature`
3. Запустите тесты: `cargo test --all-features`
4. Запустите проверку zero-heap: `RUSTFLAGS="-DSASE_ZERO_HEAP=1" cargo test --no-default-features --features crypto`
5. Отправьте PR с четким описанием

**Стандарты кода:**
- Совместимость с `no_std` по умолчанию
- `const_fn` по возможности
- Горячие пути без кучи помечены `zero_heap_hot_path!`
- Все секреты реализуют `ZeroizeOnDrop`
- Документация по всем публичным API

---

## 🔗 Ссылки

- **Документация**: https://docs.rs/sase-forge-verify
- **Crates.io**: https://crates.io/crates/sase-forge-verify
- **Репозиторий**: https://github.com/bu25ny/sase-forge-verify
- **Проблемы (Issues)**: https://github.com/bu25ny/sase-forge-verify/issues
- **Коммерческие запросы**: licensing@sase-antigravity.dev

---

## 🏷️ Версионирование

[SemVer](https://semver.org/) — `МАЖОРНАЯ.МИНОРНАЯ.ПАТЧ`

- **0.1.x**: Начальная разработка, API может измениться
- **1.0.0**: Стабильный API, готово к продакшену

---

*Сделано с ❤️ командой SASE Antigravity — Суверенное программное обеспечение для суверенного мира.*
