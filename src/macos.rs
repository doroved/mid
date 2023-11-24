#[cfg(target_os = "macos")]
use crate::errors::MIDError;

#[cfg(target_os = "macos")]
use std::{process::Command, str};

#[cfg(target_os = "macos")]
pub(crate) fn get_mid_string() -> Result<String, MIDError> {
    let system_profiler_output = Command::new("system_profiler")
        .arg("SPHardwareDataType")
        .output()?;

    let system_profiler_output_str = str::from_utf8(&system_profiler_output.stdout)?;

    let targets = [
        "Model Number",
        "Chip",
        "Cores",
        "Memory",
        "Serial Number",
        "Hardware UUID",
        "Provisioning UDID",
    ];

    let mut result = Vec::new();

    parse_and_push(&system_profiler_output_str, &targets, &mut result);

    if result.len() != targets.len() {
        return Err(MIDError::ResultLengthMismatchError);
    }

    println!("MID result: {:?}", result);

    let combined_string = result.join(",");

    Ok(combined_string)
}

#[cfg(target_os = "macos")]
fn parse_and_push(output_str: &str, targets: &[&str], result: &mut Vec<String>) {
    let lines: Vec<&str> = output_str.lines().collect();

    for target in targets {
        let target = target.to_lowercase();

        for line in &lines {
            let line = line.to_lowercase();

            if line.contains(&target) {
                let parts: Vec<&str> = line.split(":").collect();
                let value = parts[1].trim().to_string();

                if target == "memory" || target == "cores" {
                    let parts: Vec<&str> = value.split_whitespace().collect();

                    if let Some(memory_or_cores) = parts.get(0) {
                        result.push(memory_or_cores.to_string());
                    }
                } else {
                    result.push(value);
                }
            }
        }
    }
}
