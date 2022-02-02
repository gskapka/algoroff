#[macro_use]
extern crate quick_error;
use rust_algorand::AlgorandKeys;
use serde_json::json;

mod errors;
mod get_cli_args;
mod types;
mod usage_info;

use crate::{
    errors::AppError,
    get_cli_args::{get_cli_args, CliArgs},
    usage_info::USAGE_INFO,
};

fn main() {
    match get_cli_args().and_then(|cli_args| match cli_args {
        CliArgs {
            cmd_generateKey: true,
            ..
        } => {
            let key = AlgorandKeys::create_random();
            Ok(json!({"key": hex::encode(&key.to_bytes()), "address": format!("{}", key.to_address()?)}))
        },
        _ => Err(AppError::Custom(USAGE_INFO.to_string())),
    }) {
        Ok(res) => println!("{res}"),
        Err(err) => println!("{err}"),
    }
}
