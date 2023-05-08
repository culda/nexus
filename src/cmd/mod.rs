use std::str::FromStr;

use ethers::{
    prelude::{k256, SignerMiddleware},
    providers::{Http, Provider},
    types::H160,
    utils::parse_ether,
};
use ethers_signers::Wallet;

use crate::{
    constants::{INCH_NATIVE_ADDRESS, WETH_ETH_ADDRESS},
    inch::swap,
};

pub async fn swap_weth_for_token_1inch(
    client: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
    token: &str,
    amount: &str,
) {
    swap(
        client,
        H160::from_str(WETH_ETH_ADDRESS).unwrap(),
        H160::from_str(token).unwrap(),
        parse_ether(amount).unwrap(),
        false,
    )
    .await;
}
