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
        algoroff assetTransferTx <amount> <id> <receiver> <firstValid> <genesisHash> [--fee=<ualgos>] [--lastValid=<round>] [--key=<path>]

Commands:

    assetTransferTx             ❍ Create anx asset transfer transaction.
    generateKey                 ❍ Generate a random Algorand private key.
    showAddress                 ❍ Show the address of the GPG-encrypted private key.
    <id>                        ❍ ID number of the asset in question.
    <amount>                    ❍ Amount to transer.
    <receiver>                  ❍ The receiver of the transaction.
    <firstValid>                ❍ The first round after which the tx will be valid.
    <genesisHash>               ❍ The genesis hash of the network you wish to transact on.

Options:

    --help                      ❍ Show this message.
    --version                   ❍ Returns the version of the tool.
    --fee=<ualgos>              ❍ Fee in micro algos [default: 1000]
    --key=<path>                ❍ Path to GPG encrypted key file [default: /key.gpg]
    --lastValid=<round>         ❍ The last round in which the tx will still be valid. If omitted,
                                 this will be first valud + 1000.

";
