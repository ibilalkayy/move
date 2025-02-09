use crate::cli::flags::total_amount::{TotalCategory, UpdateTotalCategory};
use csv::Writer;
use rusqlite::{params, Connection, Result, ToSql};
use tabled::{Table, Tabled};
use crate::common::common::create_file;

#[derive(Tabled)]
struct CategoryRow {
    #[tabled(rename = "Category")]
    category: String,

    #[tabled(rename = "Label")]
    label: String,
}

impl TotalCategory {
    pub fn insert_total_category(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "insert into totalcategories(category, label) values(?1, ?2)",
            &[&self.category, &self.label],
        )?;
        Ok(())
    }

    pub fn get_total_categories(&self, conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare("select category, label from totalcategories")?;

        let rows = stmt.query_map(params![], |row| {
            Ok(CategoryRow {
                category: row.get(0)?,
                label: row.get(1)?,
            })
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?)
        }

        let file_path = create_file("categories.csv");

        let mut wtr = Writer::from_writer(file_path);

        wtr.write_record(&["Category", "Label"]).unwrap();

        for categories in result {
            wtr.write_record(&[categories.category, categories.label])
                .unwrap();
        }

        wtr.flush().unwrap();

        Ok(())
    }
}

pub fn view_total_categories(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT category, label FROM totalcategories")?;

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

impl UpdateTotalCategory {
    pub fn update_total_category(&self, conn: &Connection) -> Result<()> {
        if let Some(old_category) = &self.old_category {
            let mut query = String::from("update totalcategories set ");
            let mut fields = Vec::new();
            let mut value: Vec<&dyn ToSql> = Vec::new();

            if let Some(new_category) = &self.new_category {
                fields.push("category = ?");
                value.push(new_category);
            }

            if let Some(label) = &self.label {
                fields.push("label = ?");
                value.push(label);
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
            Err(rusqlite::Error::InvalidQuery)
        }
    }
}
