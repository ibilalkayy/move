use rusqlite::{Connection, Result};
use crate::cli::flags::total_amount::AddTotalCategory;

impl AddTotalCategory {
    pub fn insert_total_category(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "insert into totalcategories(category, label) values($1, $2)",
            &[&self.category, &self.label],
        )?;
        Ok(())
    }
}
