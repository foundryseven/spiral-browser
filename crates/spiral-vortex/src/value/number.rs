//! Number-related utilities for JS values.

/// JS number precision: all numbers are IEEE-754 f64.
///
/// This module provides helpers for the ToUint32, ToInt32, and
/// ToIntegerOrInfinity abstract operations used by bitwise operators,
/// array indexing, and other spec algorithms.
use std::f64;

/// ToInt32 (§7.1.5). Wraps a float into an i32 using the spec algorithm.
pub fn to_int32(n: f64) -> i32 {
    if !n.is_finite() || n == 0.0 {
        return 0;
    }
    let n = n.trunc();
    let n = n % 4_294_967_296.0; // 2^32
    let n = if n < 0.0 { n + 4_294_967_296.0 } else { n };
    // Cast to u32 first (safe because 0 <= n < 2^32), then reinterpret as i32.
    (n as u32) as i32
}

/// ToUint32 (§7.1.6). Wraps a float into a u32 using the spec algorithm.
pub fn to_uint32(n: f64) -> u32 {
    if !n.is_finite() || n == 0.0 {
        return 0;
    }
    let n = n.trunc();
    let n = n % 4_294_967_296.0;
    let n = if n < 0.0 { n + 4_294_967_296.0 } else { n };
    n as u32
}

/// Number.isNaN (§20.1.2.2). Unlike the global `isNaN`, this does NOT
/// coerce its argument — it only returns true for the NaN number value.
pub fn is_nan(n: f64) -> bool {
    n.is_nan()
}

/// Number.isFinite (§20.1.2.3).
pub fn is_finite(n: f64) -> bool {
    n.is_finite()
}

/// Number.isInteger (§20.1.2.3).
pub fn is_integer(n: f64) -> bool {
    n.is_finite() && n.trunc() == n
}

/// Number.isSafeInteger (§20.1.2.4).
pub fn is_safe_integer(n: f64) -> bool {
    is_integer(n) && n.abs() <= 9_007_199_254_740_991.0 // 2^53 - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_int32() {
        assert_eq!(to_int32(0.0), 0);
        assert_eq!(to_int32(1.0), 1);
        assert_eq!(to_int32(-1.0), -1);
        assert_eq!(to_int32(4_294_967_296.0), 0); // 2^32 wraps to 0
        assert_eq!(to_int32(f64::NAN), 0);
        assert_eq!(to_int32(f64::INFINITY), 0);
    }

    #[test]
    fn test_to_uint32() {
        assert_eq!(to_uint32(0.0), 0);
        assert_eq!(to_uint32(1.0), 1);
        assert_eq!(to_uint32(-1.0), 4_294_967_295); // wraps to 2^32 - 1
    }

    #[test]
    fn test_is_nan() {
        assert!(is_nan(f64::NAN));
        assert!(!is_nan(0.0));
    }

    #[test]
    fn test_is_integer() {
        assert!(is_integer(1.0));
        assert!(!is_integer(1.5));
        assert!(!is_integer(f64::NAN));
    }

    #[test]
    fn test_is_safe_integer() {
        assert!(is_safe_integer(9_007_199_254_740_991.0));
        assert!(!is_safe_integer(9_007_199_254_740_992.0));
    }
}
