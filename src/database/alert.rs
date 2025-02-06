use rusqlite::{Connection, Result};
use crate::cli::flags::alert::AlertData;
use tabled::{Table, Tabled};
use rusqlite::params;

#[derive(Tabled)]
pub struct AlertRow {
    #[tabled(rename = "Category")]
    pub category: String,

    #[tabled(rename = "Frequency")]
    pub frequency: String,

    #[tabled(rename = "Method")]
    pub method: String,

    #[tabled(rename = "Day")]
    pub day: String,

    #[tabled(rename = "Hour")]
    pub hour: String,

    #[tabled(rename = "Minute")]
    pub minute: String,

    #[tabled(rename = "Second")]
    pub second: String,

    #[tabled(rename = "Weekday")]
    pub weekday: String,
}

impl AlertData {
    pub fn insert_alert(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "insert into alert(category, frequency, method, dayz, hourz, minutez, secondz, weekdays) values($1, $2, $3, $4, $5, $6, $7, $8)",
            &[&self.category, &self.frequency, &self.method, &self.day, &self.hour, &self.minute, &self.second, &self.weekday],
        )?;
        Ok(())
    }
}

pub fn view_alert(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT category, frequency, method, dayz, hourz, minutez, secondz, weekdays FROM alert",
    )?;
    
    let rows = stmt.query_map(params![], |row| {
        Ok(AlertRow {
            category: row.get(0)?,
            frequency: row.get(1)?,
            method: row.get(2)?,
            day: row.get(3)?,
            hour: row.get(4)?,
            minute: row.get(5)?,
            second: row.get(6)?,
            weekday: row.get(7)?,
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