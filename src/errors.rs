use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Gpio(rppal::gpio::Error),
}

impl From<rppal::gpio::Error> for Error {
    fn from(err: rppal::gpio::Error) -> Self {
        Self::Gpio(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gpio(e) => write!(f, "GPIO error: {e}"),
        }
    }
}
