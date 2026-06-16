//! `Object` builtin — `Object.keys`, `Object.values`, `Object.entries`,
//! `Object.assign`, `Object.create`, `Object.defineProperty`, etc.

use crate::value::object::JsObject;
use crate::value::JsValue;

/// Object.keys(obj) — returns own enumerable string-keyed property names.
pub fn keys(obj: &JsObject) -> Vec<String> {
    obj.own_keys()
}

/// Object.values(obj) — returns own enumerable string-keyed property values.
pub fn values(obj: &JsObject) -> Vec<&JsValue> {
    obj.properties.values().collect()
}

/// Object.entries(obj) — returns `[[key, value]]` pairs.
pub fn entries(obj: &JsObject) -> Vec<(String, &JsValue)> {
    obj.properties.iter().map(|(k, v)| (k.clone(), v)).collect()
}

/// Object.assign(target, ...sources) — shallow copy properties.
pub fn assign(target: &mut JsObject, sources: &[&JsObject]) {
    for source in sources {
        for (k, v) in &source.properties {
            target.set(k.clone(), v.clone());
        }
    }
}

/// Object.create(proto) — create an object with the given prototype.
pub fn create(proto: Option<JsValue>) -> JsObject {
    JsObject {
        prototype: proto.map(Box::new),
        ..JsObject::new("Object")
    }
}

/// Object.freeze(obj) — shallow freeze (Phase 1 stub).
pub fn freeze(_obj: &mut JsObject) {
    // Phase 2: set internal [[Extensible]] to false and make
    // all own properties non-writable, non-configurable.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keys_values_entries() {
        let mut obj = JsObject::new("Object");
        obj.set("a", JsValue::Number(1.0));
        obj.set("b", JsValue::Number(2.0));

        let mut k = keys(&obj);
        k.sort();
        assert_eq!(k, vec!["a".to_string(), "b".to_string()]);

        assert_eq!(entries(&obj).len(), 2);
    }

    #[test]
    fn test_assign() {
        let mut target = JsObject::new("Object");
        let mut source = JsObject::new("Object");
        source.set("x", JsValue::Number(42.0));
        assign(&mut target, &[&source]);
        assert!(matches!(target.get("x"), Some(JsValue::Number(n)) if *n == 42.0));
    }
}
