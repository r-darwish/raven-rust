extern crate libc;
extern crate regex;
extern crate time;
extern crate uuid;
extern crate rustc_serialize;
extern crate hyper;

const CLIENT_STRING: &'static str = "raven-rust/0.1";

mod dsn;
mod client;
mod error;
mod protocol;
mod hostname;

pub use client::Client;
pub use error::{RavenResult, RavenError};
