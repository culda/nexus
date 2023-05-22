use std::{future::Future, pin::Pin, str::FromStr};

use ethers::{
    providers::Middleware,
    types::{H160, U256},
};
use futures_util::FutureExt;
use paris::{error, info};

use crate::{contract_bindings::StarkgetEthBridge, evmclient::EvmSigner};

const GOERLI_STARKGATE_ADDRESS: &str = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";
const MAINNET_STARKGATE_ADDRESS: &str = "0xc488ddfcbeeaf100b32d80d9913ff978922fa487";

pub fn deposit<'a>(
    client: EvmSigner,
    amount: U256,
) -> impl FnOnce(U256) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> + 'a {
    move |l2_recipient: U256| {
        async move {
            let address = match client.get_chainid().await.unwrap().to_string().as_str() {
                "1" => H160::from_str(MAINNET_STARKGATE_ADDRESS).unwrap(),
                "5" => H160::from_str(GOERLI_STARKGATE_ADDRESS).unwrap(),
                _ => {
                    error!("Unsupported chain id");
                    std::process::exit(1);
                }
            };
            info!("L1 Starkgate deposit address: {}", address);

            let client = StarkgetEthBridge::new(address, client.into());

            let gas_limit = client
                .deposit(amount, l2_recipient)
                .value(amount)
                .estimate_gas()
                .await
                .unwrap();

            let tx = client
                .deposit(amount, l2_recipient)
                .value(amount)
                .gas(gas_limit);

            info!("Sending deposit tx: {:#?}", tx);

            tx.send().await.unwrap_or_else(|err| {
                error!("Failed to deposit: {}", err);
                std::process::exit(1);
            });
        }
        .boxed()
    }
}
