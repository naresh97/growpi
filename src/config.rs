pub const LOGIC_LEVEL: f32 = 3.3;
pub const THERMISTOR_ANALOG_PIN: u8 = 0;
pub const THERMISTOR_VOLTAGE_DIVIDER_RESISTANCE: f32 = 9_700.;

pub const LIGHT_RELAY_PIN: u8 = 0;
pub const FAN_RELAY_PIN: u8 = 1;
pub const WATER_PUMP_RELAY_PIN: u8 = 2;
pub const RELAY_GPIO_PINS: [Option<u8>; 4] = [Some(17), Some(27), Some(22), None];
