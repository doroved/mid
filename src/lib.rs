mod errors;
mod linux;
mod macos;
mod windows;

use errors::MIDError;
use ring::hmac;

#[cfg(target_os = "linux")]
use linux::get_mid_string;
#[cfg(target_os = "macos")]
use macos::get_mid_string;
#[cfg(target_os = "windows")]
use windows::get_mid_string;

pub fn get(key: &str) -> Result<String, MIDError> {
    match get_mid_string() {
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

// cargo test -- --nocapture
#[test]
fn machineid() {
    match get("mykey") {
        Ok(_) => {}
        Err(err) => println!("MID error: {}", err.to_string()),
    }
}
