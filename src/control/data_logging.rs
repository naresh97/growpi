use std::time::Duration;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{error::GenericResult, sensors, state::ProgramStateShared};

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
    pub async fn push(program_state: ProgramStateShared) -> GenericResult<()> {
        let program_state = program_state.lock().await;
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
        let data_logging_settings = program_state
            .lock()
            .await
            .config
            .data_logging_settings
            .clone();
        let (enabled, frequency_mins) = (
            data_logging_settings.enabled,
            data_logging_settings.frequency_mins,
        );
        if enabled {
            let _ = DataRecords::push(program_state.clone());
        }
        tokio::time::sleep(Duration::from_mins(frequency_mins)).await;
    }
}
