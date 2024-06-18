use crate::{
    control::{
        data_logging::data_logging_loop, imaging::imaging_loop, light::light_control_loop,
        soil::soil_moisture_control_loop, temperature::temperature_control_loop,
        ventilation::ventilation_control_loop,
    },
    state::ProgramStateShared,
};
use tokio::join;

mod data_logging;
pub mod imaging;
mod light;
mod soil;
mod temperature;
mod ventilation;

pub async fn control_thread(program_state: ProgramStateShared) {
    join!(
        ventilation_control_loop(program_state.clone()),
        light_control_loop(program_state.clone()),
        temperature_control_loop(program_state.clone()),
        soil_moisture_control_loop(program_state.clone()),
        data_logging_loop(program_state.clone()),
        imaging_loop(program_state.clone())
    );
}
