use std::result::Result;
use rustc_serialize::{Encodable, Encoder, json};
use std::convert::From;
use uuid::Uuid;
use time::{now_utc,Tm};
use super::dsn::DSN;
use super::CLIENT_STRING;

enum Level {
    Error
}

struct Event {
    event_id: String,
    message: String,
    timestamp: Tm,
    level: Level,
}

impl Encodable for Event {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        let time_str = format!("{}", self.timestamp.rfc3339());
        encoder.emit_struct("Event", 4, |e| {
            try!(e.emit_struct_field("event_id", 0, |e| self.event_id.encode(e)));
            try!(e.emit_struct_field("message", 1, |e| self.message.encode(e)));
            try!(e.emit_struct_field("timestamp", 2, |e| time_str.encode(e)));
            try!(e.emit_struct_field("level", 2, |e| match self.level {
                Level::Error => "error",
            }.encode(e)));
            Ok(())
        })
    }
}

impl Event {
    pub fn new(message: &str) -> Event {
        Event {
            event_id: Uuid::new_v4().to_simple_string(),
            message: From::from(message),
            timestamp: now_utc(),
            level: Level::Error }
    }
}

fn get_header(dsn: &DSN) -> String {
    format!(
        "Sentry sentry_version=5, sentry_timestamp={}, sentry_key={}, sentry_client={}, sentry_secret={}",
        now_utc().to_timespec().sec,
        dsn.public_key(),
        CLIENT_STRING,
        dsn.private_key())
}

#[test]
fn json_encode() {
    let event = Event::new("Testing one two three");
    assert!(json::encode(&event).is_ok());
}
