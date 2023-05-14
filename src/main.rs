use clap::{App, Arg};

use nexus::{
    client::NexusClient,
    cmd::swap_tokens,
    constants::{weth_address, INCH_NATIVE_ADDRESS},
};

use ethers::types::Chain;
use paris::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Nexus")
        .about("Defi like a boss")
        .subcommand(
            App::new("buy")
                .about("Buy a token")
                .arg(
                    Arg::with_name("token")
                        .short("t")
                        .long("token")
                        .value_name("TOKEN")
                        .required(true),
                )
                .arg(
                    Arg::with_name("chain")
                        .short("c")
                        .long("chain")
                        .value_name("mainnet / goerli / zksync / arbitrum")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("amount")
                        .short("a")
                        .long("amount")
                        .value_name("Eth amount")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("slippage")
                        .short("s")
                        .long("slippage")
                        .value_name("SLIPPAGE")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("native")
                        .short("n")
                        .long("native")
                        .value_name("NATIVE"),
                )
                .arg(Arg::with_name("allowmax").long("allowmax")),
        )
        .get_matches();

    match matches.subcommand() {
        ("buy", Some(buy_matches)) => {
            let token = buy_matches.value_of("token").unwrap();
            let chain = buy_matches
                .value_of("chain")
                .unwrap()
                .parse::<Chain>()
                .unwrap_or_else(|_| {
                    error!(
                        "Invalid chain name. Valid chains: mainnet / goerli / zksync / arbitrum"
                    );
                    std::process::exit(1);
                });
            let amount = buy_matches.value_of("amount").unwrap();
            let slippage = buy_matches
                .value_of("slippage")
                .unwrap_or("0.02")
                .parse::<f32>()
                .unwrap_or_else(|_| {
                    error!("Invalid slippage value");
                    std::process::exit(1);
                });
            let native = buy_matches
                .value_of("native")
                .unwrap_or("true")
                .parse::<bool>()
                .unwrap();
            let allow_max = buy_matches
                .value_of("allowmax")
                .unwrap_or("true")
                .parse::<bool>()
                .unwrap();

            info!(
                "Buy token: {}, chain: {}, amount: {}, slippage: {}, native: {}, allowmax: {}",
                token, chain, amount, slippage, native, allow_max
            );

            let client = NexusClient::new(chain).await;

            let from_token = match native {
                true => INCH_NATIVE_ADDRESS,
                false => weth_address(chain),
            };

            swap_tokens(
                client.signer,
                from_token,
                token,
                amount,
                slippage,
                allow_max,
            )
            .await;
        }
        _ => {
            println!("no subcommand provided");
        }
    }
    Ok(())
}
