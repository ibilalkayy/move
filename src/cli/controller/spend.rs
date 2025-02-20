use crate::cli::subcommands::spend::{SpendInfo, SpendSubcommand};
use crate::database::db::connection;

pub fn handle_spending(info: SpendInfo) {
    match info.spend_subcommand {
        SpendSubcommand::Money(spend) => {
            let conn = connection().expect("failed to connect to the database");
            let result = spend.insert_spending(&conn);
            match result {
                Ok(_) => println!("Money is successfully spent on a particular category"),
                Err(error) => panic!("Err: {}", error),
            }
        }

        SpendSubcommand::History(spend) => {
            let conn = connection().expect("failed to connect to the database");
            let result = spend.view_spending(&conn);
            match result {
                Ok(_) => (),
                Err(error) => panic!("Err: {}", error),
            }
        }

        SpendSubcommand::Delete(spend) => {
            let conn = connection().expect("failed to connect to the database");
            let result = spend.delete_spending(&conn);
            match result {
                Ok(_) => println!("The row(s) in spending is successfully deleted"),
                Err(error) => panic!("Err: {}", error),
            }
        }

        SpendSubcommand::Get(spend) => {
            let conn = connection().expect("failed to connect to the database");
            let result = spend.get_spending(&conn);
            match result {
                Ok(_) => println!("Spending data is successfully saved in a CSV file"),
                Err(error) => panic!("Err: {}", error),
            }
        }
    }
}
