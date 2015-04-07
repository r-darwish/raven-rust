use libc::{c_int, size_t};

#[link(name = "c")]
extern {
    fn gethostname(name: *const u8, name_len: size_t) -> c_int;
}

fn strlen(s: &[u8]) -> usize {
    for (i, c) in s.iter().enumerate() {
        if *c == 0 {
            return i;
        }
    }

    s.len()
}

pub fn get_hostname() -> Option<String> {
    let buffer: [u8; 1024] = [0; 1024];
    unsafe {
        if gethostname(buffer.as_ptr(), buffer.len() as size_t) != 0 {
            return None;
        }
    };

    match String::from_utf8(buffer[..strlen(&buffer)].to_vec()) {
        Ok(s) => Some(s),
        _ => None
    }
}
