#[macro_use]
extern crate quick_error;

mod decrypt_private_key;
mod errors;
mod generate_key;
mod get_cli_args;
mod show_address;
mod types;
mod usage_info;

use crate::{
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
        _ => Err(AppError::Custom(USAGE_INFO.to_string())),
    }) {
        Ok(res) => println!("{res}"),
        Err(err) => println!("{err}"),
    }
}
