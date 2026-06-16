//! Default filter policy — "worst offenders only".

use crate::rule::Severity;

/// The filter policy slider position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PolicyLevel {
    /// No filtering at all.
    Off,
    /// Block only the worst offenders (CBA "least preferred"). This is the default.
    WorstOffenders,
    /// Block common annoyances.
    CommonAnnoyances,
    /// Block privacy-invasive trackers.
    PrivacyFocused,
    /// Block hard CBA/W3C violations.
    Strict,
    /// Block almost everything.
    Maximum,
}

// Clippy suggests `#[derive(Default)]`, but the `#[default]` variant
// attribute is a nightly-only feature. The manual impl is the
// stable-only path.
#[allow(clippy::derivable_impls)]
impl Default for PolicyLevel {
    fn default() -> Self {
        Self::WorstOffenders
    }
}

impl PolicyLevel {
    /// The minimum severity that this policy level will block.
    #[must_use]
    pub fn minimum_severity(&self) -> Option<Severity> {
        match self {
            Self::Off => None,
            Self::WorstOffenders => Some(Severity::WorstOffender),
            Self::CommonAnnoyances => Some(Severity::Annoying),
            Self::PrivacyFocused => Some(Severity::Privacy),
            Self::Strict => Some(Severity::Spec),
            Self::Maximum => Some(Severity::Critical),
        }
    }

    /// Whether a rule with the given severity should be blocked
    /// under this policy level.
    #[must_use]
    pub fn should_block(&self, severity: Severity) -> bool {
        match self.minimum_severity() {
            Some(min) => severity >= min,
            None => false,
        }
    }

    /// A short, stable identifier (used for log keys and the
    /// [`crate::runtime::FilterHook::policy_name`] surface).
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::WorstOffenders => "worst-offenders",
            Self::CommonAnnoyances => "common-annoyances",
            Self::PrivacyFocused => "privacy-focused",
            Self::Strict => "strict",
            Self::Maximum => "maximum",
        }
    }
}

impl std::fmt::Display for PolicyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_worst_offenders() {
        assert_eq!(PolicyLevel::default(), PolicyLevel::WorstOffenders);
    }

    #[test]
    fn worst_offenders_blocks_worst() {
        let policy = PolicyLevel::WorstOffenders;
        assert!(policy.should_block(Severity::WorstOffender));
        assert!(policy.should_block(Severity::Annoying));
        assert!(policy.should_block(Severity::Critical));
    }

    #[test]
    fn off_blocks_nothing() {
        let policy = PolicyLevel::Off;
        assert!(!policy.should_block(Severity::Critical));
    }

    #[test]
    fn maximum_blocks_critical() {
        let policy = PolicyLevel::Maximum;
        assert!(policy.should_block(Severity::Critical));
    }
}
