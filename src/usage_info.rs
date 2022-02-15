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
        algoroff assetOptInTx <assetId> <firstValid> [--fee=<ualgos>] [--genesisId=<str>] [--key=<path>]
        algoroff payTx <amount> <receiver> <firstValid> [--key=<path>] [--fee<ualgos>] [--note=<hex>] [--genesisId=<str>]
        algoroff assetTransferTx <amount> <id> <receiver> <firstValid> [--genesisId=<str>] [--fee=<ualgos>] [--key=<path>] [--note=<hex>]
        algoroff assetConfigTx <totalUnits> <firstValid> [--genesisId=<str>] [--fee=<ualgos>] [--key=<path>] [--metadataHash=<str>] [--assetName=<str>] [--assetUrl=<str>] [--clawback=<str>] [--decimals=<u64>] [--frozen=<bool>] [--freezer=<str>] [--manager=<str>] [--reserve=<str>] [--unitName=<str>]

Commands:

    payTx                       ❍ Create a pay transaction.
    assetTransferTx             ❍ Create an asset transfer transaction.
    generateKey                 ❍ Generate a random Algorand private key.
    showAddress                 ❍ Show the address of the GPG-encrypted private key.
    assetOptIn                  ❍ Create tx to opt in to receiving an asset.
    <id>                        ❍ ID number of the asset in question.
    <amount>                    ❍ Amount to transfer.
    <receiver>                  ❍ The receiver of the transaction.
    <firstValid>                ❍ The first round after which the tx will be valid.
    <totalUnits>                ❍ The number of units of an asset to create.

Options:

    --help                      ❍ Show this message.
    --version                   ❍ Returns the version of the tool.
    --fee=<ualgos>              ❍ Fee in micro algos [default: 1000]
    --assetName=<str>           ❍ The name of the asset [default: ]
    --assetUrl=<str>            ❍ URL of the asset's website [default: ]
    --decimals=<u64>            ❍ Number of decimals of the asset [default: 0]
    --metadataHash=<str>        ❍ Hash of data relevant to the asset [default: ]
    --unitName=<str>            ❍ Name of the Algorand asset being create [default: ]
    --key=<path>                ❍ Path to GPG encrypted key file [default: ./key.gpg]
    --freezer=<str>             ❍ Algorand account who can freeze the asset [default: ]
    --note=<hex>                ❍ An optional note to add to Algo transaction [default: 0x]
    --frozen=<bool>             ❍ Whether or not the asset is frozen upon creation [default: false]
    --clawback=<str>            ❍ Algorand address that can move asset from any to any address [default: ]
    --manager=<str>             ❍ Algorand account who can configure/destroy asset after creation [default: ]
    --genesisId=<str>           ❍ The genesis ID of the chain you want to transact on [default: mainnet-v1.0]
    --reserve=<str>             ❍ Algorand account who holds the reserve/non-minted units of the asset [default: ]
";
