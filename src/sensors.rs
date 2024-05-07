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
