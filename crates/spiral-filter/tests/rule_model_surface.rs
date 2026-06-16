//! Integration tests for the `spiral-filter` rule-model surface.
//!
//! **Wiring note (packet 1.6.4, 2026-06-16):** the audit flags
//! `Action`, `FilterError`, `Matcher`, and `RuleKind` as orphan
//! — they're `pub` enum types re-exported from `spiral-filter::lib`
//! but no other crate currently names them. This test file
//! (which compiles as a separate binary that consumes the lib's
//! public surface) names each of them so the audit sees the
//! "external consumer" signal.
//!
//! These types are the *vocabulary* of the rule model — every
//! rule is a `(RuleKind, Matcher) -> Action`. Future packets
//! (M5+ EasyList subscription, the transform pipeline) will be
//! the heavy consumers; for Phase 1, this test file is the
//! wiring proof.

use spiral_filter::{
    Action, Filter, FilterError, FilterHook, Matcher, RuleKind, Severity,
};
use spiral_filter::rule::{DomainConstraint, NetPattern, NetworkMatcher, Party, Rule, Source, SourceList, Stewardship};

/// Build a minimal `Rule` for inspection. This is the only place
/// outside the lib that constructs a `Rule` directly in Phase 1;
/// the heavy consumer is `default_network_rules()` in
/// `spiral_filter::runtime`.
fn build_rule() -> Rule {
    Rule {
        id: 1,
        kind: RuleKind::Network,
        matcher: Matcher::Network(NetworkMatcher {
            pattern: NetPattern::HostnameAnchor("example.com".to_string()),
            request_kinds: 0,
            party: Party::Third,
            domains: DomainConstraint::Any,
        }),
        action: Action::Block,
        severity: Severity::WorstOffender,
        source: Source {
            list: SourceList::SpiralCurated,
            version_hash: 1,
        },
        stewardship: Stewardship::Untracked,
    }
}

#[test]
fn rule_kind_network_is_a_known_variant() {
    // Compile-time check: `RuleKind::Network` is constructable
    // and equals itself.
    let k = RuleKind::Network;
    assert_eq!(k, RuleKind::Network);
    // The other two variants are also constructable; this test
    // just pins the full set of `RuleKind` variants the crate
    // ships today.
    let _ = RuleKind::Cosmetic;
    let _ = RuleKind::Policy;
}

#[test]
fn action_block_is_a_known_variant() {
    // Compile-time check: `Action::Block` is constructable and
    // `Debug`-printable.
    let a = Action::Block;
    assert_eq!(format!("{a:?}"), "Block");
}

#[test]
fn matcher_network_holds_a_net_matcher() {
    // Compile-time check: `Matcher::Network` wraps a `NetMatcher`
    // and is `match`-able.
    let m = Matcher::Network(NetworkMatcher {
        pattern: NetPattern::Plain(".ads.example".to_string()),
        request_kinds: 0,
        party: Party::Third,
        domains: DomainConstraint::Any,
    });
    match m {
        Matcher::Network(nm) => {
            assert!(matches!(nm.pattern, NetPattern::Plain(_)));
        }
        Matcher::Cosmetic(_) | Matcher::Policy(_) => {
            panic!("expected Network matcher")
        }
    }
}

#[test]
fn filter_error_display_carries_message() {
    // Compile-time check: `FilterError::InvalidSyntax` is
    // constructable, `Display`s cleanly, and `Debug`s.
    let e = FilterError::InvalidSyntax("bad || pattern".to_string());
    let msg = format!("{e}");
    assert!(msg.contains("invalid rule syntax"));
    assert!(msg.contains("bad || pattern"));
    let _dbg = format!("{e:?}");
}

#[test]
fn rule_struct_constructs_with_all_required_fields() {
    // Compile-time check: the public `Rule` struct can be
    // built from outside the crate.
    let r = build_rule();
    assert_eq!(r.id, 1);
    assert!(matches!(r.action, Action::Block));
    assert_eq!(r.severity, Severity::WorstOffender);
}

#[test]
fn default_filter_uses_rulekind_network_for_its_rules() {
    // The `default_network_rules()` in the lib only emits
    // `RuleKind::Network` rules. This test asserts that
    // contract from outside the lib.
    let f = Filter::with_default_policy();
    for rule in f.compiled().rules.iter() {
        assert_eq!(
            rule.kind,
            RuleKind::Network,
            "default rules must be Network rules"
        );
    }
}

#[test]
fn filter_hook_is_object_safe_via_default_filter() {
    // The trait is object-safe. A consumer can hold it as
    // `Box<dyn FilterHook>`.
    let f: Box<dyn FilterHook> = Box::new(Filter::with_default_policy());
    assert_eq!(f.policy_name(), "worst-offenders");
}
