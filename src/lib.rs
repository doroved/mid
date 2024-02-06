//! Creating a Machine ID for MacOS/Linux/Windows.
//!
//! ```
//! let machine_id = mid::get("mykey").unwrap();
//! ```

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

/// Gets unique platform metrics and returns a `Result`, which can be a MID hash (SHA-256) or a `MIDError`.
///
/// # Errors
///
/// Returns [`Err`] if an error occurred while creating the MachineID.
///
/// # Examples
///
/// ```
/// fn get_machine_id() -> Result<String, String> {
///   // Generate a key on macOS/Linux with: openssl rand -hex 32
///   let key = "293273abaf6fcb31d4a9b47b70a20b21133ff08852834c52c5f42ed8153b274a";
///
///   match mid::get(key) {
///       Ok(mid) => Ok(mid),
///       Err(err) => {
///           println!("MID error: {}", err.to_string());
///           Err(err.to_string())
///       }
///   }
/// }
/// ```
pub fn get(key: &str) -> Result<String, MIDError> {
    match get_mid_result() {
        Ok(mid) => {
            let mid_bytes = mid.as_bytes();

            let hmac_key = hmac::Key::new(hmac::HMAC_SHA256, key.as_bytes());
            let signature = hmac::sign(&hmac_key, mid_bytes);
            let signature_hex = hex::encode(signature.as_ref());

            Ok(signature_hex)
        }
        Err(err) => Err(err),
    }
}

/// Display MID result/hash in the console.
///
/// `MID result` - array of OS parameters
///
/// `MID hash` - SHA-256 hash from result
pub fn print_mid(key: &str) {
    match get_mid_result() {
        Ok(mid) => {
            let mid_result: Vec<String> = mid.split('|').map(|s| s.to_string()).collect();
            let mid_bytes = mid.as_bytes();

            let hmac_key = hmac::Key::new(hmac::HMAC_SHA256, key.as_bytes());
            let signature = hmac::sign(&hmac_key, mid_bytes);
            let mid_hash = hex::encode(signature.as_ref());

            println!("MID result: {:?}", mid_result);
            println!("MID hash: {}", mid_hash);
        }
        Err(_) => {}
    }
}

#[test]
fn mid_info() {
    match get("mykey") {
        Ok(_) => print_mid("mykey"),
        Err(err) => println!("MID error: {}", err.to_string()),
    }
}
