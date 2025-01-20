use crate::cli::{command::Move, subcommands::Command};
use crate::db::db::create_table;
use clap::Parser;
use std::{fs::File, io::Write};


pub fn cli() {
    let moves = Move::parse();
    match moves.command {
        Command::Init(details) => {
            let data_detail = format!(
                "DATABASE_URL=postgres://{}:{}@{}:{}/{}",
                details.db_username, details.password, details.host, details.port, details.name,
            );

            let mut data_file = File::create(".env").expect("Failed to create a file");
            data_file
                .write(data_detail.as_bytes())
                .expect("Failed to write in the file");

            let _ = create_table();
            println!("Successfully created the table");
        }

        Command::Budget(_details) => {
            println!("Manage the budget");
        }

        Command::Spend(_details) => {
            println!("Manage the spending");
        }

        Command::TotalAmount(_details) => {
            println!("Manage the total amount");
        }
    }
}
