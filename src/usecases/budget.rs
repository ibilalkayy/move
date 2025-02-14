use rusqlite::{Connection, Result};

pub fn convert_to_u64(opt: Option<String>) -> u64 {
    opt.and_then(|s| s.parse::<u64>().ok()).unwrap_or(0)
}

pub fn budget_total_equal(conn: &Connection) -> Result<(u64, u64, bool), rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT total_amount FROM totalamount")?;
    let total_amount: u64 = stmt.query_row([], |row| row.get::<_, String>(0))?
        .parse::<u64>()
        .unwrap_or(0);

    let mut stmt = conn.prepare("SELECT SUM(amount) FROM budget")?;
    let budget_amount: u64 = stmt.query_row([], |row| row.get::<_, Option<u64>>(0))?
        .unwrap_or(0);

    Ok((total_amount, budget_amount, budget_amount < total_amount))
}
