use crate::MIDError;
use std::{ffi::OsStr, process::Command};

pub(crate) fn run_shell_comand<S, I>(shell: S, args: I) -> Result<String, MIDError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new(shell)
        .args(args)
        .output()
        .map_err(MIDError::ExecuteProcessError)
        .and_then(|output| String::from_utf8(output.stdout).map_err(MIDError::ParseError))
}

pub(crate) fn parse_and_push(output_str: &str, targets: &[&str], result: &mut Vec<String>) {
    let lines: Vec<&str> = output_str.lines().collect();

    for target in targets {
        for line in &lines {
            let line = line.to_lowercase();

            if line.contains(&target.to_lowercase()) {
                let parts: Vec<&str> = line.split(":").collect();
                let value = parts[1].trim().to_string();

                if !value.is_empty() {
                    result.push(value);
                }
            }
        }
    }
}
