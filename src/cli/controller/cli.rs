use crate::cli::command::command::Move;
use crate::cli::subcommands::subcommands::Command;
use crate::database::db::create_table;
use clap::Parser;

use crate::cli::controller::{
    alert::handle_alert, budget::handle_budget, cred::handle_cred, spend::handle_spending,
    total_amount::handle_total_amount, status::handle_status,
};

pub fn cli() {
    let result = create_table();
    match result {
        Ok(_) => (),
        Err(error) => panic!("Err: {}", error),
    }
    let moves = Move::parse();
    match moves.command {
        Command::Cred(details) => handle_cred(details),
        Command::TotalAmount(details) => handle_total_amount(details),
        Command::Budget(details) => handle_budget(details),
        Command::Alert(details) => handle_alert(details),
        Command::Spend(details) => handle_spending(details),
        Command::Status(details) => handle_status(details),
    }
}
