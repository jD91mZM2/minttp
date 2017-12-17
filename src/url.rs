use std::fmt::Write;
use std::fmt;
use std::str::FromStr;

/// URL struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url {
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub query: Option<String>,
    pub fullpath: String
}

/// Encode a URL component into a new `String`.
pub fn encode(input: &str) -> String {
    let mut output = String::with_capacity(input.len());

    for code in input.bytes() {
        if (code >= 'a' as u8 && code <= 'z' as u8) || (code >= 'A' as u8 && code <= 'Z' as u8) {
            output.push(code as char);
        } else {
            write!(output, "%{:X}", code).unwrap();
        }
    }

    output
}
impl FromStr for Url {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts: Vec<_> = s.splitn(2, "://").collect();
        let protocol = if parts.len() == 2 { parts[0] } else { "http" };

        parts = parts[parts.len() - 1].splitn(2, ':').collect();
        let mut host = parts[0];

        let port = if let Some(port) = parts.get(1) {
            port.splitn(2, '/').next().unwrap().parse()?
        } else {
            host = host.splitn(2, '/').next().unwrap();

            if protocol == "https" { 443 } else { 80 }
        };

        parts = parts[parts.len() - 1].splitn(2, '/').collect();

        let mut fullpath = '/'.to_string();
        let mut path = '/'.to_string();
        let mut query = None;
        if let Some(part) = parts.get(1) {
            let parts: Vec<_> = part.splitn(2, '?').collect();
            fullpath.push_str(part);
            path.push_str(parts[0]);

            query = parts.get(1).map(|s| s.to_string())
        }

        Ok(Url {
            protocol: protocol.to_string(),
            host: host.to_string(),
            port: port,
            path: path,
            query: query,
            fullpath: fullpath
        })
    }
}
impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}://{}:{}{}",
            self.protocol,
            self.host,
            self.port,
            self.fullpath
        )
    }
}

#[cfg(test)]
mod test {
    use super::Url;

    #[test]
    fn test1() {
        let url = "protocol://example.com:123/path?key=val"
            .parse::<Url>()
            .unwrap();
        assert_eq!(
            url,
            Url {
                protocol: "protocol".to_string(),
                host: "example.com".to_string(),
                port: 123,
                path: "/path".to_string(),
                query: Some("key=val".to_string()),
                fullpath: "/path?key=val".to_string()
            }
        );
    }
    #[test]
    fn test2() {
        let url = "example.com/path".parse::<Url>().unwrap();
        assert_eq!(
            url,
            Url {
                protocol: "http".to_string(),
                host: "example.com".to_string(),
                port: 80,
                path: "/path".to_string(),
                query: None,
                fullpath: "/path".to_string()
            }
        );
    }
}
