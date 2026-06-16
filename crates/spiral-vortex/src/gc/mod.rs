//! Garbage collector for the Vortex JavaScript engine.
//!
//! Phase 1: stop-the-world mark-sweep GC, per-origin arenas.
//! Each origin in the renderer process has its own `OriginArena`.
//! Collecting origin X does not pause origin Y — this is the
//! structural memory win of the shared-everything multi-process
//! design.
//!
//! # Module structure
//!
//! - `header`: 4-byte cell header (type, mark, finalizer, origin).
//! - `key`: `GcKey` — versioned, origin-branded handle.
//! - `arena`: `OriginArena` — one per origin.
//! - `heap`: `VortexHeap` — process-wide, owns N arenas.
//!
//! # Phase progression
//!
//! - Phase 1 (M4–M9): stop-the-world mark-sweep per origin.
//! - Phase 2 (M7–M9): + nursery (bumpalo-style minor GC).
//! - Phase 3 (M10–M18): + incremental mark.
//! - Phase 4 (M19–M30): + concurrent mark on background thread.

pub mod arena;
pub mod header;
pub mod heap;
pub mod key;

pub use arena::{CollectStats, OriginArena};
pub use header::{CellHeader, CellType};
pub use heap::VortexHeap;
pub use key::{CellPayload, GcKey, Shape, TaggedCell};
