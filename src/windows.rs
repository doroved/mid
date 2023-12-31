#[cfg(target_os = "windows")]
use crate::errors::MIDError;

#[cfg(target_os = "windows")]
use crate::utils::run_shell_comand;

#[cfg(target_os = "windows")]
pub(crate) fn get_mid_result() -> Result<String, MIDError> {
    let csproduct_output = run_shell_comand(
        "powershell",
        [
            "-command",
            r#"Get-WmiObject Win32_ComputerSystemProduct | Select-Object -ExpandProperty UUID"#,
        ],
    )
    .unwrap_or("".into());

    let bios_output = run_shell_comand(
        "powershell",
        [
            "-command",
            r#"Get-WmiObject Win32_BIOS | Select-Object -ExpandProperty SerialNumber"#,
        ],
    )
    .unwrap_or("".into());

    let baseboard_output = run_shell_comand(
        "powershell",
        [
            "-command",
            r#"Get-WmiObject Win32_BaseBoard | Select-Object -ExpandProperty SerialNumber"#,
        ],
    )
    .unwrap_or("".into());

    let cpu_output = run_shell_comand(
        "powershell",
        [
            "-command",
            r#"Get-WmiObject Win32_Processor | Select-Object -ExpandProperty ProcessorId"#,
        ],
    )
    .unwrap_or("".into());

    let mut result = Vec::new();

    parse_and_push(&csproduct_output, &mut result);
    parse_and_push(&bios_output, &mut result);
    parse_and_push(&baseboard_output, &mut result);
    parse_and_push(&cpu_output, &mut result);

    if result.is_empty() {
        return Err(MIDError::ResultMidError);
    }

    println!("MID result: {:?}", result);

    let combined_string = result.join("|");

    Ok(combined_string)
}

#[cfg(target_os = "windows")]
fn parse_and_push(output_str: &str, result: &mut Vec<String>) {
    let trimmed_lower = output_str.trim().to_lowercase();

    if !trimmed_lower.is_empty() {
        result.push(trimmed_lower);
    }
}
