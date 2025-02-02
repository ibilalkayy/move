use crate::cli::command::command::Move;
use crate::cli::subcommands::subcommands::Command;
use clap::Parser;

use crate::cli::controller::{
    budget::handle_budget, init::handle_init, spend::handle_spending,
    total_amount::handle_total_amount,
};

pub fn cli() {
    let moves = Move::parse();
    match moves.command {
        Command::Init(details) => handle_init(details),
        Command::TotalAmount(details) => handle_total_amount(details),
        Command::Budget(details) => handle_budget(details),
        Command::Spend(details) => handle_spending(details),
    }
}
