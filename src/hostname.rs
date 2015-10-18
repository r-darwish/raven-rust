use libc::{c_int, size_t};

#[link(name = "c")]
extern {
    fn gethostname(name: *const u8, name_len: size_t) -> c_int;
}

pub fn get_hostname() -> Option<String> {
    let buffer: [u8; 1024] = [0; 1024];
    unsafe {
        if gethostname(buffer.as_ptr(), buffer.len() as size_t) != 0 {
            return None;
        }
    };

    match String::from_utf8(buffer.to_vec()) {
        Ok(s) => Some(s),
        _ => None
    }
}
