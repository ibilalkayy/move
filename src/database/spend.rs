use crate::cli::flags::spend::{SpendCategory, SpendData};
use crate::common::common::create_file;
use crate::usecases::{
    budget::{budget_amount, budget_category_exists, get_budget_amount, category_spend_sum, total_spend_sum},
    total_amount::{total_amount_exists, calculate_total},
    total_categories::total_category_exists,
    status::status,
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
        
        let total_category_present = total_category_exists(conn, category).unwrap_or_else(|e| panic!("Err: {}", e));
        if !total_category_present {
            panic!("Err: {} category is not present in the total categories list", category);
        }
    
        let total_amount_present = total_amount_exists(conn).unwrap_or_else(|e| panic!("Err: {}", e));
        if !total_amount_present {
            panic!("Err: Amount is not present in the total amount list");
        }
    
        let budget_category_present = budget_category_exists(conn, category).unwrap_or_else(|e| panic!("Err: {}", e));
        if !budget_category_present {
            panic!("Err: {} category is not present in the budget list", category);
        }
    
        let budget_amount = budget_amount(conn, category).unwrap_or_else(|e| panic!("Err: {}", e));
        let spending_amount = self.amount.unwrap_or_else(|| panic!("Err: Spending amount is not given"));
        
        if spending_amount > budget_amount {
            panic!("Err: Spending amount exceeded the budget amount");
        }
    
        let spend_sum = category_spend_sum(conn, category).unwrap_or_else(|e| panic!("Err: {}", e));
        let added_spend_amount = spend_sum + spending_amount;
        
        let budget_amount = get_budget_amount(conn, category).unwrap_or_else(|e| panic!("Err: {}", e));
        if added_spend_amount > budget_amount {
            panic!("Err: Spending amount exceeded the budget amount");
        }
    
        let total_spend_sum = total_spend_sum(conn).unwrap_or_else(|e| panic!("Err: {}", e));
        let total_spend_amount = total_spend_sum + spending_amount;
    
        let status = status(conn).unwrap_or_else(|e| panic!("Err: {}", e));
        if status != "active" {
            panic!("Err: The status is not active yet");
        }
    
        conn.execute(
            "insert into spend(category, amount) values(?1, ?2)",
            (&self.category, spending_amount),
        )?;
        
        calculate_total(conn, spending_amount, total_spend_amount);
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
            panic!("Err: {} category is not present in the total categories list", &self.category);
        }

        if !total_amount_present {
            panic!("Err: Amount is not present in the total amount list");
        }

        if !budget_category_present {
            panic!("Err: {} category is not present in the budget list", &self.category);
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

    pub fn get_spending(&self, conn: &Connection) -> Result<()> {
        let total_category_present = total_category_exists(conn, &self.category)?;
        let total_amount_present = total_amount_exists(conn)?;
        let budget_category_present = budget_category_exists(conn, &self.category)?;

        if !total_category_present {
            panic!("Err: {} category is not present in the total categories list", &self.category);
        }

        if !total_amount_present {
            panic!("Err: Amount is not present in the total amount list");
        }

        if !budget_category_present {
            panic!("Err: {} category is not present in the budget list", &self.category);
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
            .expect("failed to write the data in a CSV file");

        for spending in result {
            wtr.write_record(&[spending.category, spending.amount.to_string()])
                .expect("failed to write the data in a CSV file");
        }
        wtr.flush().expect("failed to flush the content");
    
        Ok(())
    }

    pub fn delete_spending(&self, conn: &Connection) -> Result<()> {
        let total_category_present = total_category_exists(conn, &self.category)?;
        let total_amount_present = total_amount_exists(conn)?;
        let budget_category_present = budget_category_exists(conn, &self.category)?;

        if !total_category_present {
            panic!("Err: {} category is not present in the total categories list", &self.category);
        }

        if !total_amount_present {
            panic!("Err: Amount is not present in the total amount list");
        }

        if !budget_category_present {
            panic!("Err: {} category is not present in the budget list", &self.category);
        }

        let affected_rows = conn.execute(
            "delete from spend where category = ?",
            &[&self.category],
        )?;

        if affected_rows == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        Ok(())
    }
}
