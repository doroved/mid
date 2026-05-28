#![cfg(target_os = "linux")]

use std::collections::HashSet;

use crate::errors::MIDError;

pub(crate) fn get_mid_result() -> Result<String, MIDError> {
    let sources = [
        "/etc/machine-id",
        "/var/lib/dbus/machine-id",
        "/sys/class/dmi/id/product_uuid",
    ];

    let mut mid_result = HashSet::new();

    for path in &sources {
        if let Ok(content) = std::fs::read_to_string(path) {
            let clean = content.trim().to_lowercase().replace("-", "");
            if !clean.is_empty() {
                mid_result.insert(clean);
            }
        }
    }

    if mid_result.is_empty() {
        return Err(MIDError::ResultMidError);
    }

    let mut result: Vec<&str> = mid_result.iter().map(|s| s.as_str()).collect();
    result.sort();

    Ok(result.join("|"))
}
