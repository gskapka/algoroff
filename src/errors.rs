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
    }
}
