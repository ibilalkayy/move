use crate::cli::subcommands::spend::{GetSubcommand, SpendInfo, SpendSubcommand};
use crate::database::db::connection;
use crate::database::spend::get_all_spending;

pub fn handle_spending(info: SpendInfo) {
    match info.spend_subcommand {
        SpendSubcommand::Money(spend) => {
            let conn = connection().expect("Err: failed to connect to the database");
            let result = spend.insert_spending(&conn);
            match result {
                Ok(_) => (),
                Err(error) => panic!("Err: {}", error),
            }
        }

        SpendSubcommand::History(spend) => {
            let conn = connection().expect("Err: failed to connect to the database");
            let result = spend.view_spending(&conn);
            match result {
                Ok(_) => (),
                Err(error) => panic!("Err: {}", error),
            }
        }

        SpendSubcommand::Delete(spend) => {
            let conn = connection().expect("Err: failed to connect to the database");
            let result = spend.delete_spending(&conn);
            match result {
                Ok(_) => println!("The spending data is successfully deleted"),
                Err(error) => panic!("Err: {}", error),
            }
        }

        SpendSubcommand::Get(spend) => match spend.get_subcommand {
            GetSubcommand::Category(category_spending) => {
                let conn = connection().expect("Err: failed to connect to the database");
                let result = category_spending.get_category_spending(&conn);
                match result {
                    Ok(_) => println!("Spending data is successfully saved in a CSV file"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
            GetSubcommand::All => {
                let conn = connection().expect("Err: failed to connect to the database");
                let result = get_all_spending(&conn);
                match result {
                    Ok(_) => println!("All the spending data is successfully saved in a CSV file"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
        },
    }
}
