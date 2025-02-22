use rusqlite::{Connection, Result};

pub fn status(conn: &Connection) -> Result<String> {
    let mut stmt = conn.prepare("select statuss from statuss")?;
    let status: String = stmt.query_row([], |row| row.get(0))?;
    Ok(status)
}
