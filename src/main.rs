#[macro_use]
extern crate dotenv_codegen;

use clap::App;

use nexus::{cmd::buy_token_weth, constants::USDC_ETH_ADDRESS};

use ethers::{
    abi::Address,
    middleware::SignerMiddleware,
    prelude::{coins_bip39::English, MnemonicBuilder, TransactionRequest},
    providers::{Http, Middleware, Provider, ProviderExt},
    types::{transaction::eip2718::TypedTransaction, Chain, H160},
    utils::{parse_ether, parse_units},
};
use ethers_signers::Signer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Nexus")
        // .arg(
        //     Arg::with_name("dapp")
        //         .required(true)
        //         .takes_value(true)
        //         .possible_values(&["syncswap"])
        //         .help("The name of the dapp to execute"),
        // )
        // .arg(
        //     Arg::with_name("test")
        //         .short("t")
        //         .long("test")
        //         .help("Enables test mode"),
        // )
        .get_matches();

    // let zk_rpc = if test_mode {
    //     "https://testnet.era.zksync.dev"
    // } else {
    //     "https://mainnet.era.zksync.io"
    // };

    let phrase = dotenv!("MNEMONIC");
    let mainnet_rpc = dotenv!("ALCHEMY_MAINNET_RPC");

    let builder = MnemonicBuilder::<English>::default()
        .phrase(phrase)
        .derivation_path(format!("m/44'/60'/0'/0/{}", 0).as_str())
        .unwrap();

    let wallet = builder.build().unwrap().with_chain_id(Chain::Mainnet);
    let mut provider = Provider::connect(mainnet_rpc).await;
    provider.set_chain(Chain::Mainnet);
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    let c = client.get_chainid().await?;
    let address: Address = client.address();
    let balance = provider.get_balance(wallet.address(), None).await.unwrap();
    let address_str = format!("{:#x}", address);

    println!("Balance: {}", balance);
    println!("Chain ID: {}", c);
    println!("Address: {}", address_str);

    let amount = parse_ether("0.05").unwrap().to_string();

    let swap = buy_token_weth(client, USDC_ETH_ADDRESS, amount.as_str());
    swap.await;

    // let tx = swap_eth_for_usdc(client, ethers::types::U256::from(0), amount).await?;

    // client.send_transaction(typed, None).await?;

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

    // let tx = TransactionRequest::new()
    //     .to("0xBc63552E466B4fd2B6fbC5a3D1f3bD556c45FD7a")
    //     .chain_id(324u64)
    //     .from(address)
    //     .value(1_000_000u128);

    // let typed = TypedTransaction::Legacy(tx);

    // let signature = client.sign_transaction(&typed, address).await?;

    Ok(())
}
