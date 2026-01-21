#![cfg(target_os = "ios")]

use crate::errors::MIDError;
use nanoid::nanoid;
use security_framework::passwords::{get_generic_password, set_generic_password};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

const ACCOUNT_NAME: &str = "MID";

pub fn get_mid_result(service_name: &str) -> Result<String, MIDError> {
    match get_generic_password(service_name, ACCOUNT_NAME) {
        Ok(data) => String::from_utf8(data).map_err(MIDError::ParseError),
        Err(_) => generate_and_save(service_name),
    }
}

fn generate_and_save(service_name: &str) -> Result<String, MIDError> {
    let nid = nanoid!();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let combined = format!("{}{}", nid, timestamp);

    let mut hasher = Sha256::new();
    hasher.update(combined);
    let hash_result = hasher.finalize();
    let mid_hash = hex::encode(hash_result);

    match set_generic_password(service_name, ACCOUNT_NAME, mid_hash.as_bytes()) {
        Ok(_) => Ok(mid_hash),
        Err(_) => Err(MIDError::ResultMidError),
    }
}
