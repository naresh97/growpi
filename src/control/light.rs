use std::time::Duration;

use chrono::{Local, Timelike};

use crate::{
    actuators,
    error::{lock_err, GenericResult},
    state::ProgramStateShared,
};

fn light_control(program_state: ProgramStateShared) -> GenericResult<()> {
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

pub async fn light_control_loop(program_state: ProgramStateShared) {
    loop {
        let _ = light_control(program_state.clone());
        tokio::time::sleep(Duration::from_hours(1)).await;
    }
}
