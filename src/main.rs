#![feature(duration_constructors)]
#![allow(dead_code)]

use cli_mode::run_cli;
use config::Configuration;
use server::run_server;
use state::init_state;

mod actuators;
mod cli_mode;
mod config;
mod control;
mod error;
mod io;
mod sensors;
mod server;
mod state;
mod history;

fn load_config() -> config::Configuration {
    let config = Configuration::from_file(std::path::Path::new("./growpi.toml"));
    if let Err(config) = &config {
        println!("Could not load config: {}", config);
    }

    config.unwrap_or_default()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let config = load_config();
    let program_state = init_state(config).unwrap();

    let program_state_clone = program_state.clone();
    let control_thread_handle =
        tokio::spawn(async move { control::control_thread(program_state_clone).await });

    let args = std::env::args().collect::<Vec<_>>();

    let mode = args.get(1).map(|x| x.as_str());

    match mode {
        Some("cli") => run_cli(program_state.clone()),
        _ => run_server(program_state.clone()).await,
    }

    let _ = control_thread_handle.await;
}
