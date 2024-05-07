use std::time::Duration;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    config::DataLoggingSettings,
    error::GenericResult,
    sensors,
    state::{lock_state, ProgramStateShared},
};

#[derive(Serialize, Deserialize)]
pub struct DataRecord {
    pub timestamp: i64,
    pub temperature: f32,
    pub soil_mositure: f32,
}

#[derive(Serialize, Deserialize)]
pub struct DataRecords {
    pub records: Vec<DataRecord>,
}

const FILE_PATH: &str = "./growpi.datalog.csv";
impl DataRecords {
    pub fn push(program_state: ProgramStateShared) -> GenericResult<()> {
        let program_state = lock_state(&program_state)?;
        let config = &program_state.config;
        let record = DataRecord {
            timestamp: Utc::now().timestamp(),
            temperature: sensors::get_temperature(config)?,
            soil_mositure: sensors::get_soil_moisture(config)?,
        };
        let mut writer = csv::WriterBuilder::new()
            .has_headers(true)
            .from_path(FILE_PATH)?;
        writer.serialize(record)?;
        writer.flush()?;
        Ok(())
    }
}

pub async fn data_logging_loop(program_state: ProgramStateShared) {
    loop {
        let DataLoggingSettings {
            enabled,
            frequency_mins,
        } = lock_state(&program_state)
            .map(|state| state.config.data_logging_settings.clone())
            .unwrap_or(DataLoggingSettings {
                enabled: true,
                frequency_mins: 60,
            });
        if enabled {
            let _ = DataRecords::push(program_state.clone());
        }
        tokio::time::sleep(Duration::from_mins(frequency_mins)).await;
    }
}
