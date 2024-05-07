#![allow(dead_code)]

use cli_mode::run_cli;

mod actuators;
mod cli_mode;
mod config;
mod error;
mod io;
mod sensors;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if let Some(mode) = args.first().map(|x| x.as_str()) {
        match mode {
            "cli" => run_cli(),
            _ => run_cli(),
        }
    }
}
