use crate::cli::subcommands::cred::{CredInfo, CredSubcommand};
use crate::data::handle_data::insert_data;

pub fn handle_cred(details: CredInfo) {
    match details.cred_subcommand {
        CredSubcommand::Blockchain(blockchain) => {
            let header = ["Private Key", "Alchemy URL"];
            let blockchain_detail = vec![vec![blockchain.private_key, blockchain.alchemy_url]];

            insert_data(&header, blockchain_detail, blockchain.file_path.as_str()).unwrap();
            println!("Blockchain data is successfully inserted");
        }

        CredSubcommand::Gmail(gmail) => {
            let header = ["Gmail Username", "Gmail Address", "Gmail App Password"];
            let gmail_detail = vec![vec![gmail.username, gmail.address, gmail.app_password]];

            insert_data(&header, gmail_detail, gmail.file_path.as_str()).unwrap();
            println!("Gmail data is successfully inserted");
        }
    }
}
