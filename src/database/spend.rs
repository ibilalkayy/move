use crate::cli::flags::spend::{SpendCategory, SpendData};
use crate::common::common::create_file;
use crate::middleware::middleware::http_provider;
use crate::usecases::{
    budget::{budget_amount, budget_category_exists, budget_data_exists, calculate_budget},
    cred::{cred_exists, give_data},
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
    amount: f64,
}

impl SpendData {
    pub async fn insert_spending(&self, conn: &Connection) -> Result<()> {
        let category = self.category.as_deref().unwrap_or("");

        let total_category_present = total_category_exists(conn, category)?;
        let total_amount_present = total_amount_exists(conn)?;
        let budget_category_present = budget_category_exists(conn, category)?;
        let cred_present = cred_exists(conn)?;

        if !total_category_present {
            panic!("❌ {} category is not added to the total categories list. See 'move total-amount -h'", category);
        }
        if !total_amount_present {
            panic!("❌ No amount is added to the total amount list. See 'move total-amount -h'");
        }
        if !budget_category_present {
            panic!(
                "❌ {} category is not added to the budget list. See 'move budget -h'",
                category
            );
        }

        if !cred_present {
            panic!("❌ No credentials are added to the cred list. See 'move cred -h'");
        }

        let (budget_amount, _) = budget_amount(conn, category)?;
        let spending_amount = self
            .amount
            .unwrap_or_else(|| panic!("❌ No spending amount is provided"));
        if spending_amount > budget_amount {
            panic!("❌ Spending amount exceeded the budget amount");
        }

        let category_spend_sum = spending_sum_category(conn, category)
            .expect("❌ Failed to sum the category spending amount");
        let category_spend_amount = category_spend_sum + spending_amount;
        if category_spend_amount > budget_amount {
            panic!("❌ Spending amount exceeded the budget amount");
        }

        let total_spend_sum = spending_sum(conn).expect("❌ Failed to sum all the spending amount");
        let total_spend_amount = total_spend_sum + spending_amount;

        let status = status(conn).expect("❌ Failed to get the status");
        if status != "active" {
            panic!("❌ No active status yet. See 'move status -h'");
        }

        conn.execute(
            "insert into spend(category, amount, recepient_address) values(?1, ?2, ?3)",
            (&self.category, spending_amount, &self.recepient_address),
        )
        .expect("❌ Failed to insert the spending data");

        let keys: [String; 2] = [
            self.private_key.clone().unwrap_or_default(),
            self.alchemy_url_key.clone().unwrap_or_default(),
        ];

        let (private_key, alchemy_url, chain_id) = give_data(conn, keys)?;
        let recepient_address = self.recepient_address.clone().unwrap_or_default();

        let result = http_provider(
            alchemy_url,
            private_key,
            recepient_address,
            spending_amount,
            chain_id,
        )
        .await;

        match result {
            Ok(_) => println!("✅ Transaction function executed!"),
            Err(error) => println!("❌ Transaction Failed: {:?}", error),
        }

        let spending_sum_category = spending_sum_category(conn, category)
            .expect("❌ Failed to sum the category spending amount");
        calculate_total(conn, spending_amount, total_spend_amount);
        calculate_budget(conn, category, spending_amount, spending_sum_category);
        println!("✅ Money is spent on the {} category", category);

        Ok(())
    }
}

impl SpendCategory {
    pub fn view_spending(&self, conn: &Connection) -> Result<()> {
        let total_category_present = total_category_exists(conn, &self.category)?;
        let total_amount_present = total_amount_exists(conn)?;
        let budget_category_present = budget_category_exists(conn, &self.category)?;

        if !total_category_present {
            panic!("❌ {} category is not added to the total categories list. See 'move total-amount -h'", &self.category);
        }
        if !total_amount_present {
            panic!("❌ No amount is added to the total amount list. See 'move total-amount -h'");
        }
        if !budget_category_present {
            panic!(
                "❌ {} category is not added to the budget list. See 'move budget -h'",
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
            panic!("❌ {} category is not added to the total categories list. See 'move total-amount -h'", &self.category);
        }
        if !total_amount_present {
            panic!("❌ No amount is added to the total amount list. See 'move total-amount -h'");
        }
        if !budget_category_present {
            panic!(
                "❌ {} category is not added to the budget list. See 'move budget -h'",
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
            .expect("❌ Failed to write into a CSV file");

        for spending in result {
            wtr.write_record(&[spending.category, spending.amount.to_string()])
                .expect("❌ Failed to write into a CSV file");
        }
        wtr.flush().expect("❌ Failed to flush the content");

        Ok(())
    }

    pub fn delete_spending(&self, conn: &Connection) -> Result<()> {
        let total_category_present =
            total_category_exists(conn, &self.category).expect("❌ Failed to get the category");
        let total_amount_present =
            total_amount_exists(conn).expect("❌ Failed to get the total amount");
        let budget_category_present = budget_category_exists(conn, &self.category)
            .expect("❌ Failed to get the budget category");

        if !total_category_present {
            panic!("❌ {} category is not added to the total categories list. See 'move total-amount -h'", &self.category);
        }
        if !total_amount_present {
            panic!("❌ No amount is added to the total amount list. See 'move total-amount -h'");
        }
        if !budget_category_present {
            panic!(
                "❌ {} category is not added to the budget list. See 'move budget -h'",
                &self.category
            );
        }

        let rows =
            conn.execute("delete from spend where category = ?", &[&self.category])?;

        if rows == 0 {
            panic!("❌ No category is added yet. See 'move spend -h'");
        }

        Ok(())
    }
}

pub fn get_all_spending(conn: &Connection) -> Result<()> {
    let total_amount_present = total_amount_exists(conn).expect("❌ Failed to get the category");
    let budget_data_present =
        budget_data_exists(conn).expect("❌ Failed to get the budget category");

    if !total_amount_present {
        panic!("❌ No amount is added to the total amount list. See 'move total-amount -h'");
    }
    if !budget_data_present {
        panic!("❌ No budget is added to the budget list. See 'move budget -h'");
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
        .expect("❌ Failed to write into a CSV file");

    for spending in result {
        wtr.write_record(&[spending.category, spending.amount.to_string()])
            .expect("❌ Failed to write into a CSV file");
    }
    wtr.flush().expect("❌ Failed to flush the content");

    Ok(())
}
