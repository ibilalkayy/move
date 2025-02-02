use crate::cli::flags::total_amount::{AddTotalAmount, RemoveTotal, UpdateTotalAmount};
use crate::cli::subcommands::total_amount::StatusTotal;
use crate::database::db::connection;
use tabled::{Table, Tabled};
use std::error::Error;

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
    pub fn insert_total_amount(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let row_exists = client.query_one("SELECT EXISTS (SELECT 1 FROM totalamount)", &[])?.get(0);

        if row_exists {
            println!("The total amount data is already inserted");
            return Ok(())
        }

        let _ = client.execute(
            "insert into totalamount(total_amount, spent_amount, remaining_amount, statuss) values($1, $2, $3, $4)",
            &[&self.amount, &"0", &"0", &"inactive"],
        )?;
        println!("Total amount category is successfully saved");
        Ok(())
    }
}

pub fn view_total_amount() -> Result<(), Box<dyn Error>> {
    let mut client = connection()?;
    let mut rows = Vec::new();

    for row in client.query(
        "select total_amount, spent_amount, remaining_amount, statuss from totalamount",
        &[],
    )? {
        let total_amount: String = row.get(0);
        let spent_amount: String = row.get(1);
        let remaining_amount: String = row.get(2);
        let status: String = row.get(3);

        rows.push(TotalAmountRow {
            total_amount,
            spent_amount,
            remaining_amount,
            status,
        });
        break;
    }

    let table = Table::new(rows);
    println!("{}", table);

    Ok(())
}

impl UpdateTotalAmount {
    pub fn update_total(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute(
            "update totalamount set total_amount=$1, spent_amount=$2, remaining_amount=$3",
            &[&self.amount, &"0", &"0"],
        )?;
        Ok(())
    }
}

impl RemoveTotal {
    pub fn remove_total(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute(
            "delete from totalamount where category=$1",
            &[&self.category],
        )?;
        Ok(())
    }
}

impl StatusTotal {
    pub fn update_status(&self, status: String) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute("update totalamount set statuss=$1", &[&status])?;
        Ok(())
    }

    pub fn check_status(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        for row in client.query("select statuss from totalamount", &[])? {
            let status: String = row.get(0);
            println!("Total amount status: {}", status);
        }
        Ok(())
    }
}