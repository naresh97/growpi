use serde::{Deserialize, Serialize};

use crate::error::GenericResult;

#[derive(Serialize, Deserialize)]
pub struct RelaySettings {
    pub light_pin: u8,
    pub fan_pin: u8,
    pub water_pump_pin: u8,
    pub relay_gpio_pins: Vec<Option<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct ThermistorSettings {
    pub pin: u8,
    pub voltage_divider_resistance: f32,
    pub nominal_resistance: f32,
    pub nominal_temperature: f32,
    pub thermal_constant: f32,
}

#[derive(Serialize, Deserialize)]
pub struct SoilMoistureSettings {
    pub pin: u8,
    pub voltage_100: f32,
    pub voltage_nominal: f32,
    pub moisture_nominal: f32,
}

#[derive(Serialize, Deserialize)]
pub struct BoardSettings {
    pub logic_level: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub board_settings: BoardSettings,
    pub relay_settings: RelaySettings,
    pub soil_moisture_settings: SoilMoistureSettings,
    pub thermistor_settings: ThermistorSettings,
}

impl Configuration {
    fn from_file(path: &std::path::Path) -> GenericResult<Configuration> {
        let text = std::fs::read_to_string(path)?;
        let config: Configuration = toml::from_str(text.as_str())?;
        Ok(config)
    }
    fn save_to_file(path: &std::path::Path, config: &Configuration) -> GenericResult<()> {
        let text = toml::to_string_pretty(config)?;
        std::fs::write(path, text)?;
        Ok(())
    }
}

const THERMISTOR_NOMINAL_RESISTANCE: f32 = 10_000.;
const THERMISTOR_NOMINAL_TEMPERATURE: f32 = 298.15;
const THERMISTOR_CONSTANT: f32 = 3950.;

impl Default for Configuration {
    fn default() -> Self {
        Self {
            board_settings: BoardSettings { logic_level: 3.3 },
            relay_settings: RelaySettings {
                light_pin: 0,
                fan_pin: 1,
                water_pump_pin: 2,
                relay_gpio_pins: [Some(17), Some(27), Some(22), None].to_vec(),
            },
            soil_moisture_settings: SoilMoistureSettings {
                pin: 1,
                voltage_100: 1.417,
                voltage_nominal: 2.823,
                moisture_nominal: 0.41,
            },
            thermistor_settings: ThermistorSettings {
                pin: 0,
                voltage_divider_resistance: 9_700.,
                nominal_resistance: 10_000.,
                nominal_temperature: 298.15,
                thermal_constant: 3950.,
            },
        }
    }
}
