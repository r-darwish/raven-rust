use std::borrow::Borrow;
use std::error::Error;
use std::fmt::{Formatter, Result};
use super::dsn::DSN;
use super::error::{RavenResult, RavenError};
use super::protocol::{get_sentry_header, encode};
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
    fn parse_header(_: &[Vec<u8>]) -> hyper::error::Result<Self> { Ok(SentryHeader {content: String::new()}) }
}

impl hyper::header::HeaderFormat for SentryHeader {
    fn fmt_header(&self, fmt: &mut Formatter) -> Result {
        fmt.write_str(&self.content)
    }
}

impl Client {
    pub fn new(dsn: Option<DSN>, server_name: Option<String>) -> Client {
        Client {dsn: dsn, server_name: server_name}
    }

    pub fn capture_message<M, StrPairs, S1, S2>(&self, message: M, tags: StrPairs) -> RavenResult<()>
            where M: AsRef<str>,
                  StrPairs: IntoIterator,
                  StrPairs::Item: Borrow<(S1, S2)>,
                  S1: AsRef<str>,
                  S2: AsRef<str> {

        let dsn = match self.dsn {
            None => return Ok(()),
            Some(ref dsn) => dsn
        };

        let client = hyper::Client::new();

        let event = try!(encode(
            message.as_ref(),
            tags.into_iter(),
            self.server_name.as_ref().map(|s| &s[..])
        ));

        let response = try!(client.post(dsn.endpoint())
            .header(SentryHeader { content: get_sentry_header(dsn) })
            .body(&event as &str)
            .send());
        if response.status != StatusCode::Ok {
            return Err(RavenError::SentryError(response.status));
        }

        Ok(())
    }

    pub fn capture_error<F, StrPairs, S1, S2>(&self, err: &F, tags: StrPairs) -> RavenResult<()>
            where F: Error,
                  StrPairs: IntoIterator,
                  StrPairs::Item: Borrow<(S1, S2)>,
                  S1: AsRef<str>,
                  S2: AsRef<str> {
        let message = format!("{}", err);
        self.capture_message(&message, tags)
    }
}
