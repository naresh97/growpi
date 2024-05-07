use std::time::Duration;

use crate::state::ProgramStateShared;

pub async fn soil_moisture_control_loop(program_state: ProgramStateShared) {
    let loop_duration = program_state
        .lock()
        .map(|program_state| program_state.config.controller_settings.soil_loop_hours)
        .unwrap_or(1);

    loop {
        tokio::time::sleep(Duration::from_hours(loop_duration)).await;
    }
}
