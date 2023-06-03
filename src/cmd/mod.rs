use clap::{App, ArgMatches, SubCommand};

use self::{
    starknet::{create_account_args, deposit_l1_l2_args, info_account_args, swap_args},
    swap::inch_swap_args,
};

pub mod starknet;
pub mod swap;

pub fn parse_args() -> ArgMatches<'static> {
    App::new("Nexus")
        .about("Defi like a boss")
        .subcommand(
            SubCommand::with_name("buy")
                .about("Buy a token")
                .args(&inch_swap_args()),
        )
        .subcommand(
            SubCommand::with_name("sell")
                .about("Sell a token")
                .args(&inch_swap_args()),
        )
        .subcommand(
            SubCommand::with_name("sn")
                .about("Starknet operations")
                .subcommand(
                    SubCommand::with_name("deposit")
                        .about("Deposit to a Starknet account")
                        .args(&deposit_l1_l2_args()),
                )
                .subcommand(
                    SubCommand::with_name("new")
                        .about("Create a new Starknet account")
                        .args(&create_account_args()),
                )
                .subcommand(
                    SubCommand::with_name("info")
                        .about("Get info about an account")
                        .args(&info_account_args()),
                )
                .subcommand(
                    SubCommand::with_name("swap")
                        .about("Swap tokens")
                        .args(&swap_args()),
                ),
        )
        .get_matches()
}
