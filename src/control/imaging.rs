use std::{path::Path, time::Duration};

use crate::{
    actuators,
    error::GenericResult,
    io::{self, RelaySwitchState},
    state::{lock_state, ProgramStateShared},
};

pub const IMAGE_PATH: &str = "./growpi.image.jpeg";

pub async fn save_latest_image(program_state: ProgramStateShared) -> GenericResult<()> {
    let resolution = lock_state(&program_state)
        .map(|state| {
            state
                .config
                .data_logging_settings
                .imaging_resolution
                .clone()
        })
        .unwrap_or(io::ImageResolution::R360p);

    let light_state: RelaySwitchState;
    {
        let mut program_state = lock_state(&program_state)?;
        light_state = actuators::get_light_state(&mut program_state)?;
        actuators::switch_lights(RelaySwitchState::On, &mut program_state)?;
    }
    io::capture_image(&resolution, get_image_path()).await?;
    {
        let mut program_state = lock_state(&program_state)?;
        actuators::switch_lights(light_state, &mut program_state)?;
    }
    Ok(())
}

pub fn get_image_path() -> &'static Path {
    Path::new(IMAGE_PATH)
}

pub async fn imaging_loop(program_state: ProgramStateShared) {
    loop {
        let imaging_frequency = lock_state(&program_state)
            .map(|state| state.config.data_logging_settings.imaging_frequency_minutes)
            .map(|f| match f {
                0 => None,
                n => Some(n),
            })
            .unwrap_or(None);
        match imaging_frequency {
            Some(f) => {
                let _ = save_latest_image(program_state.clone()).await;
                tokio::time::sleep(Duration::from_mins(f)).await;
            }
            None => tokio::time::sleep(Duration::from_hours(24)).await,
        };
    }
}
