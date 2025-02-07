use rusqlite::{Connection, Result, ToSql};
use crate::cli::flags::budget::{CreateBudget, BudgetData, UpdateBudget};
use tabled::{Table, Tabled};
use rusqlite::params;

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