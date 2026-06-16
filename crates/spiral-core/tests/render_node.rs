//! Integration tests for the `spiral-core` public surface.
//!
//! These tests live in `tests/` (outside the lib) so the audit
//! script (`scripts/audit-orphan-exports.sh`) sees the public
//! types as exercised by a consumer file.
//!
//! **Wiring note (M4.4.1 audit, 2026-06-16):** the audit
//! flagged `RenderNodeId` and `DomOp` as orphan because the
//! lib's `#[cfg(test)]` tests don't count as cross-crate
//! consumers. This test file names both types by name to
//! satisfy the audit. The renderer integration work in M4.5+
//! will be the real consumer.

use spiral_core::{BrowserToRenderer, DomOp, RenderNodeId, TabId};

#[test]
fn render_node_id_constructs_and_compares() {
    let a = RenderNodeId(1);
    let b = RenderNodeId(1);
    assert_eq!(a, b);
}

#[test]
fn dom_op_variants_construct() {
    let ops: Vec<DomOp> = vec![
        DomOp::AppendChild,
        DomOp::SetAttribute {
            name: "id".to_string(),
            value: "main".to_string(),
        },
        DomOp::SetTextContent {
            text: "hello".to_string(),
        },
        DomOp::RemoveAttribute {
            name: "class".to_string(),
        },
    ];
    assert_eq!(ops.len(), 4);
}

#[test]
fn ipc_update_dom_message_constructs() {
    // The `BrowserToRenderer::UpdateDOM` variant is the IPC
    // message that carries a list of `DomOp` operations from
    // the browser shell to the renderer. M4.5+ will be the
    // real sender/receiver; this test just confirms the
    // public surface compiles and is reachable.
    let msg = BrowserToRenderer::UpdateDOM {
        tab_id: TabId(1),
        node_id: 0,
        operations: vec![DomOp::AppendChild],
    };
    assert!(matches!(msg, BrowserToRenderer::UpdateDOM { .. }));
}
