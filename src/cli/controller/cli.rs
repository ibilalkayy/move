use crate::cli::command::command::Move;
use crate::cli::subcommands::subcommands::Command;
use clap::Parser;

use crate::cli::controller::{
    budget::handle_budget, cred::handle_cred, 
    total_amount::handle_total_amount,
    spend::handle_spending,
};

pub fn cli() {
    let moves = Move::parse();
    match moves.command {
        Command::Cred(details) => handle_cred(details),
        Command::TotalAmount(details) => handle_total_amount(details),
        Command::Budget(details) => handle_budget(details),
        Command::Spend(details) => handle_spending(details),
    }
}
