/*
 * 1inch Aggregation protocol API
 *
 *  <h2>Ethereum Network</h2> Using 1inch Aggregation protocol API, you can find the best route to exchange assets and make the exchange. <br><br> Step by step: 1. Lookup addresses of tokens you want to swap, for example ‘0xxx’ , ‘0xxxx’ for DAI -> 1INCH 2. Check for allowance of 1inch router contract to spend source asset (/approve/allowance) 3. If necessary, give approval for 1inch router to spend source token (/approve/transaction) 4. Monitor the best exchange route using (/quote) 5. When you ready use to perform swap (/swap)  
 *
 * The version of the OpenAPI document: 5.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SwapResponseDtoTx : Transaction object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SwapResponseDtoTx {
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "data")]
    pub data: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    #[serde(rename = "gas")]
    pub gas: i32,
}

impl SwapResponseDtoTx {
    /// Transaction object
    pub fn new(from: String, to: String, data: String, value: String, gas_price: String, gas: i32) -> SwapResponseDtoTx {
        SwapResponseDtoTx {
            from,
            to,
            data,
            value,
            gas_price,
            gas,
        }
    }
}


