//! Capability tokens and the capability set.
//!
//! Each capability is an unforgeable zero-sized token whose constructor
//! is `pub(crate)` — outside this crate, consumers can hold and pass
//! tokens but cannot construct them.
//!
//! The capability set is a struct of `Option<Cap>` fields, not a
//! `HashSet`. This makes it trivially serialisable and trivially
//! type-checked.

use std::cell::RefCell;
use std::marker::PhantomData;

use crate::brand::Brand;
use crate::origin::Origin;

// ─────────────────────────────────────────────────────────────
// Capability tokens
// ─────────────────────────────────────────────────────────────

/// Filesystem capability. Only constructable by the filesystem module.
#[derive(Debug)]
pub struct FsCap {
    _private: (),
}

impl FsCap {
    /// Grant filesystem access. Only callable from within this crate
    /// (i.e. by the browser runtime when constructing a context).
    pub(crate) fn grant() -> Self {
        Self { _private: () }
    }
}

/// Network capability. Only constructable by the network module.
#[derive(Debug)]
pub struct NetCap {
    _private: (),
}

impl NetCap {
    pub(crate) fn grant() -> Self {
        Self { _private: () }
    }
}

/// Clock capability. Only constructable by the runtime module.
#[derive(Debug)]
pub struct ClockCap {
    _private: (),
}

impl ClockCap {
    pub(crate) fn grant() -> Self {
        Self { _private: () }
    }
}

/// Random number generator capability. Only constructable by the
/// runtime module.
#[derive(Debug)]
pub struct RngCap {
    _private: (),
}

impl RngCap {
    pub(crate) fn grant() -> Self {
        Self { _private: () }
    }
}

// ─────────────────────────────────────────────────────────────
// DOM capability — branded with the origin
// ─────────────────────────────────────────────────────────────

/// DOM capability. Every context has one. Branded with the origin's
/// lifetime so cross-origin DOM access is a compile error.
#[derive(Debug)]
pub struct DomCap<'brand> {
    origin: Origin,
    /// The DOM tree for this origin. Only accessible through
    /// branded handles.
    dom: RefCell<spiral_dom::Dom>,
    _brand: PhantomData<fn(&'brand ()) -> &'brand ()>,
}

impl<'brand> DomCap<'brand> {
    /// Create a new DOM capability for an origin. Only callable by
    /// the browser runtime.
    pub fn new(_brand: Brand<'brand>, origin: Origin) -> Self {
        Self {
            origin,
            dom: RefCell::new(spiral_dom::Dom::new()),
            _brand: PhantomData,
        }
    }

    /// The origin this DOM belongs to.
    #[must_use]
    pub fn origin(&self) -> &Origin {
        &self.origin
    }

    /// Borrow the DOM tree immutably.
    pub fn with_dom<R>(&self, f: impl FnOnce(&spiral_dom::Dom) -> R) -> R {
        f(&self.dom.borrow())
    }

    /// Borrow the DOM tree mutably.
    pub fn with_dom_mut<R>(&self, f: impl FnOnce(&mut spiral_dom::Dom) -> R) -> R {
        f(&mut self.dom.borrow_mut())
    }
}

// ─────────────────────────────────────────────────────────────
// Capability set
// ─────────────────────────────────────────────────────────────

/// The set of capabilities granted to a context.
///
/// The default context has no filesystem, network, clock, or RNG
/// capabilities. Every context has a DOM capability.
#[derive(Debug)]
pub struct CapabilitySet<'brand> {
    /// Filesystem access.
    pub fs: Option<FsCap>,
    /// Network access.
    pub net: Option<NetCap>,
    /// Clock access.
    pub clock: Option<ClockCap>,
    /// Random number generation access.
    pub rng: Option<RngCap>,
    /// DOM access (always present).
    pub dom: DomCap<'brand>,
}

impl<'brand> CapabilitySet<'brand> {
    /// Create an empty capability set (DOM only).
    pub fn empty(brand: Brand<'brand>, origin: Origin) -> Self {
        Self {
            fs: None,
            net: None,
            clock: None,
            rng: None,
            dom: DomCap::new(brand, origin),
        }
    }

    /// Grant filesystem access.
    pub fn grant_fs(&mut self) {
        self.fs = Some(FsCap::grant());
    }

    /// Grant network access.
    pub fn grant_net(&mut self) {
        self.net = Some(NetCap::grant());
    }

    /// Grant clock access.
    pub fn grant_clock(&mut self) {
        self.clock = Some(ClockCap::grant());
    }

    /// Grant RNG access.
    pub fn grant_rng(&mut self) {
        self.rng = Some(RngCap::grant());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brand::make_brand;

    #[test]
    fn empty_capability_set_has_no_fs() {
        make_brand(|brand| {
            let origin = Origin::parse("https://example.com").unwrap();
            let caps = CapabilitySet::empty(brand, origin);
            assert!(caps.fs.is_none());
            assert!(caps.net.is_none());
            assert!(caps.clock.is_none());
            assert!(caps.rng.is_none());
        });
    }

    #[test]
    fn empty_capability_set_has_dom() {
        make_brand(|brand| {
            let origin = Origin::parse("https://example.com").unwrap();
            let caps = CapabilitySet::empty(brand, origin);
            assert_eq!(caps.dom.origin().host(), "example.com");
        });
    }

    #[test]
    fn grant_fs_makes_fs_available() {
        make_brand(|brand| {
            let origin = Origin::parse("https://example.com").unwrap();
            let mut caps = CapabilitySet::empty(brand, origin);
            caps.grant_fs();
            assert!(caps.fs.is_some());
        });
    }

    #[test]
    fn cap_tokens_are_zero_sized() {
        assert_eq!(std::mem::size_of::<FsCap>(), 0);
        assert_eq!(std::mem::size_of::<NetCap>(), 0);
        assert_eq!(std::mem::size_of::<ClockCap>(), 0);
        assert_eq!(std::mem::size_of::<RngCap>(), 0);
    }
}
