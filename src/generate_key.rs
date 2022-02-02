use crate::types::Result;
use rust_algorand::AlgorandKeys;
use serde_json::json;

pub fn generate_key() -> Result<String> {
    let key = AlgorandKeys::create_random();
    Ok(
        json!({"key": hex::encode(&key.to_bytes()), "address": format!("{}", key.to_address()?)})
            .to_string(),
    )
}
