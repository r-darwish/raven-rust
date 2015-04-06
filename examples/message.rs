extern crate raven;

use std::env;

fn main() {
    let client = raven::Client::from_string(&*env::var("SENTRY_DSN").ok().expect("missing SENTRY_DSN environment variable")).unwrap();
    client.capture_message("Something aweful just happened").unwrap();
    println!("Success. Check Sentry for your error");
}
