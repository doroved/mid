use smbioslib::*;

use crate::errors::MIDError;

pub(crate) fn get_mid_result() -> Result<String, MIDError> {
    let data = table_load_from_device().map_err(|_| MIDError::ResultMidError)?;

    // UUID — SMBIOS Type 1 (System Information)
    // Equivalent to: Get-WmiObject Win32_ComputerSystemProduct | Select-Object -ExpandProperty UUID
    let uuid = data
        .find_map(|si: SMBiosSystemInformation| match si.uuid() {
            Some(SystemUuidData::Uuid(uuid)) => Some(uuid.to_string()),
            _ => None,
        })
        .unwrap_or_default();

    // BIOS Serial Number — SMBIOS Type 1 (System Information)
    // Equivalent to: Get-WmiObject Win32_BIOS | Select-Object -ExpandProperty SerialNumber
    let bios_serial = data
        .find_map(|si: SMBiosSystemInformation| {
            let s = si.serial_number().to_string();
            if s.is_empty() { None } else { Some(s) }
        })
        .unwrap_or_default();

    // Baseboard Serial Number — SMBIOS Type 2 (Baseboard Information)
    // Equivalent to: Get-WmiObject Win32_BaseBoard | Select-Object -ExpandProperty SerialNumber
    let baseboard_serial = data
        .find_map(|bi: SMBiosBaseboardInformation| {
            let s = bi.serial_number().to_string();
            if s.is_empty() { None } else { Some(s) }
        })
        .unwrap_or_default();

    // Processor ID — SMBIOS Type 4 (Processor Information)
    // Equivalent to: Get-WmiObject Win32_Processor | Select-Object -ExpandProperty ProcessorId
    let processor_id = data
        .find_map(|pi: SMBiosProcessorInformation| {
            pi.processor_id().map(|id| {
                // WMI formats processor_id as a 16-char uppercase hex string from 8 bytes (little-endian)
                // We convert to the same format, then lowercase to match the original behavior
                id.iter()
                    .rev()
                    .map(|b| format!("{:02X}", b))
                    .collect::<String>()
            })
        })
        .unwrap_or_default();

    let combined = format!(
        "{}|{}|{}|{}",
        uuid, bios_serial, baseboard_serial, processor_id
    );

    let result = combined
        .trim_start_matches('|')
        .trim_end_matches('|')
        .to_lowercase();

    if result.is_empty() {
        return Err(MIDError::ResultMidError);
    }

    Ok(result)
}
