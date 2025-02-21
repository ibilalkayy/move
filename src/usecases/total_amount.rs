use rusqlite::{Connection, Result};

pub fn total_amount_exists(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare("select exists(select 1 from totalamount)")?;
    let exists: bool = stmt.query_row([], |row| row.get(0))?;
    Ok(exists)
}

pub fn total_amount_status(conn: &Connection) -> Result<String> {
    let mut stmt = conn.prepare("select statuss from totalamount")?;
    let status: String = stmt.query_row([], |row| row.get(0))?;
    Ok(status)
}
