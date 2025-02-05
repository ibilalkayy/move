use crate::cli::subcommands::budget::{AlertSubcommand, BudgetInfo, BudgetSubcommand};
use crate::data::handle_data::insert_data;

pub fn handle_budget(details: BudgetInfo) {
    match details.budget_subcommand {
        BudgetSubcommand::Create(create_budget) => {
            let header = ["Category", "Amount"];
            let budget_data = vec![vec![create_budget.category, create_budget.amount]];
            insert_data(&header, budget_data, "budget_data.csv", "Budget").unwrap();
        }

        BudgetSubcommand::View(_budget_data) => {
            println!("Budget is successfully viewed");
        }

        BudgetSubcommand::List => {
            println!("Budget is successfully listed");
        }

        BudgetSubcommand::Update(_update_budget) => {
            println!("Budget is successfully updated");
        }

        BudgetSubcommand::Delete(_budget_data) => {
            println!("category data has been successfully deleted");
        }

        BudgetSubcommand::Alert(alert_budget) => match alert_budget.alert_subcommand {
            AlertSubcommand::Set(alert_data) => {
                let header = ["Category", "Frequency", "Method", "Day", "Minute", "Second", "Weekday"];
                let alert_data = vec![vec![
                    alert_data.category, alert_data.frequency, alert_data.method, 
                    alert_data.day, alert_data.minute, alert_data.second, alert_data.weekday,
                ]];
                insert_data(&header, alert_data, "alert_data.csv", "Alert").unwrap();
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
        },
    }
}
