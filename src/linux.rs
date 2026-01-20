#[cfg(target_os = "linux")]
use std::collections::HashSet;
#[cfg(target_os = "linux")]
use std::fs;

#[cfg(target_os = "linux")]
use crate::errors::MIDError;

#[cfg(target_os = "linux")]
use crate::utils::run_shell_command;

#[cfg(target_os = "linux")]
pub(crate) fn get_mid_result() -> Result<String, MIDError> {
    let hostnamectl_status = run_shell_command("hostnamectl", ["status"])
        .ok()
        .and_then(parse_mid);

    let dbus_mid = fs::read_to_string("/var/lib/dbus/machine-id").ok();
    let etc_mid = fs::read_to_string("/etc/machine-id").ok();

    let combined_string = process_output([hostnamectl_status, dbus_mid, etc_mid]);

    if combined_string.is_empty() {
        return Err(MIDError::ResultMidError);
    }

    Ok(combined_string)
}

#[cfg(target_os = "linux")]
fn process_output(output: [Option<String>; 3]) -> String {
    let mut md5_hashes = HashSet::new();

    for hash in output.into_iter().flatten() {
        if hash.len() == 32 && hash.chars().all(|c| c.is_ascii_hexdigit()) {
            md5_hashes.insert(hash);
        }
    }

    md5_hashes
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join("|")
        .trim()
        .to_string()
}

#[cfg(target_os = "linux")]
fn parse_mid(hostnamectl_status: String) -> Option<String> {
    const PATTERN: &str = "Machine ID: ";

    let pos = hostnamectl_status.find(PATTERN)?;
    let after_pattern = hostnamectl_status.split_at(pos + PATTERN.len()).1;

    Some(after_pattern.lines().next()?.to_owned())
}
