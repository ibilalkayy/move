use crate::cli::subcommands::total_amount::{
    AddTotalSubcommand, StatusSubcommand, TotalAmountInfo, TotalAmountSubcommand,
    UpdateTotalSubcommand, ViewSubcommand, RemoveTotalSubcommand, GetTotalSubcommand,
};

use crate::database::{
    db::connection,
    total_amount::{view_total_amount, delete_total_amount, update_status},
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

        TotalAmountSubcommand::Status(status_total) => match status_total.status_subcommand {
            StatusSubcommand::Active => {
                let conn = connection().unwrap();
                let result = update_status(&conn, "active".to_string());
                match result {
                    Ok(_) => println!("Total amount status is successfully updated"),
                    Err(error) => println!("Error: {}", error),
                }
            }

            StatusSubcommand::Inactive => {
                let conn = connection().unwrap();
                let result = update_status(&conn, "inactive".to_string());
                match result {
                    Ok(_) => println!("Total amount status is successfully updated"),
                    Err(error) => println!("Error: {}", error),
                }
            }
        },

        TotalAmountSubcommand::Update(update) => match update.update_subcommand {
            UpdateTotalSubcommand::Amount(total_amount) => {
                let conn = connection().unwrap();
                let result = total_amount.update_total_amount(&conn);
                match result {
                    Ok(_) => println!("Total amount data is successfully updated"),
                    Err(error) => println!("Error: {}", error),
                }
            }

            UpdateTotalSubcommand::Categories(total_category) => {
                let conn = connection().unwrap();
                let result = total_category.update_total_category(&conn);
                match result {
                    Ok(_) => println!("Total amount category data is successfully updated"),
                    Err(rusqlite::Error::QueryReturnedNoRows) => println!("Error: No matching record found"),
                    Err(e) => println!("Database error: {:?}", e),
                }
            }
        },

        TotalAmountSubcommand::Remove(remove_total) => match remove_total.remove_subcommand {
            RemoveTotalSubcommand::Amount => {
                let conn = connection().unwrap();
                let result = delete_total_amount(&conn);
                match result {
                    Ok(_) => println!("Total amount data is successfully deleted"),
                    Err(error) => println!("Error: {}", error),
                }
            }

            RemoveTotalSubcommand::Category(category) => {
                let conn = connection().unwrap();
                let result = category.delete_total_category(&conn);
                match result {
                    Ok(_) => println!("Total amount category is successfully deleted"),
                    Err(error) => println!("Error: {}", error),
                }
            }
        }

        TotalAmountSubcommand::Get(get) => match get.get_subcommand {
            GetTotalSubcommand::Amount(get) => {
                let conn = connection().unwrap();
                let result = get.get_total_amount(&conn);
                match result {
                    Ok(_) => println!("The total amount data is successfully saved in a CSV file"),
                    Err(error) => println!("Error: {}", error),
                }
            }
            GetTotalSubcommand::Category(get) => {
                let conn = connection().unwrap();
                let result = get.get_total_categories(&conn);
                match result {
                    Ok(_) => println!("The category data is successfully saved in a CSV file"),
                    Err(error) => println!("Error: {}", error),
                }
            }
        }


    }
}
