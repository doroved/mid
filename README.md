[![Crates.io Version](https://img.shields.io/crates/v/mid)](https://crates.io/crates/mid)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/mid?style=flat&color=white)](https://crates.io/crates/mid)
[![docs.rs](https://img.shields.io/docsrs/mid?style=flat&color=orange)](https://docs.rs/mid)

**Made with ❤️ for [Tauri](https://tauri.app)**

[README RU](./README.ru.md)

## Latest Change Log

**v5.0.1** - January 21, 2026

- Excluded image from crate to reduce package size.

**v5.0.0** - January 21, 2026

> [!IMPORTANT]
> The machine ID for Linux will change in this version. **Use this version only for new projects or update the device hashes of your current users.**

- Added support for iOS.
- Added an additional source for obtaining the identifier in Linux via `/sys/class/dmi/id/product_uuid`.

[Full Change Log](./CHANGELOG.md)

# mid

Creating a Machine ID hash.

Utilizes the most static system parameters possible to generate reliable device hashes for licensing your software.

## Supported Platforms

- [x] MacOS
- [x] Windows
- [x] Linux
- [x] iOS

List of parameters that are used on each platform.

## MacOS

```bash
system_profiler SPHardwareDataType
```

The [command](https://ss64.com/osx/system_profiler.html) returns information about the computer's hardware characteristics. Parameters used:

- **Model Number**: This parameter represents the computer or device model number. It is used for uniquely identifying a specific model within the manufacturer's range.

- **Serial Number**: This parameter is the unique serial number of the computer or device. It is used to identify a specific unit within a particular model.

- **Hardware UUID**: This parameter represents the hardware UUID of the computer or device. It serves to provide unique identification of a specific unit across different systems and environments.

- ~~**Provisioning UDID**: This parameter represents the device's unique device identifier (UDID), which can be used in the provisioning process or device setup, usually in a corporate or managed environment.~~

```bash
system_profiler SPSecureElementDataType
```

The command returns information about the Secure Element. This element is used to store encrypted data, such as information about payment cards and other confidential data. Parameters used:

- ~~**Platform ID**: The unique identifier of the platform to which the Secure Element belongs.~~
- **SEID**: The unique identifier of the Secure Element. Created during the NFC chip firmware at the manufacturer's factory.

## Windows

[PowerShell](https://en.wikipedia.org/wiki/PowerShell) - expandable automation tool. Parameters used:

- `powershell -command "Get-WmiObject Win32_ComputerSystemProduct"`: Returns the unique product identifier (UUID) of the computer. Usually associated with the computer's motherboard. In rare cases, it may change after replacing or reinstalling the motherboard or after changing the device's BIOS/UEFI.

- `powershell -command "Get-WmiObject Win32_BIOS"`: Returns the computer's BIOS serial number. It usually remains constant and does not change.

- `powershell -command "Get-WmiObject Win32_BaseBoard"`: Returns the serial number of the computer's baseboard. It usually remains constant and does not change.

- `powershell -command "Get-WmiObject Win32_Processor"`: Returns the computer's processor identifier. It should remain unchanged, except in cases of processor replacement.

## Linux

- [machine-id](https://man7.org/linux/man-pages/man5/machine-id.5.html): A machine identifier (ID) that is used to uniquely identify a computer on Linux systems.

> **Unfortunately this parameter is subject to user modification and no reliable solution for Linux has been found yet.**

## iOS

- **Keychain**: The identifier is generated randomly (SHA-256 of Nanoid + Timestamp) and stored in the Keychain. It persists across app reinstallations on real devices.

[iOS Instructions](./IOS_INSTRUCTIONS.md)

## Installation

Add the dependency to Cargo.toml

```toml
[dependencies]
mid = "5.0.1"
```

Or install using Cargo CLI

```bash
cargo add mid
```

## How to Use

### Get machine ID hash

```rust
let machine_id = mid::get("mySecretKey").unwrap();
```

```
Example: 3f9af06fd78d3390ef35e059623f58af03b7f6ca91690f5af031b774fd541977
```

### Get MID key/result/hash data

```rust
let mid_data = mid::data("mySecretKey").unwrap();
```

```
MacOS example: MidData { key: "mySecretKey", result: ["ModelNumber", "SerialNumber", "HardwareUUID", "SEID"], hash: "3f9af06fd78d3390ef35e059623f58af03b7f6ca91690f5af031b774fd541977" }
```

### Output the MID key/result/hash to the console in `debug_assertions` mode

```rust
mid::print("mySecretKey");
```

```
MacOS example:
MID.print[key]: mySecretKey
MID.print[result]: ["ModelNumber", "SerialNumber", "HardwareUUID", "SEID"]
MID.print[hash]: 3f9af06fd78d3390ef35e059623f58af03b7f6ca91690f5af031b774fd541977
```

- `MID key` - The secret key for hashing
- `MID result` - Array of OS parameters
- `MID hash` - SHA-256 hash from result

### Get additional device data

This data does not contribute to the device hash. Currently available for `MacOS` only.

```rust
let additional_data = mid::additional_data().unwrap();
println!("{:?}", additional_data);
```

```
AdditionalData { username: "doroved", hostname: "MacBook-Pro--doroved.local", model_name: "MacBook Pro", os_name: "Sequoia", os_version: "15.7", os_full: "Sequoia 15.7", chip: "Apple M1 Pro", chip_short: "m1 pro", memsize: 16, cpu_core_count: 8, languages: ["ru-RU", "bg-RU", "en-RU"] }
```

## Subscribe to my X

Here I will share my developments and projects
https://x.com/doroved

## References

- [machineid-rs](https://github.com/Taptiive/machineid-rs)
- [machine_uuid](https://github.com/choicesourcing/machine_uuid)
- [rust-machine-id](https://github.com/mathstuf/rust-machine-id)
- [app-machine-id](https://github.com/d-k-bo/app-machine-id)
