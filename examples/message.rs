extern crate raven;

use std::env;

fn main() {
    let dsn = raven::DSN::from_string(&env::var("SENTRY_DSN").ok().expect("missing SENTRY_DSN environment variable")).unwrap();
    let hostname = raven::get_hostname().ok().expect("Unable to get the hostname");
    let client = raven::Client::new(dsn, Some(hostname));
    client.capture_message("Something aweful just happened", &[("version", "stable")]).unwrap();
    println!("Success. Check Sentry for your error");
}
