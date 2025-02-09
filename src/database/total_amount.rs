use rusqlite::{Connection, Result};
use crate::cli::flags::total_amount::{AddTotalAmount, UpdateTotalAmount, RemoveTotalCategory};
use std::{fs, fs::File, process::exit};
use tabled::{Table, Tabled};
use rusqlite::params;
use csv::Writer;

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
            exit(0);
        }
        
        conn.execute(
            "insert into totalamount(total_amount, spent_amount, remaining_amount, statuss) values(?1, ?2, ?3, ?4)",
            &[&self.amount, &Some("0".to_string()), &Some("0".to_string()), &Some("inactive".to_string())],
        )?;
        Ok(())
    }

    pub fn get_total_amount(&self, conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare("select total_amount, spent_amount, remaining_amount, statuss from totalamount")?;

        let rows = stmt.query_map(params![], |row| {
            Ok(TotalAmountRow {
                total_amount: row.get(0)?,
                spent_amount: row.get(1)?,
                remaining_amount: row.get(2)?,
                status: row.get(3)?,
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

        let merge_path = joined_dir.join("total_amount_data.csv");
        let file_path = File::create(merge_path).expect("failed to create a file");

        let mut wtr = Writer::from_writer(file_path);

        wtr.write_record(&["Total Amount", "Spent Amount", "Remaining Amount", "Status"]).unwrap();

        for amount in result {
            wtr.write_record(&[
                amount.total_amount,
                amount.spent_amount,
                amount.remaining_amount,
                amount.status,
            ]).unwrap();
        }

        wtr.flush().unwrap();

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

impl UpdateTotalAmount {
    pub fn update_total_amount(&self, conn: &Connection) -> Result<()> {
        conn.execute("update totalamount set total_amount=?", &[&self.amount])?;
        Ok(())
    }
}

impl RemoveTotalCategory {
    pub fn delete_total_category(&self, conn: &Connection) -> Result<()> {
        let affected_rows = conn.execute("DELETE FROM totalcategories where category=?", &[&self.category])?;
        
        if affected_rows == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
        }
        
        Ok(())
    }
}

pub fn delete_total_amount(conn: &Connection) -> Result<()> {
    let affected_rows = conn.execute("DELETE FROM totalamount", [])?;
    
    if affected_rows == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
    }
    
    Ok(())
}