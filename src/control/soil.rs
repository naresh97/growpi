use std::time::Duration;

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
    actuators::pump_water(
        watering_amount.try_into().unwrap_or(100),
        &mut program_state,
    )?;
    Ok(())
}
