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
    inch::InchApi,
};

pub async fn buy_token_weth(
    client: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
    token: &str,
    amount: &str,
) {
    let api = InchApi::new(client);
    api.swap(WETH_ETH_ADDRESS, token, amount).await;
}
