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

pub struct NexusClient {
    pub signer: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
}

impl NexusClient {
    pub async fn new(chain: Chain) -> Self {
        let rpc_url = match chain {
            Chain::Mainnet => dotenv!("MAINNET_RPC"),
            Chain::Goerli => dotenv!("GOERLI_RPC"),
            Chain::ZkSync => "https://mainnet.era.zksync.io",
            Chain::Arbitrum => "https://arb1.arbitrum.io/rpc",
            _ => {
                error!("Unsupported chain: {}", chain);
                std::process::exit(1);
            }
        };

        let phrase = dotenv!("MNEMONIC");

        let builder = MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .derivation_path(format!("m/44'/60'/0'/0/{}", 0).as_str())
            .unwrap();

        let wallet = builder.build().unwrap().with_chain_id(chain);

        let mut provider = Provider::connect(rpc_url).await;
        provider.set_chain(chain);

        let signer = SignerMiddleware::new(provider.clone(), wallet.clone());

        NexusClient { signer }
    }

    pub fn address(&self) -> String {
        format!("{:#x}", self.signer.address())
    }
}
