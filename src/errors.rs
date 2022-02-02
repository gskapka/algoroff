quick_error! {
    #[derive(Debug)]
    pub enum AppError {
        Custom(err: String) {
            from()
            from(err: &str) -> (err.into())
            display("✘ {}", err)
        }
        DocoptError(err: docopt::Error) {
            from()
            display("✘ Docopt error: {}", err)
        }
        AlgorandError(err: rust_algorand::AlgorandError) {
            from()
            display("✘ Algorand error: {}", err)
        }
        IOError(err: std::io::Error) {
            from()
            display("✘ I/O error: {}", err)
        }
        UTF8Error(err: std::str::Utf8Error) {
            from()
            display("✘ UTF8 error: {}", err)
        }
        HexError(err: hex::FromHexError) {
            from()
            display("✘ Hex error: {}", err)
        }
    }
}
