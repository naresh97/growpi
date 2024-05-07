use tokio::join;

use crate::{
    control::{
        light::light_control_loop, soil::soil_moisture_control_loop,
        temperature::temperature_control_loop,
    },
    state::ProgramStateShared,
};

mod light;
mod soil;
mod temperature;

pub async fn control_thread(program_state: ProgramStateShared) {
    join!(
        light_control_loop(program_state.clone()),
        temperature_control_loop(program_state.clone()),
        soil_moisture_control_loop(program_state.clone())
    );
}
