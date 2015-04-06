use std::fmt;
use std::error::Error;
use std::result::Result;

#[derive(Debug, PartialEq)]
pub enum RavenError {
    InvalidDSN,
}

impl Error for RavenError {
    fn description(&self) -> &str {
        match *self {
            RavenError::InvalidDSN => "Invalid DSN"
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for RavenError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.description().fmt(fmt)
    }
}

pub type RavenResult<T> = Result<T, RavenError>;
