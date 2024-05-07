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
