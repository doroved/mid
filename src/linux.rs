#![cfg(target_os = "linux")]

use std::collections::HashSet;
use crate::errors::MIDError;
use crate::utils::run_shell_command;

pub(crate) fn get_mid_result() -> Result<String, MIDError> {
    let machine_output = run_shell_command(
        "sh",
        [
            "-c",
            r#"hostnamectl status | awk '/Machine ID:/ {print $3}'; cat /var/lib/dbus/machine-id || true; cat /etc/machine-id || true; cat /sys/class/dmi/id/product_uuid || true"#,
        ],
    )?;

    let combined_string = process_output(&machine_output);

    if combined_string.is_empty() {
        return Err(MIDError::ResultMidError);
    }

    Ok(combined_string)
}

fn process_output(output_str: &str) -> String {
    let mut mid_result = HashSet::new();

    for line in output_str.to_lowercase().lines() {
        let clean_line = line.trim().replace("-", "");
        if !clean_line.is_empty() {
            mid_result.insert(clean_line);
        }
    }

    let mut result: Vec<&str> = mid_result.iter().map(|s| s.as_str()).collect();
    result.sort();

    result.join("|").trim().to_string()
}
