use dotenv::dotenv;
use postgres::{Client, NoTls, Error};
use std::{fs, env, path::Path};

fn connection() -> Result<Client, Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set in .env");
    let client = Client::connect(&database_url, NoTls)?;
    Ok(client)
}

pub fn create_table() -> Result<(), Error> {
    let mut client = connection()?;
    let sql_file_path = Path::new("sql/create_table.sql");
    let sql_query = fs::read_to_string(sql_file_path).expect("Failed to read the SQL file");
    client.batch_execute(&sql_query)?;
    Ok(())
}