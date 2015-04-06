use regex::Regex;
use std::convert::From;

#[derive(PartialEq, Debug)]
pub struct DSN {
    protocol: String,
    public_key: String,
    secret_key: String,
    host: String,
    path: String,
    project_id: String
}

impl DSN {
    pub fn from_string(url: &str) -> Option<DSN> {
        let regex = Regex::new(r"^(?P<protocol>.*?)://(?P<public_key>.*?):(?P<secret_key>.*?)@(?P<host>.*?)/(?P<path>.*/)(?P<project_id>.*)$").unwrap();
        let captures = match regex.captures(url) {
            None => return None,
            Some(cap) => cap
        };

        let dsn = DSN {
            protocol: From::from(captures.name("protocol").unwrap()),
            public_key: From::from(captures.name("public_key").unwrap()),
            secret_key: From::from(captures.name("secret_key").unwrap()),
            host: From::from(captures.name("host").unwrap()),
            path: From::from(captures.name("path").unwrap()),
            project_id: From::from(captures.name("project_id").unwrap()),
        };

        Some(dsn)
    }
}

impl ToString for DSN {
    fn to_string(&self) -> String {
        format!("{}://{}:{}@{}/{}{}", self.protocol, self.public_key, self.secret_key, self.host, self.path, self.project_id)
    }
}

#[test]
fn valid_dsn() {
    let s = "https://public:secret@example.com/sentry/long/path/project-id";
    assert_eq!(s, DSN::from_string(s).unwrap().to_string());
}

#[test]
fn invalid_dsn() {
    let s = "https://publicsecret@example.com/sentry/long/path/project-id";
    assert_eq!(None, DSN::from_string(s));
}
