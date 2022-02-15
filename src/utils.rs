pub fn strip_hex_prefix(hex: &str) -> String {
    const LOWERCASE_HEX_PREFIX: &str = "0x";
    const UPPERCASE_HEX_PREFIX: &str = "0X";
    if hex.starts_with(LOWERCASE_HEX_PREFIX) || hex.starts_with(UPPERCASE_HEX_PREFIX) {
        hex.trim_start_matches(LOWERCASE_HEX_PREFIX)
            .trim_start_matches(UPPERCASE_HEX_PREFIX)
            .to_string()
    } else {
        hex.to_string()
    }
}
