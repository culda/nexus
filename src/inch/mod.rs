mod aggregation_router_v5;
use crate::erc20::erc20::ApproveCall;

use std::str::FromStr;

use ethers::{
    prelude::{k256, SignerMiddleware},
    providers::{Http, Provider},
    types::{Address, Bytes, H160, U256},
};

use ethers_signers::Wallet;

use crate::constants::{INCH_EXECUTOR_ETH_ADDRESS, INCH_ROUTER_ETH_ADDRESS};

pub async fn swap(
    client: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
    from: Address,
    to: Address,
    amount: U256,
    needs_approve: bool,
) {
    let router = aggregation_router_v5::AggregationRouterV5::new(
        H160::from_str(INCH_ROUTER_ETH_ADDRESS).unwrap(),
        client.clone().into(),
    );

    let min_return_amount = U256::zero();

    let desc = aggregation_router_v5::SwapDescription {
        amount,
        src_token: from,
        dst_token: to,
        src_receiver: H160::from_str(INCH_EXECUTOR_ETH_ADDRESS).unwrap(),
        dst_receiver: client.address(),
        min_return_amount,
        flags: U256::from(0),
    };

    let mut permit = Bytes::from([]);

    // if needs_approve {
    //     permit = ApproveCall {
    //         spender: H160::from_str(INCH_EXECUTOR_ETH_ADDRESS).unwrap(),
    //         value: U256::max_value(),
    //     }
    //     .into()
    // }

    let swap = router.swap(
        H160::from_str(INCH_EXECUTOR_ETH_ADDRESS).unwrap(),
        desc,
        permit,
        Bytes::from([]),
    );

    let gas_limit = match swap.estimate_gas().await {
        Ok(gas) => gas,
        Err(e) => {
            println!("error {:?}", e);
            return;
        }
    };

    println!("gas limit {}", gas_limit);

    let tx = swap.gas(gas_limit);
    let exec = tx.send().await.unwrap();
    println!("tx submitted {:?}", exec);
}
