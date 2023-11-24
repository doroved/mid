use ring::hmac;
use std::{process::Command, str};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MIDError {
    #[error("Command 'system_profiler' not found")]
    CommandError(#[from] std::io::Error),

    #[error("Error converting system_profiler output to UTF-8")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Result length does not match target length")]
    ResultLengthMismatchError,
}

pub fn get(key: &str) -> Result<String, MIDError> {
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

    println!("MachineID result: {:?}", result);

    if result.len() != targets.len() {
        return Err(MIDError::ResultLengthMismatchError);
    }

    let combined_string = result.join(",");
    let combined_bytes = combined_string.as_bytes();

    let hmac_key = hmac::Key::new(hmac::HMAC_SHA256, key.as_bytes());
    let signature = hmac::sign(&hmac_key, combined_bytes);
    let signature_hex = hex::encode(signature.as_ref());

    Ok(signature_hex)
}

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
