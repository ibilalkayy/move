use crate::cli::flags::budget::{BudgetCategory, BudgetData, UpdateBudget};
use crate::common::common::create_file;
use crate::usecases::{
    budget::{budget_category_exists, budget_total_equal, convert_to_u64},
    total_amount::total_amount_exists,
    total_categories::total_category_exists,
};
use csv::Writer;
use rusqlite::{params, Connection, Result, ToSql};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct BudgetRow {
    #[tabled(rename = "Category")]
    category: String,

    #[tabled(rename = "Amount")]
    amount: String,
}

impl BudgetData {
    pub fn insert_budget(&self, conn: &Connection) -> Result<()> {
        if total_category_exists(conn, &self.category)? {
            if total_amount_exists(conn)? {
                if budget_category_exists(conn, &self.category)? {
                    panic!("Category {} is already present", &self.category);
                } else {
                    match budget_total_equal(conn, "") {
                        Ok((total_amount, budget_total_sum, _, _)) => {
                            let given_amount = self.amount.parse::<u64>().unwrap();
                            let budget_amount = budget_total_sum + given_amount;

                            if budget_amount <= total_amount {
                                conn.execute(
                                    "insert into budget(category, amount) values(?1, ?2)",
                                    &[&self.category, &self.amount],
                                )?;
                            } else {
                                panic!(
                                    "Budget amount exceeded the total amount: {} > {}",
                                    budget_amount, total_amount
                                );
                            }
                        }
                        Err(error) => panic!("Err: {}", error),
                    }
                }
            } else {
                panic!("Total amount is not present in the list");
            }
        } else {
            panic!(
                "Category {} is not present in the total categories list",
                self.category
            );
        }
        Ok(())
    }

    pub fn get_budget(&self, conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare("select category, amount from budget")?;

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
        let file_path = create_file("budget.csv");

        let mut wtr = Writer::from_writer(file_path);
        wtr.write_record(&["Category", "Amount"])
            .expect("failed to write the data in a CSV file");

        for budget in results {
            wtr.write_record(&[budget.category, budget.amount])
                .expect("failed to write the data in a CSV file");
        }
        wtr.flush().expect("failed to flush the content");

        Ok(())
    }
}

impl BudgetCategory {
    pub fn view_budget(&self, conn: &Connection, category: &str) -> Result<()> {
        let mut stmt = conn.prepare("select category, amount from budget where category = ?")?;

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

    pub fn delete_budget(&self, conn: &Connection) -> Result<()> {
        let affected_rows =
            conn.execute("delete from budget where category = ?", &[&self.category])?;

        if affected_rows == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
        }

        Ok(())
    }
}

pub fn show_budget(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("select category, amount from budget")?;

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

impl UpdateBudget {
    pub fn update_budget(&self, conn: &Connection) -> Result<()> {
        let new_category: &str = self.new_category.as_deref().unwrap_or("");

        let mut query = String::from("update budget set ");
        let mut fields = Vec::new();
        let mut value: Vec<&dyn ToSql> = Vec::new();

        if let Some(new_category) = &self.new_category {
            fields.push("category = ?");
            value.push(new_category);
        }

        if let Some(amount) = &self.amount {
            fields.push("amount = ?");
            value.push(amount);
        }

        if fields.is_empty() {
            return Err(rusqlite::Error::InvalidQuery);
        }

        query.push_str(&fields.join(", "));
        query.push_str("where category = ?");

        value.push(&self.old_category);

        if total_category_exists(conn, new_category)? {
            if total_category_exists(conn, &self.old_category)? {
                if total_amount_exists(conn)? {
                    if budget_category_exists(conn, &self.old_category)? {
                        match budget_total_equal(conn, &self.old_category) {
                            Ok((total_amount, _, budget_except_sum, _)) => {
                                let given_amount = convert_to_u64(self.amount.clone());
                                let budget_amount = budget_except_sum + given_amount;

                                if budget_amount <= total_amount {
                                    let affected_rows =
                                        conn.execute(&query, rusqlite::params_from_iter(value))?;

                                    if affected_rows == 0 {
                                        return Err(rusqlite::Error::QueryReturnedNoRows);
                                    }
                                } else {
                                    panic!(
                                        "Budget amount exceeded the total amount: {} > {}",
                                        budget_amount, total_amount,
                                    );
                                }
                            }
                            Err(error) => panic!("Err: {}", error),
                        }
                    } else {
                        panic!(
                            "Category {} is not present in the budget record. First add a budget",
                            &self.old_category
                        );
                    }
                } else {
                    panic!("Total amount is not present in the record");
                }
            } else {
                panic!(
                    "Category {} is not present in the total categories list",
                    &self.old_category
                );
            }
        } else {
            panic!(
                "Category {} is not present in the total categories list. First add one",
                new_category
            );
        }
        Ok(())
    }
}
