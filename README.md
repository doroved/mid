[![Crates.io Version](https://img.shields.io/crates/v/mid)](https://crates.io/crates/mid)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/mid)](https://crates.io/crates/mid)
[![docs.rs](https://img.shields.io/docsrs/mid)](https://docs.rs/mid)

# mid

Создание хэша идентификатора устройства для MacOS/Windows/Linux.

Используются максимально статичные параметры системы, которые позволяют генерировать надежные хеши устройств для лицензирования ваших программ.

## История изменений

**v2.0.0** - 24 марта, 2024

- Вернул `to_lowercase()` для Windows MID result, который по ошибке был убран в v1.1.3. **Это изменит текущие хеши Windows устройств!** При необходимости, используйте версию 2.0.0 только для новых проектов или попросите пользователей перепривязать лицензию на новые хеши в текущем проекте.
- Добавлена функция `mid::data`, которая возвращает структуру данных: key, result, hash.
- `mid::print` выводит данные в консоль только в режиме отладки, они не попадут в релизную сборку проекта.
- В Linux используются 3 источника получения **machine-id**.
- Секретный ключ для хеширования не может быть пустым.
- Проведен полный рефакторинг кода.

---

Список параметров, которые используются на каждой из платформ.

## MacOS

```bash
system_profiler SPHardwareDataType
```

[Команда](https://ss64.com/osx/system_profiler.html) возвращает информацию о аппаратных характеристиках компьютера. Используемые параметры:

- **Model Number**: Этот параметр представляет собой номер модели компьютера или устройства. Он используется для уникальной идентификации конкретной модели в рамках производителя.

- **Serial Number**: Этот параметр является уникальным серийным номером компьютера или устройства. Он используется для идентификации конкретной единицы в рамках определенной модели.

- **Hardware UUID**: Этот параметр представляет уникальный идентификатор аппаратного обеспечения (UUID) компьютера или устройства. Он служит для обеспечения уникальной идентификации конкретной единицы в различных системах и средах.

- **Provisioning UDID**: Этот параметр представляет уникальный идентификатор устройства (UDID), который может использоваться в процессе предоставления или настройки устройства, обычно в корпоративной или управляемой среде.

```bash
system_profiler SPSecureElementDataType
```

Команда возвращает информацию о безопасном элементе (Secure Element). Этот элемент используется для хранения зашифрованных данных, таких как информация о платежных картах и другие конфиденциальные данные. Используемые параметры:

- **Platform ID**: Уникальный идентификатор платформы, к которой относится Secure Element (безопасный элемент).
- **SEID**: Уникальный идентификатор безопасного элемента (Secure Element ID). Создается при прошивки NFC чипа на заводе производителя.

## Windows

[PowerShell](https://ru.wikipedia.org/wiki/PowerShell) - расширяемое средство автоматизации. Используемые параметры:

- `powershell -command "Get-WmiObject Win32_ComputerSystemProduct"`: Возвращает уникальный идентификатор (UUID) продукта компьютера. Обычно связан с материнской платой компьютера. В редких случаях он может измениться после замены или переустановки материнской платы или после изменения BIOS/UEFI на устройстве.

- `powershell -command "Get-WmiObject Win32_BIOS"`: Возвращает серийный номер BIOS компьютера. Обычно остается постоянным и не подлежит изменению.

- `powershell -command "Get-WmiObject Win32_BaseBoard"`: Возвращает серийный номер базовой платы компьютера. Обычно остается постоянным и не подлежит изменению.

- `powershell -command "Get-WmiObject Win32_Processor"`: Возвращает идентификатор процессора компьютера. Должен оставаться неизменным, за исключением случаев замены процессора.

## Linux

- `machine-id`: Идентификатор (ID) машины, который используется для уникальной идентификации компьютера в Linux системах.

> **К сожалению, этот параметр подвержен изменениям, и пока не найдено надежного решения для Linux**.

## Как установить

Добавить зависимость в Cargo.toml

```toml
[dependencies]
mid = "2.0.0"
```

Или установить с помощью Cargo CLI

```bash
cargo add mid
```

## Как использовать

Получить хеш устройства

```rust
let machine_id = mid::get("mySecretKey").unwrap();
```

Получить данные MID key/result/hash

```rust
let mid_data = mid::data("mySecretKey").unwrap();
```

Вывести в консоль MID key/result/hash в режиме `debug_assertions`

```rust
mid::print("mySecretKey");
```

- `MID key` - Секретный ключ для хэширования
- `MID result` - Массив параметров OS
- `MID hash` - Хеш SHA-256 от result

## Подписывайтесь на мой Х

Здесь я буду делиться своими разработками и проектами
https://x.com/doroved

## Ссылки

- [machineid-rs](https://github.com/Taptiive/machineid-rs)
- [machine_uuid](https://github.com/choicesourcing/machine_uuid)
- [rust-machine-id](https://github.com/mathstuf/rust-machine-id)
- [app-machine-id](https://github.com/d-k-bo/app-machine-id)

[README RU](./README_RU.md)

# mid

Creating a Machine ID hash for MacOS/Windows/Linux.

Utilizes the most static system parameters possible to generate reliable device hashes for licensing your software.

## Change Log

**v2.0.0** - March 24, 2024

- Returned `to_lowercase()` for Windows MID result, which was mistakenly removed in v1.1.3. **This will change the current Windows device hashes!** If necessary, use version 2.0.0 for new projects only, or ask users to re-bind the license for new hashes in the current project.
- Added `mid::data` function that returns data structure: key, result, hash.
- `mid::print` outputs data to the console only in debug mode, it will not be included in the release build of the project.
- Linux uses 3 sources to get **machine-id**.
- The secret key for hashing cannot be empty.
- Complete code refactoring has been performed.

---

List of parameters that are used on each platform.

## MacOS

```bash
system_profiler SPHardwareDataType
```

The [command](https://ss64.com/osx/system_profiler.html) returns information about the computer's hardware characteristics. Parameters used:

- **Model Number**: This parameter represents the computer or device model number. It is used for uniquely identifying a specific model within the manufacturer's range.

- **Serial Number**: This parameter is the unique serial number of the computer or device. It is used to identify a specific unit within a particular model.

- **Hardware UUID**: This parameter represents the hardware UUID of the computer or device. It serves to provide unique identification of a specific unit across different systems and environments.

- **Provisioning UDID**: This parameter represents the device's unique device identifier (UDID), which can be used in the provisioning process or device setup, usually in a corporate or managed environment.

```bash
system_profiler SPSecureElementDataType
```

The command returns information about the Secure Element. This element is used to store encrypted data, such as information about payment cards and other confidential data. Parameters used:

- **Platform ID**: The unique identifier of the platform to which the Secure Element belongs.
- **SEID**: The unique identifier of the Secure Element. Created during the NFC chip firmware at the manufacturer's factory.

## Windows

[PowerShell](https://en.wikipedia.org/wiki/PowerShell) - expandable automation tool. Parameters used:

- `powershell -command "Get-WmiObject Win32_ComputerSystemProduct"`: Returns the unique product identifier (UUID) of the computer. Usually associated with the computer's motherboard. In rare cases, it may change after replacing or reinstalling the motherboard or after changing the device's BIOS/UEFI.

- `powershell -command "Get-WmiObject Win32_BIOS"`: Returns the computer's BIOS serial number. It usually remains constant and does not change.

- `powershell -command "Get-WmiObject Win32_BaseBoard"`: Returns the serial number of the computer's baseboard. It usually remains constant and does not change.

- `powershell -command "Get-WmiObject Win32_Processor"`: Returns the computer's processor identifier. It should remain unchanged, except in cases of processor replacement.

## Linux

- `machine-id`: A machine identifier (ID) that is used to uniquely identify a computer on Linux systems.

> **Unfortunately, this parameter is subject to change, and a reliable solution for Linux has not been found yet.**

## Installation

Add the dependency to Cargo.toml

```toml
[dependencies]
mid = "2.0.0"
```

Or install using Cargo CLI

```bash
cargo add mid
```

## How to Use

Get machine ID hash

```rust
let machine_id = mid::get("mySecretKey").unwrap();
```

Get MID key/result/hash data

```rust
let mid_data = mid::data("mySecretKey").unwrap();
```

Output the MID key/result/hash to the console in `debug_assertions` mode

```rust
mid::print("mySecretKey");
```

- `MID key` - The secret key for hashing
- `MID result` - Array of OS parameters
- `MID hash` - SHA-256 hash from result

## Subscribe to my X

Here I will share my developments and projects
https://x.com/doroved

## References

- [machineid-rs](https://github.com/Taptiive/machineid-rs)
- [machine_uuid](https://github.com/choicesourcing/machine_uuid)
- [rust-machine-id](https://github.com/mathstuf/rust-machine-id)
- [app-machine-id](https://github.com/d-k-bo/app-machine-id)
