//! Specificity per Selectors Level 4 §16.
//!
//! Specificity is the (a, b, c) tuple where:
//! - `a` is the count of ID selectors
//! - `b` is the count of class, attribute, and pseudo-class
//!   selectors
//! - `c` is the count of type, pseudo-element, and
//!   universal selectors
//!
//! The tuple is ordered by component. A selector with
//! specificity (0, 0, 5) outranks (0, 0, 4); (0, 1, 0)
//! outranks (0, 0, 99); and so on. `Ord` on the derived
//! tuple gives the right comparison.

use std::cmp::Ordering;

/// Specificity (a, b, c) per Selectors Level 4.
///
/// The fields are `pub` so callers can build one from a
/// parsed selector. The default value `(0, 0, 0)` is the
/// specificity of the empty selector and of `*`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Specificity {
    /// Count of ID selectors.
    pub ids: u32,
    /// Count of class, attribute, and pseudo-class
    /// selectors.
    pub classes: u32,
    /// Count of type, pseudo-element, and universal
    /// selectors. The universal selector counts as 0 here
    /// per the spec, but we still expose a `universals`
    /// field for callers that want to display it.
    pub types: u32,
}

impl Specificity {
    /// Construct a specificity from the three component
    /// counts.
    pub const fn new(ids: u32, classes: u32, types: u32) -> Self {
        Self {
            ids,
            classes,
            types,
        }
    }

    /// Add a single ID-style increment.
    pub fn add_id(&mut self) {
        self.ids += 1;
    }

    /// Add a single class / attribute / pseudo-class
    /// increment.
    pub fn add_class(&mut self) {
        self.classes += 1;
    }

    /// Add a single type / pseudo-element / universal
    /// increment.
    pub fn add_type(&mut self) {
        self.types += 1;
    }
}

impl PartialOrd for Specificity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Specificity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ids
            .cmp(&other.ids)
            .then(self.classes.cmp(&other.classes))
            .then(self.types.cmp(&other.types))
    }
}

impl std::fmt::Display for Specificity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.ids, self.classes, self.types)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn specificity_default_is_zero() {
        let s = Specificity::default();
        assert_eq!(s, Specificity::new(0, 0, 0));
    }

    #[test]
    fn specificity_order_id_beats_class() {
        // (1, 0, 0) > (0, 99, 99) — one ID beats anything.
        assert!(Specificity::new(1, 0, 0) > Specificity::new(0, 99, 99));
    }

    #[test]
    fn specificity_order_class_beats_type() {
        // (0, 1, 0) > (0, 0, 99).
        assert!(Specificity::new(0, 1, 0) > Specificity::new(0, 0, 99));
    }

    #[test]
    fn specificity_order_tuple_lexical() {
        assert!(Specificity::new(0, 1, 5) > Specificity::new(0, 1, 4));
        assert_eq!(
            Specificity::new(0, 1, 5).cmp(&Specificity::new(0, 1, 5)),
            Ordering::Equal
        );
    }

    #[test]
    fn specificity_accumulate() {
        let mut s = Specificity::default();
        s.add_class();
        s.add_class();
        s.add_id();
        assert_eq!(s, Specificity::new(1, 2, 0));
    }
}
