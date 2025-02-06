use crate::cli::subcommands::cred::{CredInfo, CredSubcommand};

pub fn handle_cred(details: CredInfo) {
    match details.cred_subcommand {
        CredSubcommand::Blockchain(_blockchain) => {
            println!("Blockchain data is successfully inserted");
        }

        CredSubcommand::Gmail(_gmail) => {
            println!("Gmail data is successfully inserted");
        }

        CredSubcommand::GetBlockchain(_cred) => {
            println!("Blockchain credentials in the CSV file");
        }

        CredSubcommand::GetGmail(_cred) => {
            println!("Gmail credentials in the CSV file");
        }
    }
}
