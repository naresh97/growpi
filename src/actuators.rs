use std::{thread, time::Duration};

use crate::{
    config::*,
    error::GenericResult,
    io::{Relay, RelaySwitchState},
};

pub fn switch_lights(
    relay: &mut Relay,
    state: RelaySwitchState,
    config: &Configuration,
) -> GenericResult<()> {
    relay.switch(config.relay_settings.light_pin, state, config)
}

pub fn switch_fan(
    relay: &mut Relay,
    state: RelaySwitchState,
    config: &Configuration,
) -> GenericResult<()> {
    relay.switch(config.relay_settings.fan_pin, state, config)
}

pub fn switch_water_pump(
    relay: &mut Relay,
    state: RelaySwitchState,
    config: &Configuration,
) -> GenericResult<()> {
    relay.switch(config.relay_settings.water_pump_pin, state, config)
}

pub fn pump_water(
    water_mass_g: u16,
    relay: &mut Relay,
    config: &Configuration,
) -> GenericResult<()> {
    let duration_ms = water_mass_g as f32 / config.water_pump_settings.grams_per_millisecond;
    let duration_ms = duration_ms.round() as u64;
    let duration = Duration::from_millis(duration_ms);
    switch_water_pump(relay, RelaySwitchState::On, config)?;
    thread::sleep(duration);
    switch_water_pump(relay, RelaySwitchState::Off, config)?;
    Ok(())
}
