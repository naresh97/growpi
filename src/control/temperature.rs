use std::time::Duration;

use crate::{
    actuators,
    error::GenericResult,
    sensors,
    state::{lock_state, ProgramStateShared},
};

fn temperature_control(program_state: ProgramStateShared) -> GenericResult<()> {
    let mut program_state = lock_state(&program_state)?;
    let config = &program_state.config.controller_settings;

    let current_temperature = sensors::get_temperature(&program_state.config)?;
    if current_temperature > config.temperature_set_point_upper {
        actuators::switch_fan(crate::io::RelaySwitchState::On, &mut program_state)?;
    } else if current_temperature < config.temperature_set_point_lower {
        actuators::switch_fan(crate::io::RelaySwitchState::Off, &mut program_state)?;
    }
    Ok(())
}

pub async fn temperature_control_loop(program_state: ProgramStateShared) {
    let loop_duration = program_state
        .lock()
        .map(|program_state| {
            program_state
                .config
                .controller_settings
                .temperature_loop_mins
        })
        .unwrap_or(1);

    loop {
        let _ = temperature_control(program_state.clone());
        tokio::time::sleep(Duration::from_mins(loop_duration)).await;
    }
}
