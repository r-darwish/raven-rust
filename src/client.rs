use super::dsn::DSN;
use super::error::RavenResult;

pub struct Client {
    dsn: DSN
}

impl Client {
    pub fn new(dsn: DSN) -> Client {
        Client {dsn: dsn}
    }

    pub fn from_string(s: &str) -> RavenResult<Client> {
        let dsn = try!(DSN::from_string(s));
        Ok(Client::new(dsn))
    }
}
