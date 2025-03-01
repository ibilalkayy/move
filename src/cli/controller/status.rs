use crate::cli::subcommands::status::{StatusInfo, StatusSubcommand};
use crate::database::{
    db::connection,
    status::{update_status, view_status},
};

pub fn handle_status(info: StatusInfo) {
    let conn = connection().expect("❌ Failed to establish the DB connection");
    match info.status_info {
        StatusSubcommand::Active => {
            let result = update_status(&conn, "active");
            match result {
                Ok(_) => println!("✅ Status is activated now"),
                Err(error) => panic!("❌ {}", error),
            }
        }
        StatusSubcommand::Inactive => {
            let result = update_status(&conn, "inactive");
            match result {
                Ok(_) => println!("✅ Status became inactive"),
                Err(error) => panic!("❌ {}", error),
            }
        }
        StatusSubcommand::Check => {
            let result = view_status(&conn);
            match result {
                Ok(_) => (),
                Err(error) => panic!("❌ {}", error),
            }
        }
    }
}
