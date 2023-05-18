use clap::{Arg, ArgMatches};
use ethers::types::Chain;
use paris::error;

pub struct SwapArgs<'a> {
    pub token: &'a str,
    pub chain: Chain,
    pub amount: &'a str,
    pub slippage: f32,
    pub allow_max: bool,
    pub native: bool,
}

pub fn match_swap_args<'a>(matches: &'a ArgMatches<'a>) -> SwapArgs<'a> {
    let token = matches.value_of("token").unwrap();
    let chain = matches
        .value_of("chain")
        .unwrap()
        .parse::<Chain>()
        .unwrap_or_else(|_| {
            error!("Invalid chain name. Valid chains: mainnet / goerli / zksync / arbitrum");
            std::process::exit(1);
        });
    let amount = matches.value_of("amount").unwrap();
    let slippage = matches
        .value_of("slippage")
        .unwrap_or("0.02")
        .parse::<f32>()
        .unwrap_or_else(|_| {
            error!("Invalid slippage value");
            std::process::exit(1);
        });
    let allow_max = matches
        .value_of("allowmax")
        .unwrap_or("true")
        .parse::<bool>()
        .unwrap();
    let native = matches
        .value_of("native")
        .unwrap_or("true")
        .parse::<bool>()
        .unwrap();
    SwapArgs {
        token: token,
        chain: chain,
        amount: amount,
        slippage: slippage,
        allow_max: allow_max,
        native: native,
    }
}

pub fn swap_args() -> Vec<Arg<'static, 'static>> {
    vec![
        Arg::with_name("token")
            .short("t")
            .long("token")
            .value_name("Token contract address")
            .required(true),
        Arg::with_name("chain")
            .short("c")
            .long("chain")
            .value_name("mainnet / goerli / zksync / arbitrum")
            .takes_value(true)
            .required(true),
        Arg::with_name("amount")
            .short("a")
            .long("amount")
            .value_name("ETH amount in human format")
            .takes_value(true)
            .required(true),
        Arg::with_name("slippage")
            .short("s")
            .long("slippage")
            .value_name("float format. default: 0.02")
            .takes_value(true),
        Arg::with_name("native")
            .short("n")
            .long("native")
            .value_name("true / false. default: true. use native token instead of WETH")
            .takes_value(true),
        Arg::with_name("allowmax").long("allowmax"),
    ]
}
