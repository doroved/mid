#[cfg(target_os = "linux")]
use crate::errors::MIDError;

#[cfg(target_os = "linux")]
use crate::utils::parse_and_push;

#[cfg(target_os = "linux")]
use crate::utils::run_shell_comand;

#[cfg(target_os = "linux")]
pub(crate) fn get_mid_result() -> Result<String, MIDError> {
    let machine_output = run_shell_comand("cat", ["/etc/machine-id"])?;
    let machine_id = machine_output.trim().to_lowercase();

    let dmidecode_output = run_shell_comand("sudo dmidecode", ["-t", "system"])?;

    let targets = ["Serial Number", "UUID"];

    let mut result = Vec::new();

    result.push(machine_id);
    parse_and_push(&dmidecode_output, &targets, &mut result);

    if result.is_empty() {
        return Err(MIDError::ResultMidError);
    }

    println!("MID result: {:?}", result);

    let combined_string = result.join("|");

    Ok(combined_string)
}
