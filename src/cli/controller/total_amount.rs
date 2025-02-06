use crate::cli::subcommands::total_amount::{
    AddTotalSubcommand, StatusSubcommand, TotalAmountInfo, TotalAmountSubcommand,
    UpdateTotalSubcommand, ViewSubcommand, RemoveTotalSubcommand,
};

use crate::database::db::connection;

use crate::database::{
    total_amount::view_total_amount,
    total_categories::view_total_categories,
};

pub fn handle_total_amount(details: TotalAmountInfo) {
    match details.total_amount {
        TotalAmountSubcommand::Add(add_total) => match add_total.add_subcommand {
            AddTotalSubcommand::Amount(amount) => {
                let conn = connection().unwrap();
                let result = amount.insert_total_amount(&conn);
                match result {
                    Ok(_) => println!("Total amount is successfully saved"),
                    Err(error) => println!("Err: {}", error),
                }
            }

            AddTotalSubcommand::Category(category) => {
                let conn = connection().unwrap();
                let result = category.insert_total_category(&conn);
                match result {
                    Ok(_) => println!("Category is successfully saved"),
                    Err(error) => println!("Err: {}", error),
                }
            }
        },

        TotalAmountSubcommand::View(view_total) => match view_total.view_subcommand {
            ViewSubcommand::Amount => {
                let conn = connection().unwrap();
                let _ = view_total_amount(&conn);
            }

            ViewSubcommand::Categories => {
                let conn = connection().unwrap();
                let _ = view_total_categories(&conn);
            }
        },

        TotalAmountSubcommand::Status(status_total) => match status_total.status_subcommand {
            StatusSubcommand::Active => {
                println!("Total amount is activated");
            }

            StatusSubcommand::Inactive => {
                println!("Total amount is now inactive");
            }

            StatusSubcommand::Check => {
                println!("total amount status is checked out");
            }
        },

        TotalAmountSubcommand::Update(update) => match update.update_subcommand {
            UpdateTotalSubcommand::Amount(_update_total) => {
                println!("Total amount is successfully updated");
            }

            UpdateTotalSubcommand::Categories(_update_category) => {
                println!("Total amount is successfully updated");
            }
        },

        TotalAmountSubcommand::Remove(remove_total) => match remove_total.remove_subcommand {
            RemoveTotalSubcommand::Amount => {
                println!("Total amount is successfully removed");
            }

            RemoveTotalSubcommand::Category(_remove_category) => {
                println!("Total amount category is successfully removed");
            }
        }

        TotalAmountSubcommand::Get(_get_total) => {
            println!("Get the total amount")
        }


    }
}
