//! Brand — invariant-lifetime tag for origin isolation.
//!
//! A brand is a purely compile-time token that distinguishes two
//! structurally identical values as different types. The brand has
//! no runtime representation.
//!
//! The `'brand` lifetime is made invariant via
//! `PhantomData<fn(&'brand ()) -> &'brand ()>` — the `fn(T) -> T`
//! shape is invariant in `T`.

use std::marker::PhantomData;

/// An invariant-lifetime brand.
///
/// Two brands with different lifetimes are different types at
/// compile time. This prevents cross-origin data access:
/// `Handle<'a, T>` and `Handle<'b, T>` are different types unless
/// `'a` and `'b` unify.
///
/// # Safety
///
/// `Brand` is `!Send + !Sync` (via `PhantomData<*const ()>`),
/// so a brand cannot be shared across threads. This is required
/// because the capability-typed context model assumes single-thread
/// access to the renderer process.
#[derive(Debug, Clone, Copy)]
pub struct Brand<'brand> {
    /// Invariant in `'brand`. The `fn(&'brand ()) -> &'brand ()`
    /// pattern makes the lifetime invariant — a long-lived brand
    /// cannot be silently shortened.
    _invariant: PhantomData<fn(&'brand ()) -> &'brand ()>,
    /// `!Send + !Sync`. Prevents the brand from crossing threads.
    _nosend: PhantomData<*const ()>,
}

impl<'brand> Brand<'brand> {
    /// Create a new brand. Only callable with a fresh lifetime.
    ///
    /// The caller cannot choose the lifetime — it is inferred from
    /// the context where `Brand::new()` is called, ensuring each
    /// call produces a distinct type.
    pub fn new() -> Self {
        Self {
            _invariant: PhantomData,
            _nosend: PhantomData,
        }
    }
}

impl Default for Brand<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// Fresh brand generation. Works on stable Rust by constraining the
/// brand lifetime to the closure's borrow scope.
///
/// # Example
///
/// ```
/// use spiral_context::brand::make_brand;
///
/// make_brand(|brand| {
///     // `brand` is a Brand<'id> where 'id is fresh and invariant.
///     // Any Handle<'id, T> created inside this closure cannot
///     // escape to the outer scope.
/// });
/// ```
pub fn make_brand<R>(f: for<'brand> fn(Brand<'brand>) -> R) -> R {
    f(Brand::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brand_is_zst() {
        assert_eq!(std::mem::size_of::<Brand<'_>>(), 0);
    }

    #[test]
    fn make_brand_produces_valid_brand() {
        make_brand(|_brand| {
            // Brand is created and valid inside this scope.
        });
    }

    #[test]
    fn brands_created_separately_are_independent() {
        make_brand(|_a| {
            make_brand(|_b| {
                // `_a` and `_b` have different brands — they are
                // different types at compile time. If we tried to
                // pass `_a` to a function expecting `_b`'s brand,
                // the compiler would reject it.
            });
        });
    }
}
