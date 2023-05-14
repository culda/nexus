use std::time::{SystemTime, UNIX_EPOCH};

use ethers::{
    abi::{encode, encode_packed, ethabi::Bytes, Address, Tokenizable},
    types::U256,
    utils::keccak256,
};

pub mod erc20;

// Not used
pub async fn get_permit_full(
    name: &str,
    sender: Address,
    spender: Address,
    amount: U256,
    chain_id: U256,
    nonce: U256,
    verifying_contract: Address,
) -> [u8; 32] {
    let domain_separator = encode_domain_separator(name, chain_id, verifying_contract);
    let deadline: U256 = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 300)
        .into();

    let permit_hash = keccak256(encode(&[
        keccak256(
            "Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)",
        )
        .into_token(),
        sender.into_token(),
        spender.into_token(),
        amount.into_token(),
        nonce.into_token(),
        deadline.into_token(),
    ]));

    let encoded_permit = encode_packed(&[
        0x0901.into_token(),
        domain_separator.into_token(),
        permit_hash.into_token(),
    ])
    .unwrap();

    keccak256(encoded_permit)
}

fn encode_domain_separator(name: &str, chain_id: U256, verifying_contract: Address) -> Bytes {
    let domain_separator = encode(&[
        keccak256(
            "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)",
        )
        .into_token(),
        keccak256(name).into_token(),
        keccak256("1").into_token(),
        chain_id.into_token(),
        verifying_contract.into_token(),
    ]);

    keccak256(domain_separator).to_vec()
}
