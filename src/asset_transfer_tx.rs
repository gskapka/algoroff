use crate::{decrypt_private_key::decrypt_private_key, types::Result, utils::strip_hex_prefix};
use rust_algorand::{AlgorandAddress, AlgorandGenesisId, AlgorandTransaction, MicroAlgos};
use std::str::FromStr;

pub fn asset_transfer_tx(
    amount: u64,
    id: u64,
    receiver: &str,
    first_valid: u64,
    genesis_id: &str,
    fee: u64,
    path_to_pk: &str,
    note: &str,
) -> Result<String> {
    let pk = decrypt_private_key(path_to_pk)?;
    let bytes = hex::decode(strip_hex_prefix(note))?;
    Ok(AlgorandTransaction::asset_transfer(
        id,
        MicroAlgos::new(fee),
        amount,
        if bytes.is_empty() { None } else { Some(bytes) },
        first_valid,
        pk.to_address()?,
        AlgorandGenesisId::from_str(genesis_id)?.hash()?,
        None,
        AlgorandAddress::from_str(receiver)?,
    )?
    .sign(&pk)?
    .to_hex()?)
}
