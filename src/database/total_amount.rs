use crate::cli::flags::total_amount::{TotalAmount, UpdateTotalAmount};
use crate::common::common::create_file;
use csv::Writer;
use rusqlite::{params, Connection, Result};
use tabled::{Table, Tabled};
use crate::usecases::total_amount::total_amount_exists;

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
        let find_amount = total_amount_exists(conn);

        match find_amount {
            Ok(true) => panic!("Total amount is already given"),
            Ok(false) => {
                conn.execute(
                    "insert into totalamount(total_amount, spent_amount, remaining_amount, statuss) values(?1, ?2, ?3, ?4)",
                    &[&self.amount, &Some("0".to_string()), &Some("0".to_string()), &Some("inactive".to_string())],
                )?;
            },
            Err(error) => panic!("Err: {}", error),
        }
        Ok(())
    }

    pub fn get_total_amount(&self, conn: &Connection) -> Result<()> {
        let find_amount = total_amount_exists(conn);
        match find_amount {
            Ok(true) => {
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
            }
            Ok(false) => panic!("Total amount data is not present in the record"),
            Err(error) => panic!("Err: {}", error),
        }

        Ok(())
    }
}

pub fn view_total_amount(conn: &Connection) -> Result<()> {
    let find_amount = total_amount_exists(conn);
    match find_amount {
        Ok(true) => {
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
        }
        Ok(false) => panic!("Total amount data is not present in the record"),
        Err(error) => panic!("Err: {}", error),
    }

    Ok(())
}

impl UpdateTotalAmount {
    pub fn update_total_amount(&self, conn: &Connection) -> Result<()> {
        let find_amount = total_amount_exists(conn);
        match find_amount {
            Ok(true) => {
                conn.execute("update totalamount set total_amount=?", &[&self.amount])?;
            },
            Ok(false) => panic!("Total amount data is not present in the record"),
            Err(error) => panic!("Err: {}", error),
        }
        Ok(())
    }
}

pub fn delete_total_amount(conn: &Connection) -> Result<()> {
    let find_amount = total_amount_exists(conn);
    match find_amount {
        Ok(true) => {
            let affected_rows = conn.execute("delete from totalamount", [])?;

            if affected_rows == 0 {
                return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
            }
        }
        Ok(false) => panic!("Total amount data is not present in the record"),
        Err(error) => panic!("Err: {}", error),
    }

    Ok(())
}

pub fn update_total_status(conn: &Connection, status: String) -> Result<()> {
    let find_amount = total_amount_exists(conn);
    match find_amount {
        Ok(true) => {
            conn.execute("update totalamount set statuss=?", &[&status])?;
        },
        Ok(false) => panic!("Total amount data is not present in the record"),
        Err(error) => panic!("Err: {}", error),
    }
    Ok(())
}
