use crate::cli::subcommands::budget::{AlertSubcommand, BudgetInfo, BudgetSubcommand};

pub fn handle_budget(details: BudgetInfo) {
    match details.budget_subcommand {
        BudgetSubcommand::Create(_create_budget) => {
            println!("Budget is successfully created");
        }

        BudgetSubcommand::View(_budget_data) => {
            println!("Budget is successfully viewed");
        }

        BudgetSubcommand::List => {
            println!("Budget is successfully listed");
        }

        BudgetSubcommand::Get(_get_budget) => {
            println!("Budget data is successfully stored in the CSV file");
            // let result = get_budget.get_budget();
            // match result {
            //     Ok(_) => println!("Budget data is successfully stored in the CSV file"),
            //     Err(err) => println!("Error: {}", err),
            // }
        }

        BudgetSubcommand::Update(_update_budget) => {
            println!("Budget is successfully updated");
        }

        BudgetSubcommand::Delete(_budget_data) => {
            println!("category data has been successfully deleted");
        }

        BudgetSubcommand::Alert(alert_budget) => match alert_budget.alert_subcommand {
            AlertSubcommand::Set(_alert_data) => {
                println!("Alert is successfully created")
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
