use clap::{App, ArgMatches, SubCommand};

use self::{starknet::create_account_args, swap::swap_args};

pub mod starknet;
pub mod swap;

pub fn parse_args() -> ArgMatches<'static> {
    App::new("Nexus")
        .about("Defi like a boss")
        .subcommand(
            SubCommand::with_name("buy")
                .about("Buy a token")
                .args(&swap_args()),
        )
        .subcommand(
            SubCommand::with_name("sell")
                .about("Sell a token")
                .args(&swap_args()),
        )
        .subcommand(
            SubCommand::with_name("sn")
                .about("Starknet operations")
                .subcommand(
                    SubCommand::with_name("new")
                        .about("Create a new Starknet account")
                        .args(&create_account_args()),
                ),
        )
        .get_matches()
}
