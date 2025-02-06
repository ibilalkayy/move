use crate::cli::subcommands::spend::{SpendInfo, SpendSubcommand};

pub fn handle_spending(details: SpendInfo) {
    match details.spend_subcommand {
        SpendSubcommand::Money(_spend_data) => {
            println!("Spending data is successfully saved");
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

        SpendSubcommand::Get(_spending) => {
            println!("get subcommand");
        }
    }
}
