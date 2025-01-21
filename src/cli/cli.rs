use crate::cli::{command::Move, subcommands::Command, subcommands::InitSubcommand};
use crate::db::db::create_table;
use clap::Parser;
use std::{io::Write, fs::OpenOptions};

fn file_options(data_detail: String) {
    let file_path = ".env";
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)
        .expect("failed to open a file");

    file.write_all(data_detail.as_bytes()).expect("failed to write to file");
}

pub fn cli() {
    let moves = Move::parse();
    match moves.command {
        Command::Init(details) => {
            match details.init_subcommand {
                InitSubcommand::Database(db_info) => {
                    let data_detail = format!(
                        "\nDATABASE_URL=postgres://{}:{}@{}:{}/{}",
                        db_info.db_username, db_info.password, db_info.host, db_info.port, db_info.name,
                    );

                    file_options(data_detail);
                    println!("Database data is successfully inserted");

                    let _ = create_table();
                }

                InitSubcommand::Blockchain(b_info) => {
                    let data_detail = format!(
                        "\nPRIVATE_KEY={}\nALCHEMY_URL={}\n", b_info.private_key, b_info.alchemy_url,
                    );
                    
                    file_options(data_detail);
                    println!("Blockchain data is successfully inserted");
                }

                InitSubcommand::Gmail(g_info) => {
                    let data_detail = format!(
                        "\nGMAIL_USERNAME={}\nGMAIL_ADDRESS={}\nGMAIL_APP_PASSWORD={}",
                        g_info.username, g_info.gmail, g_info.app_password,
                    );
                    
                    file_options(data_detail);
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
