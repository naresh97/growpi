use std::{path::Path, time::Duration};

use crate::{io, state::ProgramStateShared};

pub const IMAGE_PATH: &str = "./growpi.image.jpeg";

pub async fn save_latest_image(program_state: ProgramStateShared) -> anyhow::Result<()> {
    let resolution = program_state
        .lock()
        .await
        .config
        .data_logging_settings
        .imaging_resolution
        .clone();

    io::capture_image(&resolution, get_image_path()).await?;
    Ok(())
}

pub fn get_image_path() -> &'static Path {
    Path::new(IMAGE_PATH)
}

pub async fn imaging_loop(program_state: ProgramStateShared) {
    loop {
        let imaging_frequency = match program_state
            .lock()
            .await
            .config
            .data_logging_settings
            .imaging_frequency_minutes
        {
            0 => None,
            n => Some(n),
        };

        match imaging_frequency {
            Some(f) => {
                let _ = save_latest_image(program_state.clone()).await;
                tokio::time::sleep(Duration::from_mins(f)).await;
            }
            None => tokio::time::sleep(Duration::from_hours(24)).await,
        };
    }
}
