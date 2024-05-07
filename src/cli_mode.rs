use rustyline::{config::Configurer, error::ReadlineError, history::FileHistory};

use crate::{
    error::GenericResult,
    io::{self, get_input_voltage},
};

struct LoopFlags {
    exit: bool,
}

fn process_input(input: String, program_state: &mut ProgramState) -> GenericResult<LoopFlags> {
    let args = input.split(' ').collect::<Vec<_>>();
    let main_command = *args.first().ok_or("No main command found.")?;
    match main_command {
        "ana" => command_ana(&args)?,
        "rel" => command_rel(&args, program_state)?,
        "exit" => return Ok(LoopFlags { exit: true }),
        _ => return Err("Unknown main command".into()),
    };

    Ok(LoopFlags { exit: false })
}

fn command_rel(args: &[&str], program_state: &mut ProgramState) -> GenericResult<()> {
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
            program_state.relay.switch(pin, state?)?
        }
        None => {
            println!("Toggling relay");
            program_state.relay.toggle(pin)?
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

    let voltage = get_input_voltage(pin)?;
    println!("Voltage read: {}", voltage);

    Ok(())
}

fn cli_loop(rl: &mut CLIEditor, program_state: &mut ProgramState) -> GenericResult<LoopFlags> {
    let readline = rl.readline("growpi>> ");

    match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_str())?;
            process_input(line, program_state)
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

fn init_state() -> GenericResult<ProgramState> {
    Ok(ProgramState {
        relay: io::Relay::new()?,
    })
}

pub fn run_cli() {
    let mut rl = init_readline().unwrap();
    let mut program_state = init_state().unwrap();

    'cli_loop: loop {
        match cli_loop(&mut rl, &mut program_state) {
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
