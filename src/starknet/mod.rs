pub mod starkgate;

use bip32::{Mnemonic, XPrv};
use dotenv::dotenv;
use ethers::types::U256;
use num_bigint::BigUint;
use paris::info;
use sha256::digest;
use starknet::{
    accounts::{AccountDeployment, ArgentAccountFactory},
    core::{
        chain_id::{MAINNET, TESTNET},
        types::FieldElement,
    },
    providers::{jsonrpc::HttpTransport, JsonRpcClient},
    signers::{LocalWallet, Signer, SigningKey},
};
use starknet_curve::curve_params::EC_ORDER;
use std::{future::Future, pin::Pin, str::FromStr, thread, time};
use url::Url;

const ARGENT_PROXY_HASH: &str =
    "0x025ec026985a3bf9d0cc1fe17326b245dfdc3ff89b8fde106542a3ea56c5a918";
const ARGENT_IMPL_HASH: &str = "0x033434ad846cdd5f23eb73ff09fe6fddd568284a0fb7d1be20ee482f044dabe2";
const BASE_DERIVATION_PATH: &str = "m/44'/0'/0'/0";
const ARGENT_DERIVATION_PATH: &str = "m/44'/9004'/0'/0";

pub struct StarkClient {
    pub signer: LocalWallet,
    // argent_factory: ArgentAccountFactory<LocalWallet, SequencerGatewayProvider>,
    argent_factory: ArgentAccountFactory<LocalWallet, JsonRpcClient<HttpTransport>>,
}

impl StarkClient {
    pub async fn new(test: bool) -> Self {
        dotenv().ok();
        let chain_id = match test {
            true => TESTNET,
            false => MAINNET,
        };
        let rpc_url = match test {
            true => dotenv!("STARKNET_GOERLI_RPC"),
            false => dotenv!("STARKNET_MAINNET_RPC"),
        };
        let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(rpc_url).unwrap()));

        let phrase = dotenv!("MNEMONIC");
        let mnemonic = Mnemonic::new(&phrase, Default::default()).unwrap();
        let seed = mnemonic.to_seed("");
        let base_deriv =
            XPrv::derive_from_path(&seed, &BASE_DERIVATION_PATH.parse().unwrap()).unwrap();
        let prv_key: [u8; 32] = base_deriv.private_key().to_bytes().into();

        let argent_deriv =
            XPrv::derive_from_path(&prv_key, &ARGENT_DERIVATION_PATH.parse().unwrap()).unwrap();

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

        Self {
            signer,
            argent_factory,
        }
    }

    pub async fn create_argent_deployment<'f, DepositFn>(&'f mut self, deposit_fn: DepositFn)
    where
        DepositFn: FnOnce(U256) -> Pin<Box<dyn Future<Output = ()> + Send + 'f>>,
    {
        let salt = self.signer.get_public_key().await.unwrap().scalar();
        let deployment = AccountDeployment::new(salt, &self.argent_factory);

        let address_str = format!("{:#064x}", deployment.address());
        info!("Argent address: {}", address_str);
        let address = U256::from_str(address_str.as_str()).unwrap();
        deposit_fn(address).await;

        // wait 10 seconds for deposit to be confirmed
        let ten_seconds = time::Duration::from_millis(10);
        thread::sleep(ten_seconds);

        let est_fee = deployment.estimate_fee().await.unwrap();
        info!("Deployment estimated fee: {}", est_fee.overall_fee);
        let result = deployment.send().await;
        match result {
            Ok(tx) => {
                dbg!(tx);
            }
            Err(err) => {
                eprintln!("Error: {err}");
            }
        }
    }
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
