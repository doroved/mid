#[cfg(target_os = "linux")]
use crate::errors::MIDError;

#[cfg(target_os = "linux")]
use crate::utils::run_shell_comand;

#[cfg(target_os = "linux")]
pub(crate) fn get_mid_result() -> Result<String, MIDError> {
    let machine_output = run_shell_comand("cat", ["/etc/machine-id"])?;
    let machine_id = machine_output.trim().to_lowercase();

    println!("MID result: {}", machine_id);

    Ok(machine_id)
}
