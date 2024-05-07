use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::{
    actuators,
    error::GenericResult,
    state::{lock_state, ProgramStateShared},
};

pub async fn soil_moisture_control_loop(program_state: ProgramStateShared) {
    loop {
        let _ = soil_moisture_control(program_state.clone());
        let watering_frequency_hours = program_state
            .lock()
            .map(|program_state| {
                program_state
                    .config
                    .controller_settings
                    .watering_frequency_hours
            })
            .unwrap_or(72);
        tokio::time::sleep(Duration::from_hours(watering_frequency_hours)).await;
    }
}

fn soil_moisture_control(program_state: ProgramStateShared) -> GenericResult<()> {
    let mut program_state = lock_state(&program_state)?;
    let config = &program_state.config.controller_settings;
    let watering_amount = config.watering_amount_grams;
    let last_watering_time = program_state
        .history
        .watering_records
        .iter()
        .max_by_key(|x| x.time)
        .and_then(|record| DateTime::from_timestamp(record.time, 0));
    if let Some(last_watering_time) = last_watering_time {
        let hours_passed = (Utc::now() - last_watering_time).num_hours();
        if hours_passed as u64 <= config.watering_frequency_hours {
            return Err("Watered too soon ago".into());
        }
    } else {
        return Err("Could not load last watering time".into());
    }
    actuators::pump_water(
        watering_amount.try_into().unwrap_or(100),
        &mut program_state,
    )?;
    Ok(())
}
