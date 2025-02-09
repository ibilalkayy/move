use rusqlite::{Connection, Result};
use crate::cli::flags::spend::{SpendData, SpendFinder};
use tabled::{Table, Tabled};
use std::{fs, fs::File};
use rusqlite::params;
use csv::Writer;

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

    pub fn get_spending(&self, conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare("select category, amount from spend")?;

        let rows = stmt.query_map(params![], |row| {
            Ok(SpendingRow {
                category: row.get(0)?,
                amount: row.get(1)?,
            })
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?)
        }

        let home_dir = dirs::home_dir().expect("failed to get the home directory");
        let joined_dir = home_dir.join("move");

        if !joined_dir.exists() {
            fs::create_dir_all(&joined_dir).expect("Failed to create directory");
        }

        let merge_path = joined_dir.join("spending_data.csv");
        let file_path = File::create(merge_path).expect("failed to create a file");

        let mut wtr = Writer::from_writer(file_path);

        wtr.write_record(&["Category", "Amount"]).unwrap();

        for spending in result {
            wtr.write_record(&[
                spending.category,
                spending.amount,
            ]).unwrap();
        }

        wtr.flush().unwrap();

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