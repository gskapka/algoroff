use crate::{decrypt_private_key::decrypt_private_key, types::Result};
use rust_algorand::{AlgorandAddress, AlgorandHash, AlgorandTransaction, MicroAlgos};
use std::str::FromStr;

pub fn asset_transfer_tx(
    amount: u64,
    id: u64,
    receiver: &str,
    first_valid: u64,
    genesis_hash: &str,
    fee: u64,
    path_to_pk: &str,
) -> Result<String> {
    let pk = decrypt_private_key(path_to_pk)?;
    Ok(AlgorandTransaction::asset_transfer(
        id,
        MicroAlgos::new(fee),
        amount,
        None,
        first_valid,
        pk.to_address()?,
        AlgorandHash::from_str(genesis_hash)?,
        None,
        AlgorandAddress::from_str(receiver)?,
    )?
    .sign(&pk)?
    .to_hex()?)
}
