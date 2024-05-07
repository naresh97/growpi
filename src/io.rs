use ads1x1x::{Ads1x1x, ChannelSelection, DynamicOneShot};
use nb::block;
use rppal::gpio::{Gpio, OutputPin};

use crate::{config::*, error::GenericResult};

pub fn get_input_voltage(pin: u8) -> GenericResult<f32> {
    const ADS1115_DEFAULT_RANGE: f32 = 4.096;

    let adc = rppal::i2c::I2c::new()?;
    let mut adc = Ads1x1x::new_ads1115(adc, ads1x1x::SlaveAddr::Alternative(false, false));
    adc.set_full_scale_range(ads1x1x::FullScaleRange::Within4_096V)
        .map_err(|_| "Could not set full scale range")?;
    let channel: ChannelSelection = match pin {
        0 => ChannelSelection::SingleA0,
        1 => ChannelSelection::SingleA1,
        2 => ChannelSelection::SingleA2,
        3 => ChannelSelection::SingleA3,
        _ => return Err(format!("Pin {} not available. Only 0-3", pin).into()),
    };
    let result = block!(adc.read(channel)).map_err(|e| format!("{:?}", e))?;
    let result = result as f32;
    let result = result / i16::MAX as f32 * ADS1115_DEFAULT_RANGE;
    Ok(result)
}

pub struct Relay {
    relay_pins: [Option<rppal::gpio::OutputPin>; RELAY_GPIO_PINS.len()],
}
pub enum RelaySwitchState {
    On,
    Off,
}
impl Relay {
    pub fn new() -> GenericResult<Relay> {
        let mut output_pins = RELAY_GPIO_PINS.map(|pin| {
            pin.and_then(|pin| {
                let result =
                    (|| -> GenericResult<OutputPin> { Ok(Gpio::new()?.get(pin)?.into_output()) })();
                result.ok()
            })
        });
        for pin in output_pins.iter_mut().flatten() {
            // The relay turns ON on LOW
            pin.set_high();
        }
        Ok(Relay {
            relay_pins: output_pins,
        })
    }
    pub fn toggle(&mut self, pin: u8) -> GenericResult<()> {
        let pin = self.get_output_pin(pin)?;
        pin.toggle();
        Ok(())
    }

    pub fn switch(&mut self, pin: u8, state: RelaySwitchState) -> GenericResult<()> {
        let pin = self.get_output_pin(pin)?;
        match state {
            RelaySwitchState::On => pin.set_low(),
            RelaySwitchState::Off => pin.set_high(),
        }
        Ok(())
    }

    fn get_output_pin(&mut self, pin: u8) -> GenericResult<&mut OutputPin> {
        Ok(self
            .relay_pins
            .get_mut(pin as usize)
            .ok_or(format!(
                "Pin {} not within pin array with length {}",
                pin,
                RELAY_GPIO_PINS.len()
            ))?
            .as_mut()
            .ok_or("Pin not configured.")?)
    }
}
