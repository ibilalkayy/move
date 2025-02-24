use rusqlite::{Connection, Result};

pub fn spending_sum(conn: &Connection) -> Result<u64, rusqlite::Error> {
    let mut stmt = conn.prepare("select sum(amount) from spend")?;
    let spend_sum: u64 = stmt
        .query_row([], |row| row.get::<_, Option<u64>>(0))?
        .unwrap_or(0);

    Ok(spend_sum)
}

pub fn spending_sum_category(conn: &Connection, category: &str) -> Result<u64, rusqlite::Error> {
    let mut stmt = conn.prepare("select sum(amount) from spend where category =?")?;
    let spend_sum: u64 = stmt
        .query_row([category], |row| row.get::<_, Option<u64>>(0))?
        .unwrap_or(0);

    Ok(spend_sum)
}