use std::error::Error;
use std::fmt::{Formatter, Result};
use super::dsn::DSN;
use super::error::{RavenResult, RavenError};
use super::protocol::{get_sentry_header, Event};
use rustc_serialize::json;
use hyper;
use hyper::status::StatusCode;

pub struct Client {
    dsn: Option<DSN>
}

#[derive(Clone, Debug)]
struct SentryHeader {
    content: String,
}

impl hyper::header::Header for SentryHeader {
    fn header_name() -> &'static str { "X-Sentry-Auth" }
    fn parse_header(_: &[Vec<u8>]) -> Option<Self> { None }
}

impl hyper::header::HeaderFormat for SentryHeader {
    fn fmt_header(&self, fmt: &mut Formatter) -> Result {
        fmt.write_str(&self.content)
    }
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

        let event = try!(json::encode(&Event::new(message)));
        let response = try!(client.post(dsn.endpoint())
            .header(SentryHeader { content: get_sentry_header(dsn) })
            .body(&event as &str)
            .send());
        if response.status != StatusCode::Ok {
            return Err(RavenError::SentryError(response.status));
        }

        Ok(())
    }

    pub fn capture_error<F: Error>(&self, err: &F) -> RavenResult<()> {
        self.report(&chained_description(err))
    }
}

fn chained_description<F: Error>(err: &F) -> String {
    let mut current_error: &Error = err;
    let mut description = String::new();

    loop {
        description.push_str(current_error.description());
        match current_error.cause() {
            None => break,
            Some(cause) => {
                description.push_str(": ");
                current_error = cause;
            }
        };
    }

    return description;
}
