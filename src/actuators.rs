use std::{thread, time::Duration};

use crate::{history::WateringRecord, io::RelaySwitchState, sensors, state::ProgramState};

pub fn switch_lights(
    state: RelaySwitchState,
    program_state: &mut ProgramState,
) -> anyhow::Result<()> {
    program_state
        .relay
        .switch(program_state.config.relay_settings.light_pin, state)
}

pub fn switch_fan(state: RelaySwitchState, program_state: &mut ProgramState) -> anyhow::Result<()> {
    program_state
        .relay
        .switch(program_state.config.relay_settings.fan_pin, state)
}

pub fn switch_water_pump(
    state: RelaySwitchState,
    program_state: &mut ProgramState,
) -> anyhow::Result<()> {
    program_state
        .relay
        .switch(program_state.config.relay_settings.water_pump_pin, state)
}

pub fn get_light_state(program_state: &mut ProgramState) -> anyhow::Result<RelaySwitchState> {
    let pin = program_state.config.relay_settings.light_pin;
    program_state.relay.get_state(pin)
}
pub fn get_water_pump_state(program_state: &mut ProgramState) -> anyhow::Result<RelaySwitchState> {
    let pin = program_state.config.relay_settings.water_pump_pin;
    program_state.relay.get_state(pin)
}
pub fn get_fan_state(program_state: &mut ProgramState) -> anyhow::Result<RelaySwitchState> {
    let pin = program_state.config.relay_settings.fan_pin;
    program_state.relay.get_state(pin)
}

pub fn pump_water(water_mass_g: u16, program_state: &mut ProgramState) -> anyhow::Result<()> {
    let duration_ms = water_mass_g as f32
        / program_state
            .config
            .water_pump_settings
            .grams_per_millisecond;
    let duration_ms = duration_ms.round() as u64;
    let duration = Duration::from_millis(duration_ms);
    let moisture_before_watering = sensors::get_soil_moisture(&program_state.config)?;
    switch_water_pump(RelaySwitchState::On, program_state)?;
    thread::sleep(duration);
    switch_water_pump(RelaySwitchState::Off, program_state)?;

    program_state
        .history
        .watering_records
        .push(WateringRecord::new(
            water_mass_g.into(),
            moisture_before_watering,
        ));
    program_state.history.save()?;

    Ok(())
}
