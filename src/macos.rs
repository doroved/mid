#[cfg(target_os = "macos")]
use crate::errors::MIDError;

#[cfg(target_os = "macos")]
use crate::utils::run_shell_comand;

#[cfg(target_os = "macos")]
pub(crate) fn get_mid_result() -> Result<String, MIDError> {
    let system_profiler_output = run_shell_comand("system_profiler", ["SPHardwareDataType"])?;

    let targets = [
        "Model Name",
        "Model Identifier",
        "Model Number",
        "Processor Name",
        "Processor Speed",
        "Number of Processors",
        "Chip",
        "Cores",
        "Memory",
        "Serial Number",
        "Hardware UUID",
        "Provisioning UDID",
    ];

    let mut result = Vec::new();

    parse_and_push(&system_profiler_output, &targets, &mut result);

    println!("MID result: {:?}", result);

    let combined_string = result.join("|");

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

                if target == "memory" || target == "cores" || target.contains("speed") {
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
