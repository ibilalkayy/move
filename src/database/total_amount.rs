use crate::cli::flags::total_amount::{TotalAmount, UpdateTotalAmount};
use crate::common::common::create_file;
use crate::usecases::total_amount::total_amount_exists;
use csv::Writer;
use rusqlite::{params, Connection, Result};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct TotalAmountRow {
    #[tabled(rename = "Total Amount")]
    total_amount: f64,

    #[tabled(rename = "Spent Amount")]
    spent_amount: f64,

    #[tabled(rename = "Remaining Amount")]
    remaining_amount: f64,
}

impl TotalAmount {
    pub fn insert_total_amount(&self, conn: &Connection) -> Result<()> {
        let find_amount = total_amount_exists(conn);

        match find_amount {
            Ok(true) => panic!("❌ Total amount is already provided"),
            Ok(false) => {
                conn.execute(
                    "insert into totalamount(total_amount, spent_amount, remaining_amount) values(?1, ?2, ?3)",
                    &[&self.amount, &Some(0.0), &self.amount],
                )?;
            }
            Err(error) => panic!("❌ {}", error),
        }
        Ok(())
    }

    pub fn get_total_amount(&self, conn: &Connection) -> Result<()> {
        let find_total_amount = total_amount_exists(conn);
        match find_total_amount {
            Ok(true) => {
                let mut stmt = conn.prepare(
                    "select total_amount, spent_amount, remaining_amount from totalamount",
                )?;

                let rows = stmt.query_map(params![], |row| {
                    Ok(TotalAmountRow {
                        total_amount: row.get(0)?,
                        spent_amount: row.get(1)?,
                        remaining_amount: row.get(2)?,
                    })
                })?;

                let mut result = Vec::new();
                for row in rows {
                    result.push(row?)
                }

                let file_path = create_file("total_amount.csv");

                let mut wtr = Writer::from_writer(file_path);

                wtr.write_record(&["Total Amount", "Spent Amount", "Remaining Amount"])
                    .expect("❌ Failed to write into a CSV file");

                for amount in result {
                    wtr.write_record(&[
                        amount.total_amount.to_string(),
                        amount.spent_amount.to_string(),
                        amount.remaining_amount.to_string(),
                    ])
                    .expect("❌ Failed to write into a CSV file");
                }

                wtr.flush().expect("❌ Failed to flush the content");
            }
            Ok(false) => panic!(
                "❌ No amount is added to the total amount list. See 'move total-amount -h'"
            ),
            Err(error) => panic!("❌ {}", error),
        }

        Ok(())
    }
}

pub fn view_total_amount(conn: &Connection) -> Result<()> {
    let find_total_amount = total_amount_exists(conn);
    match find_total_amount {
        Ok(true) => {
            let mut stmt = conn
                .prepare("select total_amount, spent_amount, remaining_amount from totalamount")?;

            let rows = stmt.query_map(params![], |row| {
                Ok(TotalAmountRow {
                    total_amount: row.get(0)?,
                    spent_amount: row.get(1)?,
                    remaining_amount: row.get(2)?,
                })
            })?;

            let mut results = Vec::new();
            for row in rows {
                results.push(row?);
            }

            let table = Table::new(results);
            println!("{}", table);
        }
        Ok(false) => {
            panic!("❌ No amount is added to the total amount list. See 'move total-amount -h'")
        }
        Err(error) => panic!("❌ {}", error),
    }

    Ok(())
}

impl UpdateTotalAmount {
    pub fn update_total_amount(&self, conn: &Connection) -> Result<()> {
        let find_total_amount = total_amount_exists(conn);
        match find_total_amount {
            Ok(true) => {
                conn.execute(
                    "update totalamount set total_amount=?, remaining_amount = ?",
                    &[&self.amount, &self.amount],
                )?;
            }
            Ok(false) => panic!(
                "❌ No amount is added to the total amount list. See 'move total-amount -h'"
            ),
            Err(error) => panic!("❌ {}", error),
        }
        Ok(())
    }
}

pub fn delete_total_amount(conn: &Connection) -> Result<()> {
    let find_total_amount = total_amount_exists(conn);
    match find_total_amount {
        Ok(true) => {
            let affected_rows = conn.execute("delete from totalamount", [])?;

            if affected_rows == 0 {
                return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
            }
        }
        Ok(false) => {
            panic!("❌ No amount is added the total amount list. See 'move total-amount -h'")
        }
        Err(error) => panic!("❌ {}", error),
    }

    Ok(())
}
