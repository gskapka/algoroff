pub static USAGE_INFO: &str = "
❍ Algoroff ❍

    Copyright Greg Kapka 2022
    Questions: gregkapka@gmail.com

❍ Info ❍

Algoroff is an offline signer for Algorand transactions.

❍ Usage ❍

Usage:  algoroff [--help]
        algoroff [--version]
        algoroff generateKey
        algoroff showAddress [--key=<path>]
        algoroff assetTransferTx <amount> <id> <receiver> <firstValid> [--genesisId=<String>] [--fee=<ualgos>] [--key=<path>] [--note=<hex>]

Commands:

    assetTransferTx             ❍ Create anx asset transfer transaction.
    generateKey                 ❍ Generate a random Algorand private key.
    showAddress                 ❍ Show the address of the GPG-encrypted private key.
    <id>                        ❍ ID number of the asset in question.
    <amount>                    ❍ Amount to transer.
    <receiver>                  ❍ The receiver of the transaction.
    <firstValid>                ❍ The first round after which the tx will be valid.

Options:

    --help                      ❍ Show this message.
    --version                   ❍ Returns the version of the tool.
    --fee=<ualgos>              ❍ Fee in micro algos [default: 1000]
    --key=<path>                ❍ Path to GPG encrypted key file [default: ./key.gpg]
    --note=<hex>                ❍ An optional note to add to Algo transaction [default: 0x]
    --genesisId=<String>        ❍ The genesis ID of the chain you want to transact on [default: mainnet-v1.0]
";
