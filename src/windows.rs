#[cfg(target_os = "windows")]
use crate::errors::MIDError;

#[cfg(target_os = "windows")]
use crate::utils::run_shell_comand;

#[cfg(target_os = "windows")]
pub(crate) fn get_mid_result() -> Result<String, MIDError> {
    let csproduct_output =
        run_shell_comand("wmic", ["csproduct", "get", "UUID"]).unwrap_or("".to_string());
    let diskdrive_output =
        run_shell_comand("wmic", ["diskdrive", "get", "serialnumber"]).unwrap_or("".to_string());
    let bios_output =
        run_shell_comand("wmic", ["bios", "get", "serialnumber"]).unwrap_or("".to_string());
    let baseboard_output =
        run_shell_comand("wmic", ["path", "win32_baseboard", "get", "serialnumber"])
            .unwrap_or("".to_string());
    let os_output =
        run_shell_comand("wmic", ["os", "get", "serialnumber"]).unwrap_or("".to_string());
    let cpu_output =
        run_shell_comand("wmic", ["cpu", "get", "processorid"]).unwrap_or("".to_string());
    let memorychip_output =
        run_shell_comand("wmic", ["memorychip", "get", "serialnumber"]).unwrap_or("".to_string());
    let original_product_key_output = run_shell_comand(
        "wmic",
        [
            "path",
            "softwarelicensingservice",
            "get",
            "OA3xOriginalProductKey",
        ],
    )
    .unwrap_or("".to_string());
    let machine_guid_output = run_shell_comand("cmd", [
        "/C",
        r#"for /f "skip=2 tokens=3" %a in ('reg query HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Cryptography /v MachineGuid') do @echo %a"#
    ]).unwrap_or("".to_string());

    let mut result = Vec::new();

    let csproduct_uuid = csproduct_output
        .split('\n')
        .nth(1)
        .unwrap()
        .trim()
        .to_lowercase();
    let diskdrive = diskdrive_output
        .split('\n')
        .nth(1)
        .unwrap()
        .trim()
        .to_lowercase();
    let bios = bios_output
        .split('\n')
        .nth(1)
        .unwrap()
        .trim()
        .to_lowercase();
    let baseboard = baseboard_output
        .split('\n')
        .nth(1)
        .unwrap()
        .trim()
        .to_lowercase();
    let os = os_output.split('\n').nth(1).unwrap().trim().to_lowercase();
    let cpu = cpu_output.split('\n').nth(1).unwrap().trim().to_lowercase();
    let original_product_key = original_product_key_output
        .split('\n')
        .nth(1)
        .unwrap()
        .trim()
        .to_lowercase();

    result.push(csproduct_uuid);
    result.push(diskdrive);
    result.push(bios);
    result.push(baseboard);
    result.push(os);
    result.push(cpu);
    result.push(memorychip_output);
    result.push(original_product_key);
    result.push(machine_guid_output);

    // if csproduct_uuid.len() != 36 {
    //     return Err(MIDError::InvalidMachineIDLengthError);
    // }

    println!("MID result: {:?}", result);

    let combined_string = result.join("|");

    Ok(combined_string)
}
