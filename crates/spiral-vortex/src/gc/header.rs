//! GC cell header — the per-allocation metadata.
//!
//! Each `TaggedCell` has a 4-byte `CellHeader` containing:
//! - 8 bits: cell type tag
//! - 1 bit: mark flag
//! - 1 bit: finalizer flag
//! - 6 bits: reserved
//! - 16 bits: origin id

use std::sync::atomic::AtomicU32;

/// Type tag for a GC cell.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    /// Slot is free.
    Free = 0,
    /// A JS object.
    Object = 1,
    /// A JS array.
    Array = 2,
    /// A JS string.
    String = 3,
    /// A closure.
    Closure = 4,
    /// A hidden class / shape.
    Shape = 5,
    /// Property backing storage.
    Property = 6,
    /// Compiled script.
    Script = 7,
}

impl CellType {
    /// Extract the type from the header bits.
    #[must_use]
    pub fn from_bits(bits: u32) -> Self {
        match (bits & 0xFF) as u8 {
            0 => Self::Free,
            1 => Self::Object,
            2 => Self::Array,
            3 => Self::String,
            4 => Self::Closure,
            5 => Self::Shape,
            6 => Self::Property,
            7 => Self::Script,
            _ => Self::Free,
        }
    }
}

const MARK_BIT: u32 = 1 << 8;
const FINALIZER_BIT: u32 = 1 << 9;

/// 4-byte cell header.
#[derive(Debug)]
pub struct CellHeader {
    bits: AtomicU32,
}

impl CellHeader {
    /// Create a new header.
    #[must_use]
    pub fn new(cell_type: CellType, origin_id: u16) -> Self {
        let bits = (cell_type as u32) | ((origin_id as u32) << 16);
        Self {
            bits: AtomicU32::new(bits),
        }
    }

    /// Get the cell type.
    #[must_use]
    pub fn cell_type(&self) -> CellType {
        CellType::from_bits(self.bits.load(std::sync::atomic::Ordering::Acquire))
    }

    /// Get the origin id.
    #[must_use]
    pub fn origin_id(&self) -> u16 {
        (self.bits.load(std::sync::atomic::Ordering::Acquire) >> 16) as u16
    }

    /// Check the mark flag.
    #[must_use]
    pub fn is_marked(&self) -> bool {
        self.bits.load(std::sync::atomic::Ordering::Acquire) & MARK_BIT != 0
    }

    /// Set the mark flag.
    pub fn set_marked(&self, marked: bool) {
        if marked {
            self.bits
                .fetch_or(MARK_BIT, std::sync::atomic::Ordering::AcqRel);
        } else {
            self.bits
                .fetch_and(!MARK_BIT, std::sync::atomic::Ordering::AcqRel);
        }
    }

    /// Check the finalizer flag.
    #[must_use]
    pub fn has_finalizer(&self) -> bool {
        self.bits.load(std::sync::atomic::Ordering::Acquire) & FINALIZER_BIT != 0
    }

    /// Set the finalizer flag.
    pub fn set_finalizer(&self, has: bool) {
        if has {
            self.bits
                .fetch_or(FINALIZER_BIT, std::sync::atomic::Ordering::AcqRel);
        } else {
            self.bits
                .fetch_and(!FINALIZER_BIT, std::sync::atomic::Ordering::AcqRel);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_type_round_trip() {
        let h = CellHeader::new(CellType::Object, 42);
        assert_eq!(h.cell_type(), CellType::Object);
        assert_eq!(h.origin_id(), 42);
    }

    #[test]
    fn header_mark_toggle() {
        let h = CellHeader::new(CellType::Object, 0);
        assert!(!h.is_marked());
        h.set_marked(true);
        assert!(h.is_marked());
        h.set_marked(false);
        assert!(!h.is_marked());
    }

    #[test]
    fn header_finalizer_toggle() {
        let h = CellHeader::new(CellType::Object, 0);
        assert!(!h.has_finalizer());
        h.set_finalizer(true);
        assert!(h.has_finalizer());
        h.set_finalizer(false);
        assert!(!h.has_finalizer());
    }

    #[test]
    fn header_origin_id_preserved_across_operations() {
        let h = CellHeader::new(CellType::Array, 12345);
        h.set_marked(true);
        h.set_finalizer(true);
        assert_eq!(h.cell_type(), CellType::Array);
        assert_eq!(h.origin_id(), 12345);
        assert!(h.is_marked());
        assert!(h.has_finalizer());
    }
}
