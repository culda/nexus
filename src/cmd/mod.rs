use ethers::{
    prelude::{k256, SignerMiddleware},
    providers::{Http, Provider},
    utils::parse_ether,
};
use ethers_signers::Wallet;
use paris::info;

use crate::{constants::INCH_NATIVE_ADDRESS, inch::InchApi};

pub async fn buy_token_native(
    client: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
    token: &str,
    amount: &str,
) {
    let api = InchApi::new(client);
    info!("1Inch API initialized");

    let amount = parse_ether(amount).unwrap().to_string();
    api.swap(INCH_NATIVE_ADDRESS, token, amount.as_str()).await;
}
