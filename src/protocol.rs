use std::borrow::Borrow;
use std::result::Result;
use rustc_serialize::json;
use rustc_serialize::{Encodable, Encoder};
use uuid::Uuid;
use time::now_utc;
use super::dsn::DSN;
use super::CLIENT_STRING;

pub fn encode<M, StrPairs, S1, S2>(message: M, tags: StrPairs, server_name: Option<&str>) -> Result<String, json::EncoderError>
        where M: AsRef<str>,
              StrPairs: IntoIterator,
              StrPairs::Item: Borrow<(S1, S2)>,
              S1: AsRef<str>,
              S2: AsRef<str> {
    let mut json = String::new();
    {
        let mut encoder = json::Encoder::new(&mut json);
        let time_str = format!("{}", now_utc().rfc3339());
        try!(encoder.emit_struct("Event", 5, |e| {
            try!(e.emit_struct_field("event_id", 0, |e| Uuid::new_v4().to_simple_string().encode(e)));
            try!(e.emit_struct_field("message", 1, |e| message.as_ref().encode(e)));
            try!(e.emit_struct_field("timestamp", 2, |e| time_str.encode(e)));
            try!(e.emit_struct_field("level", 3, |e| "error".encode(e)));
            try!(e.emit_struct_field("tags", 4, |e| {
                try!(e.emit_struct("tags", 1, |e| {
                    for (index, pair) in tags.into_iter().enumerate() {
                        let &(ref key, ref value) = pair.borrow();
                        try!(e.emit_struct_field(key.as_ref(), index, |e| value.as_ref().encode(e)));
                    }
                    Ok(())
                }));
                Ok(())
            }));
            try!(e.emit_struct_field("server_name", 5, |e| match server_name {
                Some(name) => name,
                _ => ""
            }.encode(e)));
            Ok(())
        }));
    }

    Ok(json)
}

pub fn get_sentry_header(dsn: &DSN) -> String {
    format!(
        "Sentry sentry_version=5, sentry_timestamp={}, sentry_key={}, sentry_client={}, sentry_secret={}",
        now_utc().to_timespec().sec,
        dsn.public_key(),
        CLIENT_STRING,
        dsn.private_key())
}

#[test]
fn json_encode() {
    assert!(encode("Testing one two three", &[("version", "stable")], None).is_ok());
}
