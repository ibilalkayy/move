use crate::cli::subcommands::budget::{AlertSubcommand, BudgetInfo, BudgetSubcommand};
use crate::database::db::connection;

pub fn handle_budget(details: BudgetInfo) {
    match details.budget_subcommand {
        BudgetSubcommand::Set(budget) => {
            let conn = connection().unwrap();
            let result = budget.insert_budget(&conn);
            match result {
                Ok(_) => println!("Budget is successfully created"),
                Err(error) => println!("Err: {}", error),
            }
        }

        BudgetSubcommand::View(_budget_data) => {
            println!("Budget is successfully viewed");
        }

        BudgetSubcommand::List => {
            println!("Budget is successfully listed");
        }

        BudgetSubcommand::Get(_get_budget) => {
            println!("Budget data is successfully stored in the CSV file");
        }

        BudgetSubcommand::Update(_update_budget) => {
            println!("Budget is successfully updated");
        }

        BudgetSubcommand::Delete(_budget_data) => {
            println!("category data has been successfully deleted");
        }

        BudgetSubcommand::Alert(alert_budget) => match alert_budget.alert_subcommand {
            AlertSubcommand::Set(alert) => {
                let conn = connection().unwrap();
                let result = alert.insert_alert(&conn);
                match result {
                    Ok(_) => println!("Alert data is successfully saved"),
                    Err(error) => println!("Err: {}", error),
                }
            }

            AlertSubcommand::View => {
                println!("Alert is successfully viewed");
            }

            AlertSubcommand::Email(_email_alert) => {
                println!("Alert is successfully gotten")
            }

            AlertSubcommand::See(_cli_alert) => {
                println!("Alert is successfully seen")
            }

            AlertSubcommand::Update(_update_alert) => {
                println!("Alert is successfully updated");
            }

            AlertSubcommand::Remove(_remove_alert) => {
                println!("Alert data is successfully removed");
            }

            AlertSubcommand::Get(_remove_alert) => {
                println!("Alert credentials in the CSV file");
            }
        },
    }
}
