use crate::cli::flags::spend::{SpendCategory, SpendData};
use crate::common::common::create_file;
use crate::usecases::{
    budget::{budget_amount, budget_category_exists, budget_data_exists, calculate_budget},
    spend::{spending_sum, spending_sum_category},
    status::status,
    total_amount::{calculate_total, total_amount_exists},
    total_categories::total_category_exists,
};
use csv::Writer;
use rusqlite::{params, Connection, Result};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct SpendingRow {
    #[tabled(rename = "Category")]
    category: String,

    #[tabled(rename = "Amount")]
    amount: u64,
}

impl SpendData {
    pub fn insert_spending(&self, conn: &Connection) -> Result<()> {
        let category = self.category.as_deref().unwrap_or("");

        let total_category_present = total_category_exists(conn, category)?;
        let total_amount_present = total_amount_exists(conn)?;
        let budget_category_present = budget_category_exists(conn, category)?;

        if !total_category_present {
            panic!("Err: {} category is not added to the total categories list. See 'move total-amount -h'", category);
        }
        if !total_amount_present {
            panic!("Err: amount is not added to the total amount list. See 'move total-amount -h'");
        }
        if !budget_category_present {
            panic!(
                "Err: {} category is not added to the budget list. See 'move budget -h'",
                category
            );
        }

        let (budget_amount, _) = budget_amount(conn, category)?;
        let spending_amount = self
            .amount
            .unwrap_or_else(|| panic!("Err: spending amount is not provided"));
        if spending_amount > budget_amount {
            panic!("Err: spending amount exceeded the budget amount");
        }

        let category_spend_sum = spending_sum_category(conn, category)?;
        let category_spend_amount = category_spend_sum + spending_amount;
        if category_spend_amount > budget_amount {
            panic!("Err: spending amount exceeded the budget amount");
        }

        let total_spend_sum = spending_sum(conn)?;
        let total_spend_amount = total_spend_sum + spending_amount;

        let status = status(conn)?;
        if status != "active" {
            panic!("Err: the status is not active yet. See 'move status -h'");
        }

        conn.execute(
            "insert into spend(category, amount) values(?1, ?2)",
            (&self.category, spending_amount),
        )?;

        let spending_sum_category = spending_sum_category(conn, category)?;
        calculate_total(conn, spending_amount, total_spend_amount);
        calculate_budget(conn, category, spending_amount, spending_sum_category);
        println!("Money is spent successfully on the {} category", category);

        Ok(())
    }
}

impl SpendCategory {
    pub fn view_spending(&self, conn: &Connection) -> Result<()> {
        let total_category_present = total_category_exists(conn, &self.category)?;
        let total_amount_present = total_amount_exists(conn)?;
        let budget_category_present = budget_category_exists(conn, &self.category)?;

        if !total_category_present {
            panic!("Err: {} category is not added to the total categories list. See 'move total-amount -h'", &self.category);
        }
        if !total_amount_present {
            panic!("Err: amount is not added to the total amount list. See 'move total-amount -h'");
        }
        if !budget_category_present {
            panic!(
                "Err: {} category is not added to the budget list. See 'move budget -h'",
                &self.category
            );
        }

        let mut stmt = conn.prepare("select category, amount from spend where category=?")?;
        let rows = stmt.query_map(params![&self.category], |row| {
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

    pub fn get_category_spending(&self, conn: &Connection) -> Result<()> {
        let total_category_present = total_category_exists(conn, &self.category)?;
        let total_amount_present = total_amount_exists(conn)?;
        let budget_category_present = budget_category_exists(conn, &self.category)?;

        if !total_category_present {
            panic!("Err: {} category is not added to the total categories list. See 'move total-amount -h'", &self.category);
        }
        if !total_amount_present {
            panic!("Err: amount is not added to the total amount list. See 'move total-amount -h'");
        }
        if !budget_category_present {
            panic!(
                "Err: {} category is not added to the budget list. See 'move budget -h'",
                &self.category
            );
        }

        let mut stmt = conn.prepare("select category, amount from spend where category = ?")?;
        let rows = stmt.query_map(params![&self.category], |row| {
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
        wtr.write_record(&["Category", "Amount"])
            .expect("Err: failed to write the data in a CSV file");

        for spending in result {
            wtr.write_record(&[spending.category, spending.amount.to_string()])
                .expect("Err: failed to write the data in a CSV file");
        }
        wtr.flush().expect("Err: failed to flush the content");

        Ok(())
    }

    pub fn delete_spending(&self, conn: &Connection) -> Result<()> {
        let total_category_present = total_category_exists(conn, &self.category)?;
        let total_amount_present = total_amount_exists(conn)?;
        let budget_category_present = budget_category_exists(conn, &self.category)?;

        if !total_category_present {
            panic!("Err: {} category is not added to the total categories list. See 'move total-amount -h'", &self.category);
        }
        if !total_amount_present {
            panic!("Err: amount is not added to the total amount list. See 'move total-amount -h'");
        }
        if !budget_category_present {
            panic!(
                "Err: {} category is not added to the budget list. See 'move budget -h'",
                &self.category
            );
        }

        let affected_rows =
            conn.execute("delete from spend where category = ?", &[&self.category])?;

        if affected_rows == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        Ok(())
    }
}

pub fn get_all_spending(conn: &Connection) -> Result<()> {
    let total_amount_present = total_amount_exists(conn)?;
    let budget_data_present = budget_data_exists(conn)?;

    if !total_amount_present {
        panic!("Err: amount is not added to the total amount list. See 'move total-amount -h'");
    }
    if !budget_data_present {
        panic!("Err: budget is not added to the budget list. See 'move budget -h'");
    }

    let mut stmt = conn.prepare("select category, amount from spend")?;
    let rows = stmt.query_map([], |row| {
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
    wtr.write_record(&["Category", "Amount"])
        .expect("Err: failed to write the data in a CSV file");

    for spending in result {
        wtr.write_record(&[spending.category, spending.amount.to_string()])
            .expect("Err: failed to write the data in a CSV file");
    }
    wtr.flush().expect("Err: failed to flush the content");

    Ok(())
}
