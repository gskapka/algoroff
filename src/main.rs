#[macro_use]
extern crate quick_error;

mod asset_config_tx;
mod asset_transfer_tx;
mod decrypt_private_key;
mod errors;
mod generate_key;
mod get_cli_args;
mod show_address;
mod types;
mod usage_info;
mod utils;

use crate::{
    asset_config_tx::asset_config_tx,
    asset_transfer_tx::asset_transfer_tx,
    errors::AppError,
    generate_key::generate_key,
    get_cli_args::{get_cli_args, CliArgs},
    show_address::show_address,
    usage_info::USAGE_INFO,
};

fn main() {
    match get_cli_args().and_then(|cli_args| match cli_args {
        CliArgs {
            cmd_generateKey: true,
            ..
        } => generate_key(),
        CliArgs {
            cmd_showAddress: true,
            ..
        } => show_address(&cli_args.flag_key),
        CliArgs {
            cmd_assetTransferTx: true,
            ..
        } => asset_transfer_tx(
            cli_args.arg_amount,
            cli_args.arg_id,
            &cli_args.arg_receiver,
            cli_args.arg_firstValid,
            &cli_args.flag_genesisId,
            cli_args.flag_fee,
            &cli_args.flag_key,
            &cli_args.flag_note,
        ),
        CliArgs {
            cmd_assetConfigTx: true,
            ..
        } => asset_config_tx(
            cli_args.arg_firstValid,
            &cli_args.flag_genesisId,
            cli_args.flag_fee,
            &cli_args.flag_key,
            &cli_args.flag_metadataHash,
            &cli_args.flag_assetName,
            &cli_args.flag_assetUrl,
            &cli_args.flag_clawback,
            cli_args.flag_decimals,
            cli_args.flag_frozen,
            &cli_args.flag_freezer,
            &cli_args.flag_manager,
            &cli_args.flag_reserve,
            cli_args.arg_totalUnits,
            &cli_args.flag_unitName,
        ),
        _ => Err(AppError::Custom(USAGE_INFO.to_string())),
    }) {
        Ok(res) => println!("{res}"),
        Err(err) => println!("{err}"),
    }
}
