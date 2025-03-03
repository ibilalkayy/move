use crate::cli::subcommands::budget::{BudgetInfo, BudgetSubcommand};
use crate::database::{budget::show_budget, db::connection};

pub fn handle_budget(info: BudgetInfo) {
    match info.budget_subcommand {
        BudgetSubcommand::Add(budget) => {
            let conn = connection().expect("❌ Failed to establish the DB connection");
            let result = budget.insert_budget(&conn);
            match result {
                Ok(_) => println!("✅ Budget data is successfully saved"),
                Err(error) => panic!("❌ {}", error),
            }
        }

        BudgetSubcommand::View(budget) => {
            let conn = connection().expect("❌ Failed to establish the DB connection");
            let result = budget.view_budget(&conn);
            match result {
                Ok(_) => (),
                Err(error) => panic!("❌ {}", error),
            }
        }

        BudgetSubcommand::Show => {
            let conn = connection().expect("❌ Failed to establish the DB connection");
            let result = show_budget(&conn);
            match result {
                Ok(_) => (),
                Err(error) => panic!("❌ {}", error),
            }
        }

        BudgetSubcommand::Get(budget) => {
            let conn = connection().expect("❌ Failed to establish the DB connection");
            let result = budget.get_budget(&conn);
            match result {
                Ok(_) => println!("✅ Budget data is successfully saved in a CSV file"),
                Err(error) => panic!("❌ {}", error),
            }
        }

        BudgetSubcommand::Update(budget) => {
            let conn = connection().expect("❌ Failed to establish the DB connection");
            let result = budget.update_budget(&conn);
            match result {
                Ok(_) => println!("✅ Budget data is successfully updated"),
                Err(rusqlite::Error::QueryReturnedNoRows) => {
                    panic!("❌ No matching record is found")
                }
                Err(error) => panic!("❌ {:?}", error),
            }
        }

        BudgetSubcommand::Delete(budget) => {
            let conn = connection().expect("❌ Failed to establish the DB connection");
            let result = budget.delete_budget(&conn);
            match result {
                Ok(_) => println!("✅ Budget data is successfully deleted"),
                Err(error) => panic!("❌ {}", error),
            }
        }
    }
}
