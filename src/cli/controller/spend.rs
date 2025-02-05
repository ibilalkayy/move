use crate::cli::subcommands::spend::{SpendInfo, SpendSubcommand};
use crate::data::handle_data::insert_data;

pub fn handle_spending(details: SpendInfo) {
    match details.spend_subcommand {
        SpendSubcommand::Money(spending) => {
            let header = ["Category", "Amount"];
            let spending_data = vec![vec![spending.category, spending.amount]];
            insert_data(&header, spending_data, "spending_data.csv", "Spending").unwrap();
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
