use dotenv::dotenv;
use std::env;

use ethers::{
    prelude::{
        k256::{self},
        SignerMiddleware,
    },
    providers::{Http, Provider, ProviderExt},
    types::Chain,
};
use ethers_signers::{coins_bip39::English, MnemonicBuilder, Signer, Wallet};
use paris::error;

pub type EvmSigner = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;
pub struct EvmClient {
    pub signer: EvmSigner,
}

impl EvmClient {
    pub async fn new(chain: Chain, index: &str) -> Self {
        dotenv().ok();
        let rpc_url = match chain {
            Chain::Mainnet => match env::var("MAINNET_RPC") {
                Ok(val) => val,
                Err(_) => "https://eth.llamarpc.com".to_string(),
            },
            Chain::Goerli => match env::var("GOERLI_RPC") {
                Ok(val) => val,
                Err(_) => "https://goerli.blockpi.network/v1/rpc/public	".to_string(),
            },
            Chain::ZkSync => match env::var("ZKSYNC_RPC") {
                Ok(val) => val,
                Err(_) => "https://mainnet.era.zksync.io".to_string(),
            },
            Chain::Arbitrum => match env::var("ARBITRUM_RPC") {
                Ok(val) => val,
                Err(_) => "https://arb1.arbitrum.io/rpc".to_string(),
            },
            _ => {
                error!("Unsupported chain: {}", chain);
                std::process::exit(1);
            }
        };

        let phrase = dotenv!("MNEMONIC");

        let builder = MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .derivation_path(format!("m/44'/60'/0'/0/{}", index).as_str())
            .unwrap();

        let wallet = builder.build().unwrap().with_chain_id(chain);

        let mut provider = Provider::connect(rpc_url.as_str()).await;
        provider.set_chain(chain);

        let signer = SignerMiddleware::new(provider.clone(), wallet.clone());

        EvmClient { signer }
    }

    pub fn address(&self) -> String {
        format!("{:#x}", self.signer.address())
    }
}
