use ads1x1x::{Ads1x1x, ChannelSelection, DynamicOneShot};
use async_process::Command;
use nb::block;
use rppal::gpio::{Gpio, OutputPin};
use serde::{Deserialize, Serialize};

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
    relay_pins: Vec<Option<rppal::gpio::OutputPin>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RelaySwitchState {
    On,
    Off,
}
impl Relay {
    pub fn new(config: &Configuration) -> GenericResult<Relay> {
        let mut output_pins = config
            .relay_settings
            .relay_gpio_pins
            .clone()
            .into_iter()
            .map(|pin| {
                match pin {
                    -1 => None,
                    _ => Some(pin as u8),
                }
                .and_then(|pin| {
                    let result = (|| -> GenericResult<OutputPin> {
                        Ok(Gpio::new()?.get(pin)?.into_output())
                    })();
                    result.ok()
                })
            })
            .collect::<Vec<_>>();
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

    pub fn get_state(&mut self, pin: u8) -> GenericResult<RelaySwitchState> {
        let pin = self.get_output_pin(pin)?;
        if pin.is_set_high() {
            Ok(RelaySwitchState::Off)
        } else {
            Ok(RelaySwitchState::On)
        }
    }

    fn get_output_pin(&mut self, pin: u8) -> GenericResult<&mut OutputPin> {
        Ok(self
            .relay_pins
            .get_mut(pin as usize)
            .ok_or(format!("Pin {} not within pin array", pin,))?
            .as_mut()
            .ok_or("Pin not configured.")?)
    }
}

#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize)]
pub enum ImageResolution {
    R1080p,
    R720p,
    R480p,
    R360p,
}
impl ImageResolution {
    fn get_width_height(&self) -> (u64, u64) {
        match self {
            ImageResolution::R1080p => (1920, 1080),
            ImageResolution::R720p => (1280, 720),
            ImageResolution::R480p => (640, 480),
            ImageResolution::R360p => (480, 360),
        }
    }
}
pub async fn capture_image(
    resolution: &ImageResolution,
    path: &std::path::Path,
) -> GenericResult<()> {
    let path = std::path::absolute(path)?;
    let (width, height) = resolution.get_width_height();
    Command::new("/usr/bin/libcamera-jpeg")
        .arg("-o")
        .arg(path.clone())
        .arg("-t")
        .arg("1")
        .arg("--width")
        .arg(width.to_string())
        .arg("--height")
        .arg(height.to_string())
        .status()
        .await?
        .exit_ok()?;
    Ok(())
}
