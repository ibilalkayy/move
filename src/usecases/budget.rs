use rusqlite::{Connection, Result};

pub fn convert_to_u64(opt: Option<String>) -> u64 {
    opt.and_then(|s| s.parse::<u64>().ok()).unwrap_or(0)
}

pub fn budget_total_equal(conn: &Connection, category: &str) -> Result<(u64, u64, u64, bool), rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT total_amount FROM totalamount")?;
    let total_amount: u64 = stmt.query_row([], |row| row.get::<_, String>(0))?
        .parse::<u64>()
        .unwrap_or(0);

    let mut stmt = conn.prepare("SELECT SUM(amount) FROM budget")?;
    let budget_total_sum: u64 = stmt.query_row([], |row| row.get::<_, Option<u64>>(0))?
        .unwrap_or(0);

    let mut stmt = conn.prepare("SELECT SUM(amount) FROM budget WHERE category <> ?")?;
    let budget_except_sum: Option<u64> = stmt.query_row([category], |row| row.get(0))?; 

    Ok((total_amount, budget_total_sum, budget_except_sum.unwrap_or(0), budget_total_sum < total_amount))
}

pub fn budget_category_exists(conn: &Connection, category: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT EXISTS(SELECT 1 FROM budget WHERE category = ?)")?;
    let exists: bool = stmt.query_row([category], |row| row.get(0))?;
    Ok(exists)
}