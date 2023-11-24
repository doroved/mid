#[cfg(target_os = "windows")]
use crate::errors::MIDError;

#[cfg(target_os = "windows")]
use std::{process::Command, str};

#[cfg(target_os = "windows")]
pub(crate) fn get_mid_string() -> Result<String, MIDError> {
    let wmic_output = Command::new("wmic")
        .arg("csproduct")
        .arg("get")
        .arg("UUID")
        .output()?;

    let wmic_output_str = str::from_utf8(&wmic_output.stdout)?;
    let uuid = wmic_output_str.split('\n').nth(1).unwrap().to_lowercase();

    if uuid.trim().len() != 36 {
        return Err(MIDError::InvalidMachineIDLengthError);
    }

    println!("MID result: {}", uuid);

    Ok(uuid)
}
