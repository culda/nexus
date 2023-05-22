use ethers::{types::Chain, utils::parse_ether};
use nexus::{
    cmd::{parse_args, starknet::match_create_account_args, swap::match_swap_args},
    constants::{weth_address, INCH_NATIVE_ADDRESS},
    evmclient::EvmClient,
    inch::swap::swap_tokens,
    starknet::{starkgate::deposit, StarkClient},
};

use paris::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = parse_args();
    match matches.subcommand() {
        ("buy", Some(buy_matches)) => {
            let args = match_swap_args(buy_matches);

            info!(
                "Buy token: {}, chain: {}, amount: {}, slippage: {}, native: {}, allowmax: {}",
                args.token, args.chain, args.amount, args.slippage, args.native, args.allow_max
            );

            let evmclient = EvmClient::new(args.chain, "0").await;

            let from_token = match args.native {
                true => INCH_NATIVE_ADDRESS,
                false => weth_address(args.chain),
            };

            swap_tokens(
                evmclient.signer,
                from_token,
                args.decimals,
                args.token,
                args.amount,
                args.slippage,
                args.allow_max,
            )
            .await;
        }
        ("sell", Some(sell_matches)) => {
            let args = match_swap_args(sell_matches);

            info!(
                "Sell token: {}, chain: {}, amount: {}, slippage: {}, native: {}, allowmax: {}",
                args.token, args.chain, args.amount, args.slippage, args.native, args.allow_max
            );

            let client = EvmClient::new(args.chain, "0").await;

            let to_token = match args.native {
                true => INCH_NATIVE_ADDRESS,
                false => weth_address(args.chain),
            };

            swap_tokens(
                client.signer,
                args.token,
                args.decimals,
                to_token,
                args.amount,
                args.slippage,
                args.allow_max,
            )
            .await;
        }
        ("sn", Some(sn_matches)) => match sn_matches.subcommand() {
            ("new", Some(new_matches)) => {
                let args = match_create_account_args(new_matches);

                let l1_client = EvmClient::new(Chain::Mainnet, args.index).await;

                info!("Creating Starknet account ...");
                info!("L1 address: {}", l1_client.address());

                let mut stark_client = StarkClient::new(false).await;
                let deposit_fn = deposit(l1_client.signer, parse_ether("0.01").unwrap());
                stark_client.create_argent_deployment(deposit_fn).await;
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
