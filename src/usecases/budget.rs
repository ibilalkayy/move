use rusqlite::{Connection, Result};

pub fn budget_total_equal(
    conn: &Connection,
    category: &str,
) -> Result<(u64, u64, u64, bool), rusqlite::Error> {
    let mut stmt = conn.prepare("select total_amount from totalamount")?;
    let total_amount: u64 = stmt.query_row([], |row| row.get::<_, u64>(0))?;

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

pub fn budget_amount(conn: &Connection, category: &str) -> Result<(u64, u64), rusqlite::Error> {
    let mut stmt =
        conn.prepare("select amount, remaining_amount from budget where category = ?")?;
    let amount = stmt.query_row([category], |row| row.get::<_, u64>(0))?;
    let remaining_amount = stmt.query_row([category], |row| row.get::<_, u64>(1))?;
    Ok((amount, remaining_amount))
}

pub fn calculate_budget(
    conn: &Connection,
    category: &str,
    spending_amount: u64,
    spending_sum_category: u64,
) {
    let (_, remaining) = budget_amount(conn, category).unwrap_or_else(|e| panic!("Err: {}", e));
    let remaining_amount = remaining - spending_amount;
    conn.execute(
        "update budget set spent_amount = ?, remaining_amount = ? where category = ?",
        (&spending_sum_category, &remaining_amount, &category),
    )
    .expect("Err: failed to calculate the budget amount");
}
