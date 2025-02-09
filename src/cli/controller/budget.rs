use crate::cli::subcommands::budget::{AlertSubcommand, BudgetInfo, BudgetSubcommand};
use crate::database::{db::connection, budget::show_budget, alert::view_alert};

pub fn handle_budget(info: BudgetInfo) {
    match info.budget_subcommand {
        BudgetSubcommand::Add(budget) => {
            let conn = connection().unwrap();
            let result = budget.insert_budget(&conn);
            match result {
                Ok(_) => println!("Budget data is successfully saved"),
                Err(error) => println!("Err: {}", error),
            }
        }

        BudgetSubcommand::View(budget) => {
            let conn = connection().unwrap();
            let result = budget.view_budget(&conn, &budget.category);
            match result {
                Ok(_) => (),
                Err(error) => println!("Err: {}", error),
            }
        }

        BudgetSubcommand::Show => {
            let conn = connection().unwrap();
            let result = show_budget(&conn);
            match result {
                Ok(_) => (),
                Err(error) => println!("Err: {}", error),
            }
        }

        BudgetSubcommand::Get(budget) => {
            let conn = connection().unwrap();
            let result = budget.get_budget(&conn);
            match result {
                Ok(_) => println!("Budget data is successfully saved in a CSV file"),
                Err(error) => println!("Error: {}", error),
            }
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

        BudgetSubcommand::Alert(budget) => match budget.alert_budget {
            AlertSubcommand::Add(alert) => {
                let conn = connection().unwrap();
                let result = alert.insert_alert(&conn);
                match result {
                    Ok(_) => println!("Alert data is successfully saved"),
                    Err(error) => println!("Err: {}", error),
                }
            }

            AlertSubcommand::View => {
                let conn = connection().unwrap();
                let result = view_alert(&conn);
                match result {
                    Ok(_) => (),
                    Err(error) => println!("Error: {}", error),
                }
            }

            AlertSubcommand::Email(_alert) => {
                println!("Alert is successfully gotten")
            }

            AlertSubcommand::See(_alert) => {
                println!("Alert is successfully seen")
            }

            AlertSubcommand::Update(alert) => {
                let conn = connection().unwrap();
                let result = alert.update_alert(&conn);
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
                    Ok(_) => println!("Alert data is successfully saved in a CSV file"),
                    Err(error) => println!("Error: {}", error),
                }
            }
        },
    }
}
