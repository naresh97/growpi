use std::{thread, time::Duration};

use anyhow::{anyhow, bail, Context};
use rustyline::{config::Configurer, error::ReadlineError, history::FileHistory};

use crate::{
    actuators,
    io::{self, get_input_voltage},
    sensors,
    state::ProgramStateShared,
};

struct LoopFlags {
    exit: bool,
}

async fn process_input(
    input: String,
    program_state: ProgramStateShared,
) -> anyhow::Result<LoopFlags> {
    let args = input.split(' ').collect::<Vec<_>>();
    let main_command = *args.first().context("No main command found.")?;
    match main_command {
        "ana" => command_ana(&args)?,
        "rel" => command_rel(&args, program_state).await?,
        "soil" => command_soil(&args, program_state).await?,
        "temp" => command_temp(&args, program_state).await?,
        "pump" => command_pump(&args, program_state).await?,
        "exit" => return Ok(LoopFlags { exit: true }),
        _ => bail!("Unknown main command"),
    };

    Ok(LoopFlags { exit: false })
}

async fn command_pump(args: &[&str], program_state: ProgramStateShared) -> anyhow::Result<()> {
    let mut program_state = program_state.lock().await;

    let use_grams = args
        .get(2)
        .map(|arg| matches!(*arg, "grams"))
        .unwrap_or(false);

    if use_grams {
        let grams: u16 = args.get(1).context("No mass specified.")?.parse()?;
        actuators::pump_water(grams, &mut program_state)?;
        return Ok(());
    }

    let duration_ms: u64 = args.get(1).context("No duration specified.")?.parse()?;
    let duration = Duration::from_millis(duration_ms);
    actuators::switch_water_pump(io::RelaySwitchState::On, &mut program_state)?;
    thread::sleep(duration);
    actuators::switch_water_pump(io::RelaySwitchState::Off, &mut program_state)?;

    Ok(())
}

async fn command_temp(args: &[&str], program_state: ProgramStateShared) -> anyhow::Result<()> {
    let show_loop = args
        .get(1)
        .map(|arg| matches!(*arg, "loop"))
        .unwrap_or(false);
    loop {
        let program_state = program_state.lock().await;
        let temperature = sensors::get_temperature(&program_state.config)?;
        println!("Temperature: {}C", temperature);
        if !show_loop {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}

async fn command_soil(args: &[&str], program_state: ProgramStateShared) -> anyhow::Result<()> {
    let show_loop = args
        .get(1)
        .map(|arg| matches!(*arg, "loop"))
        .unwrap_or(false);

    loop {
        let program_state = program_state.lock().await;
        let humidity = sensors::get_soil_moisture(&program_state.config)?;
        println!("Soil humidity: {}", humidity);
        if !show_loop {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

async fn command_rel(args: &[&str], program_state: ProgramStateShared) -> anyhow::Result<()> {
    let mut program_state = program_state.lock().await;

    let pin = args
        .get(1)
        .context("Must specify pin number.")?
        .parse::<u8>()
        .context("Not a valid pin number")?;

    let switch_state = args.get(2).map(|arg| match *arg {
        "1" => Ok(io::RelaySwitchState::On),
        "on" => Ok(io::RelaySwitchState::On),
        "true" => Ok(io::RelaySwitchState::On),
        "0" => Ok(io::RelaySwitchState::Off),
        "off" => Ok(io::RelaySwitchState::Off),
        "false" => Ok(io::RelaySwitchState::Off),
        _ => Err(anyhow!("Not a valid switch state")),
    });

    match switch_state {
        Some(state) => {
            println!("Switching relay");
            program_state.relay.switch(pin, state?)?
        }
        None => {
            println!("Toggling relay");
            program_state.relay.toggle(pin)?
        }
    };

    Ok(())
}

fn command_ana(args: &[&str]) -> anyhow::Result<()> {
    let pin = args
        .get(1)
        .context("Must specify pin number.")?
        .parse::<u8>()
        .context("Not a valid pin number")?;

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

async fn cli_loop(
    rl: &mut CLIEditor,
    program_state: ProgramStateShared,
) -> anyhow::Result<LoopFlags> {
    let readline = rl.readline("growpi>> ");

    match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_str())?;
            process_input(line, program_state).await
        }
        Err(ReadlineError::Eof) => Ok(LoopFlags { exit: true }),
        Err(_) => Err(anyhow!("No input")),
    }
}

type CLIEditor = rustyline::Editor<(), FileHistory>;
fn init_readline() -> anyhow::Result<CLIEditor> {
    let mut rl = rustyline::DefaultEditor::new()?;
    rl.set_max_history_size(10)?;
    Ok(rl)
}

pub async fn run_cli(program_state: ProgramStateShared) {
    let mut rl = init_readline().unwrap();

    'cli_loop: loop {
        match cli_loop(&mut rl, program_state.clone()).await {
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
