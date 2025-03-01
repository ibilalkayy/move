use crate::cli::command::command::Move;
use crate::cli::subcommands::subcommands::Command;
use crate::database::db::create_table;
use clap::Parser;

use crate::cli::controller::{
    budget::handle_budget, cred::handle_cred, spend::handle_spending, status::handle_status,
    total_amount::handle_total_amount,
};

pub async fn cli() {
    let result = create_table();
    match result {
        Ok(_) => (),
        Err(error) => panic!("❌ {}", error),
    }
    let moves = Move::parse();
    match moves.command {
        Command::Cred(details) => handle_cred(details),
        Command::TotalAmount(details) => handle_total_amount(details),
        Command::Budget(details) => handle_budget(details),
        Command::Spend(details) => handle_spending(details).await,
        Command::Status(details) => handle_status(details),
    }
}
