use crate::cli::flags::total_categories::{TotalCategory, UpdateTotalCategory};
use crate::common::common::create_file;
use crate::usecases::total_categories::{total_categories_exist, total_category_exists};
use csv::Writer;
use rusqlite::{params, Connection, Result, ToSql};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct CategoryRow {
    #[tabled(rename = "Category")]
    category: String,

    #[tabled(rename = "Label")]
    label: String,
}

impl TotalCategory {
    pub fn insert_total_category(&self, conn: &Connection) -> Result<()> {
        let find_category = total_category_exists(conn, &self.category);
        match find_category {
            Ok(true) => panic!("Category {} is already present in the record", &self.category),
            Ok(false) => {
                conn.execute(
                    "insert into totalcategories(category, label) values(?1, ?2)",
                    &[&self.category, &self.label],
                )?;
            }
            Err(error) => println!("Err: {}", error),
        }
        Ok(())
    }

    pub fn get_total_categories(&self, conn: &Connection) -> Result<()> {
        let find_category = total_categories_exist(conn);
        match find_category {
            Ok(true) => {
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

                wtr.write_record(&["Category", "Label"])
                    .expect("failed to write the data in a CSV file");

                for categories in result {
                    wtr.write_record(&[categories.category, categories.label])
                        .expect("failed to write the data in a CSV file");
                }

                wtr.flush().expect("failed to flush the content");
            }
            Ok(false) => panic!("No category exists to get"),
            Err(error) => println!("Err: {}", error),
        }
        Ok(())
    }
}

pub fn view_total_categories(conn: &Connection) -> Result<()> {
    let find_category = total_categories_exist(conn);
    match find_category {
        Ok(true) => {
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
        }
        Ok(false) => panic!("No category exists to view"),
        Err(error) => println!("Err: {}", error),
    }
    Ok(())
}

impl UpdateTotalCategory {
    pub fn update_total_category(&self, conn: &Connection) -> Result<()> {
        let new_category: &str = self.new_category.as_deref().unwrap_or("");

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

        value.push(&self.old_category);

        if !total_category_exists(conn, new_category)? { // if the new category name is already present
            let find_category = total_category_exists(conn, &self.old_category);
            match find_category {
                Ok(true) => {
                    let affected_rows = conn.execute(&query, rusqlite::params_from_iter(value))?;
                    if affected_rows == 0 {
                        return Err(rusqlite::Error::QueryReturnedNoRows);
                    }
                }
                Ok(false) => panic!("Category {} is not present in the old categories list", &self.old_category),
                Err(error) => println!("Err: {}", error),
            }
        } else {
            panic!("Category {} is already present in the new categories list", new_category);
        }

        Ok(())
    }
}
