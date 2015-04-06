extern crate regex;
extern crate time;
extern crate uuid;
extern crate rustc_serialize;

const CLIENT_STRING: &'static str = "raven-rust/0.1";

mod dsn;
mod client;
mod error;
mod protocol;

pub use client::Client;
pub use error::{RavenResult, RavenError};
