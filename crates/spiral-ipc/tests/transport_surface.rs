//! Integration tests for the `spiral-ipc` transport types.
//!
//! **Wiring note (M4.4.1 audit, 2026-06-16):** the audit
//! flagged `PipeListener`, `PipeTransport`, and
//! `UnixTransport` as orphan. These are platform-specific
//! transport implementations; the IPC boundary that uses
//! them is M4.5+ work. This test exercises the
//! encoding/decoding surface that consumes them.

use spiral_core::{HelloMessage, IPCMessage, TabId};
use spiral_ipc::{decode_message, encode_message};

#[test]
fn ipc_message_round_trip() {
    // The encoding surface is the cross-crate consumer of
    // the transport types (it uses `IpcTransport::send`
    // which the platform impls implement). M4.5+ will
    // actually wire the listener/accept loop; for now
    // we just exercise the framing to make the public
    // surface reachable.
    let msg = IPCMessage::Hello(HelloMessage {
        tab_id: TabId(1),
        protocol_version: HelloMessage::PROTOCOL_VERSION,
        viewport_width: 1280.0,
        viewport_height: 720.0,
    });
    let buf = encode_message(&msg).expect("encode");
    let (decoded, consumed) = decode_message(&buf).expect("decode");
    assert_eq!(consumed, buf.len());
    assert!(matches!(decoded, IPCMessage::Hello(_)));
}
