use crate::cli::subcommands::status::{StatusInfo, StatusSubcommand};
use crate::database::{
    db::connection,
    status::{update_status, view_status},
};

pub fn handle_status(info: StatusInfo) {
    let conn = connection().expect("Err: failed to connect to the database");
    match info.status_info {
        StatusSubcommand::Active => {
            let result = update_status(&conn, "active");
            match result {
                Ok(_) => println!("The total amount status is successfully updated"),
                Err(error) => panic!("Err: {}", error),
            }
        }
        StatusSubcommand::Inactive => {
            let result = update_status(&conn, "inactive");
            match result {
                Ok(_) => println!("The total amount status is successfully updated"),
                Err(error) => panic!("Err: {}", error),
            }
        }
        StatusSubcommand::Check => {
            let result = view_status(&conn);
            match result {
                Ok(_) => (),
                Err(error) => panic!("Err: {}", error),
            }
        }
    }
}
