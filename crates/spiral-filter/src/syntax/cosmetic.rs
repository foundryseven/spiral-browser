//! Cosmetic rule parser.
//!
//! Parses ABP/EasyList cosmetic rules of the form:
//! - `example.com##.selector` — hide elements matching `.selector` on `example.com`
//! - `##.selector` — hide elements matching `.selector` on all sites
//! - `example.com#@#.selector` — exception: do NOT hide `.selector` on `example.com`
//!
//! Returns a `CosmeticMatcher` if the input is a valid cosmetic rule,
//! or `None` if it is not.

use crate::error::FilterError;
use crate::rule::{CosmeticMatcher, HostnameScope};

/// Parse a cosmetic filter rule line.
///
/// Returns `None` if the line is not a cosmetic rule (i.e. it is
/// empty, a comment, a network rule, etc.).
pub fn parse_cosmetic(line: &str) -> Result<Option<CosmeticMatcher>, FilterError> {
    let line = line.trim();

    // Skip empty lines and comments.
    if line.is_empty() || line.starts_with('!') {
        return Ok(None);
    }

    // Find the `##` or `#@#` separator.
    let (hosts, selector, unhide) = if let Some(pos) = line.find("#@#") {
        (&line[..pos], &line[pos + 3..], true)
    } else if let Some(pos) = line.find("##") {
        (&line[..pos], &line[pos + 2..], false)
    } else {
        return Ok(None);
    };

    if selector.is_empty() {
        return Err(FilterError::InvalidSyntax(
            "empty cosmetic selector".to_string(),
        ));
    }

    let hostname_scope = if hosts.is_empty() {
        HostnameScope::Generic
    } else {
        let includes: Vec<String> = hosts
            .split(',')
            .filter(|h| !h.starts_with('~'))
            .map(|h| h.trim().to_ascii_lowercase())
            .filter(|h| !h.is_empty())
            .collect();

        let excludes: Vec<String> = hosts
            .split(',')
            .filter(|h| h.starts_with('~'))
            .map(|h| h.trim().trim_start_matches('~').to_ascii_lowercase())
            .filter(|h| !h.is_empty())
            .collect();

        if !includes.is_empty() {
            HostnameScope::Include(includes)
        } else if !excludes.is_empty() {
            HostnameScope::Exclude(excludes)
        } else {
            HostnameScope::Generic
        }
    };

    Ok(Some(CosmeticMatcher {
        selector: selector.to_string(),
        hostname_scope,
        unhide,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_generic_cosmetic() {
        let m = parse_cosmetic("##.ad-banner").unwrap().unwrap();
        assert_eq!(m.selector, ".ad-banner");
        assert!(matches!(m.hostname_scope, HostnameScope::Generic));
        assert!(!m.unhide);
    }

    #[test]
    fn parse_hosted_cosmetic() {
        let m = parse_cosmetic("example.com##.ad-banner").unwrap().unwrap();
        assert_eq!(m.selector, ".ad-banner");
        match &m.hostname_scope {
            HostnameScope::Include(hosts) => {
                assert_eq!(hosts, &["example.com"]);
            }
            _ => panic!("expected Include"),
        }
    }

    #[test]
    fn parse_exception_rule() {
        let m = parse_cosmetic("example.com#@#.ad-banner").unwrap().unwrap();
        assert!(m.unhide);
    }

    #[test]
    fn parse_empty_line_returns_none() {
        assert!(parse_cosmetic("").unwrap().is_none());
    }

    #[test]
    fn parse_comment_returns_none() {
        assert!(parse_cosmetic("! This is a comment").unwrap().is_none());
    }

    #[test]
    fn parse_non_cosmetic_returns_none() {
        assert!(parse_cosmetic("||example.com^").unwrap().is_none());
    }

    #[test]
    fn parse_empty_selector_errors() {
        assert!(parse_cosmetic("##").is_err());
    }

    #[test]
    fn parse_negated_hostname() {
        let m = parse_cosmetic("~example.com##.ad-banner").unwrap().unwrap();
        match &m.hostname_scope {
            HostnameScope::Exclude(hosts) => {
                assert_eq!(hosts, &["example.com"]);
            }
            _ => panic!("expected Exclude"),
        }
    }

    #[test]
    fn parse_multi_host_cosmetic() {
        let m = parse_cosmetic("example.com,test.org##.ad-banner")
            .unwrap()
            .unwrap();
        match &m.hostname_scope {
            HostnameScope::Include(hosts) => {
                assert_eq!(hosts.len(), 2);
                assert!(hosts.contains(&"example.com".to_string()));
                assert!(hosts.contains(&"test.org".to_string()));
            }
            _ => panic!("expected Include"),
        }
    }
}
