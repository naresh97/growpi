use std::{error::Error, time::Duration};

use rppal::gpio::Gpio;

pub struct Dht11Data {
    pub temperature: f32,
    pub humidity: f32,
}
pub struct Dht11Sensor {
    pin: rppal::gpio::IoPin,
}
impl Dht11Sensor {
    pub fn new(pin: u8) -> Result<Dht11Sensor, Box<dyn Error>> {
        let pin = Gpio::new()?.get(pin)?.into_io(rppal::gpio::Mode::Input);
        Ok(Dht11Sensor { pin })
    }

    fn expect_pulse(&mut self, expected_level: bool) -> Result<u32, Box<dyn Error>> {
        let _guard = interrupts::disable();
        let mut count: u32 = 0;
        let mut rtclock = linux_realtime::Clock::new()?;
        loop {
            if self.pin.is_high() != expected_level {
                break;
            }
            if count >= 1000 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Timeout while reading pulse",
                )
                .into());
            }
            count += 1;
            rtclock.sleep(Duration::from_micros(1))?;
        }
        Ok(count)
    }

    pub fn read(&mut self) -> Result<Dht11Data, Box<dyn Error>> {
        use rppal::gpio::{Bias, Mode};

        let mut data = [0; 5];
        let mut cycles: [u32; 80] = [0; 80];

        let mut rtclock = linux_realtime::Clock::new()?;

        self.pin.set_mode(Mode::Input);
        self.pin.set_bias(Bias::PullUp);
        rtclock.sleep(Duration::from_millis(1))?;

        self.pin.set_mode(Mode::Output);
        self.pin.set_low();
        rtclock.sleep(Duration::from_millis(20))?;

        // Timing Critical Code
        self.pin.set_mode(Mode::Input);
        self.pin.set_bias(Bias::PullUp);
        rtclock.sleep(Duration::from_micros(55))?;

        self.expect_pulse(false)?;
        self.expect_pulse(true)?;
        for i in (0..80).step_by(2) {
            cycles[i] = self.expect_pulse(false)?;
            cycles[i + 1] = self.expect_pulse(true)?;
        }

        for i in 0..40 {
            let low_cycles = cycles[2 * i];
            let high_cycles = cycles[2 * i + 1];
            data[i / 8] <<= 1;
            if high_cycles > low_cycles {
                data[i / 8] |= 1;
            }
        }

        if data[4] != ((data[0] + data[1] + data[2] + data[3]) & 0xFF) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "DHT Checksum Failure",
            )
            .into());
        }

        let mut temperature = data[2] as f32;
        if data[3] & 0x80 != 0 {
            temperature = -1. - temperature;
        }
        temperature += ((data[3] & 0x0F) as f32) * 0.1;

        let mut humidity = data[0] as f32;
        humidity += (data[1] as f32) * 0.1;

        Ok(Dht11Data {
            temperature,
            humidity,
        })
    }
}
