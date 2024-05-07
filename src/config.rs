use serde::{Deserialize, Serialize};

use crate::error::GenericResult;

#[derive(Serialize, Deserialize)]
pub struct RelaySettings {
    pub light_pin: u8,
    pub fan_pin: u8,
    pub water_pump_pin: u8,
    pub relay_gpio_pins: Vec<i16>,
}

#[derive(Serialize, Deserialize)]
pub struct ThermistorSettings {
    pub pin: u8,
    pub voltage_divider_resistance: f32,
    pub nominal_resistance: f32,
    pub nominal_temperature: f32,
    pub thermal_constant: f32,
    pub resistor: VoltageDividerResistor,
}

#[derive(Serialize, Deserialize)]
pub enum VoltageDividerResistor {
    R1,
    R2,
}

#[derive(Serialize, Deserialize)]
pub struct SoilMoistureSettings {
    pub pin: u8,
    pub voltage_100: f32,
    pub voltage_nominal: f32,
    pub moisture_nominal: f32,
}

#[derive(Serialize, Deserialize)]
pub struct WaterPumpSettings {
    pub grams_per_millisecond: f32,
}

#[derive(Serialize, Deserialize)]
pub struct BoardSettings {
    pub logic_level: f32,
}

#[derive(Serialize, Deserialize)]
pub struct ControllerSettings {
    pub temperature_set_point_upper: f32,
    pub temperature_set_point_lower: f32,
    pub temperature_loop_mins: u64,
    pub sunlight_hours: u64,
    pub watering_frequency_hours: u64,
    pub watering_amount_grams: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DataLoggingSettings {
    pub enabled: bool,
    pub frequency_mins: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub board_settings: BoardSettings,
    pub relay_settings: RelaySettings,
    pub soil_moisture_settings: SoilMoistureSettings,
    pub thermistor_settings: ThermistorSettings,
    pub water_pump_settings: WaterPumpSettings,
    pub controller_settings: ControllerSettings,
    pub data_logging_settings: DataLoggingSettings,
}

impl Configuration {
    pub fn from_file(path: &std::path::Path) -> GenericResult<Configuration> {
        let text = std::fs::read_to_string(path)?;
        let config = toml::from_str(text.as_str())?;
        Ok(config)
    }
    pub fn save_to_file(&self, path: &std::path::Path) -> GenericResult<()> {
        let text = toml::to_string_pretty(self)?;
        std::fs::write(path, text)?;
        Ok(())
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            board_settings: BoardSettings { logic_level: 3.3 },
            relay_settings: RelaySettings {
                light_pin: 0,
                fan_pin: 1,
                water_pump_pin: 2,
                relay_gpio_pins: [17, 27, 22, -1].to_vec(),
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
                resistor: VoltageDividerResistor::R2,
            },
            water_pump_settings: WaterPumpSettings {
                grams_per_millisecond: 0.05281,
            },
            controller_settings: ControllerSettings {
                temperature_set_point_upper: 35.,
                temperature_set_point_lower: 28.,
                temperature_loop_mins: 60,
                sunlight_hours: 24,
                watering_frequency_hours: 30,
                watering_amount_grams: 200,
            },
            data_logging_settings: DataLoggingSettings {
                enabled: true,
                frequency_mins: 60,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_default() {
        let config = Configuration::default();
        config
            .save_to_file(std::path::Path::new("./growpi.toml"))
            .unwrap();
    }
}
