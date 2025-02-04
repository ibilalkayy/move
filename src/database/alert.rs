use crate::cli::flags::alert::{AlertData, AlertValues};
use crate::database::db::connection;
use tabled::{Table, Tabled};
use std::error::Error;

#[derive(Tabled)]
pub struct AlertDataRow {
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
    pub fn create_alert(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute(
            "insert into alert(
                category,
                frequency,
                method,
                dayz,
                hourz,
                minutez,
                secondz,
                weekdays
            ) values($1, $2, $3, $4, $5, $6, $7, $8)",
            &[
                &self.category,
                &self.frequency,
                &self.method,
                &self.day,
                &self.hour,
                &self.minute,
                &self.second,
                &self.weekday,
            ],
        )?;
        Ok(())
    }

    pub fn update_data(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        if let Some(old_category) = &self.old_category {
            let _ = client.execute(
                "update alert set 
                category=$1,
                frequency=$2,
                method=$3,
                dayz=$4,
                hourz=$5,
                minutez=$6,
                secondz=$7,
                weekdays=$8
                where category=$9",
                &[
                    &self.category,
                    &self.frequency,
                    &self.method,
                    &self.day,
                    &self.hour,
                    &self.minute,
                    &self.second,
                    &self.weekday,
                    &old_category,
                ],
            )?;
            Ok(())
        } else {
            Err("Old category is required to update the alert".into())
        }
    }
}

impl AlertValues {
    pub fn get_alert(&self) {
        println!("Get your alert notifications through email");
    }

    pub fn see_alert(&self) {
        println!("See your alert notifications in the terminal");
    }

    pub fn remove_alert(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute("delete from alert where category=$1", &[&self.category])?;
        Ok(())
    }
}


pub fn view_alert() -> Result<(), Box<dyn Error>> {
    let mut client = connection()?;
    let mut rows = Vec::new();

    for row in client.query (
        "select
            category,
            frequency,
            method,
            dayz,
            hourz,
            minutez,
            secondz,
            weekdays
            from alert", 
        &[],
    )? {
        let category: String = row.get(0);
        let frequency: String = row.get(1);
        let method: String = row.get(2);
        let day: String = row.get(3);
        let hour: String = row.get(4);
        let minute: String = row.get(5);
        let second: String = row.get(6);
        let weekday: String = row.get(7);

        rows.push(AlertDataRow {
            category,
            frequency,
            method,
            day,
            hour,
            minute,
            second,
            weekday,
        })
    }

    let table = Table::new(rows);
    println!("{}", table);
    Ok(())
}