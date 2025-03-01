use crate::cli::subcommands::spend::{GetSubcommand, SpendInfo, SpendSubcommand};
use crate::database::db::connection;
use crate::database::spend::get_all_spending;

pub async fn handle_spending(info: SpendInfo) {
    match info.spend_subcommand {
        SpendSubcommand::Money(spend) => {
            let conn = connection().expect("❌ Failed to establish the DB connection");
            let result = spend.insert_spending(&conn).await;
            match result {
                Ok(_) => (),
                Err(error) => panic!("❌ {}", error),
            }
        }

        SpendSubcommand::History(spend) => {
            let conn = connection().expect("❌ Failed to establish the DB connection");
            let result = spend.view_spending(&conn);
            match result {
                Ok(_) => (),
                Err(error) => panic!("❌ {}", error),
            }
        }

        SpendSubcommand::Delete(spend) => {
            let conn = connection().expect("❌ Failed to establish the DB connection");
            let result = spend.delete_spending(&conn);
            match result {
                Ok(_) => println!("✅ Spending data is successfully deleted"),
                Err(error) => panic!("❌ {}", error),
            }
        }

        SpendSubcommand::Get(spend) => match spend.get_subcommand {
            GetSubcommand::Category(category_spending) => {
                let conn = connection().expect("❌ Failed to establish the DB connection");
                let result = category_spending.get_category_spending(&conn);
                match result {
                    Ok(_) => println!("✅ Spending data is saved in a CSV file"),
                    Err(error) => panic!("❌ {}", error),
                }
            }
            GetSubcommand::All => {
                let conn = connection().expect("❌ Failed to establish the DB connection");
                let result = get_all_spending(&conn);
                match result {
                    Ok(_) => println!("✅ All the spending data is saved in a CSV file"),
                    Err(error) => panic!("❌ {}", error),
                }
            }
        },
    }
}
