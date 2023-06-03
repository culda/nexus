use std::{future::Future, pin::Pin, str::FromStr};

use ethers::{
    providers::Middleware,
    types::{H160, U256},
};
use futures_util::FutureExt;
use paris::{error, info};

use crate::{contract_bindings::StarkgetEthBridge, evmclient::EvmSigner};

const GOERLI_STARKGATE_ADDRESS: &str = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";
const MAINNET_STARKGATE_ADDRESS: &str = "0xae0ee0a63a2ce6baeeffe56e7714fb4efe48d419";

pub fn deposit_l1_l2<'a>(
    client: EvmSigner,
) -> impl FnOnce(U256, U256, U256) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> + 'a {
    move |tx_value: U256, deposit_amount: U256, l2_recipient: U256| {
        async move {
            let address = match client.get_chainid().await.unwrap().to_string().as_str() {
                "1" => H160::from_str(MAINNET_STARKGATE_ADDRESS).unwrap(),
                "5" => H160::from_str(GOERLI_STARKGATE_ADDRESS).unwrap(),
                _ => {
                    error!("Unsupported chain id");
                    std::process::exit(1);
                }
            };
            let client = StarkgetEthBridge::new(address, client.into());
            let gas_limit = client
                .deposit(deposit_amount, l2_recipient)
                .value(tx_value)
                .estimate_gas()
                .await
                .unwrap();

            let tx = client
                .deposit(deposit_amount, l2_recipient)
                .value(tx_value)
                .gas(gas_limit);

            info!("Sending deposit tx ...");

            tx.send().await.unwrap_or_else(|err| {
                error!("Failed to deposit: {}", err);
                std::process::exit(1);
            });
        }
        .boxed()
    }
}
