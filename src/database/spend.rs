use rusqlite::{Connection, Result};
use crate::cli::flags::spend::SpendData;

impl SpendData {
    pub fn insert_spending(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "insert into spend(category, amount) values($1, $2)",
            &[&self.category, &self.amount],
        )?;
        Ok(())
    }
}
