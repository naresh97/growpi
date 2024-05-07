use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    error::{lock_err, GenericResult},
    sensors,
    state::ProgramStateShared,
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
    fn load() -> GenericResult<DataRecords> {
        let mut data = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(FILE_PATH)?;
        let mut result = Vec::new();
        for record in data.deserialize() {
            result.push(record?);
        }
        Ok(DataRecords { records: result })
    }
    pub fn push(program_state: ProgramStateShared) -> GenericResult<()> {
        let program_state = program_state.lock().map_err(lock_err)?;
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
