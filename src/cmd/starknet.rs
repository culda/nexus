use clap::{Arg, ArgMatches};

pub struct CreateAccountArgs<'a> {
    pub index: &'a str,
    pub path: &'a str,
}

pub fn match_create_account_args<'a>(matches: &'a ArgMatches<'a>) -> CreateAccountArgs<'a> {
    let index = matches.value_of("index").unwrap();
    let path = matches
        .value_of("path")
        .unwrap_or("m/44'/9004'/0'/0/{index}");
    CreateAccountArgs { index, path }
}

pub fn create_account_args() -> Vec<Arg<'static, 'static>> {
    vec![
        Arg::with_name("index")
            .short("i")
            .long("index")
            .value_name("Index in derivation path. Default: m/44'/9004'/0'/0/{index}")
            .required(true),
        Arg::with_name("path")
            .short("p")
            .long("path")
            .value_name("Derivation path. Default: m/44'/9004'/0'/0/{index}")
            .required(false),
    ]
}
