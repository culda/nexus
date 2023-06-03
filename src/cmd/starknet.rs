use clap::{Arg, ArgMatches};
use paris::error;

pub struct CreateAccountArgs<'a> {
    pub index: &'a str,
}

pub struct InfoAccountArgs<'a> {
    pub index: &'a str,
}

pub struct DepositArgs<'a> {
    pub index: &'a str,
    pub amount: &'a str,
}
pub struct SwapArgs<'a> {
    pub index: &'a str,
    pub from_token: &'a str,
    pub from_token_decimals: u32,
    pub to_token: &'a str,
    pub amount: &'a str,
    pub slippage: f32,
}

pub fn match_create_account_args<'a>(matches: &'a ArgMatches<'_>) -> CreateAccountArgs<'a> {
    let index = matches.value_of("index").unwrap();
    CreateAccountArgs { index }
}

pub fn match_deposit_args<'a>(matches: &'a ArgMatches<'_>) -> DepositArgs<'a> {
    let index = matches.value_of("index").unwrap();
    let amount = matches.value_of("amount").unwrap();
    DepositArgs { index, amount }
}

pub fn match_info_account_args<'a>(matches: &'a ArgMatches<'_>) -> InfoAccountArgs<'a> {
    let index = matches.value_of("index").unwrap();
    InfoAccountArgs { index }
}

pub fn match_swap_args<'a>(matches: &'a ArgMatches<'_>) -> SwapArgs<'a> {
    let index = matches.value_of("index").unwrap();
    let from_token = matches.value_of("from_token").unwrap();
    let from_token_decimals = matches
        .value_of("from_token_decimals")
        .unwrap_or("18")
        .parse::<u32>()
        .unwrap_or_else(|_| {
            error!("Invalid decimals value");
            std::process::exit(1);
        });
    let to_token = matches.value_of("to_token").unwrap();
    let amount = matches.value_of("amount").unwrap();
    let slippage = matches
        .value_of("slippage")
        .unwrap_or("0.02")
        .parse::<f32>()
        .unwrap_or_else(|_| {
            error!("Invalid slippage value");
            std::process::exit(1);
        });
    SwapArgs {
        index,
        from_token,
        from_token_decimals,
        to_token,
        amount,
        slippage,
    }
}

pub fn deposit_from_l1() -> Vec<Arg<'static, 'static>> {
    vec![
        Arg::with_name("index")
            .short("i")
            .long("index")
            .value_name("Index in derivation path")
            .required(true)
            .help("Index in derivation path. Default: m/44'/9004'/0'/0/{index}"),
        Arg::with_name("amount")
            .short("a")
            .long("amount")
            .value_name("Amount")
            .required(true)
            .help("Amount to deposit"),
    ]
}

pub fn create_account_args() -> Vec<Arg<'static, 'static>> {
    vec![
        Arg::with_name("index")
            .short("i")
            .long("index")
            .value_name("Index in derivation path")
            .required(true)
            .help("Index in derivation path. Default: m/44'/9004'/0'/0/{index}"),
        Arg::with_name("deposit")
            .short("d")
            .long("deposit")
            .value_name("Deposit amount")
            .help("Deposit amount. Only deploy fee is deposited by default"),
        Arg::with_name("no_deposit")
            .short("nod")
            .long("no-deposit")
            .help("Skip the L1->L2 bridge"),
    ]
}

pub fn info_account_args() -> Vec<Arg<'static, 'static>> {
    create_account_args()
}

pub fn swap_args() -> Vec<Arg<'static, 'static>> {
    vec![
        Arg::with_name("index")
            .short("i")
            .long("index")
            .value_name("Index in derivation path")
            .required(true)
            .help("Index in derivation path. Default: m/44'/9004'/0'/0/{index}"),
        Arg::with_name("from_token")
            .short("from")
            .long("from")
            .value_name("From token")
            .required(true)
            .help("From token"),
        Arg::with_name("from_token_decimals")
            .short("d")
            .long("decimals")
            .value_name("From token decimals")
            .help("From token decimals"),
        Arg::with_name("to_token")
            .short("to")
            .long("to")
            .value_name("To token")
            .required(true)
            .help("To token"),
        Arg::with_name("amount")
            .short("a")
            .long("amount")
            .value_name("Amount")
            .required(true)
            .help("Amount"),
        Arg::with_name("slippage")
            .short("s")
            .long("slippage")
            .value_name("Slippage")
            .help("Slippage. Defaults to 0.02"),
    ]
}
