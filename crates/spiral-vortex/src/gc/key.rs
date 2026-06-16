//! `GcKey` — stable identifier for a GC-managed allocation.
//!
//! A `GcKey` is a versioned, origin-branded handle to a heap slot.
//! The version is incremented on slot reuse, preventing stale-key
//! use-after-free at the Rust level. The origin_id prevents
//! cross-origin GC access.

use crate::gc::header::{CellHeader, CellType};

/// Stable identifier for a GC-managed allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GcKey {
    /// Index into the arena's slot array.
    pub slot: u32,
    /// Version of the slot. Incremented on slot reuse.
    pub version: u32,
    /// Origin of the allocation.
    pub origin_id: u16,
}

impl GcKey {
    /// Create a new `GcKey`.
    #[must_use]
    pub const fn new(slot: u32, version: u32, origin_id: u16) -> Self {
        Self {
            slot,
            version,
            origin_id,
        }
    }
}

/// A tagged GC-managed cell. The header is 4 bytes; the payload is
/// the actual object data.
#[derive(Debug)]
pub struct TaggedCell {
    /// 4-byte header.
    pub header: CellHeader,
    /// 2-byte size of the payload.
    pub size: u16,
    /// Payload bytes. Interpreted by the `CellType`.
    pub payload: CellPayload,
}

/// Cell payload — a tagged union over JS value types.
#[derive(Debug)]
pub enum CellPayload {
    /// Free slot.
    Free,
    /// JS object.
    Object(crate::value::object::JsObject),
    /// JS string.
    String(String),
    /// Shape (hidden class).
    Shape(Shape),
}

/// Hidden class — maps property names to slot indices.
#[derive(Debug, Default)]
pub struct Shape {
    /// Property name → slot index.
    pub properties: std::collections::HashMap<String, u16>,
    /// Next slot to allocate.
    pub next_slot: u16,
}

impl TaggedCell {
    /// Create a free cell.
    #[must_use]
    pub fn free() -> Self {
        Self {
            header: CellHeader::new(CellType::Free, 0),
            size: 0,
            payload: CellPayload::Free,
        }
    }

    /// Create an object cell.
    #[must_use]
    pub fn object(obj: crate::value::object::JsObject, origin_id: u16) -> Self {
        let size = std::mem::size_of::<crate::value::object::JsObject>() as u16;
        Self {
            header: CellHeader::new(CellType::Object, origin_id),
            size,
            payload: CellPayload::Object(obj),
        }
    }

    /// Create a string cell.
    #[must_use]
    pub fn string(s: String, origin_id: u16) -> Self {
        let size = s.len() as u16;
        Self {
            header: CellHeader::new(CellType::String, origin_id),
            size,
            payload: CellPayload::String(s),
        }
    }

    /// Get the origin id of this cell.
    #[must_use]
    pub fn origin_id(&self) -> u16 {
        self.header.origin_id()
    }

    /// Check the mark flag.
    #[must_use]
    pub fn is_marked(&self) -> bool {
        self.header.is_marked()
    }

    /// Set the mark flag.
    pub fn set_marked(&self, marked: bool) {
        self.header.set_marked(marked);
    }

    /// Get the cell type.
    #[must_use]
    pub fn cell_type(&self) -> CellType {
        self.header.cell_type()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn free_cell_creation() {
        let cell = TaggedCell::free();
        assert_eq!(cell.cell_type(), CellType::Free);
    }

    #[test]
    fn string_cell_creation() {
        let cell = TaggedCell::string("hello".to_string(), 7);
        assert_eq!(cell.cell_type(), CellType::String);
        assert_eq!(cell.origin_id(), 7);
    }

    #[test]
    fn object_cell_creation() {
        let obj = crate::value::object::JsObject::new("Object");
        let cell = TaggedCell::object(obj, 42);
        assert_eq!(cell.cell_type(), CellType::Object);
        assert_eq!(cell.origin_id(), 42);
    }

    #[test]
    fn cell_mark_flag() {
        let cell = TaggedCell::free();
        assert!(!cell.is_marked());
        cell.set_marked(true);
        assert!(cell.is_marked());
        cell.set_marked(false);
        assert!(!cell.is_marked());
    }
}
