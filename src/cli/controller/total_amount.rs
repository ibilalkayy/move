use crate::cli::subcommands::total_amount::{
    AddTotalSubcommand, StatusSubcommand, TotalAmountInfo, TotalAmountSubcommand,
    UpdateTotalSubcommand, ViewSubcommand, RemoveTotalSubcommand, GetTotalSubcommand,
};

use crate::database::{
    db::connection,
    total_amount::{view_total_amount, delete_total_amount, update_total_status},
    total_categories::view_total_categories,
};

pub fn handle_total_amount(details: TotalAmountInfo) {
    match details.total_amount_subcommand {
        TotalAmountSubcommand::Add(total) => match total.add_total {
            AddTotalSubcommand::Amount(total_amount) => {
                let conn = connection().unwrap();
                let result = total_amount.insert_total_amount(&conn);
                match result {
                    Ok(_) => println!("Total amount is successfully saved"),
                    Err(error) => println!("Err: {}", error),
                }
            }

            AddTotalSubcommand::Category(total_category) => {
                let conn = connection().unwrap();
                let result = total_category.insert_total_category(&conn);
                match result {
                    Ok(_) => println!("Category is successfully saved"),
                    Err(error) => println!("Err: {}", error),
                }
            }
        },

        TotalAmountSubcommand::View(total) => match total.view_total {
            ViewSubcommand::Amount => {
                let conn = connection().unwrap();
                let result = view_total_amount(&conn);
                match result {
                    Ok(_) => (),
                    Err(error) => println!("Error: {}", error),
                }
            }

            ViewSubcommand::Categories => {
                let conn = connection().unwrap();
                let result = view_total_categories(&conn);
                match result {
                    Ok(_) => (),
                    Err(error) => println!("Error: {}", error),
                }
            }
        },

        TotalAmountSubcommand::Status(total) => match total.status_total {
            StatusSubcommand::Active => {
                let conn = connection().unwrap();
                let result = update_total_status(&conn, "active".to_string());
                match result {
                    Ok(_) => println!("The total amount status is successfully updated"),
                    Err(error) => println!("Error: {}", error),
                }
            }

            StatusSubcommand::Inactive => {
                let conn = connection().unwrap();
                let result = update_total_status(&conn, "inactive".to_string());
                match result {
                    Ok(_) => println!("The total amount status is successfully updated"),
                    Err(error) => println!("Error: {}", error),
                }
            }
        },

        TotalAmountSubcommand::Update(total) => match total.update_total {
            UpdateTotalSubcommand::Amount(total_amount) => {
                let conn = connection().unwrap();
                let result = total_amount.update_total_amount(&conn);
                match result {
                    Ok(_) => println!("The total amount data is successfully updated"),
                    Err(error) => println!("Error: {}", error),
                }
            }

            UpdateTotalSubcommand::Categories(total_category) => {
                let conn = connection().unwrap();
                let result = total_category.update_total_category(&conn);
                match result {
                    Ok(_) => println!("The total amount category data is successfully updated"),
                    Err(rusqlite::Error::QueryReturnedNoRows) => println!("Error: No matching record found"),
                    Err(e) => println!("Database error: {:?}", e),
                }
            }
        },

        TotalAmountSubcommand::Remove(total) => match total.remove_total {
            RemoveTotalSubcommand::Amount => {
                let conn = connection().unwrap();
                let result = delete_total_amount(&conn);
                match result {
                    Ok(_) => println!("The total amount data is successfully deleted"),
                    Err(error) => println!("Error: {}", error),
                }
            }

            RemoveTotalSubcommand::Category(category) => {
                let conn = connection().unwrap();
                let result = category.delete_total_category(&conn);
                match result {
                    Ok(_) => println!("The total amount category is successfully deleted"),
                    Err(error) => println!("Error: {}", error),
                }
            }
        }

        TotalAmountSubcommand::Get(total) => match total.get_total {
            GetTotalSubcommand::Amount(total_amount) => {
                let conn = connection().unwrap();
                let result = total_amount.get_total_amount(&conn);
                match result {
                    Ok(_) => println!("The total amount data is successfully saved in a CSV file"),
                    Err(error) => println!("Error: {}", error),
                }
            }
            GetTotalSubcommand::Category(total_amount) => {
                let conn = connection().unwrap();
                let result = total_amount.get_total_categories(&conn);
                match result {
                    Ok(_) => println!("The category data is successfully saved in a CSV file"),
                    Err(error) => println!("Error: {}", error),
                }
            }
        }
    }
}
