use rusqlite::{Connection, Result};

pub fn total_category_exists(conn: &Connection, category: &str) -> Result<bool> {
    let mut stmt = conn.prepare("select exists(select 1 from totalcategories where category = ?)")?;
    let exists: bool = stmt.query_row([category], |row| row.get(0))?;
    Ok(exists)
}