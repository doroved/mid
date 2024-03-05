#[cfg(target_os = "macos")]
use crate::errors::MIDError;

#[cfg(target_os = "macos")]
use crate::utils::run_shell_comand;

#[cfg(target_os = "macos")]
pub(crate) fn get_mid_result() -> Result<String, MIDError> {
    let system_profiler_output = run_shell_comand(
        "sh",
        [
            "-c",
            r#"system_profiler SPHardwareDataType SPSecureElementDataType"#,
        ],
    )?;

    let targets = [
        "Model Number",
        "Serial Number",
        "Hardware UUID",
        "Provisioning UDID",
        "Platform ID",
        "SEID",
    ];

    let mut result = Vec::new();

    parse_and_push(&system_profiler_output, &targets, &mut result);

    if result.is_empty() {
        return Err(MIDError::ResultMidError);
    }

    let combined_string = result.join("|");

    Ok(combined_string)
}

#[cfg(target_os = "macos")]
fn parse_and_push(output_str: &str, targets: &[&str], result: &mut Vec<String>) {
    let lines: Vec<&str> = output_str.lines().collect();

    for target in targets {
        for line in &lines {
            let line = line.to_lowercase();

            if line.contains(&target.to_lowercase()) {
                let parts: Vec<&str> = line.split(":").collect();

                if parts.len() == 2 {
                    let value = parts[1].trim().to_string();

                    if !value.is_empty() {
                        result.push(value);
                    }
                }
            }
        }
    }
}
