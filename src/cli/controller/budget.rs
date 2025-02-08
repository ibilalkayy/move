use crate::cli::subcommands::budget::{AlertSubcommand, BudgetInfo, BudgetSubcommand};
use crate::database::db::connection;
use crate::database::{
    budget::list_budget,
    alert::view_alert,
};

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

        BudgetSubcommand::View(budget) => {
            let conn = connection().unwrap();
            let _ = budget.view_budget(&conn, &budget.category);
        }

        BudgetSubcommand::List => {
            let conn = connection().unwrap();
            let _ = list_budget(&conn);
        }

        BudgetSubcommand::Get(_get_budget) => {
            println!("Budget data is successfully stored in the CSV file");
        }

        BudgetSubcommand::Update(budget) => {
            let conn = connection().unwrap();
            let result = budget.update_budget(&conn);
            match result {
                Ok(_) => println!("Budget data is successfully updated"),
                Err(rusqlite::Error::QueryReturnedNoRows) => println!("Error: No matching record found"),
                Err(e) => println!("Database error: {:?}", e),
            }
        }

        BudgetSubcommand::Delete(budget) => {
            let conn = connection().unwrap();
            let result = budget.delete_budget(&conn);
            match result {
                Ok(_) => println!("The row(s) in a budget is successfully deleted"),
                Err(error) => println!("Error: {}", error),
            }
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
                let conn = connection().unwrap();
                let _ = view_alert(&conn);
            }

            AlertSubcommand::Email(_email_alert) => {
                println!("Alert is successfully gotten")
            }

            AlertSubcommand::See(_cli_alert) => {
                println!("Alert is successfully seen")
            }

            AlertSubcommand::Update(update) => {
                let conn = connection().unwrap();
                let result = update.update_alert(&conn);
                match result {
                    Ok(_) => println!("Alert data is successfully updated"),
                    Err(rusqlite::Error::QueryReturnedNoRows) => println!("Error: No matching record found"),
                    Err(e) => println!("Database error: {:?}", e),
                }
            }

            AlertSubcommand::Delete(alert) => {
                let conn = connection().unwrap();
                let result = alert.delete_alert(&conn);
                match result {
                    Ok(_) => println!("The row(s) in an alert is successfully deleted"),
                    Err(error) => println!("Error: {}", error),
                }
            }

            AlertSubcommand::Get(alert) => {
                let conn = connection().unwrap();
                let result = alert.get_alert(&conn);
                match result {
                    Ok(_) => println!("The alert data is successfully saved in a file"),
                    Err(error) => println!("Error: {}", error),
                }
            }
        },
    }
}
