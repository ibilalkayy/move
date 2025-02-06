use rusqlite::{Connection, Result};
use crate::cli::flags::budget::CreateBudget;

impl CreateBudget {
    pub fn insert_budget(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "insert into budget(category, amount) values($1, $2)",
            &[&self.category, &self.amount],
        )?;
        Ok(())
    }
}
