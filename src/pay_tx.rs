use crate::{decrypt_private_key::decrypt_private_key, types::Result, utils::strip_hex_prefix};
use std::str::FromStr;

use rust_algorand::{AlgorandAddress, AlgorandGenesisId, AlgorandTransaction, MicroAlgos};

pub fn pay_tx(
    first_valid: u64,
    genesis_id: &str,
    fee: u64,
    path_to_pk: &str,
    receiver: &str,
    note: &str,
    amount: u64,
) -> Result<String> {
    let pk = decrypt_private_key(path_to_pk)?;
    let bytes = hex::decode(strip_hex_prefix(note))?;
    Ok(AlgorandTransaction::new_payment_tx(
        amount,
        MicroAlgos::new(fee),
        if bytes.is_empty() { None } else { Some(bytes) },
        first_valid,
        pk.to_address()?,
        AlgorandAddress::from_str(receiver)?,
        AlgorandGenesisId::from_str(genesis_id)?.hash()?,
        None,
    )?
    .sign(&pk)?
    .to_hex()?)
}
