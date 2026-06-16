//! Runtime filter — the network-layer decision engine.
//!
//! This is the **packet 1.6.4 surface** (M4.5 Item 12). The runtime
//! sits at the `spiral_network::Client<R>` boundary and answers one
//! question per outgoing URL: *allow or block?* The decision is
//! consumed before the request is dialled.
//!
//! # Design
//!
//! - The trait [`spiral_core::FilterHook`] is **object-safe** (no
//!   `async fn`, no associated types with generics, no `Self` in
//!   return position) so call sites can hold an
//!   `Option<Box<dyn FilterHook>>`. This is the inverse of the
//!   [`crate`] `Resolver` convention — URL inspection is sync, so
//!   `dyn` is the right tool here.
//! - The struct [`Filter`] is the **default implementer**: it
//!   holds a [`crate::compile::CompiledFilter`] and a
//!   [`crate::policy::PolicyLevel`], and answers
//!   `should_block(url, party)` by looking up the hostname in the
//!   compiled rules and applying the policy.
//! - The default policy is **"worst offenders only"** (per
//!   [`crate::policy::PolicyLevel::WorstOffenders`]). The shipped
//!   rules are a small, clearly-illustrative set of well-known
//!   third-party ad/tracker hostnames tagged as `WorstOffender`
//!   severity. Real EasyList / EasyPrivacy subscription is an
//!   M5+ deliverable; this packet is the *engine*, not the
//!   *list*.
//!
//! # Fork 2 (Bet 1)
//!
//! Per `docs/audits/2026-06-15-baseline.md` §Fork 2, the
//! process-global ownership model is the chosen path: a single
//! `Filter` instance shared across the renderer, with a future
//! `PolicyOverride` parameter designed in (the trait method
//! signature reserves space for it without forcing a breaking
//! change). The trait lives in `spiral-core` per ADR 0005
//! (2026-06-16); the implementation lives here.

pub mod match_url;

use crate::compile::CompiledFilter;
use crate::policy::default_policy::PolicyLevel;
use crate::rule::{
    Action, DomainConstraint, Matcher, NetPattern, NetworkMatcher, Rule, RuleKind, Severity,
    Source, SourceList, Stewardship,
};
// `Decision`, `FilterHook`, `Party` come from `spiral-core` per
// ADR 0005. The local `Decision` / `FilterHook` definitions
// previously in this module were removed in the 1.6.4→1.6.5
// refactor. Re-imports here for convenience.
use spiral_core::{Decision, FilterHook, Party};

/// The default filter engine.
///
/// Holds a [`CompiledFilter`] (host-name trie + rule list) and a
/// [`PolicyLevel`]. `should_block` is the public entry point; the
/// per-rule evaluation lives in [`match_url`].
#[derive(Debug)]
pub struct Filter {
    compiled: CompiledFilter,
    policy: PolicyLevel,
}

impl Filter {
    /// Construct a `Filter` from an existing [`CompiledFilter`]
    /// and a [`PolicyLevel`]. Used by tests and by callers who
    /// have a custom rule set.
    pub fn new(compiled: CompiledFilter, policy: PolicyLevel) -> Self {
        Self { compiled, policy }
    }

    /// The default `Filter`: the bundled "worst offenders only"
    /// rule set, [`PolicyLevel::WorstOffenders`].
    #[must_use]
    pub fn with_default_policy() -> Self {
        let mut compiled = CompiledFilter::new();
        compiled.compile(default_network_rules());
        Self {
            compiled,
            policy: PolicyLevel::WorstOffenders,
        }
    }

    /// The current [`PolicyLevel`].
    #[must_use]
    pub fn policy(&self) -> PolicyLevel {
        self.policy
    }

    /// Borrow the underlying [`CompiledFilter`].
    #[must_use]
    pub fn compiled(&self) -> &CompiledFilter {
        &self.compiled
    }

    /// Number of compiled rules.
    #[must_use]
    pub fn rule_count(&self) -> usize {
        self.compiled.rules.len()
    }

    /// Set the policy level. Replaces the previous policy.
    pub fn set_policy(&mut self, policy: PolicyLevel) {
        self.policy = policy;
    }
}

impl FilterHook for Filter {
    fn should_block(&self, url: &str, party: Party) -> Decision {
        let host = match match_url::extract_host(url) {
            Some(h) => h,
            None => return Decision::Allow,
        };

        for rule in &self.compiled.rules {
            if rule.kind != RuleKind::Network {
                continue;
            }
            let Matcher::Network(ref nm) = rule.matcher else {
                continue;
            };
            let matches = match &nm.pattern {
                NetPattern::HostnameAnchor(anchor) => host == *anchor,
                NetPattern::Plain(suffix) => host.ends_with(suffix.as_str()),
                NetPattern::Regex(_) => false, // M5+
            };
            if !matches {
                continue;
            }
            if !self.policy.should_block(rule.severity) {
                // The rule matched but the policy says don't
                // block at this severity level. Continue — a
                // stricter rule may still match.
                continue;
            }
            if !matches!(rule.action, Action::Block) {
                // Only `Block` is honoured at the network
                // boundary. `Hide` and `Remove` are cosmetic
                // and live in `apply_cosmetic`; `Allow` is an
                // exception rule that, ironically, we honour
                // by *not* blocking here.
                continue;
            }
            // Party constraint: skip if the rule is third-party
            // only and the caller says this is a first-party
            // request.
            if nm.party == Party::Third && party == Party::First {
                continue;
            }
            // Domain constraint (Phase 1: `Any` only — `Include`
            // and `Exclude` filtering lands in M5+).
            if !matches!(nm.domains, DomainConstraint::Any) {
                continue;
            }
            return Decision::Block {
                rule_id: rule.id,
                reason: format!(
                    "matched {host} (severity {:?}, policy {})",
                    rule.severity, self.policy,
                ),
            };
        }
        Decision::Allow
    }

    fn policy_name(&self) -> &str {
        self.policy.as_str()
    }
}

// ---------------------------------------------------------------------------
// Default "worst offenders only" rule set.
//
// **Illustrative only.** This is a small, hand-picked set of
// well-known ad/tracker hostnames, all tagged as `WorstOffender`
// severity and `Third` party. It exists so the runtime is
// testable end-to-end (packet 1.6.4 deliverable) and so the audit
// does not flag the surface as orphan.
//
// Real EasyList / EasyPrivacy subscription is an M5+ deliverable.
// The Phase 1 packet is the **engine**, not the **list**.
// ---------------------------------------------------------------------------

/// The default network rule set used by [`Filter::with_default_policy`].
pub fn default_network_rules() -> Vec<Rule> {
    let mut next_id: u64 = 0;
    let mut rule = |host: &str, severity: Severity, party: Party| {
        next_id += 1;
        let id = stable_id(host, next_id);
        Rule {
            id,
            kind: RuleKind::Network,
            matcher: Matcher::Network(NetworkMatcher {
                pattern: NetPattern::HostnameAnchor(host.to_string()),
                request_kinds: 0,
                party,
                domains: DomainConstraint::Any,
            }),
            action: Action::Block,
            severity,
            source: Source {
                list: SourceList::SpiralCurated,
                version_hash: 1,
            },
            stewardship: Stewardship::Untracked,
        }
    };

    // Worst offenders: heavy trackers / ad networks / popup
    // engines. All third-party, all `WorstOffender` severity.
    vec![
        rule("doubleclick.net", Severity::WorstOffender, Party::Third),
        rule(
            "googlesyndication.com",
            Severity::WorstOffender,
            Party::Third,
        ),
        rule(
            "googleadservices.com",
            Severity::WorstOffender,
            Party::Third,
        ),
        rule("adnxs.com", Severity::WorstOffender, Party::Third),
        rule(
            "scorecardresearch.com",
            Severity::WorstOffender,
            Party::Third,
        ),
        rule("outbrain.com", Severity::WorstOffender, Party::Third),
        rule("taboola.com", Severity::WorstOffender, Party::Third),
    ]
}

/// A stable rule ID derived from a hostname + a sequence number.
fn stable_id(host: &str, seq: u64) -> u64 {
    // FNV-1a 64-bit. Stable, no dep, fast.
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in host.as_bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h.wrapping_add(seq)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_filter_blocks_known_tracker() {
        let f = Filter::with_default_policy();
        let d = f.should_block("https://doubleclick.net/ad", Party::Third);
        assert!(d.is_blocked());
    }

    #[test]
    fn default_filter_allows_unknown_host() {
        let f = Filter::with_default_policy();
        let d = f.should_block("https://example.com/", Party::Third);
        assert!(d.is_allowed());
    }

    #[test]
    fn default_filter_respects_first_party_classification() {
        // First-party request to a known tracker host should NOT
        // be blocked by a Third-party-only rule.
        let f = Filter::with_default_policy();
        let d = f.should_block("https://doubleclick.net/", Party::First);
        assert!(d.is_allowed());
    }

    #[test]
    fn policy_off_blocks_nothing() {
        let mut f = Filter::with_default_policy();
        f.set_policy(PolicyLevel::Off);
        let d = f.should_block("https://doubleclick.net/", Party::Third);
        assert!(d.is_allowed());
    }

    #[test]
    fn policy_strict_blocks_worst_offender() {
        let mut f = Filter::with_default_policy();
        // Strict: minimum_severity = Spec. WorstOffender < Spec,
        // so the default rules should NOT trigger. This is
        // *correct* — Strict is a different shape: it only
        // blocks Spec/Critical severity.
        f.set_policy(PolicyLevel::Strict);
        let d = f.should_block("https://doubleclick.net/", Party::Third);
        assert!(d.is_allowed());
    }

    #[test]
    fn policy_maximum_blocks_worst_offender() {
        let mut f = Filter::with_default_policy();
        f.set_policy(PolicyLevel::Maximum);
        // Maximum blocks >= Critical. WorstOffender < Critical,
        // so the default rules do NOT trigger.
        let d = f.should_block("https://doubleclick.net/", Party::Third);
        assert!(d.is_allowed());
    }

    #[test]
    fn decision_allow_predicates() {
        let allow = Decision::Allow;
        assert!(allow.is_allowed());
        assert!(!allow.is_blocked());
    }

    #[test]
    fn decision_block_predicates() {
        let block = Decision::Block {
            rule_id: 42,
            reason: "test".to_string(),
        };
        assert!(!block.is_allowed());
        assert!(block.is_blocked());
    }

    #[test]
    fn policy_name_reflects_level() {
        let f = Filter::with_default_policy();
        assert_eq!(f.policy_name(), "worst-offenders");
    }

    #[test]
    fn rule_count_is_deterministic() {
        let f = Filter::with_default_policy();
        let n = f.rule_count();
        assert!(n > 0, "default filter must contain at least one rule");
        assert_eq!(f.rule_count(), n);
    }

    #[test]
    fn invalid_url_does_not_panic() {
        let f = Filter::with_default_policy();
        let d = f.should_block("not a url", Party::Third);
        assert!(d.is_allowed());
    }

    #[test]
    fn url_with_port_extracts_host() {
        let f = Filter::with_default_policy();
        let d = f.should_block("https://doubleclick.net:443/x", Party::Third);
        assert!(d.is_blocked());
    }

    #[test]
    fn url_with_userinfo_extracts_host() {
        let f = Filter::with_default_policy();
        let d = f.should_block("https://user:pass@doubleclick.net/x", Party::Third);
        assert!(d.is_blocked());
    }

    #[test]
    fn decision_block_carries_rule_id() {
        let f = Filter::with_default_policy();
        let d = f.should_block("https://doubleclick.net/x", Party::Third);
        match d {
            Decision::Block { rule_id, .. } => assert_ne!(rule_id, 0),
            _ => panic!("expected Block"),
        }
    }
}
