use crate::cli::flags::spend::{SpendCategory, SpendData};
use csv::Writer;
use rusqlite::{params, Connection, Result};
use std::{fs, fs::File};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct SpendingRow {
    #[tabled(rename = "Category")]
    category: String,

    #[tabled(rename = "Amount")]
    amount: String,
}

fn create_file(path: &str) -> File {
    let home_dir = dirs::home_dir().expect("Failed to get the home directory");
    let joined_dir = home_dir.join("move");

    if !joined_dir.exists() {
        fs::create_dir_all(&joined_dir).expect("Failed to create directory");
    }

    let merge_path = joined_dir.join(path);
    let file_path = File::create(merge_path).expect("Failed to create a file");
    return file_path;
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

        wtr.write_record(&["Category", "Amount"]).unwrap();

        for spending in result {
            wtr.write_record(&[spending.category, spending.amount])
                .unwrap();
        }

        wtr.flush().unwrap();

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
