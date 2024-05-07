use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::error::GenericResult;

#[derive(Serialize, Deserialize)]
pub struct WateringRecord {
    pub time: i64,
    pub amount: u64,
    pub moisture_before_watering: f32,
}

impl WateringRecord {
    pub fn new(amount: u64, moisture_before_watering: f32) -> WateringRecord {
        WateringRecord {
            time: Utc::now().timestamp(),
            amount,
            moisture_before_watering,
        }
    }
}

#[derive(Default)]
pub struct History {
    pub watering_records: Vec<WateringRecord>,
}

const FILE_PATH: &str = "./growpi.history.csv";

impl History {
    pub fn save(&self) -> GenericResult<()> {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(true)
            .from_path(FILE_PATH)?;
        for record in &self.watering_records {
            writer.serialize(record)?;
        }
        writer.flush()?;
        Ok(())
    }

    pub fn load() -> GenericResult<History> {
        let mut history = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(FILE_PATH)?;
        let mut result = Vec::new();
        for record in history.deserialize() {
            result.push(record?);
        }
        Ok(History {
            watering_records: result,
        })
    }
}

#[cfg(test)]
mod tests {

    use chrono::Local;

    use super::*;

    #[test]
    fn test_write_default() {
        let mut history = History::default();
        history.watering_records.push(WateringRecord {
            time: Local::now().timestamp(),
            amount: 456,
            moisture_before_watering: 71.1,
        });
        history.save().unwrap();
    }
}
