use regex::Regex;
use std::convert::From;
use super::error::{RavenError, RavenResult};

#[derive(PartialEq, Debug, Clone)]
pub struct DSN {
    protocol: String,
    public_key: String,
    secret_key: String,
    host: String,
    path: String,
    project_id: String,
    endpoint: String
}

impl DSN {
    pub fn from_string(url: &str) -> RavenResult<Option<DSN>> {
        if url.is_empty() {
            return Ok(None);
        }

        let regex = Regex::new(r"^(?P<protocol>.*?)://(?P<public_key>.*?):(?P<secret_key>.*?)@(?P<host>.*?)/(?P<path>.*/)?(?P<project_id>.*)$").unwrap();
        let captures = match regex.captures(url) {
            None => return Err(RavenError::InvalidDSN),
            Some(cap) => cap
        };

        let protocol = From::from(captures.name("protocol").unwrap());
        let host = From::from(captures.name("host").unwrap());
        let path = From::from(captures.name("path").unwrap_or(""));
        let project_id = From::from(captures.name("project_id").unwrap());
        let endpoint = format!("{}://{}/{}api/{}/store/", protocol, host, path, project_id);
        let dsn = DSN {
            protocol: protocol,
            public_key: From::from(captures.name("public_key").unwrap()),
            secret_key: From::from(captures.name("secret_key").unwrap()),
            host: host,
            path: path,
            project_id: project_id,
            endpoint: endpoint,
        };

        Ok(Some(dsn))
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn public_key(&self) -> &str {
        &self.public_key
    }

    pub fn private_key(&self) -> &str {
        &self.secret_key
    }
}

impl ToString for DSN {
    fn to_string(&self) -> String {
        format!("{}://{}:{}@{}/{}{}", self.protocol, self.public_key, self.secret_key, self.host, self.path, self.project_id)
    }
}

#[test]
fn empty_dsn() {
    assert_eq!(None, DSN::from_string("").unwrap());
}

#[test]
fn valid_dsn() {
    let s = "https://public:secret@example.com/sentry/long/path/project-id";
    let dsn = DSN::from_string(s).unwrap().unwrap();
    assert_eq!(s, dsn.to_string());
    assert_eq!("https://example.com/sentry/long/path/api/project-id/store/", dsn.endpoint())
}

#[test]
fn empty_path() {
    let s = "https://b70a31b3510c4cf793964a185cfe1fd0:b7d80b520139450f903720eb7991bf3d@example.com/project-id";
    let dsn = DSN::from_string(s).unwrap().unwrap();
    assert_eq!(s, dsn.to_string());
    assert_eq!("https://example.com/api/project-id/store/", dsn.endpoint())
}


#[test]
fn invalid_dsn() {
    let s = "https://publicsecret@example.com/sentry/long/path/project-id";
    let err = DSN::from_string(s).unwrap_err();
    match err {
        RavenError::InvalidDSN => (),
        _ => panic!(format!("Unexpected error: {}", err))
    }
}
