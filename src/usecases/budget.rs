use rusqlite::{Connection, Result};

pub fn budget_total_equal(
    conn: &Connection,
    category: &str,
) -> Result<(f64, f64, f64, bool), rusqlite::Error> {
    let mut stmt = conn.prepare("select total_amount from totalamount")?;
    let total_amount: f64 = stmt.query_row([], |row| row.get::<_, f64>(0))?;

    let mut stmt = conn.prepare("select sum(amount) from budget")?;
    let budget_total_sum: f64 = stmt
        .query_row([], |row| row.get::<_, Option<f64>>(0))?
        .unwrap_or(0.0);

    let mut stmt = conn.prepare("select sum(amount) from budget where category <> ?")?;
    let budget_except_sum: Option<f64> = stmt.query_row([category], |row| row.get(0))?;

    Ok((
        total_amount,
        budget_total_sum,
        budget_except_sum.unwrap_or(0.0),
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

pub fn budget_amount(conn: &Connection, category: &str) -> Result<(f64, f64), rusqlite::Error> {
    let mut stmt =
        conn.prepare("select amount, remaining_amount from budget where category = ?")?;
    let amount = stmt.query_row([category], |row| row.get::<_, f64>(0))?;
    let remaining_amount = stmt.query_row([category], |row| row.get::<_, f64>(1))?;
    Ok((amount, remaining_amount))
}

pub fn calculate_budget(
    conn: &Connection,
    category: &str,
    spending_amount: f64,
    spending_sum_category: f64,
) {
    let (_, remaining) =
        budget_amount(conn, category).unwrap_or_else(|error| panic!("❌ {}", error));
    let remaining_amount = remaining - spending_amount;
    conn.execute(
        "update budget set spent_amount = ?, remaining_amount = ? where category = ?",
        (&spending_sum_category, &remaining_amount, &category),
    )
    .expect("❌ Failed to calculate the budget amount");
}
