<a href="https://crates.io/crates/mid"><img src="https://img.shields.io/crates/v/mid?style=for-the-badge&logo=rust&color=orange" /></a>
<a href="https://docs.rs/mid/latest/mid/">
<img src="https://img.shields.io/badge/docs-latest-blue.svg?style=for-the-badge&logo=rust&color=blue"
      alt="docs.rs docs" />
</a>

[README RU](./README_RU.md)

## MachineID for Rust

Obtain a unique hashed identifier for your MacOS/Linux/Windows-based device.

An excellent solution for licensing your programs, which uses maximally static and mostly immutable data.

Next, get acquainted with the parameters we use on each platform.

### MacOS

```bash
system_profiler SPHardwareDataType
```

The command returns information about the computer's hardware characteristics. Parameters used:

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

### Linux

- `cat /etc/machine-id`: Returns the machine identifier (ID) used for unique identification of the computer in Linux systems. Unfortunately, this parameter is subject to change, and a reliable solution for Linux has not been found yet.

### Windows

- `wmic csproduct get UUID`: Returns the unique product identifier (UUID) of the computer. Usually associated with the computer's motherboard. In rare cases, it may change after replacing or reinstalling the motherboard or after changing the device's BIOS/UEFI.

- `wmic bios get serialnumber`: Returns the computer's BIOS serial number. It usually remains constant and does not change.

- `wmic path win32_baseboard get serialnumber`: Returns the serial number of the computer's baseboard. It usually remains constant and does not change.

- `wmic cpu get processorid`: Returns the computer's processor identifier. It should remain unchanged, except in cases of processor replacement.

## Installation

Add the dependency to Cargo.toml

```toml
[dependencies]
mid = "1.0.0"
```

Or install using Cargo CLI

```bash
cargo add mid
```

## How to Use

As simple as it gets

```rust
let machine_id = mid::get("mykey").unwrap();
```

Usage in a function with error handling

```rust
fn get_machine_id() -> Result<String, String> {
    // openssl rand -hex 32
    let key = "293273abaf6fcb31d4a9b47b70a20b21133ff08852834c52c5f42ed8153b274a";

    match mid::get(key) {
        Ok(mid) => Ok(mid),
        Err(err) => {
            println!("MID error: {}", err.to_string());
            Err(err.to_string())
        }
    }
}
```

### Subscribe to my X

Here I will share my developments and projects
https://x.com/doroved

### Links

- [machineid-rs](https://github.com/Taptiive/machineid-rs)
- [machine_uuid](https://github.com/choicesourcing/machine_uuid)
- [rust-machine-id](https://github.com/mathstuf/rust-machine-id)
- [app-machine-id](https://github.com/d-k-bo/app-machine-id)
