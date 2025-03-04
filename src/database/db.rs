use rusqlite::Connection;
use std::fs;

const INIT_SQL: &str = include_str!("../../sql/create_table.sql");

pub fn connection() -> rusqlite::Result<Connection> {
    let home_dir = dirs::home_dir().expect("❌ Failed to get the home directory");
    let data_dir = home_dir.join("move");

    fs::create_dir_all(&data_dir).expect("❌ Failed to create the move directory");

    let db_path = data_dir.join("database.db");

    Connection::open(db_path)
}

pub fn create_table() -> rusqlite::Result<()> {
    let conn = connection().expect("❌ Failed to establish the DB connection");

    conn.execute_batch(INIT_SQL)?;
    
    Ok(())
}
