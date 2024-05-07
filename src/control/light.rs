use std::time::Duration;

use chrono::{Local, Timelike};

use crate::{
    actuators,
    error::GenericResult,
    state::{lock_state, ProgramStateShared},
};

fn light_control(program_state: ProgramStateShared) -> GenericResult<()> {
    let program_state = program_state.clone();
    let mut program_state = lock_state(&program_state)?;
    let local = Local::now();
    let hour = local.time().hour();
    if hour as u64 <= program_state.config.controller_settings.sunlight_hours {
        actuators::switch_lights(crate::io::RelaySwitchState::On, &mut program_state)?;
    } else {
        actuators::switch_lights(crate::io::RelaySwitchState::Off, &mut program_state)?;
    }

    Ok(())
}

pub async fn light_control_loop(program_state: ProgramStateShared) {
    loop {
        let _ = light_control(program_state.clone());
        tokio::time::sleep(Duration::from_hours(1)).await;
    }
}
