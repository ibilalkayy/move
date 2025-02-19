use crate::cli::subcommands::cred::{
    AddSubcommand, CredInfo, CredSubcommand, DeleteSubcommand, GetSubcommand, UpdateSubcommand,
    ViewSubcommand,
};

use crate::database::{
    cred::{delete_blockchain, delete_gmail, view_blockchain, view_gmail},
    db::connection,
};

pub fn handle_cred(info: CredInfo) {
    match info.cred_subcommand {
        CredSubcommand::Add(cred) => match cred.add_cred {
            AddSubcommand::Blockchain(blockchain) => {
                let conn = connection().expect("failed to connect to the database");
                let result = blockchain.insert_blockchain(&conn);
                match result {
                    Ok(_) => println!("Blockchain data is successfully saved"),
                    Err(error) => panic!("Err: {}", error),
                }
            }

            AddSubcommand::Gmail(cred) => {
                let conn = connection().expect("failed to connect to the database");
                let result = cred.insert_gmail(&conn);
                match result {
                    Ok(_) => println!("Gmail data is successfully saved"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
        },

        CredSubcommand::View(cred) => match cred.view_cred {
            ViewSubcommand::Blockchain => {
                let conn = connection().expect("failed to connect to the database");
                let result = view_blockchain(&conn);
                match result {
                    Ok(_) => (),
                    Err(error) => panic!("Err: {}", error),
                }
            }

            ViewSubcommand::Gmail => {
                let conn = connection().expect("failed to connect to the database");
                let result = view_gmail(&conn);
                match result {
                    Ok(_) => (),
                    Err(error) => panic!("Err: {}", error),
                }
            }
        },

        CredSubcommand::Update(cred) => match cred.update_cred {
            UpdateSubcommand::Blockchain(blockchain) => {
                let conn = connection().expect("failed to connect to the database");
                let result = blockchain.update_blockchain(&conn);
                match result {
                    Ok(_) => println!("Blockchain data is successfully updated"),
                    Err(error) => panic!("Err: {}", error),
                }
            }

            UpdateSubcommand::Gmail(cred) => {
                let conn = connection().expect("failed to connect to the database");
                let result = cred.update_gmail(&conn);
                match result {
                    Ok(_) => println!("Gmail data is successfully updated"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
        },

        CredSubcommand::Delete(cred) => match cred.delete_cred {
            DeleteSubcommand::Blockchain => {
                let conn = connection().expect("failed to connect to the database");
                let result = delete_blockchain(&conn);
                match result {
                    Ok(_) => println!("Blockchain data is successfully deleted"),
                    Err(error) => panic!("Err: {}", error),
                }
            }

            DeleteSubcommand::Gmail => {
                let conn = connection().expect("failed to connect to the database");
                let result = delete_gmail(&conn);
                match result {
                    Ok(_) => println!("Gmail data is successfully deleted"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
        },

        CredSubcommand::Get(cred) => match cred.get_cred {
            GetSubcommand::Blockchain(blockchain) => {
                let conn = connection().expect("failed to connect to the database");
                let result = blockchain.get_blockchain(&conn);
                match result {
                    Ok(_) => println!("The blockchain data is successfully saved in a CSV file"),
                    Err(error) => panic!("Err:: {}", error),
                }
            }

            GetSubcommand::Gmail(cred) => {
                let conn = connection().expect("failed to connect to the database");
                let result = cred.get_gmail(&conn);
                match result {
                    Ok(_) => println!("The gmail data is successfully saved in a CSV file"),
                    Err(error) => panic!("Err:: {}", error),
                }
            }
        },
    }
}
