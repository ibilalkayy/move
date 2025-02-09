use crate::cli::subcommands::spend::{SpendInfo, SpendSubcommand};
use crate::database::db::connection;

pub fn handle_spending(info: SpendInfo) {
    match info.spend_subcommand {
        SpendSubcommand::Money(spend) => {
            let conn = connection().unwrap();
            let result = spend.insert_spending(&conn);
            match result {
                Ok(_) => println!("Spending data is successfully saved"),
                Err(error) => println!("Err: {}", error),
            }
        }

        SpendSubcommand::History(spend) => {
            let conn = connection().unwrap();
            let result = spend.view_spending(&conn, &spend.category);
            match result {
                Ok(_) => (),
                Err(error) => println!("Err: {}", error),
            }
        }

        SpendSubcommand::Delete(spend) => {
            let conn = connection().unwrap();
            let result = spend.delete_spending(&conn);
            match result {
                Ok(_) => println!("The row(s) in spending is successfully deleted"),
                Err(error) => println!("Error: {}", error),
            }
        }

        SpendSubcommand::Get(spend) => {
            let conn = connection().unwrap();
            let result = spend.get_spending(&conn);
            match result {
                Ok(_) => println!("Spending data is successfully saved in a CSV file"),
                Err(error) => println!("Error: {}", error),
            }
        }
    }
}
