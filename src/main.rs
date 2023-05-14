use clap::{App, Arg, ArgMatches};

use nexus::{
    client::NexusClient,
    cmd::swap_tokens,
    constants::{weth_address, INCH_NATIVE_ADDRESS},
};

use ethers::types::Chain;
use paris::{error, info};

struct BuySellArgs<'a> {
    token: &'a str,
    chain: Chain,
    amount: &'a str,
    slippage: f32,
    allow_max: bool,
    native: bool,
}

fn buysell_args<'a>(matches: &'a ArgMatches<'a>) -> BuySellArgs<'a> {
    let token = matches.value_of("token").unwrap();
    let chain = matches
        .value_of("chain")
        .unwrap()
        .parse::<Chain>()
        .unwrap_or_else(|_| {
            error!("Invalid chain name. Valid chains: mainnet / goerli / zksync / arbitrum");
            std::process::exit(1);
        });
    let amount = matches.value_of("amount").unwrap();
    let slippage = matches
        .value_of("slippage")
        .unwrap_or("0.02")
        .parse::<f32>()
        .unwrap_or_else(|_| {
            error!("Invalid slippage value");
            std::process::exit(1);
        });
    let allow_max = matches
        .value_of("allowmax")
        .unwrap_or("true")
        .parse::<bool>()
        .unwrap();
    let native = matches
        .value_of("native")
        .unwrap_or("true")
        .parse::<bool>()
        .unwrap();
    BuySellArgs {
        token: token,
        chain: chain,
        amount: amount,
        slippage: slippage,
        allow_max: allow_max,
        native: native,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let common_args = [
        Arg::with_name("token")
            .short("t")
            .long("token")
            .value_name("Token contract address")
            .required(true),
        Arg::with_name("chain")
            .short("c")
            .long("chain")
            .value_name("mainnet / goerli / zksync / arbitrum")
            .takes_value(true)
            .required(true),
        Arg::with_name("amount")
            .short("a")
            .long("amount")
            .value_name("ETH amount in human format")
            .takes_value(true)
            .required(true),
        Arg::with_name("slippage")
            .short("s")
            .long("slippage")
            .value_name("float format. default: 0.02")
            .takes_value(true),
        Arg::with_name("native")
            .short("n")
            .long("native")
            .value_name("true / false. default: true. use native token instead of WETH")
            .takes_value(true),
        Arg::with_name("allowmax").long("allowmax"),
    ];

    let matches = App::new("Nexus")
        .about("Defi like a boss")
        .subcommand(App::new("buy").about("Buy a token").args(&common_args))
        .subcommand(App::new("sell").about("Sell a token").args(&common_args))
        .get_matches();

    match matches.subcommand() {
        ("buy", Some(buy_matches)) => {
            let buy_args = buysell_args(buy_matches);

            info!(
                "Buy token: {}, chain: {}, amount: {}, slippage: {}, native: {}, allowmax: {}",
                buy_args.token,
                buy_args.chain,
                buy_args.amount,
                buy_args.slippage,
                buy_args.native,
                buy_args.allow_max
            );

            let client = NexusClient::new(buy_args.chain).await;

            let from_token = match buy_args.native {
                true => INCH_NATIVE_ADDRESS,
                false => weth_address(buy_args.chain),
            };

            swap_tokens(
                client.signer,
                from_token,
                buy_args.token,
                buy_args.amount,
                buy_args.slippage,
                buy_args.allow_max,
            )
            .await;
        }
        ("sell", Some(sell_matches)) => {
            let sell_args = buysell_args(sell_matches);

            info!(
                "Sell token: {}, chain: {}, amount: {}, slippage: {}, native: {}, allowmax: {}",
                sell_args.token,
                sell_args.chain,
                sell_args.amount,
                sell_args.slippage,
                sell_args.native,
                sell_args.allow_max
            );

            let client = NexusClient::new(sell_args.chain).await;

            let to_token = match sell_args.native {
                true => INCH_NATIVE_ADDRESS,
                false => weth_address(sell_args.chain),
            };

            swap_tokens(
                client.signer,
                sell_args.token,
                to_token,
                sell_args.amount,
                sell_args.slippage,
                sell_args.allow_max,
            )
            .await;
        }
        _ => {
            println!("no subcommand provided");
        }
    }
    Ok(())
}
