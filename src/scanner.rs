use std::{thread, time::Duration};

use rppal::gpio::{InputPin, Level};

use crate::errors::Result;

pub struct Scanner {
    pin_number: u8,
    pin: InputPin,
}

struct Pulse {
    value: Level,
    length: u128,
}

impl Scanner {
    pub fn try_new(pin_number: u8) -> Result<Self> {
        let pin = rppal::gpio::Gpio::new()?.get(pin_number)?.into_input();
        Ok(Self { pin_number, pin })
    }

    pub fn pin(&self) -> u8 {
        self.pin_number
    }

    pub fn scan_blocking(&mut self) -> Result<u64> {
        let mut command: Vec<Pulse> = Vec::with_capacity(80);
        let mut count1 = 0u32;
        let mut previous = Level::Low;
        let mut value = self.pin.read();

        while self.pin.read() == Level::High {
            thread::sleep(Duration::from_micros(100));
        }

        let mut start_time = std::time::Instant::now();

        loop {
            thread::sleep(Duration::from_nanos(50));

            if value != previous {
                let pulse_length = start_time.elapsed();
                start_time = std::time::Instant::now();

                command.push(Pulse {
                    value: previous,
                    length: pulse_length.as_micros(),
                });
            }

            if value == Level::High {
                count1 += 1;
            }

            previous = value;
            value = self.pin.read();

            if count1 > 10000 {
                break;
            }
        }

        let mut binary = 0u64;
        let mut bin_length = 0u8;

        for item in command {
            if item.value == Level::Low {
                continue;
            }
            if item.length > 1000 {
                binary = binary << 2 | 1;
                bin_length += 1;
            } else {
                binary <<= 1;
                bin_length += 1;
            }

            if bin_length > 34 {
                break;
            }
        }

        Ok(binary)
    }
}
