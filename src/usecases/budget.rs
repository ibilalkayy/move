use rusqlite::{Connection, Result};

pub fn budget_total_equal(conn: &Connection) -> Result<bool, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT total_amount FROM totalamount")?;
    let total_amount: u64 = stmt.query_row([], |row| row.get::<_, String>(0))?
        .parse::<u64>()
        .unwrap_or(0);

    let mut stmt = conn.prepare("SELECT SUM(amount) FROM budget")?;
    let budget_amount: u64 = stmt.query_row([], |row| row.get::<_, Option<u64>>(0))?
        .unwrap_or(0);

    Ok(budget_amount < total_amount)
}
