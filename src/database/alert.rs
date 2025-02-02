use crate::cli::flags::alert::{AlertData, AlertValues};
use crate::database::db::connection;
use std::error::Error;

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
