use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::{
    error::GenericResult,
    state::{lock_state, ProgramStateShared},
};

pub async fn soil_moisture_control_loop(program_state: ProgramStateShared) {
    let loop_duration = program_state
        .lock()
        .map(|program_state| program_state.config.controller_settings.soil_loop_hours)
        .unwrap_or(1);

    loop {
        let _ = soil_moisture_control(program_state.clone());
        tokio::time::sleep(Duration::from_hours(loop_duration)).await;
    }
}

fn soil_moisture_control(program_state: ProgramStateShared) -> GenericResult<()> {
    let program_state = lock_state(&program_state)?;
    let config = &program_state.config.controller_settings;
    let history = &program_state.history.watering_records;

    let water_amount_over_window: u64 = history
        .iter()
        .filter(|record| {
            if let Some(time) = DateTime::from_timestamp(record.time, 0) {
                let delta = Utc::now() - time;
                let delta = delta.num_hours();
                delta <= config.max_water_window_hours.try_into().unwrap_or(24)
            } else {
                false
            }
        })
        .map(|record| record.amount)
        .sum();

    Ok(())
}
