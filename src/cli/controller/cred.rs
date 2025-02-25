use crate::cli::subcommands::cred::{CredInfo, CredSubcommand};

use crate::database::{
    cred::{delete_blockchain, view_blockchain},
    db::connection,
};

pub fn handle_cred(info: CredInfo) {
    match info.cred_subcommand {
        CredSubcommand::Add(cred) => {
            let conn = connection().expect("Err: failed to connect to the database");
            let result = cred.insert_blockchain(&conn);
            match result {
                Ok(_) => println!("Blockchain data is successfully saved"),
                Err(error) => panic!("Err: {}", error),
            }
        }

        CredSubcommand::View => {
            let conn = connection().expect("Err: failed to connect to the database");
            let result = view_blockchain(&conn);
            match result {
                Ok(_) => (),
                Err(error) => panic!("Err: {}", error),
            }
        }

        CredSubcommand::Update(cred) => {
            let conn = connection().expect("Err: failed to connect to the database");
            let result = cred.update_blockchain(&conn);
            match result {
                Ok(_) => println!("Blockchain data is successfully updated"),
                Err(error) => panic!("Err: {}", error),
            }
        }

        CredSubcommand::Delete => {
            let conn = connection().expect("Err: failed to connect to the database");
            let result = delete_blockchain(&conn);
            match result {
                Ok(_) => println!("Blockchain data is successfully deleted"),
                Err(error) => panic!("Err: {}", error),
            }
        }
    }
}
