use crate::cli::flags::budget::{BudgetCategory, BudgetData, UpdateBudget};
use crate::common::common::create_file;
use crate::usecases::{
    budget::{budget_category_exists, budget_total_equal},
    total_amount::total_amount_exists,
    total_categories::{total_categories_exist, total_category_exists},
};
use csv::Writer;
use rusqlite::{params, Connection, Result, ToSql};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct BudgetRow {
    #[tabled(rename = "Category")]
    category: String,

    #[tabled(rename = "Amount")]
    amount: f64,

    #[tabled[rename = "Spent Amount"]]
    spent_amount: f64,

    #[tabled[rename = "Remaining Amount"]]
    remaining_amount: f64,
}

impl BudgetData {
    pub fn insert_budget(&self, conn: &Connection) -> Result<()> {
        let category_present = total_category_exists(conn, &self.category);
        let total_amount_present = total_amount_exists(conn);
        let budget_category_present = budget_category_exists(conn, &self.category);
        let is_budget_total_equal = budget_total_equal(conn, "");

        match category_present {
            Ok(true) => match total_amount_present {
                Ok(true) => match budget_category_present {
                    Ok(true) => panic!(
                        "Err: {} category is already present in the budget list. See 'move budget -h'",
                        &self.category
                    ),
                    Ok(false) => match is_budget_total_equal {
                        Ok((total_amount, budget_total_sum, _, _)) => {
                            let budget_amount = budget_total_sum + self.amount;

                            if budget_amount <= total_amount {
                                conn.execute(
                                    "insert into budget (category, amount, spent_amount, remaining_amount) values (?1, ?2, ?3, ?4)",
                                    (&self.category, self.amount, 0, self.amount),
                                )?;
                            } else {
                                panic!(
                                    "Err: budget amount exceeded the total amount: {} > {}",
                                    budget_amount, total_amount
                                );
                            }
                        }
                        Err(error) => panic!("Err: {}", error),
                    },
                    Err(error) => panic!("Err: {}", error),
                },
                Ok(false) => panic!("Err: amount is not added to the total amount list. See 'move total-amount -h'"),
                Err(error) => panic!("Err: {}", error),
            },
            Ok(false) => panic!(
                "Err: {} category is not added to the total categories list. See 'move total-amount -h'",
                self.category
            ),
            Err(error) => panic!("Err: {}", error),
        }
        Ok(())
    }

    pub fn get_budget(&self, conn: &Connection) -> Result<()> {
        let category_present = total_category_exists(conn, &self.category);
        match category_present {
            Ok(true) => {
                let mut stmt = conn.prepare(
                    "select category, amount, spent_amount, remaining_amount from budget",
                )?;

                let rows = stmt.query_map(params![], |row| {
                    Ok(BudgetRow {
                        category: row.get(0)?,
                        amount: row.get(1)?,
                        spent_amount: row.get(2)?,
                        remaining_amount: row.get(3)?,
                    })
                })?;

                let mut results = Vec::new();
                for row in rows {
                    results.push(row?);
                }
                let file_path = create_file("budget.csv");

                let mut wtr = Writer::from_writer(file_path);
                wtr.write_record(&["Category", "Amount"])
                    .expect("Err: failed to write the data in a CSV file");

                for budget in results {
                    wtr.write_record(&[budget.category, budget.amount.to_string()])
                        .expect("Err: failed to write the data in a CSV file");
                }
                wtr.flush().expect("Err: failed to flush the content");
            }
            Ok(false) => panic!(
                "Err: {} category is not added to the total categories list. See 'move total-amount -h'",
                &self.category
            ),
            Err(error) => panic!("Err: {}", error),
        }
        Ok(())
    }
}

impl BudgetCategory {
    pub fn view_budget(&self, conn: &Connection) -> Result<()> {
        let category_present = total_category_exists(conn, &self.category);
        match category_present {
            Ok(true) => {
                let mut stmt = conn.prepare("select category, amount, spent_amount, remaining_amount from budget where category = ?")?;

                let rows = stmt.query_map(params![self.category], |row| {
                    Ok(BudgetRow {
                        category: row.get(0)?,
                        amount: row.get(1)?,
                        spent_amount: row.get(2)?,
                        remaining_amount: row.get(3)?,
                    })
                })?;

                let mut results = Vec::new();
                for row in rows {
                    results.push(row?);
                }

                let table = Table::new(results);
                println!("{}", table);
            }
            Ok(false) => panic!(
                "Err: {} category is not added to the total categories list. See 'move total-amount -h'",
                &self.category
            ),
            Err(error) => panic!("Err: {}", error),
        }

        Ok(())
    }

    pub fn delete_budget(&self, conn: &Connection) -> Result<()> {
        let category_present = total_category_exists(conn, &self.category);
        match category_present {
            Ok(true) => {
                let affected_rows =
                    conn.execute("delete from budget where category = ?", &[&self.category])?;

                if affected_rows == 0 {
                    panic!("Err: budget is not added yet. See 'move budget -h'");
                }
            }
            Ok(false) => panic!(
                "Err: {} category is not added to the total categories list. See 'move total-amount -h'",
                &self.category
            ),
            Err(error) => panic!("Err: {}", error),
        }

        Ok(())
    }
}

pub fn show_budget(conn: &Connection) -> Result<()> {
    let category_present = total_categories_exist(conn);
    match category_present {
        Ok(true) => {
            let mut stmt = conn
                .prepare("select category, amount, spent_amount, remaining_amount from budget")?;

            let rows = stmt.query_map(params![], |row| {
                Ok(BudgetRow {
                    category: row.get(0)?,
                    amount: row.get(1)?,
                    spent_amount: row.get(2)?,
                    remaining_amount: row.get(3)?,
                })
            })?;

            let mut results = Vec::new();
            for row in rows {
                results.push(row?);
            }

            let table = Table::new(results);
            println!("{}", table);
        }
        Ok(false) => panic!(
            "Err: no category is added to the total categories list. See 'move total-amount -h'"
        ),
        Err(error) => panic!("Err: {}", error),
    }
    Ok(())
}

impl UpdateBudget {
    pub fn update_budget(&self, conn: &Connection) -> Result<()> {
        let new_budget_category: Option<&str> = self.new_category.as_deref();

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

        if let Some(amount) = &self.amount {
            fields.push("remaining_amount = ?");
            value.push(amount);
        }

        if fields.is_empty() {
            return Err(rusqlite::Error::InvalidQuery);
        }

        query.push_str(&fields.join(", "));
        query.push_str(" where category = ?");

        value.push(&self.old_category);

        let old_category_present = total_category_exists(conn, &self.old_category);
        let total_amount_present = total_amount_exists(conn);
        let budget_old_category_present = budget_category_exists(conn, &self.old_category);
        let is_budget_total_equal = budget_total_equal(conn, &self.old_category);

        // Only check if the new category exists *if* it's provided
        let total_new_category_present = if let Some(new_category) = new_budget_category {
            total_category_exists(conn, new_category)
        } else {
            Ok(false)
        };

        // Only check if the budget contains new category *if* it's provided
        let budget_new_category_present = if let Some(new_category) = new_budget_category {
            budget_category_exists(conn, new_category)
        } else {
            Ok(false)
        };

        match old_category_present {
            Ok(true) => match total_amount_present {
                Ok(true) => match budget_old_category_present {
                    Ok(true) => match total_new_category_present {
                        Ok(true) => match budget_new_category_present {
                            Ok(true) => panic!(
                                "Err: {} category is already present in the budget list. See 'move budget -h'",
                                new_budget_category.unwrap()
                            ),
                            Ok(false) => match is_budget_total_equal {
                                Ok((total_amount, _, budget_except_sum, _)) => {
                                    let mut budget_amount: f64 = 0.0;
                                    match self.amount {
                                        Some(amount) => {
                                            budget_amount = budget_except_sum + amount
                                        },
                                        None => {},
                                    }
                                    if budget_amount <= total_amount {
                                        let affected_rows = conn
                                            .execute(&query, rusqlite::params_from_iter(value))?;
                                        if affected_rows == 0 {
                                            panic!("Err: budget is not added yet. See 'move budget -h'");
                                        }
                                    } else {
                                        panic!(
                                            "Err: budget amount exceeded the total amount: {} > {}",
                                            budget_amount, total_amount,
                                        );
                                    }
                                }
                                Err(error) => panic!("Err: {}", error),
                            },
                            Err(error) => panic!("Err: {}", error),
                        },
                        Ok(false) => {
                            let affected_rows =
                                conn.execute(&query, rusqlite::params_from_iter(value))?;
                            if affected_rows == 0 {
                                panic!("Err: budget is not added yet. See 'move budget -h'");
                            }
                        }
                        Err(error) => panic!("Err: {}", error),
                    },
                    Ok(false) => panic!(
                        "Err: {} category is not added to the budget list. See 'move budget -h'",
                        &self.old_category
                    ),
                    Err(error) => panic!("Err: {}", error),
                },
                Ok(false) => panic!("Err: amount is not added to the total amount list. See 'move total-amount -h'"),
                Err(error) => panic!("Err: {}", error),
            },
            Ok(false) => panic!(
                "Err: {} category is not added to the total categories list. See 'move total-amount -h'",
                self.old_category
            ),
            Err(error) => panic!("Err: {}", error),
        }
        Ok(())
    }
}
