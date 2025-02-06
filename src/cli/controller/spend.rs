use crate::cli::subcommands::spend::{SpendInfo, SpendSubcommand};
use crate::database::db::connection;

pub fn handle_spending(details: SpendInfo) {
    match details.spend_subcommand {
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
            let _ = spend.view_spending(&conn, &spend.category);
        }

        SpendSubcommand::Remove => {
            println!("remove command");
        }

        SpendSubcommand::Show => {
            println!("show command");
        }

        SpendSubcommand::Get(_spending) => {
            println!("get subcommand");
        }
    }
}
