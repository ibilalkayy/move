use rusqlite::{Connection, Result};

pub fn total_amount_exists(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare("select exists(select 1 from totalamount)")?;
    let exists: bool = stmt.query_row([], |row| row.get(0))?;
    Ok(exists)
}
