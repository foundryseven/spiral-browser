//! URL host extraction — small, dependency-free, and only the
//! bits the filter needs.
//!
//! The trait [`crate::runtime::FilterHook::should_block`] takes
//! `&str` (not `&url::Url`) so that `spiral-filter` does not force
//! the `url` crate on its consumers. This module is the host
//! extractor: it returns the bare host string from a URL, or
//! `None` if the input is not a recognisable http/https URL.
//!
//! Supported forms (Phase 1):
//!
//! - `http://host[/path][?query][#fragment]`
//! - `https://host[:port][/path][?query][#fragment]`
//! - `https://user:pass@host[:port]/...` (userinfo stripped)
//!
//! IPv6 brackets and exotic schemes are deferred to M5+.

/// Extract the host from a URL string. Returns `None` for inputs
/// that don't have a recognisable http/https scheme or have an
/// empty host.
#[must_use]
pub fn extract_host(url: &str) -> Option<String> {
    let rest = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))?;

    // Userinfo: strip `user[:pass]@` if present.
    let after_userinfo = match rest.rfind('@') {
        Some(idx) => &rest[idx + 1..],
        None => rest,
    };

    // Host ends at the first '/', ':', '?', or '#'.
    let host_end = after_userinfo
        .find(|c: char| c == '/' || c == ':' || c == '?' || c == '#')
        .unwrap_or(after_userinfo.len());
    let host = &after_userinfo[..host_end];
    if host.is_empty() {
        None
    } else {
        Some(host.to_ascii_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_https() {
        assert_eq!(extract_host("https://example.com/"), Some("example.com".to_string()));
        assert_eq!(extract_host("https://example.com"), Some("example.com".to_string()));
    }

    #[test]
    fn basic_http() {
        assert_eq!(extract_host("http://example.com/foo"), Some("example.com".to_string()));
    }

    #[test]
    fn with_port() {
        assert_eq!(extract_host("https://example.com:8080/x"), Some("example.com".to_string()));
    }

    #[test]
    fn with_userinfo() {
        assert_eq!(extract_host("https://u:p@example.com/x"), Some("example.com".to_string()));
    }

    #[test]
    fn with_query_and_fragment() {
        assert_eq!(extract_host("https://example.com/x?y=1#z"), Some("example.com".to_string()));
    }

    #[test]
    fn host_is_lowercased() {
        assert_eq!(extract_host("https://EXAMPLE.COM/"), Some("example.com".to_string()));
    }

    #[test]
    fn rejects_non_http_scheme() {
        assert_eq!(extract_host("ftp://example.com"), None);
        assert_eq!(extract_host("file:///etc/passwd"), None);
        assert_eq!(extract_host("javascript:alert(1)"), None);
    }

    #[test]
    fn rejects_empty_host() {
        assert_eq!(extract_host("https:///foo"), None);
        assert_eq!(extract_host("not a url"), None);
    }
}
