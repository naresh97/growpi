use std::time::Duration;

use chrono::{Local, Timelike};
use tokio::join;

use crate::{
    actuators,
    error::{lock_err, GenericResult},
    sensors,
    state::ProgramStateShared,
};

async fn temperature_control(program_state: ProgramStateShared) -> GenericResult<()> {
    let mut program_state = program_state.lock().map_err(lock_err)?;
    let current_temperature = sensors::get_temperature(&program_state.config)?;
    if current_temperature > 28. {
        actuators::switch_fan(crate::io::RelaySwitchState::On, &mut program_state)?;
    } else {
        actuators::switch_fan(crate::io::RelaySwitchState::Off, &mut program_state)?;
    }
    Ok(())
}

async fn temperature_control_loop(program_state: ProgramStateShared) {
    loop {
        let _ = temperature_control(program_state.clone()).await;
        tokio::time::sleep(Duration::from_mins(1)).await;
    }
}
async fn soil_moisture_control_loop(program_state: ProgramStateShared) {
    loop {
        tokio::time::sleep(Duration::from_days(1)).await;
    }
}

async fn light_control(program_state: ProgramStateShared) -> GenericResult<()> {
    let program_state = program_state.clone();
    let mut program_state = program_state.lock().map_err(lock_err)?;
    let local = Local::now();
    let hour = local.time().hour();
    const HOURS_ON: u32 = 24;
    if hour <= HOURS_ON {
        actuators::switch_lights(crate::io::RelaySwitchState::On, &mut program_state)?;
    } else {
        actuators::switch_lights(crate::io::RelaySwitchState::Off, &mut program_state)?;
    }

    Ok(())
}

async fn light_control_loop(program_state: ProgramStateShared) {
    loop {
        let _ = light_control(program_state.clone()).await;
        tokio::time::sleep(Duration::from_hours(1)).await;
    }
}

pub async fn control_thread(program_state: ProgramStateShared) {
    join!(
        light_control_loop(program_state.clone()),
        temperature_control_loop(program_state.clone()),
        soil_moisture_control_loop(program_state.clone())
    );
}
