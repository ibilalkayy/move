use rusqlite::{Connection, Result};
use crate::cli::flags::budget::{CreateBudget, BudgetData};
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
            "insert into budget(category, amount) values($1, $2)",
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