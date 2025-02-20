use crate::cli::subcommands::total_amount::{
    AddTotalSubcommand, GetTotalSubcommand, RemoveTotalSubcommand, StatusSubcommand,
    TotalAmountInfo, TotalAmountSubcommand, UpdateTotalSubcommand, ViewSubcommand,
};

use crate::database::{
    db::connection,
    total_amount::{delete_total_amount, update_total_status, view_total_amount},
    total_categories::view_total_categories,
};

pub fn handle_total_amount(info: TotalAmountInfo) {
    match info.total_amount_subcommand {
        TotalAmountSubcommand::Add(total) => match total.add_total {
            AddTotalSubcommand::Amount(total_amount) => {
                let conn = connection().expect("failed to connect to the database");
                let result = total_amount.insert_total_amount(&conn);
                match result {
                    Ok(_) => println!("Total amount is successfully saved"),
                    Err(error) => panic!("Err: {}", error),
                }
            }

            AddTotalSubcommand::Category(total_category) => {
                let conn = connection().expect("failed to connect to the database");
                let result = total_category.insert_total_category(&conn);
                match result {
                    Ok(_) => println!("Category is successfully saved"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
        },

        TotalAmountSubcommand::View(total) => match total.view_total {
            ViewSubcommand::Amount => {
                let conn = connection().expect("failed to connect to the database");
                let result = view_total_amount(&conn);
                match result {
                    Ok(_) => (),
                    Err(error) => panic!("Err: {}", error),
                }
            }

            ViewSubcommand::Categories => {
                let conn = connection().expect("failed to connect to the database");
                let result = view_total_categories(&conn);
                match result {
                    Ok(_) => (),
                    Err(error) => panic!("Err: {}", error),
                }
            }
        },

        TotalAmountSubcommand::Status(total) => match total.status_total {
            StatusSubcommand::Active => {
                let conn = connection().expect("failed to connect to the database");
                let result = update_total_status(&conn, "active");
                match result {
                    Ok(_) => println!("The total amount status is successfully updated"),
                    Err(error) => panic!("Err: {}", error),
                }
            }

            StatusSubcommand::Inactive => {
                let conn = connection().expect("failed to connect to the database");
                let result = update_total_status(&conn, "inactive");
                match result {
                    Ok(_) => println!("The total amount status is successfully updated"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
        },

        TotalAmountSubcommand::Update(total) => match total.update_total {
            UpdateTotalSubcommand::Amount(total_amount) => {
                let conn = connection().expect("failed to connect to the database");
                let result = total_amount.update_total_amount(&conn);
                match result {
                    Ok(_) => println!("The total amount data is successfully updated"),
                    Err(error) => panic!("Err: {}", error),
                }
            }

            UpdateTotalSubcommand::Categories(total_category) => {
                let conn = connection().expect("failed to connect to the database");
                let result = total_category.update_total_category(&conn);
                match result {
                    Ok(_) => println!("The total amount category data is successfully updated"),
                    Err(rusqlite::Error::QueryReturnedNoRows) => {
                        panic!("Err: No matching record found")
                    }
                    Err(error) => panic!("Err: {:?}", error),
                }
            }
        },

        TotalAmountSubcommand::Remove(total) => match total.remove_total {
            RemoveTotalSubcommand::Amount => {
                let conn = connection().expect("failed to connect to the database");
                let result = delete_total_amount(&conn);
                match result {
                    Ok(_) => println!("The total amount data is successfully deleted"),
                    Err(error) => panic!("Err: {}", error),
                }
            }

            RemoveTotalSubcommand::Category(category) => {
                let conn = connection().expect("failed to connect to the database");
                let result = category.delete_total_category(&conn);
                match result {
                    Ok(_) => println!("The total amount category is successfully deleted"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
        },

        TotalAmountSubcommand::Get(total) => match total.get_total {
            GetTotalSubcommand::Amount(total_amount) => {
                let conn = connection().expect("failed to connect to the database");
                let result = total_amount.get_total_amount(&conn);
                match result {
                    Ok(_) => println!("The total amount data is successfully saved in a CSV file"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
            GetTotalSubcommand::Category(total_amount) => {
                let conn = connection().expect("failed to connect to the database");
                let result = total_amount.get_total_categories(&conn);
                match result {
                    Ok(_) => println!("The category data is successfully saved in a CSV file"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
        },
    }
}
