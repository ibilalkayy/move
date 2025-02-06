use rusqlite::{Connection, Result};
use crate::cli::flags::total_amount::AddTotalCategory;
use tabled::{Table, Tabled};
use rusqlite::params;

#[derive(Tabled)]
struct CategoryRow {
    #[tabled(rename = "Category")]
    category: String,

    #[tabled(rename = "Label")]
    label: String,
}

impl AddTotalCategory {
    pub fn insert_total_category(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "insert into totalcategories(category, label) values($1, $2)",
            &[&self.category, &self.label],
        )?;
        Ok(())
    }
}

pub fn view_total_categories(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT category, label FROM totalcategories",
    )?;

    let rows = stmt.query_map(params![], |row| {
        Ok(CategoryRow {
            category: row.get(0)?,
            label: row.get(1)?,
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