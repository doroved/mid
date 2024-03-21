<a href="https://crates.io/crates/mid"><img src="https://img.shields.io/crates/v/mid?style=for-the-badge&logo=rust&color=orange" /></a>
<a href="https://docs.rs/mid/latest/mid/">
<img src="https://img.shields.io/badge/docs-latest-blue.svg?style=for-the-badge&logo=rust&color=blue"
      alt="docs.rs docs" />
</a>

## MachineID для Rust

Создание уникального хэша идентификатора устройства для MacOS/Linux/Windows.

Отличное решение для лицензирования ваших программ, которое использует максимально статичные и в основном неизменяемые данные.

Далее, ознакомьтесь с параметрами, которые мы используем на каждой из платформ.

### MacOS

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

### Linux

- `cat /etc/machine-id`: Возвращает идентификатор (ID) машины, который используется для уникальной идентификации компьютера в Linux системах. **К сожалению, этот параметр подвержен изменениям, и пока не найдено надежного решения для Linux**.

### Windows

[WMIC](https://ss64.com/nt/wmic.html) - Windows Management Instrumentation Command.

> Инструмент WMIC [устарел](https://arc.net/l/quote/zgcodjij) в Windows 10, версии 21H1 и полугодовом канальном выпуске 21H1 для Windows Server.

Теперь все команды вызываются через PowerShell.

- ~~wmic csproduct get UUID~~ `powershell -command "Get-WmiObject Win32_ComputerSystemProduct"`: Возвращает уникальный идентификатор (UUID) продукта компьютера. Обычно связан с материнской платой компьютера. В редких случаях он может измениться после замены или переустановки материнской платы или после изменения BIOS/UEFI на устройстве.

- ~~wmic bios get serialnumber~~ `powershell -command "Get-WmiObject Win32_BIOS"`: Возвращает серийный номер BIOS компьютера. Обычно остается постоянным и не подлежит изменению.

- ~~wmic path win32_baseboard get serialnumber~~ `powershell -command "Get-WmiObject Win32_BaseBoard"`: Возвращает серийный номер базовой платы компьютера. Обычно остается постоянным и не подлежит изменению.

- ~~wmic cpu get processorid~~ `powershell -command "Get-WmiObject Win32_Processor"`: Возвращает идентификатор процессора компьютера. Должен оставаться неизменным, за исключением случаев замены процессора.

## Как установить

Добавить зависимость в Cargo.toml

```toml
[dependencies]
mid = "1.2.0"
```

Или установить с помощью Cargo CLI

```bash
cargo add mid
```

## Как использовать

Максимально просто

```rust
let machine_id = mid::get("mySecretKey").unwrap();
```

Использование в функции с обработкой ошибки

```rust
fn get_machine_id() -> Result<String, String> {
    match mid::get("mySecretKey") {
        Ok(mid) => Ok(mid),
        Err(err) => {
            println!("MID error: {}", err.to_string());
            Err(err.to_string())
        }
    }
}
```

Так же можно вывести в консоль MID result/hash

- `MID result` - массив параметров OS
- `MID hash` - хеш SHA-256 от result

```rust
mid::print("mySecretKey");
```

### Подписывайтесь на мой Х

Здесь я буду делиться своими разработками и проектами
https://x.com/doroved

### Ссылки

- [machineid-rs](https://github.com/Taptiive/machineid-rs)
- [machine_uuid](https://github.com/choicesourcing/machine_uuid)
- [rust-machine-id](https://github.com/mathstuf/rust-machine-id)
- [app-machine-id](https://github.com/d-k-bo/app-machine-id)
