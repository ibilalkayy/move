use crate::cli::flags::alert::{AlertCategory, AlertData};
use crate::common::common::create_file;
use crate::usecases::{
    alert::{alert_exists, alerts_exist},
    budget::budget_category_exists,
};
use csv::Writer;
use rusqlite::params;
use rusqlite::{Connection, Result, ToSql};
use tabled::{Table, Tabled};

#[derive(Tabled, Debug)]
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
        let category_name = &self.category.as_deref().unwrap_or("");
        let find_alert = alert_exists(conn, category_name);
        let find_budget_category = budget_category_exists(conn, category_name);

        match find_alert {
            Ok(true) => panic!("Alert data is already having {} category", category_name),
            Ok(false) => {
                match find_budget_category {
                    Ok(true) => {
                        conn.execute(
                            "insert into alert(category, frequency, method, dayz, hourz, minutez, secondz, weekdays) values(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                            &[&self.category, &self.frequency, &self.method, &self.day, &self.hour, &self.minute, &self.second, &self.weekday],
                        )?;
                    }
                    Ok(false) => panic!("{} category doesn't have budget data for alert insertion", category_name),
                    Err(error) => panic!("Err: {}", error),
                }
            },
            Err(error) => panic!("Err: {}", error),
        }
        Ok(())
    }

    pub fn update_alert(&self, conn: &Connection) -> Result<()> {
        let new_category = self.category.as_deref().unwrap_or("");
        let old_category = self.old_category.as_deref().unwrap_or("");
        let find_alert = alert_exists(conn, old_category);
        let find_budget_old_category = budget_category_exists(conn, old_category);
        let find_budget_new_category = budget_category_exists(conn, new_category);
        
        if !alert_exists(conn, new_category)? {
            match find_alert {
                Ok(true) => {
                    match find_budget_old_category {
                        Ok(true) => {
                            match find_budget_new_category {
                                Ok(true) => {
                                    let mut query = String::from("update alert set ");
                                    let mut fields = Vec::new();
                                    let mut values: Vec<&dyn ToSql> = Vec::new();
                            
                                    if let Some(category) = &self.category {
                                        fields.push("category = ?");
                                        values.push(category);
                                    }
                                    if let Some(frequency) = &self.frequency {
                                        fields.push("frequency = ?");
                                        values.push(frequency);
                                    }
                                    if let Some(method) = &self.method {
                                        fields.push("method = ?");
                                        values.push(method);
                                    }
                                    if let Some(day) = &self.day {
                                        fields.push("dayz = ?");
                                        values.push(day);
                                    }
                                    if let Some(hour) = &self.hour {
                                        fields.push("hourz = ?");
                                        values.push(hour);
                                    }
                                    if let Some(minute) = &self.minute {
                                        fields.push("minutez = ?");
                                        values.push(minute);
                                    }
                                    if let Some(second) = &self.second {
                                        fields.push("secondz = ?");
                                        values.push(second);
                                    }
                                    if let Some(weekday) = &self.weekday {
                                        fields.push("weekdays = ?");
                                        values.push(weekday);
                                    }
                            
                                    if fields.is_empty() {
                                        return Err(rusqlite::Error::InvalidQuery); // No fields to update
                                    }
                            
                                    query.push_str(&fields.join(", "));
                                    query.push_str(" WHERE category = ?");
                            
                                    values.push(&self.old_category); // WHERE condition
                            
                                    // Execute query only once
                                    let affected_rows = conn.execute(&query, rusqlite::params_from_iter(values))?;
                            
                                    if affected_rows == 0 {
                                        return Err(rusqlite::Error::QueryReturnedNoRows);
                                    }
                                }
                                Ok(false) => panic!("Budget is not having the data of new category {}", new_category),
                                Err(error) => panic!("Err: {}", error),
                            }
                        },
                        Ok(false) => panic!("Budget is not having a data of the old category {}", old_category),
                        Err(error) => panic!("Err: {}", error),
                    }
                }
                Ok(false) => panic!("{} category is not added to the alert record", self.category.as_deref().unwrap_or("")),
                Err(error) => panic!("Err: {}", error),
            }
        } else {
            panic!("Alert data already has a {} category", new_category);
        }

        Ok(())
    }

    pub fn get_alert(&self, conn: &Connection) -> Result<()> {
        let find_alert = alerts_exist(conn);
        match find_alert {
            Ok(true) => {
                let mut stmt = conn.prepare(
                    "select category, frequency, method, dayz, hourz, minutez, secondz, weekdays from alert",
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
        
                let file_path = create_file("alert.csv");
        
                let mut wtr = Writer::from_writer(file_path);
        
                wtr.write_record(&[
                    "Category",
                    "Frequency",
                    "Method",
                    "Day",
                    "Hour",
                    "Minute",
                    "Second",
                    "Weekday",
                ])
                .expect("failed to write the data in a CSV file");
        
                for alert in results {
                    wtr.write_record(&[
                        alert.category,
                        alert.frequency,
                        alert.method,
                        alert.day,
                        alert.hour,
                        alert.minute,
                        alert.second,
                        alert.weekday,
                    ])
                    .expect("failed to write the data in CSV file");
                }
        
                wtr.flush().expect("failed to flush the content");
            },
            Ok(false) => panic!("No alert data is present to get"),
            Err(error) => panic!("Err: {}", error),
        }
        Ok(())
    }
}

pub fn view_alert(conn: &Connection) -> Result<()> {
    let find_alert = alerts_exist(conn);
    match find_alert {
        Ok(true) => {
            let mut stmt = conn.prepare(
                "select category, frequency, method, dayz, hourz, minutez, secondz, weekdays from alert",
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
        
        },
        Ok(false) => panic!("No alert data is present to be viewed"),
        Err(error) => panic!("Err: {}", error),
    }

    Ok(())
}

impl AlertCategory {
    pub fn delete_alert(&self, conn: &Connection) -> Result<()> {
        let find_alert = alert_exists(conn, &self.category);
        match find_alert {
            Ok(true) => {
                let affected_rows =
                conn.execute("delete from alert where category = ?", &[&self.category])?;

                if affected_rows == 0 {
                    return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
                }
            },
            Ok(false) => panic!("Alert data doesn't have {} category", &self.category),
            Err(error) => panic!("Err: {}", error),
        }
        Ok(())
    }
}
