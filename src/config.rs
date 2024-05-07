pub const LOGIC_LEVEL: f32 = 3.3;
pub const THERMISTOR_ANALOG_PIN: u8 = 0;
pub const THERMISTOR_VOLTAGE_DIVIDER_RESISTANCE: f32 = 9_700.;

pub const LIGHT_RELAY_PIN: u8 = 0;
pub const FAN_RELAY_PIN: u8 = 1;
pub const WATER_PUMP_RELAY_PIN: u8 = 2;
pub const RELAY_GPIO_PINS: [Option<u8>; 4] = [Some(17), Some(27), Some(22), None];

pub const SOIL_MOISTURE_PIN: u8 = 1;
pub const SOIL_MOISTURE_100_VOLTAGE: f32 = 1.417;
pub const SOIL_NOMINAL_MOISTURE: f32 = 0.41;
pub const SOIL_NOMINAL_MOISTURE_VOLTAGE: f32 = 2.823;
