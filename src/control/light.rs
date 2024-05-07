use std::time::Duration;

use chrono::{Local, Timelike};

use crate::{
    actuators,
    error::GenericResult,
    state::{lock_state, ProgramStateShared},
};

fn should_turn_on_light(on_hours: u64, lights_out: u64, current_hour: u64) -> bool {
    let off_hours = 24 - on_hours;
    (lights_out..(lights_out + off_hours))
        .map(|x| x % 24)
        .any(|x| x == current_hour)
}

fn light_control(program_state: ProgramStateShared) -> GenericResult<()> {
    let program_state = program_state.clone();
    let mut program_state = lock_state(&program_state)?;

    let on_hours = program_state.config.controller_settings.sunlight_hours;
    let current_hour = Local::now().time().hour() as u64;
    let lights_out_hour = program_state.config.controller_settings.lights_off_hour;

    if should_turn_on_light(on_hours, lights_out_hour, current_hour) {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_should_turn_on_light() {
        assert!(!should_turn_on_light(24, 5, 4));
        assert!(!should_turn_on_light(24, 5, 5));
        assert!(!should_turn_on_light(24, 5, 6));
        assert!(!should_turn_on_light(24, 0, 0));
        assert!(!should_turn_on_light(24, 0, 23));
        assert!(!should_turn_on_light(24, 0, 1));

        assert!(!should_turn_on_light(23, 5, 4));
        assert!(should_turn_on_light(23, 5, 5));
        assert!(!should_turn_on_light(23, 5, 6));
        assert!(!should_turn_on_light(23, 0, 23));
        assert!(should_turn_on_light(23, 0, 0));
        assert!(!should_turn_on_light(23, 0, 1));

        assert!(!should_turn_on_light(20, 22, 21));
        assert!(should_turn_on_light(20, 22, 22));
        assert!(should_turn_on_light(20, 22, 23));
        assert!(should_turn_on_light(20, 22, 0));
        assert!(should_turn_on_light(20, 22, 1));
        assert!(!should_turn_on_light(20, 22, 2));
    }
}
