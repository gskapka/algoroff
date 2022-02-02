use rust_algorand::AlgorandKeys;
use std::process::Command;

use crate::types::{Byte, Result};

fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).is_file()
}

fn strip_new_lines_from_str(string: String) -> String {
    string.replace('\n', "")
}

fn bytes_to_utf8_str(bytes: &[Byte]) -> Result<String> {
    Ok(std::str::from_utf8(bytes)?.to_string())
}

fn convert_bytes_to_string_with_no_new_lines(bytes: &[Byte]) -> Result<String> {
    bytes_to_utf8_str(bytes).map(strip_new_lines_from_str)
}

fn check_keyfile_exists(path: &str) -> Result<()> {
    if !file_exists(path) {
        return Err(format!("✘ Keyfile not found @ path: {}!", path).into());
    }
    Ok(())
}

fn maybe_decrypt_keyfile(path: &str) -> Result<String> {
    let output = Command::new("gpg").arg("-d").arg(path).output()?;
    #[allow(clippy::len_zero)] // NOTE: False positive of this lint be in this fxn!
    if output.stdout.len() == 0 {
        Err(convert_bytes_to_string_with_no_new_lines(&output.stderr)?.into())
    } else {
        convert_bytes_to_string_with_no_new_lines(&output.stdout)
    }
}

fn get_private_key_from_hex(hex: String) -> Result<AlgorandKeys> {
    Ok(AlgorandKeys::from_bytes(&hex::decode(hex)?)?)
}

pub fn decrypt_private_key(path: &str) -> Result<AlgorandKeys> {
    check_keyfile_exists(path)
        .and_then(|_| maybe_decrypt_keyfile(path))
        .and_then(get_private_key_from_hex)
}
