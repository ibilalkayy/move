use crate::cli::subcommands::budget::{AlertSubcommand, BudgetInfo, BudgetSubcommand};
use crate::database::budget::list_budget;
use crate::database::db::create_table;

pub fn handle_budget(details: BudgetInfo) {
    match details.budget_subcommand {
        BudgetSubcommand::Create(create_budget) => {
            let _ = create_table();
            let result = create_budget.insert_budget();
            match result {
                Ok(_) => println!("Budget is successfully created"),
                Err(err) => println!("Error: {}", err),
            }
        }

        BudgetSubcommand::View(budget_data) => {
            let _ = budget_data.view_budget();
        }

        BudgetSubcommand::List => {
            let _ = list_budget();
        }

        BudgetSubcommand::Get(get_budget) => {
            let result = get_budget.get_budget();
            match result {
                Ok(_) => println!("Budget data is successfully stored in the CSV file"),
                Err(err) => println!("Error: {}", err),
            }
        }

        BudgetSubcommand::Update(update_budget) => {
            let result = update_budget.update_budget();
            match result {
                Ok(_) => println!("Budget is successfully updated"),
                Err(err) => println!("Error: {}", err),
            }
        }

        BudgetSubcommand::Delete(budget_data) => {
            let result = budget_data.delete_budget();
            match result {
                Ok(category) => {
                    println!("{} category data has been successfully deleted", category)
                }
                Err(err) => println!("err: {}", err),
            }
        }

        BudgetSubcommand::Alert(alert_budget) => match alert_budget.alert_subcommand {
            AlertSubcommand::Set(alert_data) => {
                let _ = create_table();
                let result = alert_data.create_alert();
                match result {
                    Ok(_) => println!("Alert is successfully created"),
                    Err(err) => println!("Error: {}", err),
                }
            }

            AlertSubcommand::Email(email_alert) => {
                email_alert.get_alert();
            }

            AlertSubcommand::See(cli_alert) => {
                cli_alert.see_alert();
            }

            AlertSubcommand::Update(update_alert) => {
                let result = update_alert.update_data();
                match result {
                    Ok(_) => println!("Alert is successfully updated"),
                    Err(err) => println!("Error: {}", err),
                }
            }

            AlertSubcommand::Remove(remove_alert) => {
                let result = remove_alert.remove_alert();
                match result {
                    Ok(_) => println!("Alert data is successfully removed"),
                    Err(err) => println!("Error: {}", err),
                }
            }
        },
    }
}
