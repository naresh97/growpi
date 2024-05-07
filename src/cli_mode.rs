use std::{thread, time::Duration};

use rustyline::{config::Configurer, error::ReadlineError, history::FileHistory};

use crate::{
    actuators,
    config::Configuration,
    error::GenericResult,
    io::{self, get_input_voltage},
    sensors,
};

struct LoopFlags {
    exit: bool,
}

fn process_input(
    input: String,
    program_state: &mut ProgramState,
    config: &Configuration,
) -> GenericResult<LoopFlags> {
    let args = input.split(' ').collect::<Vec<_>>();
    let main_command = *args.first().ok_or("No main command found.")?;
    match main_command {
        "ana" => command_ana(&args)?,
        "rel" => command_rel(&args, program_state, config)?,
        "soil" => command_soil(&args, config)?,
        "temp" => command_temp(&args, config)?,
        "pump" => command_pump(&args, program_state, config)?,
        "exit" => return Ok(LoopFlags { exit: true }),
        _ => return Err("Unknown main command".into()),
    };

    Ok(LoopFlags { exit: false })
}

fn command_pump(
    args: &[&str],
    program_state: &mut ProgramState,
    config: &Configuration,
) -> GenericResult<()> {
    let use_grams = args
        .get(2)
        .map(|arg| matches!(*arg, "grams"))
        .unwrap_or(false);

    if use_grams {
        let grams: u16 = args.get(1).ok_or("No mass specified.")?.parse()?;
        actuators::pump_water(grams, &mut program_state.relay, config)?;
        return Ok(());
    }

    let duration_ms: u64 = args.get(1).ok_or("No duration specified.")?.parse()?;
    let duration = Duration::from_millis(duration_ms);
    actuators::switch_water_pump(&mut program_state.relay, io::RelaySwitchState::On, config)?;
    thread::sleep(duration);
    actuators::switch_water_pump(&mut program_state.relay, io::RelaySwitchState::Off, config)?;

    Ok(())
}

fn command_temp(args: &[&str], config: &Configuration) -> GenericResult<()> {
    let show_loop = args
        .get(1)
        .map(|arg| matches!(*arg, "loop"))
        .unwrap_or(false);
    loop {
        let temperature = sensors::get_temperature(config)?;
        println!("Temperature: {}C", temperature);
        if !show_loop {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}

fn command_soil(args: &[&str], config: &Configuration) -> GenericResult<()> {
    let show_loop = args
        .get(1)
        .map(|arg| matches!(*arg, "loop"))
        .unwrap_or(false);

    loop {
        let humidity = sensors::get_soil_moisture(config)?;
        println!("Soil humidity: {}", humidity);
        if !show_loop {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn command_rel(
    args: &[&str],
    program_state: &mut ProgramState,
    config: &Configuration,
) -> GenericResult<()> {
    let pin = args
        .get(1)
        .ok_or("Must specify pin number.")?
        .parse::<u8>()
        .map_err(|_| "Not a valid pin number")?;

    let switch_state = args.get(2).map(|arg| match *arg {
        "1" => Ok(io::RelaySwitchState::On),
        "on" => Ok(io::RelaySwitchState::On),
        "true" => Ok(io::RelaySwitchState::On),
        "0" => Ok(io::RelaySwitchState::Off),
        "off" => Ok(io::RelaySwitchState::Off),
        "false" => Ok(io::RelaySwitchState::Off),
        _ => Err("Not a valid switch state"),
    });

    match switch_state {
        Some(state) => {
            println!("Switching relay");
            program_state.relay.switch(pin, state?, config)?
        }
        None => {
            println!("Toggling relay");
            program_state.relay.toggle(pin, config)?
        }
    };

    Ok(())
}

fn command_ana(args: &[&str]) -> GenericResult<()> {
    let pin = args
        .get(1)
        .ok_or("Must specify pin number.")?
        .parse::<u8>()
        .map_err(|_| "Not a valid pin number")?;

    let show_loop = args
        .get(2)
        .map(|arg| matches!(*arg, "loop"))
        .unwrap_or(false);

    loop {
        let voltage = get_input_voltage(pin)?;
        println!("Voltage read: {}", voltage);
        if !show_loop {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn cli_loop(
    rl: &mut CLIEditor,
    program_state: &mut ProgramState,
    config: &Configuration,
) -> GenericResult<LoopFlags> {
    let readline = rl.readline("growpi>> ");

    match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_str())?;
            process_input(line, program_state, config)
        }
        Err(ReadlineError::Eof) => Ok(LoopFlags { exit: true }),
        Err(_) => Err("No input".into()),
    }
}

type CLIEditor = rustyline::Editor<(), FileHistory>;
fn init_readline() -> GenericResult<CLIEditor> {
    let mut rl = rustyline::DefaultEditor::new()?;
    rl.set_max_history_size(10)?;
    Ok(rl)
}
struct ProgramState {
    relay: io::Relay,
}

fn init_state(config: &Configuration) -> GenericResult<ProgramState> {
    Ok(ProgramState {
        relay: io::Relay::new(config)?,
    })
}

pub fn run_cli() {
    let mut rl = init_readline().unwrap();

    let config = Configuration::from_file(std::path::Path::new("./growpi.toml"));
    if let Err(config) = &config {
        println!("Could not load config: {}", config);
    }
    let config = config.unwrap_or_default();

    let mut program_state = init_state(&config).unwrap();

    'cli_loop: loop {
        match cli_loop(&mut rl, &mut program_state, &config) {
            Ok(loop_flags) => {
                if loop_flags.exit {
                    println!("Leaving CLI");
                    break 'cli_loop;
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}
