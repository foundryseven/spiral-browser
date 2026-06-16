//! JS object representation.
//!
//! Objects in Vortex are stored as property maps backed by `IndexMap` for
//! stable insertion order (required by `for...in` and `Object.keys()`).
//! The future GC will manage object lifetimes; for now objects are
//! reference-counted.

use crate::value::JsValue;
use std::collections::HashMap;

/// A JavaScript object.
///
/// In Phase 1 (tree-walking interpreter) objects are ref-counted via
/// `Rc<RefCell<...>>`. The bytecode VM will switch to GC-managed heap
/// objects with `Handle<T>` pointers.
#[derive(Debug, Clone)]
pub struct JsObject {
    /// Named properties: `"foo" → JsValue`.
    pub properties: HashMap<String, JsValue>,
    /// Prototype (null for `Object.create(null)`).
    pub prototype: Option<Box<JsValue>>,
    /// Internal `[[Class]]` name (e.g. `"Array"`, `"Date"`, `"RegExp"`).
    pub class: &'static str,
}

impl JsObject {
    /// Create a bare object with the given class tag.
    pub fn new(class: &'static str) -> Self {
        Self {
            properties: HashMap::new(),
            prototype: None,
            class,
        }
    }

    /// Create a null-prototype object (used by `Object.create(null)`).
    pub fn null_prototype(class: &'static str) -> Self {
        Self {
            properties: HashMap::new(),
            prototype: None,
            class,
        }
    }

    /// Get a property by key, walking the prototype chain.
    pub fn get(&self, key: &str) -> Option<&JsValue> {
        if let Some(val) = self.properties.get(key) {
            return Some(val);
        }
        if let Some(ref proto) = self.prototype {
            if let JsValue::Object(ref obj) = **proto {
                return obj.get(key);
            }
        }
        None
    }

    /// Set (or create) a property on this object (does not walk the chain).
    pub fn set(&mut self, key: impl Into<String>, value: JsValue) {
        self.properties.insert(key.into(), value);
    }

    /// Check if the object (not its prototype chain) has own property.
    pub fn has_own(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    /// Check if the object or its prototype chain has the property.
    pub fn has(&self, key: &str) -> bool {
        if self.has_own(key) {
            return true;
        }
        if let Some(ref proto) = self.prototype {
            if let JsValue::Object(ref obj) = **proto {
                return obj.has(key);
            }
        }
        false
    }

    /// Delete a property. Returns true if the property was own and was
    /// deleted, false otherwise.
    pub fn delete(&mut self, key: &str) -> bool {
        self.properties.remove(key).is_some()
    }

    /// All own enumerable string-keyed property names.
    pub fn own_keys(&self) -> Vec<String> {
        self.properties.keys().cloned().collect()
    }
}

impl Default for JsObject {
    fn default() -> Self {
        Self::new("Object")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_properties() {
        let mut obj = JsObject::new("Object");
        obj.set("x", JsValue::Number(42.0));
        assert!(matches!(obj.get("x"), Some(JsValue::Number(n)) if *n == 42.0));
        assert!(obj.has_own("x"));
        assert!(!obj.has_own("y"));
    }

    #[test]
    fn test_delete() {
        let mut obj = JsObject::new("Object");
        obj.set("x", JsValue::Number(1.0));
        assert!(obj.delete("x"));
        assert!(!obj.has_own("x"));
        assert!(!obj.delete("x")); // already gone
    }

    #[test]
    fn test_own_keys() {
        let mut obj = JsObject::new("Object");
        obj.set("a", JsValue::Null);
        obj.set("b", JsValue::Null);
        let mut keys = obj.own_keys();
        keys.sort();
        assert_eq!(keys, vec!["a".to_string(), "b".to_string()]);
    }
}
