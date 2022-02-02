use crate::{decrypt_private_key::decrypt_private_key, types::Result};

pub fn show_address(path: &str) -> Result<String> {
    decrypt_private_key(path).and_then(|pk| Ok(pk.to_address()?.to_string()))
}
