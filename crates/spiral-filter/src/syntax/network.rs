//! Network rule parser.
//!
//! Parses ABP/EasyList network rules of the form:
//! - `||example.com^` — block requests to `example.com`
//! - `||example.com/path^` — block requests to `example.com/path`
//! - `@@||example.com^` — exception: allow requests to `example.com`
//! - `||example.com^$third-party` — block third-party requests to `example.com`
//!
//! Returns a `NetworkMatcher` if the input is a valid network rule,
//! or `None` if it is not.

use crate::error::FilterError;
use crate::rule::{DomainConstraint, NetPattern, NetworkMatcher, Party};

/// Parse a network filter rule line.
///
/// Returns `None` if the line is not a network rule (i.e. it is
/// empty, a comment, a cosmetic rule, etc.).
pub fn parse_network(line: &str) -> Result<Option<ParsedNetworkRule>, FilterError> {
    let line = line.trim();

    // Skip empty lines and comments.
    if line.is_empty() || line.starts_with('!') {
        return Ok(None);
    }

    // Skip cosmetic rules.
    if line.contains("##") || line.contains("#@#") {
        return Ok(None);
    }

    // Skip HTML filter rules.
    if line.contains("#?#") || line.contains("#$#") {
        return Ok(None);
    }

    let is_exception = line.starts_with("@@");
    let pattern = if is_exception { &line[2..] } else { line };

    // Split on `$` to get options.
    let (pattern_part, options_part) = if let Some(dollar) = pattern.find('$') {
        (&pattern[..dollar], Some(&pattern[dollar + 1..]))
    } else {
        (pattern, None)
    };

    // Parse the pattern.
    let net_pattern = parse_pattern(pattern_part)?;

    // Parse options.
    let mut party = Party::Any;
    let mut domains = DomainConstraint::Any;

    if let Some(opts) = options_part {
        for opt in opts.split(',') {
            let opt = opt.trim();
            match opt {
                "third-party" => party = Party::Third,
                "first-party" | "1p" => party = Party::First,
                _ if opt.starts_with("domain=") => {
                    let domain_spec = &opt[7..];
                    domains = parse_domain_constraint(domain_spec)?;
                }
                _ => {} // unknown options are silently ignored
            }
        }
    }

    Ok(Some(ParsedNetworkRule {
        matcher: NetworkMatcher {
            pattern: net_pattern,
            request_kinds: 0, // all types
            party,
            domains,
        },
        is_exception,
    }))
}

/// A parsed network rule with exception flag.
#[derive(Debug)]
pub struct ParsedNetworkRule {
    /// The matcher.
    pub matcher: NetworkMatcher,
    /// Whether this is an exception rule (`@@` prefix).
    pub is_exception: bool,
}

/// Parse the pattern part of a network rule.
fn parse_pattern(pattern: &str) -> Result<NetPattern, FilterError> {
    let pattern = pattern.trim();

    if pattern.is_empty() {
        return Err(FilterError::InvalidPattern("empty pattern".to_string()));
    }

    // Hostname anchor: ||example.com^
    if let Some(rest) = pattern.strip_prefix("||") {
        if rest.is_empty() {
            return Err(FilterError::InvalidPattern(
                "empty hostname anchor".to_string(),
            ));
        }
        // Take everything up to the first `^` or end.
        let end = rest.find('^').unwrap_or(rest.len());
        if end == 0 {
            return Err(FilterError::InvalidPattern(
                "empty hostname anchor".to_string(),
            ));
        }
        return Ok(NetPattern::HostnameAnchor(rest[..end].to_ascii_lowercase()));
    }

    // Regex pattern: /pattern/
    if pattern.starts_with('/') && pattern.ends_with('/') && pattern.len() > 2 {
        return Ok(NetPattern::Regex(pattern[1..pattern.len() - 1].to_string()));
    }

    // Plain substring match.
    Ok(NetPattern::Plain(pattern.to_string()))
}

/// Parse a `domain=` constraint.
fn parse_domain_constraint(spec: &str) -> Result<DomainConstraint, FilterError> {
    let mut includes = Vec::new();
    let mut excludes = Vec::new();

    for domain in spec.split('|') {
        let domain = domain.trim();
        if domain.is_empty() {
            continue;
        }
        if let Some(excluded) = domain.strip_prefix('~') {
            excludes.push(excluded.to_ascii_lowercase());
        } else {
            includes.push(domain.to_ascii_lowercase());
        }
    }

    if !includes.is_empty() {
        Ok(DomainConstraint::Include(includes))
    } else if !excludes.is_empty() {
        Ok(DomainConstraint::Exclude(excludes))
    } else {
        Ok(DomainConstraint::Any)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hostname_anchor() {
        let r = parse_network("||example.com^").unwrap().unwrap();
        assert!(matches!(r.matcher.pattern, NetPattern::HostnameAnchor(_)));
        assert!(!r.is_exception);
    }

    #[test]
    fn parse_exception_rule() {
        let r = parse_network("@@||example.com^").unwrap().unwrap();
        assert!(r.is_exception);
    }

    #[test]
    fn parse_third_party_option() {
        let r = parse_network("||example.com^$third-party")
            .unwrap()
            .unwrap();
        assert_eq!(r.matcher.party, Party::Third);
    }

    #[test]
    fn parse_domain_option() {
        let r = parse_network("||example.com^$domain=test.org")
            .unwrap()
            .unwrap();
        match &r.matcher.domains {
            DomainConstraint::Include(domains) => {
                assert_eq!(domains, &["test.org"]);
            }
            _ => panic!("expected Include"),
        }
    }

    #[test]
    fn parse_empty_line_returns_none() {
        assert!(parse_network("").unwrap().is_none());
    }

    #[test]
    fn parse_comment_returns_none() {
        assert!(parse_network("! comment").unwrap().is_none());
    }

    #[test]
    fn parse_cosmetic_returns_none() {
        assert!(parse_network("example.com##.ad").unwrap().is_none());
    }

    #[test]
    fn parse_plain_pattern() {
        // Patterns without || and without /.../ wrapping are plain.
        let r = parse_network("banner").unwrap().unwrap();
        assert!(matches!(r.matcher.pattern, NetPattern::Plain(_)));
    }

    #[test]
    fn parse_regex_pattern() {
        let r = parse_network("/ad[0-9]+\\.js/").unwrap().unwrap();
        assert!(matches!(r.matcher.pattern, NetPattern::Regex(_)));
    }

    #[test]
    fn parse_empty_pattern_errors() {
        // After stripping ||, empty hostname anchor.
        assert!(parse_network("||").is_err());
    }

    #[test]
    fn parse_first_party_option() {
        let r = parse_network("||example.com^$first-party")
            .unwrap()
            .unwrap();
        assert_eq!(r.matcher.party, Party::First);
    }

    #[test]
    fn parse_multi_domain_option() {
        let r = parse_network("||example.com^$domain=a.com|b.com|~c.com")
            .unwrap()
            .unwrap();
        match &r.matcher.domains {
            DomainConstraint::Include(includes) => {
                assert_eq!(includes.len(), 2);
                assert!(includes.contains(&"a.com".to_string()));
                assert!(includes.contains(&"b.com".to_string()));
            }
            _ => panic!("expected Include"),
        }
    }
}
