use crate::error::GenericResult;

struct LoopFlags {
    exit: bool,
}

fn process_input(input: String) -> GenericResult<LoopFlags> {
    todo!()
}

fn cli_loop() -> GenericResult<LoopFlags> {
    print!("Enter command: ");
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string)?;
    process_input(input_string)
}

pub fn run_cli() {
    'cli_loop: loop {
        match cli_loop() {
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

// fn process_command(command: String, program_state: &mut ProgramState) -> GenericResult<bool> {
//     let command = command
//         .strip_suffix('\n')
//         .ok_or("Couldn't strip whitespace")?;
//     let args = command.split(' ').collect::<Vec<_>>();

//     let main_command = *args.first().ok_or("No command found")?;
//     match main_command {
//         "water" => toggle_water(&args, program_state)?,
//         "fan" => toggle_fan(program_state)?,
//         "light" => toggle_light(program_state)?,
//         "ana" => analogue_command(&args, program_state)?,
//         "exit" => return Ok(true),
//         _ => return Err(format!("Main command '{}' invalid", main_command).into()),
//     }

//     Ok(false)
// }

// fn analogue_command(args: &[&str], program_state: &mut ProgramState) -> GenericResult<()> {
//     let pin = args.get(1).ok_or("No pin given")?;
//     let pin: u8 = pin.parse()?;

//     let adc_target = rppal::i2c::I2c::new()?;
//     let mut adc_target =
//         Ads1x1x::new_ads1115(adc_target, ads1x1x::SlaveAddr::Alternative(false, false));
//     let channel: ChannelSelection = match pin {
//         0 => ChannelSelection::SingleA0,
//         1 => ChannelSelection::SingleA1,
//         2 => ChannelSelection::SingleA2,
//         3 => ChannelSelection::SingleA3,
//         _ => ChannelSelection::SingleA0,
//     };
//     loop {
//         let value = block!(adc_target.read(channel)).map_err(|e| format!("{:?}", e))?;
//         let value = value as f32;
//         let voltage = value / i16::MAX as f32 * 2.048;
//         let resistance = (3.3 / voltage - 1.) * 9_700.;
//         println!("Value: {}", value);
//         println!("resistance: {}", resistance);
//         let temp = 1. / ((1. / 298.15) + (1. / 3950. * f32::ln(resistance / 10_000.))) - 273.15;
//         println!("Temp: {}", temp);
//         //println!("Value of pin {} is: {}", pin, value);
//         thread::sleep(Duration::from_millis(2000));
//     }
//     Ok(())
// }

// fn toggle_water(args: &[&str], program_state: &mut ProgramState) -> GenericResult<()> {
//     if let Some(duration) = args.get(1) {
//         let duration: u64 = duration.parse()?;
//         println!("Turning on water for {} milliseconds", duration);
//         program_state.water_pin.set_low();
//         thread::sleep(Duration::from_millis(duration));
//         program_state.water_pin.set_high();
//     } else {
//         program_state.water_pin.toggle();
//         println!("Toggling water output");
//     }
//     Ok(())
// }

// fn toggle_light(program_state: &mut ProgramState) -> GenericResult<()> {
//     program_state.light_pin.toggle();
//     println!("Toggling light output");
//     Ok(())
// }

// fn toggle_fan(program_state: &mut ProgramState) -> GenericResult<()> {
//     program_state.fan_pin.toggle();
//     println!("Toggling fan output");
//     Ok(())
// }
