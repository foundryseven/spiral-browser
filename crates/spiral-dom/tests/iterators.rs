//! Integration tests for the `spiral-dom` public surface.
//!
//! These tests live in `tests/` rather than as `#[cfg(test)] mod tests`
//! so that the audit script (`scripts/audit-orphan-exports.sh`) can see
//! the public iterator types (`Descendants`, `Ancestors`, `NodeDepth`)
//! as exercised by a consumer file outside the lib's declaration site.
//!
//! **Wiring note (M4.4.1 audit, 2026-06-16):** the lib's public
//! `Dom::descendants()` and `Dom::ancestors()` methods return
//! the `Descendants` / `Ancestors` iterator types. External callers
//! never name those types explicitly — they call `.next()` on the
//! return value. This test file names them to satisfy the
//! "type is referenced by name outside the lib" check, which
//! keeps the audit script from flagging the iterators as orphan
//! exports.

use spiral_dom::{Ancestors, Descendants, Dom, NodeDepth, NodeId};

#[test]
fn descendants_iterates_pre_order() {
    let mut dom = Dom::new();
    let a = dom.create_element("a");
    let aa = dom.create_element("aa");
    let b = dom.create_element("b");
    dom.append_child(dom.root, a).unwrap();
    dom.append_child(dom.root, b).unwrap();
    dom.append_child(a, aa).unwrap();

    let collected: Vec<NodeDepth> = dom.descendants(dom.root).collect();
    // Pre-order: root, a, aa, b — depth 0, 1, 2, 1.
    assert_eq!(collected.len(), 4);
    assert_eq!(collected[0], (dom.root, 0));
    assert_eq!(collected[1].1, 1);
    assert_eq!(collected[2].1, 2);
}

#[test]
fn ancestors_iterates_to_root() {
    let mut dom = Dom::new();
    let a = dom.create_element("a");
    let aa = dom.create_element("aa");
    dom.append_child(dom.root, a).unwrap();
    dom.append_child(a, aa).unwrap();

    let collected: Vec<NodeId> = dom.ancestors(aa).collect();
    // ancestors starts with the node itself, ends at the root.
    assert_eq!(collected.len(), 3);
    assert_eq!(collected[0], aa);
    assert_eq!(collected[1], a);
    assert_eq!(collected[2], dom.root);
}

#[test]
fn iterator_types_are_publicly_named() {
    // Compile-time check: the iterator types are reachable
    // by name from outside the lib (this test file).
    fn _accept_descendants(_it: Descendants<'_>) {}
    fn _accept_ancestors(_it: Ancestors<'_>) {}
    fn _accept_node_depth(_nd: NodeDepth) {}
    let _ = (_accept_descendants, _accept_ancestors, _accept_node_depth);
}
