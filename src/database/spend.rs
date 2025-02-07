use rusqlite::{Connection, Result};
use crate::cli::flags::spend::{SpendData, SpendFinder};
use tabled::{Table, Tabled};
use rusqlite::params;

#[derive(Tabled)]
struct SpendingRow {
    #[tabled(rename = "Category")]
    category: String,

    #[tabled(rename = "Amount")]
    amount: String,
}

impl SpendData {
    pub fn insert_spending(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "insert into spend(category, amount) values(?1, ?2)",
            &[&self.category, &self.amount],
        )?;
        Ok(())
    }
}

impl SpendFinder {
    pub fn view_spending(&self, conn: &Connection, category: &str) -> Result<()> {
        let mut stmt = conn.prepare(
            "SELECT category, amount FROM spend where category=?",
        )?;
    
        let rows = stmt.query_map(params![&category], |row| {
            Ok(SpendingRow {
                category: row.get(0)?,
                amount: row.get(1)?,
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

    pub fn delete_spending(&self, conn: &Connection) -> Result<()> {
        let affected_rows = conn.execute("DELETE FROM spend WHERE category = ?", &[&self.category])?;
        
        if affected_rows == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
        }
        
        Ok(())
    }
}