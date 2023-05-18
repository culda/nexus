use std::str::FromStr;

use dotenv::dotenv;
use ethers::utils::hex;
use ethers_signers::{coins_bip39::English, MnemonicBuilder};
use starknet::{
    accounts::{AccountFactory, ArgentAccountFactory},
    core::{chain_id, types::FieldElement},
    macros::felt,
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, SigningKey},
};

use crate::constants::{ARGENT_IMPL_HASH, ARGENT_PROXY_HASH};

// async fn get_balance(address) {
//     let balance = IERC20Dispatcher{contract_address: erc20_address}.balanceOf(worker_address);

// use starknet::ContractAddress;

// #[abi]
// trait IERC20 {
//     fn name() -> felt252;
//     fn symbol() -> felt252;
//     fn decimals() -> u8;
//     fn totalSupply() -> u256;
//     fn balanceOf(account: ContractAddress) -> u256;
//     fn allowance(owner: ContractAddress, spender: ContractAddress) -> u256;
//     fn transfer(recipient: ContractAddress, amount: u256);
//     fn transferFrom(sender: ContractAddress, recipient: ContractAddress, amount: u256);
//     fn approve(spender: ContractAddress, amount: u256);
// }
//     return web3.eth.getBalance(address);
// }

pub struct StarkClient {
    pub signer: LocalWallet,
    pub provider: SequencerGatewayProvider,
}

impl StarkClient {
    pub fn new(test: bool) -> Self {
        dotenv().ok();
        let provider = match test {
            true => SequencerGatewayProvider::starknet_alpha_goerli(),
            false => SequencerGatewayProvider::starknet_alpha_mainnet(),
        };

        let phrase = dotenv!("MNEMONIC");
        let builder = MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .derivation_path(format!("m/44'/9004'/0'/0/{}", 0).as_str())
            .unwrap();

        let private_key = builder.build().unwrap().signer().to_bytes();
        let hex = hex::encode(private_key);

        // let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        //     FieldElement::from_byte_slice_be(&private_key).unwrap(),
        // ));
        println!("hex: {:?}", hex);

        let signer = LocalWallet::from(SigningKey::from_secret_scalar(
            FieldElement::from_hex_be(hex.as_str()).unwrap(),
        ));

        StarkClient { provider, signer }
    }

    pub async fn create_argent_account(&self) {
        // Anything you like here as salt
        let salt = felt!("12345678");

        let factory = ArgentAccountFactory::new(
            FieldElement::from_str(ARGENT_PROXY_HASH).unwrap(),
            FieldElement::from_str(ARGENT_IMPL_HASH).unwrap(),
            chain_id::TESTNET,
            FieldElement::ZERO,
            self.signer.clone(),
            &self.provider,
        )
        .await
        .unwrap();

        let deployment = factory.deploy(salt);

        let est_fee = deployment.estimate_fee().await.unwrap();

        // In an actual application you might want to add a buffer to the amount
        println!(
            "Fund at least {} wei to {:#064x}",
            est_fee.overall_fee,
            deployment.address()
        );
        println!("Press ENTER after account is funded to continue deployment...");
        std::io::stdin().read_line(&mut String::new()).unwrap();

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
