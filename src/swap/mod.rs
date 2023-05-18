use std::str::FromStr;

use ethers::{
    prelude::{
        k256::{self},
        SignerMiddleware,
    },
    providers::{Http, Provider},
    types::{Address, U256},
    utils::parse_ether,
};
use ethers_signers::Wallet;
use paris::{error, info};

use crate::{
    constants::{INCH_NATIVE_ADDRESS, INCH_ROUTER_ADDRESS},
    erc20::erc20::ERC20,
    inch::InchApi,
};

async fn check_allowance_and_approve(
    client: &SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
    sender: Address,
    token: Address,
    amount: U256,
    allow_max: bool,
) {
    let erc20 = ERC20::new(token, client.clone().into());
    let allowance = erc20
        .allowance(sender, Address::from_str(INCH_ROUTER_ADDRESS).unwrap())
        .await;

    let allowed = allowance.unwrap_or_else(|err| {
        error!("Failed to get allowance: {}", err);
        std::process::exit(1);
    });

    if amount > allowed {
        let spend_amount = match allow_max {
            true => U256::max_value(),
            false => amount,
        };

        info!(
            "<cyan>Approving</> 1Inch Router to spend {} {}",
            spend_amount, token
        );

        let approve_call = erc20.approve(
            Address::from_str(INCH_ROUTER_ADDRESS).unwrap(),
            spend_amount,
        );

        let approve_tx = approve_call.send().await.unwrap_or_else(|err| {
            error!("Failed to approve 1Inch Router: {}", err);
            std::process::exit(1);
        });

        let receipt = approve_tx.await.unwrap_or_else(|err| {
            error!("Failed to get transaction receipt: {}", err);
            std::process::exit(1);
        });

        match receipt {
            Some(_) => {
                info!("<bright-green>Token approved</>");
            }
            _ => {
                error!("<bright-red>Failed to approve token </>");
                std::process::exit(1);
            }
        }
    }
}

pub async fn swap_tokens(
    client: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
    from_token: &str,
    to_token: &str,
    amount: &str,
    slippage: f32,
    allow_max: bool,
) {
    let api = InchApi::new(client, slippage);
    let amount = parse_ether(amount).unwrap();
    let address = api.client.address();

    info!("1Inch API initialized");

    if from_token != INCH_NATIVE_ADDRESS {
        check_allowance_and_approve(
            &api.client,
            address,
            Address::from_str(from_token).unwrap(),
            amount,
            allow_max,
        )
        .await;
    }

    api.swap(from_token, to_token, amount.to_string().as_str())
        .await;
}
