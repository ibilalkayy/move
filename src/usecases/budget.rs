use rusqlite::{Connection, Result};

pub fn budget_total_equal(
    conn: &Connection,
    category: &str,
) -> Result<(u64, u64, u64, bool), rusqlite::Error> {
    let mut stmt = conn.prepare("select total_amount from totalamount")?;
    let total_amount: u64 = stmt
        .query_row([], |row| row.get::<_, u64>(0))?;

    let mut stmt = conn.prepare("select sum(amount) from budget")?;
    let budget_total_sum: u64 = stmt
        .query_row([], |row| row.get::<_, Option<u64>>(0))?
        .unwrap_or(0);

    let mut stmt = conn.prepare("select sum(amount) from budget where category <> ?")?;
    let budget_except_sum: Option<u64> = stmt.query_row([category], |row| row.get(0))?;

    Ok((
        total_amount,
        budget_total_sum,
        budget_except_sum.unwrap_or(0),
        budget_total_sum < total_amount,
    ))
}

pub fn budget_category_exists(conn: &Connection, category: &str) -> Result<bool> {
    let mut stmt = conn.prepare("select exists(select 1 from budget where category = ?)")?;
    let exists: bool = stmt.query_row([category], |row| row.get(0))?;
    Ok(exists)
}

pub fn budget_data_exists(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare("select exists(select 1 from budget)")?;
    let exists: bool = stmt.query_row([], |row| row.get(0))?;
    Ok(exists)
}

pub fn budget_amount(conn: &Connection, category: &str) -> Result<u64, rusqlite::Error> {
    let mut stmt = conn.prepare("select amount from budget where category = ?")?;
    let amount = stmt.query_row([category], |row| row.get::<_, u64>(0))?;
    Ok(amount)
}

pub fn category_spend_sum(conn: &Connection, category: &str) -> Result<u64, rusqlite::Error> {
    let mut stmt = conn.prepare("select sum(amount) from spend where category =?")?;
    let spend_sum: u64 = stmt
        .query_row([category], |row| row.get::<_, Option<u64>>(0))?
        .unwrap_or(0);

    Ok(spend_sum)
}

pub fn total_spend_sum(conn: &Connection) -> Result<u64, rusqlite::Error> {
    let mut stmt = conn.prepare("select sum(amount) from spend")?;
    let spend_sum: u64 = stmt
        .query_row([], |row| row.get::<_, Option<u64>>(0))?
        .unwrap_or(0);

    Ok(spend_sum)
}

pub fn get_budget_amount(conn: &Connection, category: &str) -> Result<u64, rusqlite::Error> {
    let mut stmt = conn.prepare("select amount from budget where category = ?")?;
    let budget_amount: u64 = stmt
        .query_row([category], |row| row.get::<_, Option<u64>>(0))?
        .unwrap_or(0);

    Ok(budget_amount)
}
