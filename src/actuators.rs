use crate::{
    config::*,
    io::{Relay, RelaySwitchState},
    GenericResult,
};

pub fn switch_lights(relay: &mut Relay, state: RelaySwitchState) -> GenericResult<()> {
    relay.switch(LIGHT_RELAY_PIN, state)
}

pub fn switch_fan(relay: &mut Relay, state: RelaySwitchState) -> GenericResult<()> {
    relay.switch(FAN_RELAY_PIN, state)
}

pub fn switch_water_pump(relay: &mut Relay, state: RelaySwitchState) -> GenericResult<()> {
    relay.switch(WATER_PUMP_RELAY_PIN, state)
}
