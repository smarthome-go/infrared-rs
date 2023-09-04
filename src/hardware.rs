use gpio_cdev::{Chip, LineHandle, LineRequestFlags};
use rppal::gpio::{InputPin, Level};

use crate::errors::Result;

pub enum HardwareInput {
    RaspberryPi { pin: InputPin },
    Cdev { handle: LineHandle },
}

impl HardwareInput {
    pub fn new_cdev(pin_number: u32, device_path: String) -> Result<Self> {
        let mut chip = Chip::new(device_path).unwrap();
        let handle = chip.get_line(pin_number)?.request(
            LineRequestFlags::INPUT,
            0,
            env!("CARGO_PKG_NAME"),
        )?;
        Ok(Self::Cdev { handle })
    }

    pub fn new_raspberry(pin_number: u8) -> Result<Self> {
        let pin = rppal::gpio::Gpio::new()?.get(pin_number)?.into_input();
        Ok(Self::RaspberryPi { pin })
    }

    pub fn read_value(&self) -> Result<Level> {
        match self {
            HardwareInput::RaspberryPi { pin } => Ok(pin.read()),
            HardwareInput::Cdev { handle } => {
                let raw_value = handle.get_value()?;
                Ok(if raw_value == 1 {
                    Level::High
                } else {
                    Level::Low
                })
            }
        }
    }
}
