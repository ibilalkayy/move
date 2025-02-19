use crate::cli::subcommands::budget::{BudgetInfo, BudgetSubcommand};
use crate::database::{budget::show_budget, db::connection};

pub fn handle_budget(info: BudgetInfo) {
    match info.budget_subcommand {
        BudgetSubcommand::Add(budget) => {
            let conn = connection().expect("failed to connect to the database");
            let result = budget.insert_budget(&conn);
            match result {
                Ok(_) => println!("Budget data is successfully saved"),
                Err(error) => panic!("Err: {}", error),
            }
        }

        BudgetSubcommand::View(budget) => {
            let conn = connection().expect("failed to connect to the database");
            let result = budget.view_budget(&conn, &budget.category);
            match result {
                Ok(_) => (),
                Err(error) => panic!("Err: {}", error),
            }
        }

        BudgetSubcommand::Show => {
            let conn = connection().expect("failed to connect to the database");
            let result = show_budget(&conn);
            match result {
                Ok(_) => (),
                Err(error) => panic!("Err: {}", error),
            }
        }

        BudgetSubcommand::Get(budget) => {
            let conn = connection().expect("failed to connect to the database");
            let result = budget.get_budget(&conn);
            match result {
                Ok(_) => println!("Budget data is successfully saved in a CSV file"),
                Err(error) => panic!("Err:: {}", error),
            }
        }

        BudgetSubcommand::Update(budget) => {
            let conn = connection().expect("failed to connect to the database");
            let result = budget.update_budget(&conn);
            match result {
                Ok(_) => println!("Budget data is successfully updated"),
                Err(rusqlite::Error::QueryReturnedNoRows) => {
                    panic!("Err:: No matching record found")
                }
                Err(e) => println!("Database error: {:?}", e),
            }
        }

        BudgetSubcommand::Delete(budget) => {
            let conn = connection().expect("failed to connect to the database");
            let result = budget.delete_budget(&conn);
            match result {
                Ok(_) => println!("The row(s) in a budget is successfully deleted"),
                Err(error) => panic!("Err:: {}", error),
            }
        }
    }
}
