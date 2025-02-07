use rusqlite::{Connection, Result};
use crate::cli::flags::total_amount::AddTotalAmount;
use tabled::{Table, Tabled};
use rusqlite::params;

#[derive(Tabled)]
struct TotalAmountRow {
    #[tabled(rename = "Total Amount")]
    total_amount: String,

    #[tabled(rename = "Spent Amount")]
    spent_amount: String,

    #[tabled(rename = "Remaining Amount")]
    remaining_amount: String,

    #[tabled(rename = "Status")]
    status: String,
}

impl AddTotalAmount {
    pub fn insert_total_amount(&self, conn: &Connection) -> Result<()> {
        let row_exists: bool = conn.query_row(
            "SELECT EXISTS (SELECT 1 FROM totalamount)",
            params![],
            |row| row.get(0),
        )?;

        if row_exists {
            println!("The total amount data is already inserted");
            return Ok(());
        }
        
        conn.execute(
            "insert into totalamount(total_amount, spent_amount, remaining_amount, statuss) values(?1, ?2, ?3, ?4)",
            &[&self.amount, &"0".to_string(), &"0".to_string(), &"inactive".to_string()],
        )?;
        Ok(())
    }
}

pub fn view_total_amount(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT total_amount, spent_amount, remaining_amount, statuss FROM totalamount",
    )?;

    let rows = stmt.query_map(params![], |row| {
        Ok(TotalAmountRow {
            total_amount: row.get(0)?,
            spent_amount: row.get(1)?,
            remaining_amount: row.get(2)?,
            status: row.get(3)?,
        })
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }

    let table = Table::new(results);
    println!("{}", table);
    
    Ok(())
}