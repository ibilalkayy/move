use crate::cli::subcommands::cred::{CredInfo, CredSubcommand, AddSubcommand, ViewSubcommand, UpdateSubcommand, DeleteSubcommand};
use crate::database::db::connection;
use crate::database::cred::{view_blockchain, view_gmail, delete_blockchain, delete_gmail};

pub fn handle_cred(details: CredInfo) {
    match details.cred_subcommand {
        CredSubcommand::Add(add) => match add.add_subcommand {
            AddSubcommand::Blockchain(blockchain) => {
                let conn = connection().unwrap();
                let result = blockchain.insert_blockchain(&conn);
                match result {
                    Ok(_) => println!("Blockchain data is successfully saved"),
                    Err(error) => println!("Err: {}", error),
                }
            }

            AddSubcommand::Gmail(gmail) => {
                let conn = connection().unwrap();
                let result = gmail.insert_gmail(&conn);
                match result {
                    Ok(_) => println!("Gmail data is successfully saved"),
                    Err(error) => println!("Err: {}", error),
                }
            }
        }

        CredSubcommand::View(view) => match view.view_subcommand {
            ViewSubcommand::Blockchain => {
                let conn = connection().unwrap();
                let result = view_blockchain(&conn);
                match result {
                    Ok(_) => (),
                    Err(error) => println!("Err: {}", error),
                }
            }

            ViewSubcommand::Gmail => {
                let conn = connection().unwrap();
                let result = view_gmail(&conn);
                match result {
                    Ok(_) => (),
                    Err(error) => println!("Err: {}", error),
                }
            }
        }

        CredSubcommand::Update(update) => match update.update_subcommand {
            UpdateSubcommand::Blockchain(blockchain) => {
                let conn = connection().unwrap();
                let result = blockchain.update_blockchain(&conn);
                match result {
                    Ok(_) => println!("Blockchain data is successfully updated"),
                    Err(error) => println!("Err: {}", error),
                }
            }

            UpdateSubcommand::Gmail(gmail) => {
                let conn = connection().unwrap();
                let result = gmail.update_gmail(&conn);
                match result {
                    Ok(_) => println!("Gmail data is successfully updated"),
                    Err(error) => println!("Err: {}", error),
                }
            }
        }

        CredSubcommand::Delete(delete) => match delete.delete_subcommand {
            DeleteSubcommand::Blockchain => {
                let conn = connection().unwrap();
                let result = delete_blockchain(&conn);
                match result {
                    Ok(_) => println!("Blockchain data is successfully deleted"),
                    Err(error) => println!("Err: {}", error),
                }
            }

            DeleteSubcommand::Gmail => {
                let conn = connection().unwrap();
                let result = delete_gmail(&conn);
                match result {
                    Ok(_) => println!("Gmail data is successfully delete"),
                    Err(error) => println!("Err: {}", error),
                }
            }
        }

        CredSubcommand::GetBlockchain(blockchain) => {
            let conn = connection().unwrap();
            let result = blockchain.get_blockchain(&conn);
            match result {
                Ok(_) => println!("The blockchain data is successfully saved in a CSV file"),
                Err(error) => println!("Error: {}", error),
            }
        }

        CredSubcommand::GetGmail(gmail) => {
            let conn = connection().unwrap();
            let result = gmail.get_gmail(&conn);
            match result {
                Ok(_) => println!("The gmail data is successfully saved in a CSV file"),
                Err(error) => println!("Error: {}", error),
            }
        }
    }
}
