use std::sync::{Arc, Mutex};

use crate::{config::Configuration, error::GenericResult, io};

pub type ProgramStateShared = Arc<Mutex<ProgramState>>;
pub struct ProgramState {
    pub config: Configuration,
    pub relay: io::Relay,
}

pub fn init_state(config: Configuration) -> GenericResult<ProgramStateShared> {
    let relay = io::Relay::new(&config)?;
    Ok(Arc::new(Mutex::new(ProgramState { config, relay })))
}
