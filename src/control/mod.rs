use crate::state::ProgramStateShared;

use data_logging::data_logging_loop;
use imaging::imaging_loop;
use light::light_control_loop;
use soil::soil_moisture_control_loop;
use temperature::temperature_control_loop;
use tokio::join;
use ventilation::ventilation_control_loop;

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
