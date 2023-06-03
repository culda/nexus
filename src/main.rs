use ethers::types::Chain;
use nexus::{
    cmd::{
        parse_args,
        starknet::{
            match_create_account_args, match_deposit_args, match_info_account_args, match_swap_args,
        },
        swap::match_inch_swap_args,
    },
    constants::{weth_address, INCH_NATIVE_ADDRESS},
    evmclient::EvmClient,
    inch::swap::swap_tokens,
    starknet::{bridge::deposit_l1_l2, jediswap::JediSwap, StarkClient},
};

use paris::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = parse_args();
    match matches.subcommand() {
        ("buy", Some(buy_matches)) => {
            let args = match_inch_swap_args(buy_matches);

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
            let args = match_inch_swap_args(sell_matches);

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
                info!("Creating Starknet account ...");

                let l1_client = EvmClient::new(Chain::Mainnet, args.index).await;
                l1_client.info_account().await;

                let stark_client = StarkClient::new(args.index, false).await;
                stark_client.create_argent_deployment().await;
            }
            ("deposit", Some(deposit_matches)) => {
                let args = match_deposit_args(deposit_matches);
                info!("Bridging to Starknet account ...");

                let l1_client = EvmClient::new(Chain::Mainnet, args.index).await;
                l1_client.info_account().await;

                let stark_client = StarkClient::new(args.index, false).await;
                let deposit_fn = deposit_l1_l2(l1_client.signer);
                stark_client.deposit_l1_l2(args.amount, deposit_fn).await;
            }
            ("info", Some(info_matches)) => {
                let args = match_info_account_args(info_matches);

                let l1_client = EvmClient::new(Chain::Mainnet, args.index).await;
                l1_client.info_account().await;

                let stark_client = StarkClient::new(args.index, false).await;
                stark_client.info_account(false).await;
            }
            ("swap", Some(swap_matches)) => {
                let args = match_swap_args(swap_matches);
                info!("Preparing swap ...");

                let l1_client = EvmClient::new(Chain::Mainnet, args.index).await;
                l1_client.info_account().await;

                let stark_client = StarkClient::new(args.index, false).await;
                stark_client.info_account(false).await;

                let swap = JediSwap::new(
                    &stark_client,
                    args.from_token,
                    args.from_token_decimals,
                    args.to_token,
                    args.amount,
                    args.slippage,
                );

                swap.execute(false).await;
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
