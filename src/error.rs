use std::fmt;
use std::error::Error;
use std::convert::From;
use std::result::Result;
use rustc_serialize::json::EncoderError;
use hyper::error::HttpError;
use hyper::status::StatusCode;

#[derive(Debug)]
pub enum RavenError {
    InvalidDSN,
    EncoderError(EncoderError),
    HttpError(HttpError),
    SentryError(StatusCode)
}

impl From<EncoderError> for RavenError {
    fn from(err: EncoderError) -> RavenError { RavenError::EncoderError(err) }
}

impl From<HttpError> for RavenError {
    fn from(err: HttpError) -> RavenError { RavenError::HttpError(err) }
}

impl Error for RavenError {
    fn description(&self) -> &str {
        match *self {
            RavenError::InvalidDSN => "Invalid DSN",
            RavenError::EncoderError(ref err) => err.description(),
            RavenError::HttpError(ref err) => err.description(),
            RavenError::SentryError(_) => "Sentry Error"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            RavenError::InvalidDSN => None,
            RavenError::EncoderError(ref err) => Some(err),
            RavenError::HttpError(ref err) => Some(err),
            RavenError::SentryError(_) => None
        }
    }
}

impl fmt::Display for RavenError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.description().fmt(fmt)
    }
}

pub type RavenResult<T> = Result<T, RavenError>;
