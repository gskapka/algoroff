use docopt::Docopt;
use serde::Deserialize;

use crate::{types::Result, usage_info::USAGE_INFO};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CliArgs {
    pub arg_id: u64,
    pub arg_amount: u64,
    pub arg_firstValid: u64,
    pub arg_receiver: String,
    pub arg_genesisHash: String,
    pub flag_fee: u64,
    pub flag_key: String,
    pub cmd_showAddress: bool,
    pub cmd_generateKey: bool,
    pub cmd_assetTransferTx: bool,
}

pub fn get_cli_args() -> Result<CliArgs> {
    match Docopt::new(USAGE_INFO).and_then(|d| d.deserialize()) {
        Ok(cli_args) => Ok(cli_args),
        Err(e) => Err(e.into()),
    }
}
