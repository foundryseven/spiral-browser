//! The `JsValue` — the fundamental unit of data in JavaScript.
//!
//! Every value in a JS program is one of these variants. The VM,
//! builtins, and DOM bindings all operate on `JsValue`.

use super::object::JsObject;
use std::fmt;

/// A JavaScript value.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum JsValue {
    /// `undefined`
    #[default]
    Undefined,
    /// `null`
    Null,
    /// Boolean.
    Bool(bool),
    /// Number (all JS numbers are IEEE-754 doubles).
    Number(f64),
    /// String (interned per-realm in a future optimisation pass).
    String(String),
    /// Symbol (not yet implemented — placeholder).
    Symbol,
    /// BigInt (not yet implemented — placeholder).
    BigInt,
    /// Object reference (GC-managed heap object).
    Object(JsObject),
    /// Function reference (GC-managed callable object).
    Function(JsObject),
}

impl JsValue {
    /// `typeof` operator result.
    pub fn type_of(&self) -> &'static str {
        match self {
            Self::Undefined => "undefined",
            Self::Null => "object", // JS quirk
            Self::Bool(_) => "boolean",
            Self::Number(_) => "number",
            Self::String(_) => "string",
            Self::Symbol => "symbol",
            Self::BigInt => "bigint",
            Self::Object(_) => "object",
            Self::Function(_) => "function",
        }
    }

    /// ToBoolean abstract operation (§7.1.2).
    pub fn to_boolean(&self) -> bool {
        match self {
            Self::Undefined | Self::Null => false,
            Self::Bool(b) => *b,
            Self::Number(n) => *n != 0.0 && !n.is_nan(),
            Self::String(s) => !s.is_empty(),
            Self::Symbol | Self::BigInt => true,
            Self::Object(_) | Self::Function(_) => true,
        }
    }

    /// ToNumber abstract operation (§7.1.3).
    pub fn to_number(&self) -> f64 {
        match self {
            Self::Undefined => f64::NAN,
            Self::Null => 0.0,
            Self::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            Self::Number(n) => *n,
            Self::String(s) => {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    return 0.0;
                }
                trimmed.parse::<f64>().unwrap_or(f64::NAN)
            }
            Self::Symbol | Self::BigInt => f64::NAN,
            Self::Object(_) | Self::Function(_) => f64::NAN,
        }
    }

    /// Is this value "loosely equal" to another (`==`)?
    /// §7.2.12 Abstract Equality Comparison.
    pub fn loose_eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Same type: use strict equality.
            (Self::Null, Self::Null) => true,
            (Self::Undefined, Self::Undefined) => true,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            // null == undefined
            (Self::Null, Self::Undefined) | (Self::Undefined, Self::Null) => true,
            // number == string: coerce string to number
            (Self::Number(_), Self::String(_)) => {
                let n = other.to_number();
                self.to_number() == n
            }
            (Self::String(_), Self::Number(_)) => {
                let n = self.to_number();
                n == other.to_number()
            }
            // bool == anything: coerce bool to number, compare
            (Self::Bool(_), _) => {
                let n = self.to_number();
                Self::Number(n).loose_eq(other)
            }
            (_, Self::Bool(_)) => {
                let n = other.to_number();
                self.loose_eq(&Self::Number(n))
            }
            // object == string/number: use ToPrimitive (stub for now)
            _ => false,
        }
    }

    /// Strict equality (`===`). §7.2.14.
    pub fn strict_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Undefined, Self::Undefined) => true,
            (Self::Null, Self::Null) => true,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Number(a), Self::Number(b)) => {
                if a.is_nan() || b.is_nan() {
                    return false;
                }
                a == b
            }
            (Self::String(a), Self::String(b)) => a == b,
            // Different types are never strictly equal.
            _ => false,
        }
    }

    /// Check if the value is nullish (null or undefined).
    pub fn is_nullish(&self) -> bool {
        matches!(self, Self::Null | Self::Undefined)
    }
}

impl fmt::Display for JsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Undefined => write!(f, "undefined"),
            Self::Null => write!(f, "null"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Number(n) => {
                if n.is_nan() {
                    write!(f, "NaN")
                } else if n.is_infinite() {
                    if n.is_sign_negative() {
                        write!(f, "-Infinity")
                    } else {
                        write!(f, "Infinity")
                    }
                } else if *n == 0.0 && n.is_sign_negative() {
                    write!(f, "-0")
                } else {
                    write!(f, "{n}")
                }
            }
            Self::String(s) => write!(f, "{s}"),
            Self::Symbol => write!(f, "Symbol()"),
            Self::BigInt => write!(f, "BigInt"),
            Self::Object(_) => write!(f, "[object Object]"),
            Self::Function(_) => write!(f, "function() {{ [native code] }}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typeof() {
        assert_eq!(JsValue::Undefined.type_of(), "undefined");
        assert_eq!(JsValue::Null.type_of(), "object");
        assert_eq!(JsValue::Bool(true).type_of(), "boolean");
        assert_eq!(JsValue::Number(1.0).type_of(), "number");
        assert_eq!(JsValue::String("hi".into()).type_of(), "string");
    }

    #[test]
    fn test_to_boolean() {
        assert!(!JsValue::Undefined.to_boolean());
        assert!(!JsValue::Null.to_boolean());
        assert!(!JsValue::Bool(false).to_boolean());
        assert!(JsValue::Bool(true).to_boolean());
        assert!(!JsValue::Number(0.0).to_boolean());
        assert!(!JsValue::Number(f64::NAN).to_boolean());
        assert!(JsValue::Number(1.0).to_boolean());
        assert!(!JsValue::String(String::new()).to_boolean());
        assert!(JsValue::String("hi".into()).to_boolean());
    }

    #[test]
    fn test_strict_eq() {
        assert!(JsValue::Null.strict_eq(&JsValue::Null));
        assert!(!JsValue::Null.strict_eq(&JsValue::Undefined));
        assert!(JsValue::Number(1.0).strict_eq(&JsValue::Number(1.0)));
        assert!(!JsValue::Number(1.0).strict_eq(&JsValue::String("1".into())));
        assert!(!JsValue::Number(f64::NAN).strict_eq(&JsValue::Number(f64::NAN)));
    }

    #[test]
    fn test_loose_eq() {
        assert!(JsValue::Null.loose_eq(&JsValue::Undefined));
        assert!(JsValue::Number(1.0).loose_eq(&JsValue::String("1".into())));
        assert!(JsValue::Bool(true).loose_eq(&JsValue::Number(1.0)));
        assert!(JsValue::Bool(false).loose_eq(&JsValue::Number(0.0)));
    }

    #[test]
    fn test_display() {
        assert_eq!(JsValue::Undefined.to_string(), "undefined");
        assert_eq!(JsValue::Null.to_string(), "null");
        assert_eq!(JsValue::Number(f64::NAN).to_string(), "NaN");
        assert_eq!(JsValue::Number(f64::INFINITY).to_string(), "Infinity");
    }
}
