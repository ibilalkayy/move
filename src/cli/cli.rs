use crate::cli::{command::Move, subcommands::Command, subcommands::InitSubcommand};
use crate::db::db::create_table;
use clap::Parser;
use std::{fs::File, io::Write};

pub fn cli() {
    let moves = Move::parse();
    match moves.command {
        Command::Init(details) => {
            match details.init_subcommand {
                InitSubcommand::Database(db_info) => {
                    let data_detail = format!(
                        "DATABASE_URL=postgres://{}:{}@{}:{}/{}",
                        db_info.db_username, db_info.password, db_info.host, db_info.port, db_info.name,
                    );

                    let mut data_file = File::create(".env").expect("Failed to create a file");
                    data_file.write(data_detail.as_bytes()).expect("Failed to write in the file");

                    println!("Database data is successfully inserted");

                    let _ = create_table();
                }

                InitSubcommand::Blockchain(b_info) => {
                    let data_detail = format!(
                        "PRIVATE_KEY={}\nALCHEMY_URL={}\n", b_info.private_key, b_info.alchemy_url,
                    );
                    
                    let mut data_file = File::create(".env").expect("Failed to create a file");
                    data_file.write(data_detail.as_bytes()).expect("Failed to write in the file");

                    println!("Blockchain data is successfully inserted");
                }

                InitSubcommand::Gmail(g_info) => {
                    let data_detail = format!(
                        "GMAIL_USERNAME={}\nGMAIL_ADDRESS={}\nGMAIL_APP_PASSWORD={}",
                        g_info.username, g_info.gmail, g_info.app_password,
                    );
                    
                    let mut data_file = File::create(".env").expect("Failed to create a file");
                    data_file.write(data_detail.as_bytes()).expect("Failed to write in the file");

                    println!("Gmail data is successfully inserted");
                }
            }
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
