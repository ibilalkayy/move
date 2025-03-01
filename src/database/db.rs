use rusqlite::Connection;
use std::{fs, path::Path};

pub fn connection() -> rusqlite::Result<Connection> {
    let home_dir = dirs::home_dir().expect("❌ Failed to get the home directory");
    let data_dir = home_dir.join("move");

    fs::create_dir_all(&data_dir).expect("❌ Failed to create a move directory");

    let db_path = data_dir.join("database.db");

    Connection::open(db_path)
}

pub fn create_table() -> rusqlite::Result<()> {
    let conn = connection().expect("❌ Failed to establish the DB connection");
    let sql_file_path = Path::new("sql/create_table.sql");
    let sql_query = fs::read_to_string(sql_file_path).expect("❌ Failed to read the SQL query");
    conn.execute_batch(sql_query.as_str())?;
    Ok(())
}
