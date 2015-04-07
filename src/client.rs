use std::error::Error;
use std::fmt::{Formatter, Result};
use super::dsn::DSN;
use super::error::{RavenResult, RavenError};
use super::protocol::{get_sentry_header, Event};
use super::hostname::get_hostname;
use rustc_serialize::json;
use hyper;
use hyper::status::StatusCode;

#[derive(Debug, Clone)]
pub struct Client {
    dsn: Option<DSN>,
    server_name: Option<String>
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
        Client {dsn: dsn, server_name: get_hostname()}
    }

    pub fn from_string(s: &str) -> RavenResult<Client> {
        let dsn = try!(DSN::from_string(s));
        Ok(Client::new(dsn))
    }

    pub fn capture_message(&self, message: &str, tags: &[(&str, &str)]) -> RavenResult<()> {
        let mut client = hyper::Client::new();
        let dsn = match self.dsn {
            None => return Ok(()),
            Some(ref dsn) => dsn
        };

        let server_name: Option<&str> = match self.server_name {
            None => None,
            Some(ref s) => Some(s)
        };
        let event = try!(json::encode(&Event::new(message, tags, server_name)));
        let response = try!(client.post(dsn.endpoint())
            .header(SentryHeader { content: get_sentry_header(dsn) })
            .body(&event as &str)
            .send());
        if response.status != StatusCode::Ok {
            return Err(RavenError::SentryError(response.status));
        }

        Ok(())
    }

    pub fn capture_error<F: Error>(&self, err: &F, tags: &[(&str, &str)]) -> RavenResult<()> {
        let message = format!("{:?}", err);
        self.capture_message(&message, tags)
    }
}
