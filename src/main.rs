use ethers::types::Chain;
use nexus::{
    cmd::{parse_args, starknet::match_create_account_args, swap::match_swap_args},
    constants::{weth_address, INCH_NATIVE_ADDRESS},
    evmclient::EvmClient,
    starknet::StarkClient,
    swap::swap_tokens,
};

use paris::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = parse_args();
    match matches.subcommand() {
        ("buy", Some(buy_matches)) => {
            let buy_args = match_swap_args(buy_matches);

            info!(
                "Buy token: {}, chain: {}, amount: {}, slippage: {}, native: {}, allowmax: {}",
                buy_args.token,
                buy_args.chain,
                buy_args.amount,
                buy_args.slippage,
                buy_args.native,
                buy_args.allow_max
            );

            let evmclient = EvmClient::new(buy_args.chain, "0").await;

            let from_token = match buy_args.native {
                true => INCH_NATIVE_ADDRESS,
                false => weth_address(buy_args.chain),
            };

            swap_tokens(
                evmclient.signer,
                from_token,
                buy_args.token,
                buy_args.amount,
                buy_args.slippage,
                buy_args.allow_max,
            )
            .await;
        }
        ("sell", Some(sell_matches)) => {
            let sell_args = match_swap_args(sell_matches);

            info!(
                "Sell token: {}, chain: {}, amount: {}, slippage: {}, native: {}, allowmax: {}",
                sell_args.token,
                sell_args.chain,
                sell_args.amount,
                sell_args.slippage,
                sell_args.native,
                sell_args.allow_max
            );

            let client = EvmClient::new(sell_args.chain, "0").await;

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
        ("sn", Some(sn_matches)) => match sn_matches.subcommand() {
            ("new", Some(new_matches)) => {
                let new_args = match_create_account_args(new_matches);

                info!("Create Starknet account: {}", new_args.index);

                let l1_client = EvmClient::new(Chain::Mainnet, new_args.index).await;
                let stark_client = StarkClient::new(true);

                stark_client.create_argent_account().await;
            }
            _ => {
                println!("no subcommand provided");
            }
        },
        _ => {
            println!("no subcommand provided");
        }
    }
    Ok(())
}
