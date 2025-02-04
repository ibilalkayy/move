use crate::cli::subcommands::total_amount::{
    AddTotalSubcommand, StatusSubcommand, TotalAmountInfo, TotalAmountSubcommand,
    UpdateTotalSubcommand, ViewSubcommand, RemoveTotalSubcommand,
};

use crate::database::{
    db::create_table, total_amount::view_total_amount, total_categories::view_total_categories, total_amount::remove_total_amount
};

pub fn handle_total_amount(details: TotalAmountInfo) {
    match details.total_amount {
        TotalAmountSubcommand::Add(add_total) => match add_total.add_subcommand {
            AddTotalSubcommand::Amount(amount) => {
                let _ = create_table();
                let _ = amount.insert_total_amount();
            }

            AddTotalSubcommand::Category(category) => {
                let _ = create_table();
                let result = category.insert_total_categories();
                match result {
                    Ok(_) => println!("Total amount category is successfully saved"),
                    Err(err) => println!("Error: {}", err),
                }
            }
        },

        TotalAmountSubcommand::View(view_total) => match view_total.view_subcommand {
            ViewSubcommand::Amount => {
                let _ = view_total_amount();
            }

            ViewSubcommand::Categories => {
                let _ = view_total_categories();
            }
        },

        TotalAmountSubcommand::Status(status_total) => match status_total.status_subcommand {
            StatusSubcommand::Active => {
                let result = status_total.update_status("active".to_string());
                match result {
                    Ok(_) => println!("Total amount is activated"),
                    Err(err) => println!("Error: {}", err),
                }
            }

            StatusSubcommand::Inactive => {
                let result = status_total.update_status("inactive".to_string());
                match result {
                    Ok(_) => println!("Total amount is now inactive"),
                    Err(err) => println!("Error: {}", err),
                }
            }

            StatusSubcommand::Check => {
                let _ = status_total.check_status();
            }
        },

        TotalAmountSubcommand::Update(update) => match update.update_subcommand {
            UpdateTotalSubcommand::Amount(update_total) => {
                let result = update_total.update_total();
                match result {
                    Ok(_) => println!("Total amount is successfully updated"),
                    Err(err) => println!("Error: {}", err),
                }
            }

            UpdateTotalSubcommand::Categories(update_category) => {
                let result = update_category.update_total_category();
                match result {
                    Ok(_) => println!("Total amount is successfully updated"),
                    Err(err) => println!("Error: {}", err),
                }
            }
        },

        TotalAmountSubcommand::Remove(remove_total) => match remove_total.remove_subcommand {
            RemoveTotalSubcommand::Amount => {
                let result = remove_total_amount();
                match result {
                    Ok(_) => println!("Total amount is successfully removed"),
                    Err(err) => println!("Error: {}", err),
                }
            }

            RemoveTotalSubcommand::Categories(remove_category) => {
                let result = remove_category.remove_total_category();
                match result {
                    Ok(_) => println!("Total amount category is successfully removed"),
                    Err(err) => println!("Error: {}", err),
                }
            }
        }
    }
}
