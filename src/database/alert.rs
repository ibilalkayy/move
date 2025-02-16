use crate::cli::flags::alert::{AlertCategory, AlertData};
use crate::common::common::create_file;
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
        conn.execute(
            "insert into alert(category, frequency, method, dayz, hourz, minutez, secondz, weekdays) values(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            &[&self.category, &self.frequency, &self.method, &self.day, &self.hour, &self.minute, &self.second, &self.weekday],
        )?;
        Ok(())
    }

    pub fn update_alert(&self, conn: &Connection) -> Result<()> {
        if let Some(old_category) = &self.old_category {
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

            values.push(old_category); // WHERE condition

            // Execute query only once
            let affected_rows = conn.execute(&query, rusqlite::params_from_iter(values))?;

            if affected_rows == 0 {
                return Err(rusqlite::Error::QueryReturnedNoRows);
            }

            Ok(())
        } else {
            Err(rusqlite::Error::InvalidQuery) // If old_category is None
        }
    }

    pub fn get_alert(&self, conn: &Connection) -> Result<()> {
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
        Ok(())
    }
}

pub fn view_alert(conn: &Connection) -> Result<()> {
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

    Ok(())
}

impl AlertCategory {
    pub fn delete_alert(&self, conn: &Connection) -> Result<()> {
        let affected_rows =
            conn.execute("delete from alert where category = ?", &[&self.category])?;

        if affected_rows == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
        }

        Ok(())
    }
}
