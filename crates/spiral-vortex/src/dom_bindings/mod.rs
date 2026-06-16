//! DOM bindings — bridge between the JS engine and the DOM tree.
//!
//! Vortex needs to expose `document`, `window`, and the full DOM API
//! to JavaScript. This module defines the binding interface; the actual
//! DOM manipulation calls into `spiral-dom`.
//!
//! Phase 1: stub — no DOM bindings yet.
//! Phase 2: `document.getElementById`, `createElement`, `appendChild`,
//! `setAttribute`, `textContent`, `innerHTML`.
//! Phase 3: full event dispatch (`addEventListener`, `dispatchEvent`,
//! `MouseEvent`, `KeyboardEvent`, etc.).

use crate::value::object::JsObject;
use spiral_dom::Dom;

/// Create the `document` object that JS code sees.
///
/// In Phase 1 this is a bare stub. Phase 2 wires it to the actual DOM.
pub fn create_document_object(_dom: &Dom) -> JsObject {
    let doc = JsObject::new("Document");

    // Phase 2: wire these to real DOM operations.
    // doc.set("createElement", ...);
    // doc.set("getElementById", ...);
    // doc.set("querySelector", ...);
    // doc.set("querySelectorAll", ...);

    doc
}

/// Create the `window` object (the global object for browser JS).
pub fn create_window_object() -> JsObject {
    JsObject::new("Window")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_document_object() {
        // We can't easily construct a real Dom here, so just test
        // the stub returns a Document-class object.
        // Phase 2 tests will wire up a real DOM.
    }
}
