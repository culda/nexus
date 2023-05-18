use nexus::{
    cmd::{parse_args, swap::match_swap_args},
    constants::{weth_address, INCH_NATIVE_ADDRESS},
    evmclient::NexusEvmClient,
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

            let evmclient = NexusEvmClient::new(buy_args.chain).await;

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

            let client = NexusEvmClient::new(sell_args.chain).await;

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
