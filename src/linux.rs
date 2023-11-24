#[cfg(target_os = "linux")]
use crate::errors::MIDError;

#[cfg(target_os = "linux")]
use std::{process::Command, str};

#[cfg(target_os = "linux")]
pub(crate) fn get_mid_string() -> Result<String, MIDError> {
    let mid_output = Command::new("cat").arg("/etc/machine-id").output()?;
    let mid_output_string = str::from_utf8(&mid_output.stdout)?;

    if mid_output_string.trim().len() != 32 {
        return Err(MIDError::InvalidMachineIDLengthError);
    }

    println!("MID result: {}", mid_output_string.to_lowercase());

    Ok(mid_output_string.to_lowercase())
}
