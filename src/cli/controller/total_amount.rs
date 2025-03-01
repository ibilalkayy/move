use crate::cli::subcommands::total_amount::{
    AddTotalSubcommand, GetTotalSubcommand, RemoveTotalSubcommand, TotalAmountInfo,
    TotalAmountSubcommand, UpdateTotalSubcommand, ViewSubcommand,
};

use crate::database::{
    db::connection,
    status::insert_status,
    total_amount::{delete_total_amount, view_total_amount},
    total_categories::view_total_categories,
};

pub fn handle_total_amount(info: TotalAmountInfo) {
    match info.total_amount_subcommand {
        TotalAmountSubcommand::Add(total) => match total.add_total {
            AddTotalSubcommand::Amount(total_amount) => {
                let conn = connection().expect("❌ Failed to establish the DB connection");
                let total_data = total_amount.insert_total_amount(&conn);
                match total_data {
                    Ok(_) => {
                        insert_status(&conn);
                        println!("✅ Total amount is successfully saved");
                    }
                    Err(error) => panic!("❌ {}", error),
                }
            }

            AddTotalSubcommand::Category(total_category) => {
                let conn = connection().expect("❌ Failed to establish the DB connection");
                let result = total_category.insert_total_category(&conn);
                match result {
                    Ok(_) => println!("✅ Category is successfully saved"),
                    Err(error) => panic!("❌ {}", error),
                }
            }
        },

        TotalAmountSubcommand::View(total) => match total.view_total {
            ViewSubcommand::Amount => {
                let conn = connection().expect("❌ Failed to establish the DB connection");
                let result = view_total_amount(&conn);
                match result {
                    Ok(_) => (),
                    Err(error) => panic!("❌ {}", error),
                }
            }

            ViewSubcommand::Categories => {
                let conn = connection().expect("❌ Failed to establish the DB connection");
                let result = view_total_categories(&conn);
                match result {
                    Ok(_) => (),
                    Err(error) => panic!("❌ {}", error),
                }
            }
        },

        TotalAmountSubcommand::Update(total) => match total.update_total {
            UpdateTotalSubcommand::Amount(total_amount) => {
                let conn = connection().expect("❌ Failed to establish the DB connection");
                let result = total_amount.update_total_amount(&conn);
                match result {
                    Ok(_) => println!("✅ Total amount is successfully updated"),
                    Err(error) => panic!("❌ {}", error),
                }
            }

            UpdateTotalSubcommand::Categories(total_category) => {
                let conn = connection().expect("❌ Failed to establish the DB connection");
                let result = total_category.update_total_category(&conn);
                match result {
                    Ok(_) => println!("✅ Category data is successfully updated"),
                    Err(rusqlite::Error::QueryReturnedNoRows) => {
                        panic!("❌ No matching data is found")
                    }
                    Err(error) => panic!("❌ {:?}", error),
                }
            }
        },

        TotalAmountSubcommand::Remove(total) => match total.remove_total {
            RemoveTotalSubcommand::Amount => {
                let conn = connection().expect("❌ Failed to establish the DB connection");
                let result = delete_total_amount(&conn);
                match result {
                    Ok(_) => println!("✅ Total amount is successfully deleted"),
                    Err(error) => panic!("❌ {}", error),
                }
            }

            RemoveTotalSubcommand::Category(category) => {
                let conn = connection().expect("❌ Failed to establish the DB connection");
                let result = category.delete_total_category(&conn);
                match result {
                    Ok(_) => println!("✅ Category is successfully deleted"),
                    Err(error) => panic!("❌ {}", error),
                }
            }
        },

        TotalAmountSubcommand::Get(total) => match total.get_total {
            GetTotalSubcommand::Amount(total_amount) => {
                let conn = connection().expect("❌ Failed to establish the DB connection");
                let result = total_amount.get_total_amount(&conn);
                match result {
                    Ok(_) => println!("✅ Total amount is saved in a CSV file"),
                    Err(error) => panic!("❌ {}", error),
                }
            }
            GetTotalSubcommand::Category(total_amount) => {
                let conn = connection().expect("❌ Failed to establish the DB connection");
                let result = total_amount.get_total_categories(&conn);
                match result {
                    Ok(_) => println!("✅ Category is saved in a CSV file"),
                    Err(error) => panic!("❌ {}", error),
                }
            }
        },
    }
}
