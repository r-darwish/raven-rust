use std::fmt;
use std::error::Error;
use std::string::FromUtf8Error;
use libc::{c_int, size_t};

#[link(name = "c")]
extern {
    fn gethostname(name: *const u8, name_len: size_t) -> c_int;
}

#[derive(Debug)]
pub enum HostnameError {
    SystemError,
    FromUtf8Error(FromUtf8Error),
}

impl Error for HostnameError {
    fn description(&self) -> &str {
        match *self {
            HostnameError::SystemError => "gethostname error",
            HostnameError::FromUtf8Error(_) => "hostname cannot be decoded",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            HostnameError::SystemError => None,
            HostnameError::FromUtf8Error(ref e) => Some(e),
        }
    }
}

impl fmt::Display for HostnameError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            HostnameError::SystemError => fmt.write_str("gethostname error"),
            HostnameError::FromUtf8Error(ref e) => write!(fmt, "hostname cannot be encoded: {}", e),
        }
    }
}



impl From<FromUtf8Error> for HostnameError { fn from(e: FromUtf8Error) -> HostnameError { HostnameError::FromUtf8Error(e)} }

pub fn get_hostname() -> Result<String, HostnameError> {
    let buffer: [u8; 1024] = [0; 1024];
    unsafe {
        if gethostname(buffer.as_ptr(), buffer.len() as size_t) != 0 {
            return Err(HostnameError::SystemError);
        }
    };

    Ok(try!(String::from_utf8(buffer.to_vec())))
}
