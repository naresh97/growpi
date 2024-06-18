use std::time::Duration;

use crate::{actuators, io, state::ProgramStateShared};

pub async fn ventilation_control_loop(program_state: ProgramStateShared) {
    loop {
        let ventilation_frequency = program_state
            .lock()
            .await
            .config
            .ventilation_settings
            .frequency_mins;
        let ventilation_frequency = Duration::from_mins(ventilation_frequency as u64);
        let _ = ventilation_control(program_state.clone()).await;
        tokio::time::sleep(ventilation_frequency).await;
    }
}

async fn ventilation_control(program_state: ProgramStateShared) -> anyhow::Result<()> {
    let ventilation_duration;
    let fan_state;
    {
        let mut program_state = program_state.lock().await;
        fan_state = actuators::get_fan_state(&mut program_state)?;
        ventilation_duration = Duration::from_mins(
            program_state
                .config
                .ventilation_settings
                .duration_mins
                .into(),
        );
        actuators::switch_fan(io::RelaySwitchState::On, &mut program_state)?;
    }
    tokio::time::sleep(ventilation_duration).await;
    {
        let mut program_state = program_state.lock().await;
        actuators::switch_fan(fan_state, &mut program_state)?;
    }
    Ok(())
}
