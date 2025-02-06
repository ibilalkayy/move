use rusqlite::{Connection, Result};
use crate::cli::flags::alert::AlertData;

impl AlertData {
    pub fn insert_alert(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "insert into alert(category, frequency, method, dayz, hourz, minutez, secondz, weekdays) values($1, $2, $3, $4, $5, $6, $7, $8)",
            &[&self.category, &self.frequency, &self.method, &self.day, &self.hour, &self.minute, &self.second, &self.weekday],
        )?;
        Ok(())
    }
}
