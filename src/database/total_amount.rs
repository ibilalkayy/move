use crate::cli::flags::{
    total_amount::{TotalAmount, UpdateTotalAmount},
    total_categories::RemoveTotalCategory,
};
use crate::common::common::create_file;
use csv::Writer;
use rusqlite::{params, Connection, Result};
use std::process::exit;
use tabled::{Table, Tabled};

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

impl TotalAmount {
    pub fn insert_total_amount(&self, conn: &Connection) -> Result<()> {
        let row_exists: bool = conn.query_row(
            "select exists (select 1 from totalamount)",
            params![],
            |row| row.get(0),
        )?;

        if row_exists {
            println!("Inserting the total amount multiple times is not allowed");
            exit(0);
        }

        conn.execute(
            "insert into totalamount(total_amount, spent_amount, remaining_amount, statuss) values(?1, ?2, ?3, ?4)",
            &[&self.amount, &Some("0".to_string()), &Some("0".to_string()), &Some("inactive".to_string())],
        )?;
        Ok(())
    }

    pub fn get_total_amount(&self, conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare(
            "select total_amount, spent_amount, remaining_amount, statuss from totalamount",
        )?;

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

        let file_path = create_file("total_amount.csv");

        let mut wtr = Writer::from_writer(file_path);

        wtr.write_record(&["Total Amount", "Spent Amount", "Remaining Amount", "Status"])
            .expect("failed to write the data in a CSV file");

        for amount in result {
            wtr.write_record(&[
                amount.total_amount,
                amount.spent_amount,
                amount.remaining_amount,
                amount.status,
            ])
            .expect("failed to write the data in a CSV file");
        }

        wtr.flush().expect("failed to flush the content");

        Ok(())
    }
}

pub fn view_total_amount(conn: &Connection) -> Result<()> {
    let mut stmt = conn
        .prepare("select total_amount, spent_amount, remaining_amount, statuss from totalamount")?;

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
        let affected_rows = conn.execute(
            "delete from totalcategories where category=?",
            &[&self.category],
        )?;

        if affected_rows == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
        }

        Ok(())
    }
}

pub fn delete_total_amount(conn: &Connection) -> Result<()> {
    let affected_rows = conn.execute("delete from totalamount", [])?;

    if affected_rows == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
    }

    Ok(())
}

pub fn update_total_status(conn: &Connection, status: String) -> Result<()> {
    conn.execute("update totalamount set statuss=?", &[&status])?;
    Ok(())
}
