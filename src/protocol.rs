use std::result::Result;
use rustc_serialize::{Encodable, Encoder};
use uuid::Uuid;
use time::{now_utc,Tm};
use super::dsn::DSN;
use super::CLIENT_STRING;

enum Level {
    Error
}

pub struct Event<'a> {
    event_id: String,
    message: &'a str,
    timestamp: Tm,
    level: Level,
    tags: Vec<(&'a str, &'a str)>,
}

impl<'a> Encodable for Event<'a> {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        let time_str = format!("{}", self.timestamp.rfc3339());
        encoder.emit_struct("Event", 5, |e| {
            try!(e.emit_struct_field("event_id", 0, |e| self.event_id.encode(e)));
            try!(e.emit_struct_field("message", 1, |e| self.message.encode(e)));
            try!(e.emit_struct_field("timestamp", 2, |e| time_str.encode(e)));
            try!(e.emit_struct_field("level", 3, |e| match self.level {
                Level::Error => "error",
            }.encode(e)));
            try!(e.emit_struct_field("tags", 4, |e| {
                try!(e.emit_struct("tags", 1, |e| {
                    for tag in self.tags.iter().enumerate() {
                        let (index, &(key, value)) = tag;
                        try!(e.emit_struct_field(key, index, |e| value.encode(e)));
                    }
                    Ok(())
                }));
                Ok(())
            }));
            Ok(())
        })
    }
}

impl<'a> Event<'a> {
    pub fn new(message: &'a str, tags: &[(&'a str, &'a str)]) -> Event<'a> {
        Event {
            event_id: Uuid::new_v4().to_simple_string(),
            message: message,
            timestamp: now_utc(),
            level: Level::Error,
            tags: tags.iter().cloned().collect() }
    }
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
    use rustc_serialize::json;

    let event = Event::new("Testing one two three", &[("version", "stable")]);
    assert!(json::encode(&event).is_ok());
}
