pub mod bridge;
pub mod constants;
pub mod jediswap;

use bip32::{DerivationPath, Mnemonic, XPrv};
use dotenv::dotenv;
use ethers::{
    types::U256,
    utils::{format_units, parse_ether},
};
use num_bigint::BigUint;
use paris::{error, info};
use sha256::digest;
use starknet::{
    accounts::{AccountDeployment, ArgentAccountFactory},
    core::{
        chain_id::{MAINNET, TESTNET},
        types::{BlockId, BlockTag, FieldElement, FunctionCall},
    },
    macros::selector,
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
    signers::{LocalWallet, Signer, SigningKey},
};
use starknet_curve::curve_params::EC_ORDER;
use std::{future::Future, pin::Pin, str::FromStr};
use url::Url;

use crate::starknet::constants::{
    ETH_ADDRESS, ETH_DECIMALS, USDC_ADDRESS, USDC_DECIMALS, WBTC_ADDRESS, WBTC_DECIMALS,
};

const ARGENT_PROXY_HASH: &str =
    "0x025ec026985a3bf9d0cc1fe17326b245dfdc3ff89b8fde106542a3ea56c5a918";
const ARGENT_IMPL_HASH: &str = "0x033434ad846cdd5f23eb73ff09fe6fddd568284a0fb7d1be20ee482f044dabe2";
const BASE_DERIVATION_PATH: &str = "m/44'/0'/0'/0";
const ARGENT_DERIVATION_PATH: &str = "m/44'/9004'/0'/0";

pub struct StarkClient {
    pub signer: LocalWallet,
    argent_factory: ArgentAccountFactory<LocalWallet, JsonRpcClient<HttpTransport>>,
    pub address: FieldElement,
}

impl StarkClient {
    pub async fn new(index: &str, test: bool) -> Self {
        dotenv().ok();
        let chain_id = chain(test);
        let provider = provider(test);

        let phrase = dotenv!("MNEMONIC");
        let mnemonic = Mnemonic::new(&phrase, Default::default()).unwrap();
        let seed = mnemonic.to_seed("");
        let base_deriv =
            XPrv::derive_from_path(&seed, &BASE_DERIVATION_PATH.parse().unwrap()).unwrap();
        let prv_key: [u8; 32] = base_deriv.private_key().to_bytes().into();

        let argent_deriv = XPrv::derive_from_path(
            &prv_key,
            &DerivationPath::from_str(format!("{}/{}", &ARGENT_DERIVATION_PATH, index).as_str())
                .unwrap(),
        )
        .unwrap();

        let prv_key: [u8; 32] = argent_deriv.private_key().to_bytes().into();
        let ground = grind_key(&prv_key);

        let signer = LocalWallet::from(SigningKey::from_secret_scalar(
            FieldElement::from_str(&ground.as_str()).unwrap(),
        ));

        let argent_factory = ArgentAccountFactory::new(
            FieldElement::from_str(ARGENT_PROXY_HASH).unwrap(),
            FieldElement::from_str(ARGENT_IMPL_HASH).unwrap(),
            chain_id,
            FieldElement::ZERO,
            signer.clone(),
            provider,
        )
        .await
        .unwrap();
        let salt = signer.get_public_key().await.unwrap().scalar();
        let deployment = AccountDeployment::new(salt, &argent_factory);
        let address = deployment.address();

        Self {
            signer,
            address,
            argent_factory,
        }
    }

    pub async fn deposit_l1_l2<'f, DepositFn>(
        &'f self,
        deposit_amount_eth: &'f str,
        deposit_fn: DepositFn,
    ) where
        DepositFn: FnOnce(U256, U256, U256) -> Pin<Box<dyn Future<Output = ()> + Send + 'f>>,
    {
        let salt = self.signer.get_public_key().await.unwrap().scalar();
        let deployment = AccountDeployment::new(salt, &self.argent_factory);
        let address = deployment.address();
        let est_fee = deployment.estimate_fee().await.unwrap();
        let l2_fee: u64 = est_fee.overall_fee * 2; // double the fee
        let deposit_amount = U256::from(parse_ether(deposit_amount_eth).unwrap());
        let tx_value = deposit_amount + U256::from(l2_fee);

        let address = U256::from(address.to_bytes_be());
        info!(
            "Deposit amount: {:?}; Value (incl. L2 fee): {:?}",
            deposit_amount, tx_value
        );
        deposit_fn(tx_value, deposit_amount, address).await;
    }

    pub async fn create_argent_deployment(&self) {
        let salt = self.signer.get_public_key().await.unwrap().scalar();
        let deployment = AccountDeployment::new(salt, &self.argent_factory);
        let address = deployment.address();

        info!("Argent address: {:#064x}", address);

        let est_fee = deployment.estimate_fee().await.unwrap();
        info!("Deployment estimated fee: {}", est_fee.overall_fee);

        let result = deployment.fee_estimate_multiplier(2.0).send().await;
        match result {
            Ok(tx) => {
                info!("Deployment tx: {:#064x}", tx.transaction_hash);
            }
            Err(err) => {
                error!("{err}");
            }
        }
    }

    pub async fn info_account(&self, test: bool) {
        dotenv().ok();
        let provider = provider(test);

        info!("<cyan>L2</> address: {:#064x}", self.address);

        let addresses = [
            (WBTC_ADDRESS, "WBTC", WBTC_DECIMALS),
            (ETH_ADDRESS, "ETH", ETH_DECIMALS),
            (USDC_ADDRESS, "USDC", USDC_DECIMALS),
        ];

        let mut balances = Vec::new();

        for (address, symbol, decimals) in &addresses {
            let balance = provider
                .call(
                    FunctionCall {
                        contract_address: FieldElement::from_str(address).unwrap(),
                        entry_point_selector: selector!("balanceOf"),
                        calldata: vec![self.address],
                    },
                    BlockId::Tag(BlockTag::Latest),
                )
                .await
                .unwrap();

            balances.push((symbol, balance, decimals));
        }

        info!("<cyan>L2 balances:</>");
        for (symbol, balance, decimals) in &balances {
            info!(
                "<cyan>{}</>: {}",
                symbol,
                format_units(
                    U256::from_big_endian(&balance[0].to_bytes_be()),
                    **decimals as u32
                )
                .unwrap()
            );
        }
    }
}

pub fn chain(test: bool) -> FieldElement {
    match test {
        true => TESTNET,
        false => MAINNET,
    }
}

pub fn provider(test: bool) -> JsonRpcClient<HttpTransport> {
    let rpc_url = match test {
        true => dotenv!("STARKNET_GOERLI_RPC"),
        false => dotenv!("STARKNET_MAINNET_RPC"),
    };

    JsonRpcClient::new(HttpTransport::new(Url::parse(rpc_url).unwrap()))
}

fn grind_key(key_seed: &[u8; 32]) -> String {
    let key_value_limit = BigUint::from_bytes_be(&EC_ORDER.to_bytes_be());

    let sha256_ec_max_digest = BigUint::parse_bytes(
        b"10000000000000000000000000000000000000000000000000000000000000000",
        16,
    )
    .unwrap();
    let max_allowed_val =
        sha256_ec_max_digest.clone() - (sha256_ec_max_digest.clone() % key_value_limit.clone());

    let mut i = 0;
    let mut key: BigUint;
    loop {
        key = hash_key_with_index(key_seed, i);
        i += 1;
        if key.lt(&max_allowed_val) {
            break;
        }
    }

    format!("0x{}", (key % key_value_limit).to_str_radix(16))
}

fn hash_key_with_index(key: &[u8; 32], index: u8) -> BigUint {
    let sl1: &[u8] = key;
    let sl2: &[u8] = &index.to_ne_bytes();
    let input = [sl1, sl2].concat();
    let result = digest(input.as_slice());
    BigUint::parse_bytes(result.as_bytes(), 16).unwrap()
}
