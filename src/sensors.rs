use crate::{config::*, io::get_input_voltage};

pub fn get_temperature(config: &Configuration) -> anyhow::Result<f32> {
    let voltage = get_input_voltage(config.thermistor_settings.pin)?;

    let k = config.board_settings.logic_level / voltage - 1.;
    let k = match config.thermistor_settings.resistor {
        VoltageDividerResistor::R1 => k,
        VoltageDividerResistor::R2 => 1. / k,
    };
    let resistance = k * config.thermistor_settings.voltage_divider_resistance;

    let temperature = 1.
        / ((1. / config.thermistor_settings.nominal_temperature)
            + (1. / config.thermistor_settings.thermal_constant
                * f32::ln(resistance / config.thermistor_settings.nominal_resistance)))
        - 273.15;
    Ok(temperature)
}

pub fn get_soil_moisture(config: &Configuration) -> anyhow::Result<f32> {
    let voltage = get_input_voltage(config.soil_moisture_settings.pin)?;

    let voltage_zero_humidity: f32 = (config.soil_moisture_settings.voltage_nominal
        - config.soil_moisture_settings.voltage_100
            * config.soil_moisture_settings.moisture_nominal)
        / (1. - config.soil_moisture_settings.moisture_nominal);

    let humidity = (voltage - voltage_zero_humidity)
        / (config.soil_moisture_settings.voltage_100 - voltage_zero_humidity);
    Ok(humidity)
}
