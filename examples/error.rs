extern crate raven;

use std::env;
use std::fs::File;

fn main() {
    let client = raven::Client::from_string(&*env::var("SENTRY_DSN").ok().expect("missing SENTRY_DSN environment variable")).unwrap();
    let err = match File::open("/path/to/nowhere!") {
        Ok(_) => panic!("This should not have happened"),
        Err(err) => err
    };
    client.capture_error(&err, &[("version", "develop"), ("os", "Linux")]).unwrap();
    println!("Success. Check Sentry for your error");
}
