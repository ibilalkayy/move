use crate::cli::subcommands::spend::{SpendInfo, SpendSubcommand};
use crate::database::db::create_table;

pub fn handle_spending(details: SpendInfo) {
    match details.spend_subcommand {
        SpendSubcommand::Money(spend_data) => {
            let _ = create_table();
            let result = spend_data.insert_spending();
            match result {
                Ok(_) => println!("Spending data is successfully saved"),
                Err(err) => println!("Error: {}", err),
            }
        }

        SpendSubcommand::History => {
            println!("history command");
        }

        SpendSubcommand::Remove => {
            println!("remove command");
        }

        SpendSubcommand::Show => {
            println!("show command");
        }
    }
}
