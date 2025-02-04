use crate::cli::flags::total_amount::{AddTotalCategory, UpdateTotalCategories};
use crate::database::db::connection;
use std::error::Error;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct TotalCategoryRow {
    #[tabled(rename = "Category")]
    category: String,

    #[tabled(rename = "Label")]
    label: String,
}

impl AddTotalCategory {
    pub fn insert_total_categories(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute(
            "insert into totalcategories(category, label) values($1, $2)",
            &[&self.category, &self.label],
        )?;
        Ok(())
    }
}

pub fn view_total_categories() -> Result<(), Box<dyn Error>> {
    let mut client = connection()?;
    let mut rows = Vec::new();

    for row in client.query("select category, label from totalcategories", &[])? {
        let category: String = row.get(0);
        let label: String = row.get(1);

        rows.push(TotalCategoryRow { category, label });
    }

    let table = Table::new(rows);
    println!("{}", table);

    Ok(())
}

impl UpdateTotalCategories {
    pub fn update_total_category(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute(
            "update totalcategories set category=$1, label=$2 where category=$3",
            &[&self.new_category, &self.label, &self.old_category],
        )?;

        let _ = client.execute(
            "update budget set category=$1 where category=$2",
            &[&self.new_category, &self.old_category],
        )?;
        Ok(())
    }
}
