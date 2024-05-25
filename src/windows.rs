#[cfg(target_os = "windows")]
use wmi::{COMLibrary, WMIConnection};

#[cfg(target_os = "windows")]
use serde::Deserialize;

#[cfg(target_os = "windows")]
use crate::errors::MIDError;

#[cfg(target_os = "windows")]
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct UUIDGeneric {
    UUID: String,
}

#[cfg(target_os = "windows")]
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct SerialNumberGeneric {
    SerialNumber: String,
}

#[cfg(target_os = "windows")]
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct ProcessorIdGeneric {
    ProcessorId: String,
}

#[cfg(target_os = "windows")]
pub fn get_mid_result() -> Result<String, MIDError> {
    let com_connection = unsafe { COMLibrary::assume_initialized() };
    let wmi_connection =
        WMIConnection::new(com_connection.into()).expect("Failed to connect to WMI");

    let computer_uuid_base: Vec<UUIDGeneric> = wmi_connection
        .raw_query("SELECT UUID from Win32_ComputerSystemProduct WHERE UUID IS NOT NULL")
        .unwrap();

    let bios_serial_base: Vec<SerialNumberGeneric> = wmi_connection
        .raw_query("SELECT SerialNumber from Win32_BIOS WHERE SerialNumber IS NOT NULL")
        .unwrap();

    let baseboard_serial_base: Vec<SerialNumberGeneric> = wmi_connection
        .raw_query("SELECT SerialNumber from Win32_BaseBoard WHERE SerialNumber IS NOT NULL")
        .unwrap();

    let processor_id_base: Vec<ProcessorIdGeneric> = wmi_connection
        .raw_query("SELECT ProcessorId from Win32_Processor WHERE ProcessorId IS NOT NULL")
        .unwrap();

    let mut result: Vec<String> = Vec::new();

    result.push(computer_uuid_base[0].UUID.to_string());
    result.push(bios_serial_base[0].SerialNumber.to_string());
    result.push(baseboard_serial_base[0].SerialNumber.to_string());
    result.push(processor_id_base[0].ProcessorId.to_string());

    if result.is_empty() {
        return Err(MIDError::ResultMidError);
    }

    let combined_string = result.join("|");

    println!("windows:result {:?}", result);
    println!("windows:combined_string {}", combined_string);

    Ok(combined_string)
}
