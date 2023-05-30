// https://starkscan.co/tx/0x015f51f91758cc85040fc33cfc66a46e8851542d9efefc4781e952674baf1b89

use std::time::{SystemTime, UNIX_EPOCH};

use ethers::types::U256;
use ethers::utils::parse_units;
use paris::info;
use starknet::accounts::Account;
use starknet::{
    accounts::{Call, SingleOwnerAccount},
    core::{types::FieldElement, utils::get_selector_from_name},
};

use crate::starknet::constants::JEDI_SWAP_ADDRESS;
use crate::starknet::{chain, provider};

use super::StarkClient;

pub struct JediSwap<'a> {
    client: &'a StarkClient,
    from_token: &'a str,
    from_token_decimals: u32,
    to_token: &'a str,
    amount: &'a str,
    slippage: f32,
}

impl<'a> JediSwap<'a> {
    pub fn new(
        client: &'a StarkClient,
        from_token: &'a str,
        from_token_decimals: u32,
        to_token: &'a str,
        amount: &'a str,
        slippage: f32,
    ) -> JediSwap<'a> {
        JediSwap {
            client,
            from_token,
            from_token_decimals,
            to_token,
            amount,
            slippage,
        }
    }
    pub async fn execute(&self, test: bool) {
        let sender_address = self.client.address;
        let provider = provider(test);
        let chain_id = chain(test);

        let account = SingleOwnerAccount::new(
            provider,
            self.client.signer.clone(),
            sender_address,
            chain_id,
        );

        info!(
            "<cyan>Swapping</> {} {} for {}",
            self.amount, self.from_token, self.to_token
        );

        let amount: U256 = parse_units(self.amount, self.from_token_decimals)
            .unwrap()
            .into();

        let amount_out_min = FieldElement::ZERO; // TODO: calculate using slippage and price

        let deadline = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 300;

        let result = account
            .execute(vec![
                Call {
                    to: FieldElement::from_hex_be(self.from_token).unwrap(),
                    selector: get_selector_from_name("approve").unwrap(),
                    calldata: vec![
                        FieldElement::from_hex_be(JEDI_SWAP_ADDRESS).unwrap(),
                        FieldElement::from_dec_str(amount.to_string().as_str()).unwrap(),
                        FieldElement::ZERO,
                    ],
                },
                Call {
                    to: FieldElement::from_hex_be(JEDI_SWAP_ADDRESS).unwrap(),
                    selector: get_selector_from_name("swap_exact_tokens_for_tokens").unwrap(),
                    calldata: vec![
                        FieldElement::from_dec_str(amount.to_string().as_str()).unwrap(),
                        FieldElement::ZERO,
                        amount_out_min,
                        FieldElement::ZERO,
                        FieldElement::TWO,
                        FieldElement::from_hex_be(self.from_token).unwrap(),
                        FieldElement::from_hex_be(self.to_token).unwrap(),
                        sender_address,
                        FieldElement::from(deadline),
                    ],
                },
            ])
            .fee_estimate_multiplier(2.0)
            .send()
            .await
            .unwrap();

        info!("<green>Tx</>: {:#064x}", result.transaction_hash);
    }
}
