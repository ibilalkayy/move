use crate::cli::subcommands::cred::{CredInfo, CredSubcommand};

use crate::database::{
    cred::{delete_blockchain, view_blockchain},
    db::connection,
};

pub fn handle_cred(info: CredInfo) {
    match info.cred_subcommand {
        CredSubcommand::Add(cred) => {
            let conn = connection().expect("❌ Failed to establish the DB connection");
            let result = cred.insert_blockchain(&conn);
            match result {
                Ok(_) => println!(
                    "
✅ Blockchain data is successfully saved\n
Keep the above 2 keys. First is for private key, and second is for alchemy url.
They're not stored in the database. You will be asked these keys in the time of spending. 
                "
                ),
                Err(error) => panic!("❌ {}", error),
            }
        }

        CredSubcommand::View => {
            let conn = connection().expect("❌ Failed to establish the DB connection");
            let result = view_blockchain(&conn);
            match result {
                Ok(_) => (),
                Err(error) => panic!("❌ {}", error),
            }
        }

        CredSubcommand::Update(cred) => {
            let conn = connection().expect("❌ Failed to establish the DB connection");
            let result = cred.update_blockchain(&conn);
            match result {
                Ok(_) => println!(
                    "
✅ Blockchain data is successfully updated\n
Keep the above 2 keys. First is for private key, and second is for alchemy url.
They're not stored in the database. You will be asked these keys in the time of spending
                "
                ),
                Err(error) => panic!("❌ {}", error),
            }
        }

        CredSubcommand::Delete => {
            let conn = connection().expect("❌ Failed to establish the DB connection");
            let result = delete_blockchain(&conn);
            match result {
                Ok(_) => println!("✅ Blockchain data is successfully deleted"),
                Err(error) => panic!("❌ {}", error),
            }
        }
    }
}
