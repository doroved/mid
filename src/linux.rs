#[cfg(target_os = "linux")]
use crate::errors::MIDError;

#[cfg(target_os = "linux")]
use crate::utils::run_shell_comand;

#[cfg(target_os = "linux")]
pub(crate) fn get_mid_result() -> Result<String, MIDError> {
    let machine_output = run_shell_comand("cat", ["/etc/machine-id"])?;

    if machine_output.trim().len() != 32 {
        return Err(MIDError::InvalidMachineIDLengthError);
    }

    println!("MID result: {}", machine_output.to_lowercase());

    Ok(machine_output.to_lowercase())
}
