//! CBA — Coalition for Better Ads thresholds.
//!
//! Spiral's default "worst offenders only" policy is derived from the
//! Coalition for Better Ads "Better Ads Standards" public spec at
//! <https://www.betterads.org/standards/>.
//!
//! # Notes on accuracy
//!
//! Some CBA-listed experiences do not have explicit numerical
//! thresholds (e.g. the desktop `prestitial_countdown` does not
//! specify a countdown duration in the CBA standard; the `3 Hz`
//! flashing animation threshold comes from W3C WCAG 2.1, not CBA).
//! For these entries, the threshold value in the `description`
//! is either omitted or sourced from the relevant spec.
//!
//! # Audit (2026-06-15)
//!
//! This file was audited against the CBA public standards page.
//! Corrections made:
//! - `pop_up` reframed as "interstitials blocking main content"
//!   rather than "new window" (CBA defines pop-up as interstitial).
//! - `prestitial_countdown` description no longer cites an
//!   invented "5 s" threshold; CBA does not specify a duration.
//! - `large_sticky` reframed as ">30% of screen real estate"
//!   (CBA does not say "viewport" or "height" specifically).
//! - `flashing_animated` reframed to cite WCAG 2.1 (>3 Hz) as
//!   the source of the threshold, not CBA.
//! - `scrollover` reframed: CBA's threshold is ">30% of the page",
//!   not "full viewport".
//! - `prestitial` (mobile) description no longer cites the
//!   invented "30% viewport" threshold; CBA does not specify one.

/// A CBA ad experience threshold.
#[derive(Debug, Clone)]
pub struct CbaThreshold {
    /// The ad experience type.
    pub experience: &'static str,
    /// The platform context.
    pub platform: &'static str,
    /// Human-readable threshold description.
    pub description: &'static str,
    /// The severity Spiral assigns to this violation.
    pub severity: super::super::rule::Severity,
}

/// Desktop CBA thresholds.
pub const DESKTOP_THRESHOLDS: &[CbaThreshold] = &[
    CbaThreshold {
        experience: "pop_up",
        platform: "desktop",
        description: "Pop-up ads: interstitial that blocks the main content of the page",
        severity: super::super::rule::Severity::WorstOffender,
    },
    CbaThreshold {
        experience: "autoplay_video_sound",
        platform: "desktop",
        description: "Auto-playing video ads with sound: audible audio with no user initiation",
        severity: super::super::rule::Severity::WorstOffender,
    },
    CbaThreshold {
        experience: "prestitial_countdown",
        platform: "desktop",
        description: "Prestitial ads with countdown: full-page ad before content with a countdown before the user can dismiss (CBA does not specify a duration threshold)",
        severity: super::super::rule::Severity::WorstOffender,
    },
    CbaThreshold {
        experience: "large_sticky",
        platform: "desktop",
        description: "Large sticky ads: > 30% of the screen's real estate (CBA spec, not viewport/height-specific)",
        severity: super::super::rule::Severity::WorstOffender,
    },
    CbaThreshold {
        experience: "ad_density_50",
        platform: "desktop",
        description: "Ad density > 50%: sum of ad heights within main content / main content height > 50%",
        severity: super::super::rule::Severity::Annoying,
    },
];

/// Mobile web CBA thresholds.
pub const MOBILE_THRESHOLDS: &[CbaThreshold] = &[
    CbaThreshold {
        experience: "pop_up",
        platform: "mobile",
        description: "Pop-up ads: post-load interstitial that blocks the page content",
        severity: super::super::rule::Severity::WorstOffender,
    },
    CbaThreshold {
        experience: "prestitial",
        platform: "mobile",
        description: "Prestitial ads: full-screen or part-screen ad before content loads, blocking the user (CBA does not specify a size threshold)",
        severity: super::super::rule::Severity::WorstOffender,
    },
    CbaThreshold {
        experience: "ad_density_30",
        platform: "mobile",
        description: "Ad density > 30%: sum of ad heights within main content / main content height > 30%",
        severity: super::super::rule::Severity::Annoying,
    },
    CbaThreshold {
        experience: "flashing_animated",
        platform: "mobile",
        description: "Flashing animated ads: animation flashes > 3 times per second (W3C WCAG 2.1, not CBA)",
        severity: super::super::rule::Severity::WorstOffender,
    },
    CbaThreshold {
        experience: "scrollover",
        platform: "mobile",
        description: "Full-screen scrollover ads: ad appears on top of content covering > 30% of the page (CBA spec, not full viewport)",
        severity: super::super::rule::Severity::WorstOffender,
    },
    CbaThreshold {
        experience: "large_sticky",
        platform: "mobile",
        description: "Large sticky ads: > 30% of the screen's real estate (CBA spec)",
        severity: super::super::rule::Severity::WorstOffender,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_thresholds_not_empty() {
        assert!(!DESKTOP_THRESHOLDS.is_empty());
    }

    #[test]
    fn mobile_thresholds_not_empty() {
        assert!(!MOBILE_THRESHOLDS.is_empty());
    }

    #[test]
    fn all_thresholds_have_descriptions() {
        for t in DESKTOP_THRESHOLDS.iter().chain(MOBILE_THRESHOLDS.iter()) {
            assert!(!t.description.is_empty());
            assert!(!t.experience.is_empty());
        }
    }

    #[test]
    fn pop_up_descriptions_dont_say_new_window() {
        // Audit fix 2026-06-15: CBA pop-up is interstitial, not new window.
        for t in DESKTOP_THRESHOLDS.iter().chain(MOBILE_THRESHOLDS.iter()) {
            if t.experience == "pop_up" {
                assert!(
                    !t.description.contains("new window"),
                    "CBA pop-up is interstitial, not new window: {}",
                    t.description
                );
            }
        }
    }

    #[test]
    fn scrollover_threshold_is_30_percent() {
        // Audit fix 2026-06-15: CBA scrollover threshold is > 30% of the
        // page, not "full viewport".
        for t in MOBILE_THRESHOLDS.iter() {
            if t.experience == "scrollover" {
                assert!(
                    t.description.contains("30%"),
                    "CBA scrollover threshold is > 30% of the page: {}",
                    t.description
                );
                assert!(
                    !t.description.contains("full viewport takeover"),
                    "CBA scrollover is not 'full viewport': {}",
                    t.description
                );
            }
        }
    }

    #[test]
    fn prestitial_countdown_does_not_invent_5s() {
        // Audit fix 2026-06-15: CBA does not specify a 5s threshold.
        for t in DESKTOP_THRESHOLDS.iter() {
            if t.experience == "prestitial_countdown" {
                assert!(
                    !t.description.contains("5 s") && !t.description.contains(">= 5"),
                    "CBA does not specify a 5s prestitial countdown threshold: {}",
                    t.description
                );
            }
        }
    }
}
