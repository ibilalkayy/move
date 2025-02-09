use crate::cli::subcommands::cred::{
    CredInfo, CredSubcommand, AddSubcommand, ViewSubcommand,
    UpdateSubcommand, DeleteSubcommand, GetSubcommand,
};

use crate::database::{
    db::connection, 
    cred::{view_blockchain, view_gmail, delete_blockchain, delete_gmail},
};

pub fn handle_cred(info: CredInfo) {
    match info.cred_subcommand {
        CredSubcommand::Add(cred) => match cred.add_cred {
            AddSubcommand::Blockchain(blockchain) => {
                let conn = connection().unwrap();
                let result = blockchain.insert_blockchain(&conn);
                match result {
                    Ok(_) => println!("Blockchain data is successfully saved"),
                    Err(error) => println!("Err: {}", error),
                }
            }

            AddSubcommand::Gmail(cred) => {
                let conn = connection().unwrap();
                let result = cred.insert_gmail(&conn);
                match result {
                    Ok(_) => println!("Gmail data is successfully saved"),
                    Err(error) => println!("Err: {}", error),
                }
            }
        }

        CredSubcommand::View(cred) => match cred.view_cred {
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

        CredSubcommand::Update(cred) => match cred.update_cred {
            UpdateSubcommand::Blockchain(blockchain) => {
                let conn = connection().unwrap();
                let result = blockchain.update_blockchain(&conn);
                match result {
                    Ok(_) => println!("Blockchain data is successfully updated"),
                    Err(error) => println!("Err: {}", error),
                }
            }

            UpdateSubcommand::Gmail(cred) => {
                let conn = connection().unwrap();
                let result = cred.update_gmail(&conn);
                match result {
                    Ok(_) => println!("Gmail data is successfully updated"),
                    Err(error) => println!("Err: {}", error),
                }
            }
        }

        CredSubcommand::Delete(cred) => match cred.delete_cred {
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
                    Ok(_) => println!("Gmail data is successfully deleted"),
                    Err(error) => println!("Err: {}", error),
                }
            }
        }

        CredSubcommand::Get(cred) => match cred.get_cred {
            GetSubcommand::Blockchain(blockchain) => {
                let conn = connection().unwrap();
                let result = blockchain.get_blockchain(&conn);
                match result {
                    Ok(_) => println!("The blockchain data is successfully saved in a CSV file"),
                    Err(error) => println!("Error: {}", error),
                }
            }

            GetSubcommand::Gmail(cred) => {
                let conn = connection().unwrap();
                let result = cred.get_gmail(&conn);
                match result {
                    Ok(_) => println!("The gmail data is successfully saved in a CSV file"),
                    Err(error) => println!("Error: {}", error),
                }
            }
        }
    }
}
