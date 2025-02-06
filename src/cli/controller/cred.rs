use crate::cli::subcommands::cred::{CredInfo, CredSubcommand};
use crate::database::db::connection;

pub fn handle_cred(details: CredInfo) {
    match details.cred_subcommand {
        CredSubcommand::Blockchain(blockchain) => {
            let conn = connection().unwrap();
            let result = blockchain.insert_blockchain(&conn);
            match result {
                Ok(_) => println!("Blockchain data is successfully saved"),
                Err(error) => println!("Err: {}", error),
            }
        }

        CredSubcommand::Gmail(gmail) => {
            let conn = connection().unwrap();
            let result = gmail.insert_gmail(&conn);
            match result {
                Ok(_) => println!("Gmail data is successfully saved"),
                Err(error) => println!("Err: {}", error),
            }
        }

        CredSubcommand::GetBlockchain(_cred) => {
            println!("Blockchain credentials in the CSV file");
        }

        CredSubcommand::GetGmail(_cred) => {
            println!("Gmail credentials in the CSV file");
        }
    }
}
