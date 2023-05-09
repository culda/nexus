use clap::{App, Arg};

use nexus::{client::NexusClient, cmd::buy_token_native};

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
                        .value_name("NATIVE")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("allow")
                        .long("allow")
                        .value_name("ALLOW")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("sell")
                .about("Sell a token")
                .arg(
                    Arg::with_name("token")
                        .short("t")
                        .long("token")
                        .value_name("TOKEN")
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
                        .value_name("NATIVE")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("allow")
                        .short("a")
                        .long("allow")
                        .value_name("ALLOW")
                        .takes_value(true),
                ),
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
            let slippage = buy_matches.value_of("slippage").unwrap_or("0.02");
            let native = buy_matches
                .value_of("native")
                .unwrap_or("true")
                .parse::<bool>()
                .unwrap();
            let allow = buy_matches
                .value_of("allow")
                .unwrap_or("true")
                .parse::<bool>()
                .unwrap();

            info!(
                "Buy token: {}, chain: {}, amount: {}, slippage: {}, native: {}, allow: {}",
                token, chain, amount, slippage, native, allow
            );

            if !native {
                error!("Only native tokens are supported for now");
                std::process::exit(1);
            }

            let client = NexusClient::new(chain).await;
            buy_token_native(client.signer, token, amount).await;
        }
        _ => {
            println!("no subcommand provided");
        }
    }
    Ok(())
}
