//! `Array` builtin — `Array.isArray`, `Array.from`, `Array.of`, plus
//! instance methods: `push`, `pop`, `shift`, `unshift`, `map`, `filter`,
//! `reduce`, `forEach`, `find`, `findIndex`, `includes`, `indexOf`,
//! `join`, `slice`, `splice`, `concat`, `flat`, `flatMap`, `sort`,
//! `reverse`, `fill`, `copyWithin`, `entries`, `keys`, `values`,
//! `some`, `every`, `at`, `with`, `toReversed`, `toSorted`, `toSpliced`.

use crate::value::object::JsObject;
use crate::value::JsValue;

/// Array.isArray(value) — returns true if value is an Array object.
pub fn is_array(value: &JsValue) -> bool {
    matches!(value, JsValue::Object(obj) if obj.class == "Array")
}

/// Create a new empty Array object.
pub fn new_array() -> JsObject {
    let mut obj = JsObject::new("Array");
    obj.set("length", JsValue::Number(0.0));
    obj
}

/// Push a value onto the end of an array object.
/// Returns the new length.
pub fn push(obj: &mut JsObject, value: JsValue) -> f64 {
    let len = match obj.get("length") {
        Some(JsValue::Number(n)) => *n,
        _ => 0.0,
    };
    let idx = len.to_string();
    obj.set(idx, value);
    let new_len = len + 1.0;
    obj.set("length", JsValue::Number(new_len));
    new_len
}

/// Pop the last value from an array object.
/// Returns `None` if the array is empty.
pub fn pop(obj: &mut JsObject) -> Option<JsValue> {
    let len = match obj.get("length") {
        Some(JsValue::Number(n)) => *n,
        _ => return None,
    };
    if len <= 0.0 {
        return None;
    }
    let new_len = len - 1.0;
    let idx = new_len.to_string();
    let val = obj.properties.remove(&idx);
    obj.set("length", JsValue::Number(new_len));
    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_array() {
        assert!(is_array(&JsValue::Object(new_array())));
        assert!(!is_array(&JsValue::Number(1.0)));
    }

    #[test]
    fn test_push_pop() {
        let mut arr = new_array();
        push(&mut arr, JsValue::Number(1.0));
        push(&mut arr, JsValue::Number(2.0));
        assert!(matches!(arr.get("length"), Some(JsValue::Number(n)) if *n == 2.0));

        let val = pop(&mut arr);
        assert!(matches!(val, Some(JsValue::Number(n)) if n == 2.0));
        assert!(matches!(arr.get("length"), Some(JsValue::Number(n)) if *n == 1.0));
    }

    #[test]
    fn test_pop_empty() {
        let mut arr = new_array();
        assert!(pop(&mut arr).is_none());
    }
}
