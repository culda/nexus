mod aggregation_router_v5;
use std::str::FromStr;

use ethers::{
    abi::ethabi::Bytes,
    prelude::{k256, signer::SignerMiddlewareError, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    types::{TransactionRequest, H160, U256},
    utils::{format_units, hex::FromHex, parse_units},
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
    pub client: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
}

impl InchApi {
    pub fn new(
        client: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
        slippage: f32,
    ) -> Self {
        let mut configuration = Configuration::new();
        configuration.base_path = "https://api.1inch.io".to_string();

        InchApi {
            configuration,
            slippage,
            client,
        }
    }

    fn address(&self) -> String {
        format!("{:#x}", self.client.address())
    }

    async fn chain_id(
        &self,
    ) -> Result<String, SignerMiddlewareError<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>
    {
        let id = self.client.get_chainid().await;
        id.map(|id| id.to_string())
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
            self.chain_id().await.unwrap().as_str(),
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
            .value(U256::from_dec_str(swap.value.as_str()).unwrap());

        let tx = self.client.send_transaction(tx, None).await;
        match tx {
            Ok(tx) => info!("<bright-green>Done</> {:?}", tx),
            Err(e) => error!("<bright-red>error</> {:?}", e),
        }
    }

    pub async fn swap(&self, from: &str, to: &str, amount: &str) {
        let swap = self.get_swap(from, to, amount).await.unwrap();

        info!(
            "<cyan>Swapping</> {:?} {} for {:?} {} ...",
            format_units(
                U256::from_dec_str(swap.from_token_amount.as_str()).unwrap(),
                swap.from_token.decimals as u32,
            )
            .unwrap(),
            swap.from_token.symbol,
            format_units(
                U256::from_dec_str(swap.to_token_amount.as_str()).unwrap(),
                swap.to_token.decimals as u32,
            )
            .unwrap(),
            swap.to_token.symbol,
        );

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
        info!(
            "<cyan>Quote</> {} {}",
            parse_units(
                quote.to_token_amount.as_str(),
                quote.to_token.decimals as u32
            )
            .unwrap(),
            quote.to_token.symbol
        );

        swap_api::exchange_controller_get_swap(
            &self.configuration,
            self.chain_id().await.unwrap().as_str(),
            from,
            to,
            amount,
            self.address().as_str(),
            self.slippage,
            None,
            None,
            None,
            None,
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
