use ethers::{
    prelude::{abi, Address},
    types::{Bytes, H160, U256},
};
use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    constants::{SYNCSWAP_ROUTER_ADDRESS, USDC_ETH_POOL_ADDRESS, WETH_ETH_ADDRESS},
    contract_bindings::sync_swap_router::{self, SwapPath, SwapStep},
    evmclient::EvmSigner,
};

pub async fn swap_eth_for_usdc(
    client: EvmSigner,
    amount_out_min: U256,
    amount: U256,
) -> Result<(), anyhow::Error> {
    // Constructs the swap paths with steps.
    // Determine withdraw mode, to withdraw native ETH or wETH on last step.
    // 0 - vault internal transfer
    // 1 - withdraw and unwrap to native ETH
    // 2 - withdraw and wrap to wETH
    let withdraw_mode = 1; // 1 or 2 to withdraw to user's wallet

    let swap_data = abi::encode(
        &[
            ethers::abi::Token::Address(H160::from_str(WETH_ETH_ADDRESS).unwrap()),
            ethers::abi::Token::Address(client.address()),
            ethers::abi::Token::Uint(U256::from(withdraw_mode)),
        ], // tokenIn, to, withdraw mode
    );

    println!("address {}", client.address());

    let steps = vec![SwapStep {
        pool: H160::from_str(USDC_ETH_POOL_ADDRESS).unwrap(),
        data: Bytes::from(swap_data),
        callback: Address::zero(),
        callback_data: Bytes::from([]),
    }];

    // If we want to use the native ETH as the input token,
    // the `tokenIn` on path should be replaced with the zero address.
    // Note: however we still have to encode the wETH address to pool's swap data.
    let native_eth_address = Address::zero();

    // We have only 1 path.
    let paths = vec![SwapPath {
        steps: steps,
        token_in: native_eth_address,
        amount_in: amount,
    }];

    // 5 minutes from now
    let deadline = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 300;

    let router = sync_swap_router::SyncSwapRouter::new(
        H160::from_str(SYNCSWAP_ROUTER_ADDRESS).unwrap(),
        client.into(),
    );

    let gas_limit = router
        .swap(paths.clone(), amount_out_min, U256::from(deadline))
        .value(amount)
        .estimate_gas()
        .await?;

    let tx = router
        .swap(paths, amount_out_min, U256::from(deadline))
        .value(amount)
        .gas(gas_limit);

    let exec = tx.send().await?;

    println!("tx: {:#?}", exec.tx_hash());

    Ok(())
}
