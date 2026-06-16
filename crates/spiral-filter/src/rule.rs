//! Rule AST — the core types for `spiral-filter`.

/// Top-level rule.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Stable 64-bit hash. Used for dedup and logging.
    pub id: u64,
    /// What kind of rule this is.
    pub kind: RuleKind,
    /// The matcher.
    pub matcher: Matcher,
    /// The action to take when the matcher fires.
    pub action: Action,
    /// How severe the violation is.
    pub severity: Severity,
    /// Where this rule came from.
    pub source: Source,
    /// Stewardship score for the matched domain.
    pub stewardship: Stewardship,
}

/// The kind of rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleKind {
    /// Matches on the outgoing request URL.
    Network,
    /// Matches on elements already in the DOM.
    Cosmetic,
    /// CBA-derived page-level threshold.
    Policy,
}

/// The matcher.
#[derive(Debug, Clone)]
pub enum Matcher {
    /// Network rule: URL pattern + request type + party + domain constraints.
    Network(NetworkMatcher),
    /// Cosmetic rule: CSS selector + hostname scope.
    Cosmetic(CosmeticMatcher),
    /// Policy rule: CBA threshold + platform context.
    Policy(PolicyMatcher),
}

/// Network rule matcher.
#[derive(Debug, Clone)]
pub struct NetworkMatcher {
    /// The URL pattern.
    pub pattern: NetPattern,
    /// Which request types this applies to.
    pub request_kinds: u32, // bitflag of ResourceKind
    /// First-party or third-party.
    pub party: Party,
    /// Domain constraint.
    pub domains: DomainConstraint,
}

/// URL pattern for network rules.
#[derive(Debug, Clone)]
pub enum NetPattern {
    /// Exact hostname prefix (e.g. `||example.com^`).
    HostnameAnchor(String),
    /// Substring match.
    Plain(String),
    /// Regex pattern.
    Regex(String),
}

/// Party constraint.
///
/// **Moved to `spiral-core` (ADR 0005, 2026-06-16).** The trait
/// `spiral_core::FilterHook` takes `Party` by value, so the type
/// must live in `spiral-core` to avoid an upward dep from
/// `spiral-core` to `spiral-filter`. This file now re-exports
/// `Party` from `spiral-core` under the original `rule::Party`
/// path; the type itself is defined in `spiral-core`.
pub use spiral_core::Party;

/// Domain constraint.
#[derive(Debug, Clone)]
pub enum DomainConstraint {
    /// No constraint.
    Any,
    /// Only on these domains.
    Include(Vec<String>),
    /// Not on these domains.
    Exclude(Vec<String>),
}

/// Cosmetic rule matcher.
#[derive(Debug, Clone)]
pub struct CosmeticMatcher {
    /// The CSS selector string.
    pub selector: String,
    /// Which hostnames this applies to.
    pub hostname_scope: HostnameScope,
    /// If true, this is an exception rule (`#@#`).
    pub unhide: bool,
}

/// Hostname scope for cosmetic rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostnameScope {
    /// Apply to all sites.
    Generic,
    /// Only on these hostnames.
    Include(Vec<String>),
    /// Not on these hostnames.
    Exclude(Vec<String>),
}

/// Policy rule matcher (CBA-derived).
#[derive(Debug, Clone)]
pub struct PolicyMatcher {
    /// What shape of ad experience to match.
    pub shape: PolicyShape,
    /// Which platform context.
    pub context: PolicyContext,
}

/// CBA ad experience shapes.
#[derive(Debug, Clone)]
pub enum PolicyShape {
    /// Sticky ad covering > N% of viewport.
    StickyHeightPercent(u8),
    /// Ad density > N% of main content.
    AdDensityPercent(u8),
    /// Fixed autoplay media.
    FixedAutoPlayMedia,
    /// Full-screen scrollover.
    Scrollover,
    /// Flashing animation > N Hz.
    FlashRate(f32),
    /// Non-skippable pre-roll > N seconds.
    NonSkippablePreRoll(u16),
    /// Interstitial shown for > N ms.
    InterstitialMs(u32),
    /// Popunder.
    Popunder,
}

/// Platform context for policy rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyContext {
    Desktop,
    MobileWeb,
    ShortFormVideo,
    MobileApp,
    Any,
}

/// The action to take.
#[derive(Debug, Clone)]
pub enum Action {
    /// Drop the request entirely.
    Block,
    /// Allow the request (exception rule).
    Allow,
    /// Hide the element via CSS.
    Hide {
        /// The CSS property to inject (e.g. `display: none`).
        css: String,
    },
    /// Remove the element from the DOM entirely.
    Remove,
    /// Inject a Content-Security-Policy header.
    Csp {
        /// The CSP policy string.
        policy: String,
    },
    /// Block and log to audit.
    BlockAndReport,
}

/// How severe a violation is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// CBA "least preferred" — on by default.
    WorstOffender,
    /// Common consumer complaints.
    Annoying,
    /// Privacy-invasive tracker.
    Privacy,
    /// Hard CBA/W3C violation.
    Spec,
    /// Malware, fingerprinting, exploit kit.
    Critical,
}

/// Where a rule came from.
#[derive(Debug, Clone)]
pub struct Source {
    /// Which list this came from.
    pub list: SourceList,
    /// Stable version hash of the source list.
    pub version_hash: u64,
}

/// The source list.
#[derive(Debug, Clone)]
pub enum SourceList {
    /// Compiled-in, CBA-derived, zero network.
    DefaultSteward,
    /// Spiral's own curated list.
    SpiralCurated,
    /// User-subscribed external list.
    UserSubscribed {
        /// The list name.
        name: String,
    },
    /// User-added via settings.
    UserCustom,
}

/// Stewardship score for a domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stewardship {
    /// No stewardship information.
    Untracked,
    /// Domain has attested to the Better Ads Standards.
    Stewarded,
    /// Domain is a known offender (count of violations).
    Offender(u32),
    /// User has pinned this domain as allowed.
    UserPinnedAllow,
    /// User has pinned this domain as blocked.
    UserPinnedBlock,
}

/// Helper: compute a stable 64-bit hash for a rule string.
#[must_use]
pub fn rule_hash(s: &str) -> u64 {
    seahash::hash(s.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_ordering() {
        assert!(Severity::WorstOffender < Severity::Annoying);
        assert!(Severity::Annoying < Severity::Privacy);
        assert!(Severity::Privacy < Severity::Spec);
        assert!(Severity::Spec < Severity::Critical);
    }

    #[test]
    fn rule_hash_stable() {
        let h1 = rule_hash("||example.com^");
        let h2 = rule_hash("||example.com^");
        assert_eq!(h1, h2);
    }

    #[test]
    fn rule_hash_distinct() {
        let h1 = rule_hash("||example.com^");
        let h2 = rule_hash("||other.com^");
        assert_ne!(h1, h2);
    }

    #[test]
    fn rule_creation() {
        let rule = Rule {
            id: rule_hash("test_rule"),
            kind: RuleKind::Cosmetic,
            matcher: Matcher::Cosmetic(CosmeticMatcher {
                selector: ".ad-banner".to_string(),
                hostname_scope: HostnameScope::Generic,
                unhide: false,
            }),
            action: Action::Hide {
                css: "display: none !important".to_string(),
            },
            severity: Severity::WorstOffender,
            source: Source {
                list: SourceList::DefaultSteward,
                version_hash: 1,
            },
            stewardship: Stewardship::Untracked,
        };

        assert_eq!(rule.kind, RuleKind::Cosmetic);
        assert_eq!(rule.severity, Severity::WorstOffender);
    }

    #[test]
    fn network_matcher_types() {
        let matcher = NetworkMatcher {
            pattern: NetPattern::HostnameAnchor("example.com".to_string()),
            request_kinds: 0b0001, // image
            party: Party::Third,
            domains: DomainConstraint::Any,
        };

        assert!(matches!(matcher.pattern, NetPattern::HostnameAnchor(_)));
        assert_eq!(matcher.party, Party::Third);
    }

    #[test]
    fn policy_shape_cba_large_sticky() {
        let shape = PolicyShape::StickyHeightPercent(30);
        assert!(matches!(shape, PolicyShape::StickyHeightPercent(30)));
    }
}
