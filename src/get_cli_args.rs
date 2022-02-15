use docopt::Docopt;
use serde::Deserialize;

use crate::{types::Result, usage_info::USAGE_INFO};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CliArgs {
    pub arg_id: u64,
    pub arg_amount: u64,
    pub arg_firstValid: u64,
    pub arg_totalUnits: u64,
    pub arg_receiver: String,
    pub flag_fee: u64,
    pub flag_key: String,
    pub flag_note: String,
    pub flag_decimals: u64,
    pub flag_frozen: bool,
    pub flag_manager: String,
    pub flag_reserve: String,
    pub flag_freezer: String,
    pub flag_assetUrl: String,
    pub flag_unitName: String,
    pub flag_clawback: String,
    pub flag_genesisId: String,
    pub flag_assetName: String,
    pub flag_metadataHash: String,
    pub cmd_payTx: bool,
    pub cmd_showAddress: bool,
    pub cmd_generateKey: bool,
    pub cmd_assetConfigTx: bool,
    pub cmd_assetTransferTx: bool,
}

pub fn get_cli_args() -> Result<CliArgs> {
    match Docopt::new(USAGE_INFO).and_then(|d| d.deserialize()) {
        Ok(cli_args) => Ok(cli_args),
        Err(e) => Err(e.into()),
    }
}
