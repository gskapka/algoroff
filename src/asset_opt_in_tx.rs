use crate::{decrypt_private_key::decrypt_private_key, types::Result};
use std::str::FromStr;

use rust_algorand::{AlgorandGenesisId, AlgorandTransaction, MicroAlgos};

pub fn asset_opt_in_tx(
    asset_id: u64,
    fee: u64,
    first_valid: u64,
    genesis_id: &str,
    path_to_pk: &str,
) -> Result<String> {
    let pk = decrypt_private_key(path_to_pk)?;
    Ok(AlgorandTransaction::asset_opt_in(
        asset_id,
        MicroAlgos::new(fee),
        first_valid,
        pk.to_address()?,
        AlgorandGenesisId::from_str(genesis_id)?.hash()?,
        None,
    )?
    .sign(&pk)?
    .to_hex()?)
}
