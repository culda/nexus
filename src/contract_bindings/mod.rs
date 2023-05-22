use ethers::prelude::abigen;

pub mod erc20;
pub mod sync_swap_router;

abigen!(
    StarkgetEthBridge,
    r"[
function deposit(uint256 amount,uint256 l2Recipient)
    ]"
);
