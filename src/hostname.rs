use std::string::FromUtf8Error;
use libc::{c_int, size_t};

#[link(name = "c")]
extern {
    fn gethostname(name: *const u8, name_len: size_t) -> c_int;
}

pub enum HostnameError {
    SystemError,
    FromUtf8Error(FromUtf8Error),
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
