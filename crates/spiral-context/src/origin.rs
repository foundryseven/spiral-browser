//! Origin — identifies a web origin (scheme + host + port).
//!
//! Origins are the isolation boundary for Spiral's shared-everything
//! renderer. Each origin gets its own context, its own capability
//! set, and its own DOM.

use std::fmt;

/// A web origin: scheme + host + port.
///
/// Two origins are equal iff all three components match.
/// This is the standard web-origin model (RFC 6454).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Origin {
    scheme: String,
    host: String,
    port: Option<u16>,
}

impl Origin {
    /// Parse an origin from a URL string.
    ///
    /// Accepts `scheme://host[:port]`. Returns `None` on malformed input.
    #[must_use]
    pub fn parse(url: &str) -> Option<Self> {
        let scheme_end = url.find("://")?;
        if scheme_end == 0 {
            return None;
        }
        let scheme = url[..scheme_end].to_ascii_lowercase();
        let rest = &url[scheme_end + 3..];

        if rest.is_empty() {
            return None;
        }

        let host_end = rest.find('/').unwrap_or(rest.len());
        let host_port = &rest[..host_end];

        let (host, port) = if let Some(colon) = host_port.rfind(':') {
            let port = host_port[colon + 1..].parse::<u16>().ok()?;
            (host_port[..colon].to_ascii_lowercase(), Some(port))
        } else {
            (host_port.to_ascii_lowercase(), None)
        };

        if host.is_empty() {
            return None;
        }

        Some(Self { scheme, host, port })
    }

    /// The scheme (e.g. `"https"`).
    #[must_use]
    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    /// The host (e.g. `"example.com"`).
    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }

    /// The port, if explicitly specified.
    #[must_use]
    pub fn port(&self) -> Option<u16> {
        self.port
    }

    /// The serialised origin (e.g. `"https://example.com:443"`).
    #[must_use]
    pub fn serialise(&self) -> String {
        match self.port {
            Some(port) => format!("{}://{}:{}", self.scheme, self.host, port),
            None => format!("{}://{}", self.scheme, self.host),
        }
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.serialise())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_https() {
        let origin = Origin::parse("https://example.com/path").unwrap();
        assert_eq!(origin.scheme(), "https");
        assert_eq!(origin.host(), "example.com");
        assert_eq!(origin.port(), None);
    }

    #[test]
    fn parse_with_port() {
        let origin = Origin::parse("http://localhost:8080").unwrap();
        assert_eq!(origin.scheme(), "http");
        assert_eq!(origin.host(), "localhost");
        assert_eq!(origin.port(), Some(8080));
    }

    #[test]
    fn parse_malformed_returns_none() {
        assert!(Origin::parse("not a url").is_none());
        assert!(Origin::parse("https://").is_none());
        assert!(Origin::parse("://host").is_none());
    }

    #[test]
    fn equality_by_components() {
        let a = Origin::parse("https://example.com").unwrap();
        let b = Origin::parse("https://example.com/page").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn inequality_by_scheme() {
        let a = Origin::parse("http://example.com").unwrap();
        let b = Origin::parse("https://example.com").unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn inequality_by_port() {
        let a = Origin::parse("http://example.com:80").unwrap();
        let b = Origin::parse("http://example.com:8080").unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn serialise_round_trip() {
        let origin = Origin::parse("https://example.com:443/path?q=1").unwrap();
        let serialised = origin.serialise();
        let reparsed = Origin::parse(&serialised).unwrap();
        assert_eq!(origin, reparsed);
    }

    #[test]
    fn display_format() {
        let origin = Origin::parse("https://example.com").unwrap();
        assert_eq!(format!("{origin}"), "https://example.com");
    }
}
