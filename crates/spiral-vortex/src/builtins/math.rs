//! `Math` builtin — `Math.abs`, `Math.floor`, `Math.ceil`, `Math.round`,
//! `Math.max`, `Math.min`, `Math.pow`, `Math.sqrt`, `Math.random`,
//! `Math.PI`, `Math.E`, etc.

/// Math.PI
pub const PI: f64 = std::f64::consts::PI;

/// Math.E
pub const E: f64 = std::f64::consts::E;

/// Math.LN2
pub const LN2: f64 = std::f64::consts::LN_2;

/// Math.LN10
pub const LN10: f64 = std::f64::consts::LN_10;

/// Math.LOG2E
pub const LOG2E: f64 = std::f64::consts::LOG2_E;

/// Math.LOG10E
pub const LOG10E: f64 = std::f64::consts::LOG10_E;

/// Math.SQRT2
pub const SQRT2: f64 = std::f64::consts::SQRT_2;

/// Math.SQRT1_2
pub const SQRT1_2: f64 = std::f64::consts::FRAC_1_SQRT_2;

/// Math.abs(x)
pub fn abs(x: f64) -> f64 {
    x.abs()
}

/// Math.floor(x)
pub fn floor(x: f64) -> f64 {
    x.floor()
}

/// Math.ceil(x)
pub fn ceil(x: f64) -> f64 {
    x.ceil()
}

/// Math.round(x) — follows JS spec: 0.5 rounds to 1 (not banker's rounding).
pub fn round(x: f64) -> f64 {
    if x.is_nan() {
        return f64::NAN;
    }
    (x + 0.5).floor()
}

/// Math.trunc(x)
pub fn trunc(x: f64) -> f64 {
    x.trunc()
}

/// Math.sign(x) — returns -1, -0, +0, +1, or NaN.
pub fn sign(x: f64) -> f64 {
    if x.is_nan() {
        return f64::NAN;
    }
    if x == 0.0 {
        return x;
    } // preserves -0
    if x > 0.0 {
        1.0
    } else {
        -1.0
    }
}

/// Math.max(...values)
pub fn max(values: &[f64]) -> f64 {
    values.iter().copied().fold(f64::NEG_INFINITY, f64::max)
}

/// Math.min(...values)
pub fn min(values: &[f64]) -> f64 {
    values.iter().copied().fold(f64::INFINITY, f64::min)
}

/// Math.pow(base, exponent)
pub fn pow(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

/// Math.sqrt(x)
pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

/// Math.log(x) — natural logarithm.
pub fn log(x: f64) -> f64 {
    x.ln()
}

/// Math.log2(x)
pub fn log2(x: f64) -> f64 {
    x.log2()
}

/// Math.log10(x)
pub fn log10(x: f64) -> f64 {
    x.log10()
}

/// Math.exp(x)
pub fn exp(x: f64) -> f64 {
    x.exp()
}

/// Math.sin(x)
pub fn sin(x: f64) -> f64 {
    x.sin()
}

/// Math.cos(x)
pub fn cos(x: f64) -> f64 {
    x.cos()
}

/// Math.tan(x)
pub fn tan(x: f64) -> f64 {
    x.tan()
}

/// Math.asin(x)
pub fn asin(x: f64) -> f64 {
    x.asin()
}

/// Math.acos(x)
pub fn acos(x: f64) -> f64 {
    x.acos()
}

/// Math.atan(x)
pub fn atan(x: f64) -> f64 {
    x.atan()
}

/// Math.atan2(y, x)
pub fn atan2(y: f64, x: f64) -> f64 {
    y.atan2(x)
}

/// Math.random() — returns a float in [0, 1).
///
/// Uses a thread-local PCG generator for reproducibility. The seed
/// is derived from system entropy at first use.
pub fn random() -> f64 {
    use std::cell::Cell;
    thread_local! {
        static STATE: Cell<u64> = const { Cell::new(0) };
    }
    STATE.with(|state| {
        let mut s = state.get();
        if s == 0 {
            // Seed from system time + process id.
            s = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64;
            s ^= std::process::id() as u64;
            if s == 0 {
                s = 1;
            }
        }
        // PCG-XSH-RR step.
        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        state.set(s);
        let xorshifted = (((s >> 18) ^ s) >> 27) as u32;
        let rot = (s >> 59) as u32;
        let result = xorshifted.rotate_right(rot);
        result as f64 / 4_294_967_296.0
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(abs(-5.0), 5.0);
        assert_eq!(abs(5.0), 5.0);
    }

    #[test]
    fn test_floor_ceil_round() {
        assert_eq!(floor(1.7), 1.0);
        assert_eq!(ceil(1.2), 2.0);
        assert_eq!(round(1.5), 2.0); // JS rounding
        assert_eq!(round(-0.5), 0.0);
    }

    #[test]
    fn test_max_min() {
        assert_eq!(max(&[1.0, 3.0, 2.0]), 3.0);
        assert_eq!(min(&[1.0, 3.0, 2.0]), 1.0);
    }

    #[test]
    fn test_random_range() {
        for _ in 0..100 {
            let r = random();
            assert!((0.0..1.0).contains(&r), "random() = {r} out of range");
        }
    }

    #[test]
    fn test_sign() {
        assert_eq!(sign(5.0), 1.0);
        assert_eq!(sign(-3.0), -1.0);
        assert_eq!(sign(0.0), 0.0);
        assert!(sign(f64::NAN).is_nan());
    }
}
