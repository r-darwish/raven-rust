use std::error::Error;
use super::dsn::DSN;
use super::error::{RavenResult, RavenError};
use super::protocol::{get_sentry_header, Event};
use rustc_serialize::json;
use std::convert::Into;
use hyper;
use hyper::status::StatusCode;

pub struct Client {
    dsn: Option<DSN>
}

impl Client {
    pub fn new(dsn: Option<DSN>) -> Client {
        Client {dsn: dsn}
    }

    pub fn from_string(s: &str) -> RavenResult<Client> {
        let dsn = try!(DSN::from_string(s));
        Ok(Client::new(dsn))
    }

    pub fn report(&self, message: &str) -> RavenResult<()> {
        let mut client = hyper::Client::new();
        let dsn = match self.dsn {
            None => return Ok(()),
            Some(ref dsn) => dsn
        };

        let header_content = get_sentry_header(dsn);
        let event = try!(json::encode(&Event::new(message)));
        let event_slice: &str = &event;
        let mut headers = hyper::header::Headers::new();
        headers.set_raw("X-Sentry-Auth", vec![Into::into(header_content)]);

        let response = try!(client.post(dsn.endpoint()).headers(headers).body(event_slice).send());
        if response.status != StatusCode::Ok {
            return Err(RavenError::SentryError(response.status));
        }

        Ok(())
    }

    pub fn capture_error<F: Error>(&self, err: &F) -> RavenResult<()> {
        self.report(err.description())
    }
}
