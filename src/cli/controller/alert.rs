use crate::cli::subcommands::alert::{AlertSubcommand, AlertInfo};
use crate::database::{db::connection, alert::view_alert};

pub fn handle_alert(info: AlertInfo) {
    match info.alert_subcommand {
        AlertSubcommand::Add(alert) => {
            let conn = connection().expect("failed to connect to the database");
            let result = alert.insert_alert(&conn);
            match result {
                Ok(_) => println!("Alert data is successfully saved"),
                Err(error) => println!("Err: {}", error),
            }
        }

        AlertSubcommand::View => {
            let conn = connection().expect("failed to connect to the database");
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
            let conn = connection().expect("failed to connect to the database");
            let result = alert.update_alert(&conn);
            match result {
                Ok(_) => println!("Alert data is successfully updated"),
                Err(rusqlite::Error::QueryReturnedNoRows) => {
                    println!("Error: No matching record found")
                }
                Err(e) => println!("Database error: {:?}", e),
            }
        }

        AlertSubcommand::Delete(alert) => {
            let conn = connection().expect("failed to connect to the database");
            let result = alert.delete_alert(&conn);
            match result {
                Ok(_) => println!("The row(s) in an alert is successfully deleted"),
                Err(error) => println!("Error: {}", error),
            }
        }

        AlertSubcommand::Get(alert) => {
            let conn = connection().expect("failed to connect to the database");
            let result = alert.get_alert(&conn);
            match result {
                Ok(_) => println!("Alert data is successfully saved in a CSV file"),
                Err(error) => println!("Error: {}", error),
            }
        }
    }
}