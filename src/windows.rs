#[cfg(target_os = "windows")]
use crate::errors::MIDError;

#[cfg(target_os = "windows")]
use crate::utils::run_shell_comand;

#[cfg(target_os = "windows")]
pub(crate) fn get_mid_result() -> Result<String, MIDError> {
    let csproduct_output =
        run_shell_comand("wmic", ["csproduct", "get", "UUID"]).unwrap_or("".into());
    let diskdrive_output =
        run_shell_comand("wmic", ["diskdrive", "get", "serialnumber"]).unwrap_or("".into());
    let bios_output =
        run_shell_comand("wmic", ["bios", "get", "serialnumber"]).unwrap_or("".into());
    let baseboard_output =
        run_shell_comand("wmic", ["path", "win32_baseboard", "get", "serialnumber"])
            .unwrap_or("".into());
    let cpu_output = run_shell_comand("wmic", ["cpu", "get", "processorid"]).unwrap_or("".into());

    let mut result = Vec::new();

    parse_and_push(&csproduct_output, &mut result);
    parse_and_push(&diskdrive_output, &mut result);
    parse_and_push(&bios_output, &mut result);
    parse_and_push(&baseboard_output, &mut result);
    parse_and_push(&cpu_output, &mut result);

    if result.is_empty() {
        return Err(MIDError::ResultWindowsMidError);
    }

    println!("MID result: {:?}", result);

    let combined_string = result.join("|");
    println!("combined_string: {}", combined_string);

    Ok(combined_string)
}

#[cfg(target_os = "windows")]
fn parse_and_push(output_str: &str, result: &mut Vec<String>) {
    if let Some(second_line) = output_str.lines().nth(1) {
        let trimmed_lower = second_line.trim().to_lowercase();
        result.push(trimmed_lower);

        // if !trimmed_lower.is_empty() {
        //     result.push(trimmed_lower);
        // }
    }
}
