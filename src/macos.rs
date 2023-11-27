#[cfg(target_os = "macos")]
use crate::errors::MIDError;

#[cfg(target_os = "macos")]
use crate::utils::parse_and_push;

#[cfg(target_os = "macos")]
use crate::utils::run_shell_comand;

#[cfg(target_os = "macos")]
pub(crate) fn get_mid_result() -> Result<String, MIDError> {
    let system_profiler_output = run_shell_comand(
        "sh",
        [
            "-c",
            r#"system_profiler SPHardwareDataType ; system_profiler SPSecureElementDataType"#,
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

    println!("MID result: {:?}", result);

    let combined_string = result.join("|");

    Ok(combined_string)
}
