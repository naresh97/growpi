use crate::{config::*, error::GenericResult, io::get_input_voltage};

pub fn get_temperature() -> GenericResult<f32> {
    const THERMISTOR_NOMINAL_RESISTANCE: f32 = 10_000.;
    const THERMISTOR_NOMINAL_TEMPERATURE: f32 = 298.15;
    const THERMISTOR_CONSTANT: f32 = 3950.;

    let voltage = get_input_voltage(THERMISTOR_ANALOG_PIN)?;
    let resistance = (LOGIC_LEVEL / voltage - 1.) * THERMISTOR_VOLTAGE_DIVIDER_RESISTANCE;
    let temperature = 1.
        / ((1. / THERMISTOR_NOMINAL_TEMPERATURE)
            + (1. / THERMISTOR_CONSTANT * f32::ln(resistance / THERMISTOR_NOMINAL_RESISTANCE)))
        - 273.15;
    Ok(temperature)
}

pub fn get_soil_moisture() -> GenericResult<f32> {
    let voltage = get_input_voltage(SOIL_MOISTURE_PIN)?;

    const VOLTAGE_ZERO_HUMIDITY: f32 = (SOIL_NOMINAL_MOISTURE_VOLTAGE
        - SOIL_MOISTURE_100_VOLTAGE * SOIL_NOMINAL_MOISTURE)
        / (1. - SOIL_NOMINAL_MOISTURE);

    let humidity =
        (voltage - VOLTAGE_ZERO_HUMIDITY) / (SOIL_MOISTURE_100_VOLTAGE - VOLTAGE_ZERO_HUMIDITY);
    Ok(humidity)
}
