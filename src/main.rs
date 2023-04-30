use std::sync::Arc;

use clap::{App, Arg};
use nexus::syncswap;
use web3::{
    transports::Http,
    types::{H160, H256},
    Web3,
};
use zksync::{Network, Wallet as ZkWallet, WalletCredentials};
use zksync_eth_signer::{EthereumSigner, PrivateKeySigner};

use ethers::{
    prelude::k256::ecdsa::SigningKey,
    providers::{Middleware, Provider, ProviderExt},
    signers::{coins_bip39::English, MnemonicBuilder, Signer},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let phrase = ""; // <-- phrase here

    // let mut wallets = Vec::new();

    for i in 0..10 {
        let builder = MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .derivation_path(format!("m/44'/60'/0'/0/{}", i).as_str())
            .unwrap();

        let wallet = builder.build().unwrap();
        let provider = Provider::connect("https://testnet.era.zksync.dev").await;

        let balance = provider.get_balance(wallet.address(), None).await.unwrap();
        println!("Balance: {}", balance);

        // let prv_key: H256 = H256::from_slice(&wallet.signer().to_bytes());

        // let signer = PrivateKeySigner::new(prv_key);

        // let address_bytes: [u8; 20] = wallet.address().to_fixed_bytes();
        // let address: H160 = H160::from_slice(&address_bytes);

        // let creds = WalletCredentials::from_eth_signer(address, signer, Network::Mainnet);

        // let wallet = ZkWallet::new(provider, creds).await;

        // wallets.push(wallet);
    }

    // let matches = App::new("Nexus")
    //     .arg(
    //         Arg::with_name("dapp")
    //             .required(true)
    //             .takes_value(true)
    //             .possible_values(&["syncswap"])
    //             .help("The name of the dapp to execute"),
    //     )
    //     .get_matches();

    // let transport = Http::new("http://localhost:8545")?;
    // let web3 = Web3::new(transport);

    // match matches.value_of("dapp") {
    //     Some("syncswap") => {
    //         let balance =
    //             syncswap::get_balance(&web3, "0x1234567890123456789012345678901234567890").await?;
    //         println!("Balance: {}", balance);
    //     }
    //     _ => println!("Unknown dapp"),
    // }

    Ok(())
}
