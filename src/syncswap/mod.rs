use std::str::FromStr;

use web3::types::{Address, U256};
use web3::Web3;

pub async fn get_balance(
    web3: &Web3<web3::transports::Http>,
    address: &str,
) -> Result<U256, Box<dyn std::error::Error>> {
    let address = Address::from_str(address)?;
    let balance = web3.eth().balance(address, None).await?;
    Ok(balance)
}
