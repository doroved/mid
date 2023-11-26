mod errors;
mod linux;
mod macos;
mod utils;
mod windows;

use errors::MIDError;
use ring::hmac;

#[cfg(target_os = "linux")]
use linux::get_mid_result;
#[cfg(target_os = "macos")]
use macos::get_mid_result;
#[cfg(target_os = "windows")]
use windows::get_mid_result;

/// Retrieves raw format of UUID retrieval.
///
/// # Examples
///
/// ```
/// #[cfg(any(macos))]
/// let untransformed = machine_uuid::get_via_windows_shell();
///
/// // c:\ wmic csproduct get UUID
/// // UUID
/// // 140EF834-2DB3-0F7A-27B4-4CEDFB73167C
///
/// ```
pub fn get(key: &str) -> Result<String, MIDError> {
    match get_mid_result() {
        Ok(mid) => {
            let mid_bytes = mid.as_bytes();

            let hmac_key = hmac::Key::new(hmac::HMAC_SHA256, key.as_bytes());
            let signature = hmac::sign(&hmac_key, mid_bytes);
            let signature_hex = hex::encode(signature.as_ref());

            println!("MID hash: {}", signature_hex);

            Ok(signature_hex)
        }
        Err(err) => Err(err),
    }
}

#[test]
fn mid_info() {
    match get("mykey") {
        Ok(_) => {}
        Err(err) => println!("MID error: {}", err.to_string()),
    }
}
