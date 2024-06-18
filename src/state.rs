use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{config::Configuration, error::GenericResult, history::History, io};

pub type ProgramStateShared = Arc<Mutex<ProgramState>>;
pub struct ProgramState {
    pub config: Configuration,
    pub relay: io::Relay,
    pub history: History,
}

fn lock_err<T: std::fmt::Display>(e: T) -> String {
    format!("{}", e)
}

pub fn init_state(config: Configuration) -> GenericResult<ProgramStateShared> {
    let relay = io::Relay::new(&config)?;
    let history = History::load().unwrap_or_default();
    Ok(Arc::new(Mutex::new(ProgramState {
        config,
        relay,
        history,
    })))
}
