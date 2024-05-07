use crate::{config::*, error::GenericResult, io::get_input_voltage};

pub fn get_temperature(config: &Configuration) -> GenericResult<f32> {
    let voltage = get_input_voltage(config.thermistor_settings.pin)?;
    let resistance = (config.board_settings.logic_level / voltage - 1.)
        * config.thermistor_settings.voltage_divider_resistance;
    let temperature = 1.
        / ((1. / config.thermistor_settings.nominal_temperature)
            + (1. / config.thermistor_settings.thermal_constant
                * f32::ln(resistance / config.thermistor_settings.nominal_resistance)))
        - 273.15;
    Ok(temperature)
}

pub fn get_soil_moisture(config: &Configuration) -> GenericResult<f32> {
    let voltage = get_input_voltage(config.soil_moisture_settings.pin)?;

    let voltage_zero_humidity: f32 = (config.soil_moisture_settings.voltage_nominal
        - config.soil_moisture_settings.voltage_100
            * config.soil_moisture_settings.moisture_nominal)
        / (1. - config.soil_moisture_settings.moisture_nominal);

    let humidity = (voltage - voltage_zero_humidity)
        / (config.soil_moisture_settings.voltage_100 - voltage_zero_humidity);
    Ok(humidity)
}
