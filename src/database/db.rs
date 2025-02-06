use rusqlite::Connection;
use std::{fs, path::Path};

pub fn connection() -> rusqlite::Result<Connection> {
    let home_dir = dirs::home_dir().expect("failed to get the home directory");
    let db_path = home_dir.join("move.db");

    Connection::open(db_path)
}

pub fn create_table() -> rusqlite::Result<()> {
    let conn = connection().unwrap();
    let sql_file_path = Path::new("sql/create_table.sql");
    let sql_query = fs::read_to_string(sql_file_path).expect("Failed to read the SQL file");
    conn.execute_batch(sql_query.as_str())?;
    Ok(())
}
