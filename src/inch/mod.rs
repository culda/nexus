mod aggregation_router_v5;

use std::{hash::Hash, str::FromStr};

use ethers::{
    abi::ethabi::Bytes,
    prelude::{k256, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    types::{transaction::eip2718::TypedTransaction, TransactionRequest, H160, U256, U64},
    utils::hex::FromHex,
};

use ethers_signers::Wallet;
use inch_api::{
    apis::{configuration::Configuration, swap_api},
    models::SwapResponseDtoTx,
};
use paris::{error, info};

pub struct InchApi {
    configuration: Configuration,
    slippage: f32,
    fee: f32,
    client: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
}

impl InchApi {
    pub fn new(client: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>) -> Self {
        let mut configuration = Configuration::new();
        configuration.base_path = "https://api.1inch.io".to_string();
        let slippage = 0.05;
        let fee = f32::min(f32::max(0.2, slippage * 0.1), 3.0);

        InchApi {
            configuration,
            slippage,
            fee,
            client,
        }
    }

    fn address(&self) -> String {
        format!("{:#x}", self.client.address())
    }

    async fn chain_id_u64(&self) -> U64 {
        let chain_id = self.client.get_chainid().await;
        match chain_id {
            Ok(id) => id.as_u64().into(),
            Err(e) => {
                println!("error {:?}", e);
                return U64::zero();
            }
        }
    }

    async fn get_quote(
        &self,
        from: &str,
        to: &str,
        amount: &str,
    ) -> std::result::Result<
        inch_api::models::QuoteResponseDto,
        inch_api::apis::Error<inch_api::apis::swap_api::ExchangeControllerGetQuoteError>,
    > {
        swap_api::exchange_controller_get_quote(
            &self.configuration,
            from,
            to,
            amount,
            None,
            None,
            None,
            None,
            Some(3.0),
            None,
            None,
            None,
        )
        .await
    }

    async fn send_swap(&self, swap: SwapResponseDtoTx) {
        let tx = TransactionRequest::new()
            .to(H160::from_str(swap.to.as_str()).unwrap())
            .data(Bytes::from_hex(swap.data[2..].to_string()).unwrap())
            .gas(U256::from(swap.gas))
            .gas_price(U256::from_dec_str(swap.gas_price.as_str()).unwrap())
            .from(self.client.address())
            .value(U256::from_str(swap.value.as_str()).unwrap());

        info!("Transaction request {:?}", tx);

        let tx = self.client.send_transaction(tx, None).await;
        match tx {
            Ok(tx) => info!("Sent: {:?}", tx),
            Err(e) => error!("error {:?}", e),
        }
    }

    pub async fn swap(&self, from: &str, to: &str, amount: &str) {
        info!("Swapping {} {} for {} ...", amount, from, to);
        let swap = self.get_swap(from, to, amount).await.unwrap();
        self.send_swap(*swap.tx).await;
    }

    async fn get_swap(
        &self,
        from: &str,
        to: &str,
        amount: &str,
    ) -> std::result::Result<
        inch_api::models::SwapResponseDto,
        inch_api::apis::Error<inch_api::apis::swap_api::ExchangeControllerGetSwapError>,
    > {
        let quote = &self.get_quote(from, to, amount).await.unwrap();
        info!("Estimted amount: {:?}", quote.to_token_amount);
        info!("Preparing swap ... ");
        swap_api::exchange_controller_get_swap(
            &self.configuration,
            from,
            to,
            amount,
            self.address().as_str(),
            self.slippage,
            None,
            None,
            None,
            Some(self.fee.to_string().as_str()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(3.0),
            Some(quote.estimated_gas.to_string().as_str()),
            None,
        )
        .await
    }
}
