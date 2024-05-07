use std::{error::Error, thread, time::Duration};

use dht11::Dht11Sensor;

mod dht11;

fn main() -> Result<(), Box<dyn Error>> {
    unsafe {
        libc::setpriority(libc::PRIO_PROCESS, 0, -20);
    }
    let mut sensor = Dht11Sensor::new(2)?;
    loop {
        match sensor.read() {
            Ok(data) => println!(
                "Humidity: {}; Temperature: {}",
                data.humidity, data.temperature
            ),
            Err(msg) => println!("Error: {}", msg),
        }
        thread::sleep(Duration::from_millis(2000));
    }

    Ok(())
}
