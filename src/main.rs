#![feature(duration_constructors)]
#![feature(exit_status_error)]

use cli_mode::run_cli;
use config::Configuration;
use server::run_server;
use state::init_state;

mod actuators;
mod cli_mode;
mod config;
mod control;
mod history;
mod io;
mod sensors;
mod server;
mod state;

fn load_config() -> config::Configuration {
    let config = Configuration::from_file(std::path::Path::new("./growpi.toml"));
    match config {
        Ok(config) => config,
        Err(_) => {
            let config = Configuration::default();
            config
                .save_to_file(std::path::Path::new("./growpi.toml"))
                .expect("Could not create default config in ./growpi.toml");
            config
        }
    }
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
        Some("cli") => run_cli(program_state.clone()).await,
        _ => run_server(program_state.clone()).await,
    }

    let _ = control_thread_handle.await;
}
