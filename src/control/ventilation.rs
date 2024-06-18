use std::time::Duration;

use crate::{
    actuators,
    error::GenericResult,
    io,
    state::{lock_state, ProgramStateShared},
};

pub async fn ventilation_control_loop(program_state: ProgramStateShared) -> GenericResult<()> {
    loop {
        let ventilation_frequency = lock_state(&program_state)?
            .config
            .ventilation_settings
            .frequency_mins;
        let ventilation_frequency = Duration::from_mins(ventilation_frequency as u64);
        ventilation_control(program_state.clone()).await?;
        tokio::time::sleep(ventilation_frequency).await;
    }
}

async fn ventilation_control(program_state: ProgramStateShared) -> GenericResult<()> {
    let mut program_state = lock_state(&program_state)?;
    let fan_state = actuators::get_fan_state(&mut program_state)?;

    let ventilation_duration = program_state.config.ventilation_settings.duration_mins;
    let ventilation_duration = Duration::from_mins(ventilation_duration.into());

    actuators::switch_fan(io::RelaySwitchState::On, &mut program_state);
    tokio::time::sleep(ventilation_duration).await;
    actuators::switch_fan(fan_state, &mut program_state);

    Ok(())
}
