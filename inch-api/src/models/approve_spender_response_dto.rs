/*
 * 1inch Aggregation protocol API
 *
 *  <h2>Ethereum Network</h2> Using 1inch Aggregation protocol API, you can find the best route to exchange assets and make the exchange. <br><br> Step by step: 1. Lookup addresses of tokens you want to swap, for example ‘0xxx’ , ‘0xxxx’ for DAI -> 1INCH 2. Check for allowance of 1inch router contract to spend source asset (/approve/allowance) 3. If necessary, give approval for 1inch router to spend source token (/approve/transaction) 4. Monitor the best exchange route using (/quote) 5. When you ready use to perform swap (/swap)  
 *
 * The version of the OpenAPI document: 5.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApproveSpenderResponseDto {
    /// Address of the 1inch router that must be trusted to spend funds for the exchange
    #[serde(rename = "address")]
    pub address: String,
}

impl ApproveSpenderResponseDto {
    pub fn new(address: String) -> ApproveSpenderResponseDto {
        ApproveSpenderResponseDto {
            address,
        }
    }
}


