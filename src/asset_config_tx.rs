use crate::{decrypt_private_key::decrypt_private_key, types::Result};
use std::str::FromStr;

use rust_algorand::{
    AlgorandAddress, AlgorandGenesisId, AlgorandHash, AlgorandTransaction, AssetParameters,
    MicroAlgos,
};

pub fn asset_config_tx(
    first_valid: u64,
    genesis_id: &str,
    fee: u64,
    path_to_pk: &str,
    metadata_hash: &str,
    asset_name: &str,
    asset_url: &str,
    clawback_address: &str,
    decimals: u64,
    default_frozen: bool,
    freeze_address: &str,
    manager_address: &str,
    reserve_address: &str,
    total_base_units: u64,
    unit_name: &str,
) -> Result<String> {
    let pk = decrypt_private_key(path_to_pk)?;
    Ok(AlgorandTransaction::new_asset_configuration_tx(
        MicroAlgos::new(fee),
        first_valid,
        pk.to_address()?,
        AlgorandGenesisId::from_str(genesis_id)?.hash()?,
        None,
        AssetParameters::new(
            if metadata_hash == "" {
                None
            } else {
                Some(AlgorandHash::from_str(metadata_hash)?)
            },
            if asset_name == "" {
                None
            } else {
                Some(asset_name.to_string())
            },
            if asset_url == "" {
                None
            } else {
                Some(asset_url.to_string())
            },
            if clawback_address == "" {
                None
            } else {
                Some(AlgorandAddress::from_str(clawback_address)?)
            },
            decimals,
            default_frozen,
            if freeze_address == "" {
                None
            } else {
                Some(AlgorandAddress::from_str(&freeze_address)?)
            },
            if manager_address == "" {
                None
            } else {
                Some(AlgorandAddress::from_str(&manager_address)?)
            },
            if reserve_address == "" {
                None
            } else {
                Some(AlgorandAddress::from_str(&reserve_address)?)
            },
            total_base_units,
            if unit_name == "" {
                None
            } else {
                Some(unit_name.to_string())
            },
        ),
    )?
    .sign(&pk)?
    .to_hex()?)
}
