use std::fmt;
use std::error::Error;
use std::convert::From;
use std::result::Result;
use rustc_serialize::json::EncoderError;
use hyper::error::Error as HttpError;
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
            RavenError::EncoderError(_) => "Encoder Error",
            RavenError::HttpError(_) => "Http Error",
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
        match *self {
            RavenError::InvalidDSN => fmt.write_str("Invalid DSN"),
            RavenError::EncoderError(ref err) => write!(fmt, "Encoding Error ({})", err),
            RavenError::HttpError(ref err) => write!(fmt, "HTTP Error ({})", err),
            RavenError::SentryError(code) => write!(fmt, "Sentry returned {}", code)
        }
    }
}

pub type RavenResult<T> = Result<T, RavenError>;
