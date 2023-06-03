use ethers::types::Chain;
use paris::error;

pub const ETH_MAINNET_RPC: &str = "https://eth.llamarpc.com";
pub const ETH_TESTNET_RPC: &str = "https://rpc.ankr.com/eth_goerli";
pub const ZKSYNC_MAINNET_RPC: &str = "https://mainnet.era.zksync.io";
pub const ZKSYNC_TESTNET_RPC: &str = "https://testnet.era.zksync.dev";

pub const SLIPPAGE_TOLERANCE: f64 = 0.05;

pub const INCH_ROUTER_ADDRESS: &str = "0x1111111254eeb25477b68fb85ed929f73a960582";
pub const INCH_EXECUTOR_ETH_ADDRESS: &str = "0x1136B25047E142Fa3018184793aEc68fBB173cE4";
pub const INCH_NATIVE_ADDRESS: &str = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";

pub const USDC_ZK_ADDRESS: &str = "0x3355df6D4c9C3035724Fd0e3914dE96A5a83aaf4";
pub const USDC_DECIMALS: u32 = 6;
pub const USDC_ETH_ADDRESS: &str = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";

pub const MUTE_ROUTER_ADDRESS: &str = "0x8B791913eB07C32779a16750e3868aA8495F5964";
pub const MUTE_ETH_ADDRESS: &str = "0xa49d7499271ae71cd8ab9ac515e6694c755d400c";
pub const MUTE_ZK_ADDRESS: &str = "0x0e97C7a0F8B2C9885C8ac9fC6136e829CbC21d42";
pub const MUTE_DAO_ADDRESS: &str = "0x4336e06Be4F62bD757c4248c48D4C0b32615A2Df";
pub const MUTE_DECIMALS: u32 = 18;

pub const WETH_ARB_ADDRESS: &str = "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1";
pub const WETH_ZK_ADDRESS: &str = "0x5aea5775959fbc2557cc8789bc1bf90a239d9a91";
pub const WETH_ETH_ADDRESS: &str = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
pub const WETH_DECIMALS: u32 = 18;

pub const SYNCSWAP_ROUTER_ADDRESS: &str = "0x2da10A1e27bF85cEdD8FFb1AbBe97e53391C0295";
pub const USDC_ETH_POOL_ADDRESS: &str = "0x80115c708E12eDd42E504c1cD52Aea96C547c05c";
pub const USDC_ETH_POOL_DECIMALS: u32 = 18;

pub const ERA_NAME_SERVICE_ADDRESS: &str = "0x935442AF47F3dc1c11F006D551E13769F12eab13";

pub const STARK_WBTC_ADDRESS: &str =
    "0x03fe2b97c1fd336e750087d68b9b867997fd64a2661ff3ca5a7c771641e8e7ac";
pub const STARK_ETH_ADDRESS: &str =
    "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
pub const STARK_USDC_ADDRESS: &str =
    "0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8";

pub fn weth_address(chain: Chain) -> &'static str {
    match chain {
        Chain::Mainnet => WETH_ETH_ADDRESS,
        Chain::ZkSync => WETH_ZK_ADDRESS,
        Chain::Arbitrum => WETH_ARB_ADDRESS,
        _ => {
            error!("unknown WETH address for chain: {}", chain);
            std::process::exit(1);
        }
    }
}
