//! Compile — rule compilation and application.
//!
//! Transforms a DOM tree by applying filter rules at parse time.

pub mod trie;

use crate::rule::Rule;
use trie::HostnameTrie;

/// A compiled filter set ready for DOM transformation.
#[derive(Debug, Default)]
pub struct CompiledFilter {
    /// Hostname trie for fast hostname-anchored rule lookup.
    pub hostname_trie: HostnameTrie,
    /// All compiled rules.
    pub rules: Vec<Rule>,
}

impl CompiledFilter {
    /// Create a new empty compiled filter.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Compile a set of rules into the filter.
    pub fn compile(&mut self, rules: Vec<Rule>) {
        for rule in &rules {
            // Index hostname-anchored rules in the trie.
            if let crate::rule::Matcher::Network(ref nm) = rule.matcher {
                if let crate::rule::NetPattern::HostnameAnchor(ref host) = nm.pattern {
                    self.hostname_trie.insert(host);
                }
            }
        }
        self.rules.extend(rules);
    }

    /// Apply cosmetic rules to a DOM tree. Returns the set of node IDs
    /// that should be removed.
    pub fn apply_cosmetic(&self, dom: &spiral_dom::Dom, hostname: &str) -> Vec<spiral_dom::NodeId> {
        let mut to_remove = Vec::new();

        for (id, _depth) in dom.descendants(dom.root) {
            if let Some(tag) = dom.get_tag(id) {
                // Simple tag-name based filtering for now.
                // Full CSS selector matching lands in M5+.
                for rule in &self.rules {
                    if rule.kind != crate::rule::RuleKind::Cosmetic {
                        continue;
                    }
                    if let crate::rule::Matcher::Cosmetic(ref cm) = rule.matcher {
                        if cm.hostname_scope == crate::rule::HostnameScope::Generic
                            || match &cm.hostname_scope {
                                crate::rule::HostnameScope::Include(hosts) => {
                                    hosts.iter().any(|h| h == hostname)
                                }
                                _ => false,
                            }
                        {
                            // Phase 1: match on tag name only.
                            // Phase 2+: match on full CSS selectors.
                            if cm.selector == tag {
                                to_remove.push(id);
                            }
                        }
                    }
                }
            }
        }

        to_remove
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_empty_filter() {
        let filter = CompiledFilter::new();
        assert!(filter.rules.is_empty());
    }

    #[test]
    fn compile_adds_rules() {
        let mut filter = CompiledFilter::new();
        let rule = Rule {
            id: 42,
            kind: crate::rule::RuleKind::Cosmetic,
            matcher: crate::rule::Matcher::Cosmetic(crate::rule::CosmeticMatcher {
                selector: "div".to_string(),
                hostname_scope: crate::rule::HostnameScope::Generic,
                unhide: false,
            }),
            action: crate::rule::Action::Hide {
                css: "display: none !important".to_string(),
            },
            severity: crate::rule::Severity::WorstOffender,
            source: crate::rule::Source {
                list: crate::rule::SourceList::DefaultSteward,
                version_hash: 1,
            },
            stewardship: crate::rule::Stewardship::Untracked,
        };
        filter.compile(vec![rule]);
        assert_eq!(filter.rules.len(), 1);
    }
}
