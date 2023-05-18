use clap::{App, ArgMatches};

use self::swap::swap_args;

pub mod swap;

pub fn parse_args() -> ArgMatches<'static> {
    App::new("Nexus")
        .about("Defi like a boss")
        .subcommand(App::new("buy").about("Buy a token").args(&swap_args()))
        .subcommand(App::new("sell").about("Sell a token").args(&swap_args()))
        .get_matches()
}
