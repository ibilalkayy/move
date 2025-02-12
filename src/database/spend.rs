use crate::cli::flags::spend::{SpendCategory, SpendData};
use csv::Writer;
use rusqlite::{params, Connection, Result};
use tabled::{Table, Tabled};
use crate::common::common::create_file;

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

        let file_path = create_file("spend.csv");

        let mut wtr = Writer::from_writer(file_path);

        wtr.write_record(&["Category", "Amount"]).expect("failed to write the data in a CSV file");

        for spending in result {
            wtr.write_record(&[spending.category, spending.amount])
                .expect("failed to write the data in a CSV file");
        }

        wtr.flush().expect("failed to flush the content");

        Ok(())
    }
}

impl SpendCategory {
    pub fn view_spending(&self, conn: &Connection, category: &str) -> Result<()> {
        let mut stmt = conn.prepare("select category, amount from spend where category=?")?;

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
        let affected_rows =
            conn.execute("delete from spend where category = ?", &[&self.category])?;

        if affected_rows == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
        }

        Ok(())
    }
}
