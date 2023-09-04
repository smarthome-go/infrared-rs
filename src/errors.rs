use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Rppal(rppal::gpio::Error),
    Cdev(gpio_cdev::Error),
}

impl From<rppal::gpio::Error> for Error {
    fn from(err: rppal::gpio::Error) -> Self {
        Self::Rppal(err)
    }
}

impl From<gpio_cdev::Error> for Error {
    fn from(value: gpio_cdev::Error) -> Self {
        Self::Cdev(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rppal(e) => write!(f, "Raspberry Pi GPIO error: {e}"),
            Self::Cdev(e) => write!(f, "CDEV GPIO error: {e}"),
        }
    }
}
