use rusqlite::{Connection, Result, ToSql};
use crate::cli::flags::budget::{CreateBudget, BudgetData, UpdateBudget};
use tabled::{Table, Tabled};
use std::{fs, fs::File};
use rusqlite::params;
use csv::Writer;

#[derive(Tabled)]
struct BudgetRow {
    #[tabled(rename = "Category")]
    category: String,

    #[tabled(rename = "Amount")]
    amount: String,
}

impl CreateBudget {
    pub fn insert_budget(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "insert into budget(category, amount) values(?1, ?2)",
            &[&self.category, &self.amount],
        )?;
        Ok(())
    }

    pub fn get_budget(&self, conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare( "SELECT category, amount FROM budget")?;
        
        let rows = stmt.query_map(params![], |row| {
            Ok(BudgetRow {
                category: row.get(0)?,
                amount: row.get(1)?,
            })
        })?;
    
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }

        let home_dir = dirs::home_dir().expect("failed to get the home directory");
        let joined_dir = home_dir.join("move");

        if !joined_dir.exists() {
            fs::create_dir_all(&joined_dir).expect("Failed to create directory");
        }

        let merge_path = joined_dir.join("budget_data.csv");
        let file_path = File::create(merge_path).expect("failed to create a file");

        let mut wtr = Writer::from_writer(file_path);

        wtr.write_record(&["Category", "Amount"]).unwrap();

        for budget in results {
            wtr.write_record(&[
                budget.category,
                budget.amount,
            ]).unwrap();
        }

        wtr.flush().unwrap();
        Ok(())
    }

}

impl BudgetData {
    pub fn view_budget(&self, conn: &Connection, category: &str) -> Result<()> {
        let mut stmt = conn.prepare(
            "SELECT category, amount FROM budget WHERE category = ?",
        )?;
        
        let rows = stmt.query_map(params![category], |row| {
            Ok(BudgetRow {
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

    pub fn delete_budget(&self, conn: &Connection) -> Result<()> {
        let affected_rows = conn.execute("DELETE FROM budget WHERE category = ?", &[&self.category])?;
        
        if affected_rows == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
        }
        
        Ok(())
    }    
}

pub fn list_budget(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT category, amount FROM budget",
    )?;
    
    let rows = stmt.query_map(params![], |row| {
        Ok(BudgetRow {
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

impl UpdateBudget {
    pub fn update_budget(&self, conn: &Connection) -> Result<()> {
        if let Some(old_category) = &self.old_category {
            let mut query = String::from("update budget set ");
            let mut fields = Vec::new();
            let mut value: Vec<&dyn ToSql> = Vec::new();

            if let Some(new_category) = &self.new_category {
                fields.push("category = ?");
                value.push(new_category);
            }

            if let Some(amount) = &self.amount {
                fields.push("amount = ?");
                value.push(amount);
            }

            if fields.is_empty() {
                return Err(rusqlite::Error::InvalidQuery);
            }

            query.push_str(&fields.join(", "));
            query.push_str("where category = ?");

            value.push(old_category);

            let affected_rows = conn.execute(&query, rusqlite::params_from_iter(value))?;
        
            if affected_rows == 0 {
                return Err(rusqlite::Error::QueryReturnedNoRows);
            }

            Ok(())
        } else {
            Err(rusqlite::Error::InvalidQuery) // If old_category is None
        }
    }
}