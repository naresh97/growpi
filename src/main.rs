use std::{error::Error, time::Duration};

use dht11::Dht11Sensor;
use linux_realtime::ThreadAttributes;

mod dht11;

fn main() -> Result<(), Box<dyn Error>> {
    let mut sensor = Dht11Sensor::new(2)?;
    let t = linux_realtime::JoinHandle::spawn(
        ThreadAttributes {
            stack_size: libc::PTHREAD_STACK_MIN,
            scheduler_policy: linux_realtime::SchedulerPolicy::Fifo,
            thread_priority: 99,
        },
        move || -> Result<(), Box<dyn Error>> {
            loop {
                let mut rtclock = linux_realtime::Clock::new()?;
                match sensor.read() {
                    Ok(data) => println!(
                        "Humidity: {}; Temperature: {}",
                        data.humidity, data.temperature
                    ),
                    Err(msg) => println!("Error: {}", msg),
                }
                for n in 1..10 {
                    rtclock.sleep(Duration::from_millis(500))?;
                }
            }
        },
    )?;
    _ = t.join();

    Ok(())
}
