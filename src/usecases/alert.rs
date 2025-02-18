use rusqlite::{Connection, Result};

pub fn alert_exists(conn: &Connection, category: &str) -> Result<bool> {
    let mut stmt =
        conn.prepare("select exists(select 1 from alert where category = ?)")?;
    let exists: bool = stmt.query_row([category], |row| row.get(0))?;
    Ok(exists)
}

pub fn alerts_exist(conn: &Connection) -> Result<bool> {
    let mut stmt =
        conn.prepare("select exists(select 1 from alert)")?;
    let exists: bool = stmt.query_row([], |row| row.get(0))?;
    Ok(exists)
}
