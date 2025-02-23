use rusqlite::{Connection, Result};

pub fn total_amount_exists(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare("select exists(select 1 from totalamount)")?;
    let exists: bool = stmt.query_row([], |row| row.get(0))?;
    Ok(exists)
}

pub fn remaining_amount(conn: &Connection) -> Result<u64> {
    let mut stmt = conn.prepare("select remaining_amount from totalamount")?;
    let remaining = stmt.query_row([], |row| row.get(0))?;
    Ok(remaining)
}

pub fn calculate_total(conn: &Connection, spent_amount: u64, total_spent_amount: u64) {
    let result = remaining_amount(conn);
    match result {
        Ok(remaining) => {
            let remaining_amount = remaining - spent_amount;
            conn.execute(
                "update totalamount set spent_amount = ?, remaining_amount = ?",
                &[&total_spent_amount, &remaining_amount],
            ).expect("Err: failed to calculate the total amount");
        }
        Err(error) => panic!("Err: {}", error),
    }
}