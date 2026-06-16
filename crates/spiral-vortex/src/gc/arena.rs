//! `OriginArena` — per-origin GC arena.
//!
//! Each origin in the renderer process has exactly one `OriginArena`.
//! The arena owns its slot array, free list, and root set. GC runs
//! per-arena and only touches that arena's slots.
//!
//! The Phase 1 implementation uses a `Vec<Option<TaggedCell>>` with
//! a versioned free list. A future Phase 2+ implementation can use
//! `SlotMap` for stable keys, but the current design is sufficient
//! and avoids the external dependency.

use std::collections::VecDeque;

use super::key::{GcKey, TaggedCell};

/// Statistics from a GC collection cycle.
#[derive(Debug, Default, Clone, Copy)]
pub struct CollectStats {
    /// Number of cells marked.
    pub marked: u32,
    /// Number of cells swept.
    pub swept: u32,
    /// Bytes freed.
    pub bytes_freed: u32,
    /// Duration of the collection in microseconds.
    pub duration_us: u64,
}

/// A per-origin GC arena.
#[derive(Debug)]
pub struct OriginArena {
    /// The origin id this arena belongs to.
    pub origin_id: u16,
    /// Slot array. `None` means a free slot.
    slots: Vec<Option<TaggedCell>>,
    /// Version per slot. Incremented when the slot is reused.
    versions: Vec<u32>,
    /// Free slot indices.
    free_list: Vec<u32>,
    /// Per-isolate root keys (always considered live).
    roots: Vec<GcKey>,
    /// Per-frame stack roots (temporary).
    stack_roots: Vec<GcKey>,
    /// Byte threshold that triggers a GC.
    pub gc_threshold: usize,
    /// Total live bytes in this arena.
    pub live_bytes: usize,
}

impl OriginArena {
    /// Create a new arena for the given origin.
    #[must_use]
    pub fn new(origin_id: u16) -> Self {
        Self {
            origin_id,
            slots: Vec::new(),
            versions: Vec::new(),
            free_list: Vec::new(),
            roots: Vec::new(),
            stack_roots: Vec::new(),
            gc_threshold: 1_048_576, // 1 MB initial
            live_bytes: 0,
        }
    }

    /// Allocate a cell. Returns a `GcKey` identifying the slot.
    pub fn alloc(&mut self, mut cell: TaggedCell) -> GcKey {
        // Force the cell's origin to match this arena.
        let header = &mut cell.header;
        *header = super::header::CellHeader::new(header.cell_type(), self.origin_id);

        let size = cell.size as usize;
        let origin_id = self.origin_id;

        if let Some(slot) = self.free_list.pop() {
            let version = self.versions[slot as usize] + 1;
            self.versions[slot as usize] = version;
            self.slots[slot as usize] = Some(cell);
            self.live_bytes += size;
            GcKey::new(slot, version, origin_id)
        } else {
            let slot = self.slots.len() as u32;
            self.slots.push(Some(cell));
            self.versions.push(1);
            self.live_bytes += size;
            GcKey::new(slot, 1, origin_id)
        }
    }

    /// Look up a cell by key. Returns `None` if the key is stale
    /// (version mismatch) or the slot is free.
    #[must_use]
    pub fn get(&self, key: GcKey) -> Option<&TaggedCell> {
        if key.origin_id != self.origin_id {
            return None;
        }
        if key.slot as usize >= self.slots.len() {
            return None;
        }
        if self.versions[key.slot as usize] != key.version {
            return None;
        }
        self.slots[key.slot as usize].as_ref()
    }

    /// Add a root key. Roots are always live.
    pub fn add_root(&mut self, key: GcKey) {
        if key.origin_id == self.origin_id {
            self.roots.push(key);
        }
    }

    /// Remove a root key.
    pub fn remove_root(&mut self, key: GcKey) {
        self.roots.retain(|k| *k != key);
    }

    /// Push a stack root. Used for temporary references during
    /// expression evaluation.
    pub fn push_stack_root(&mut self, key: GcKey) {
        if key.origin_id == self.origin_id {
            self.stack_roots.push(key);
        }
    }

    /// Pop a stack root.
    pub fn pop_stack_root(&mut self) -> Option<GcKey> {
        self.stack_roots.pop()
    }

    /// Run a mark-sweep collection. Only this arena is touched.
    pub fn collect(&mut self) -> CollectStats {
        let start = std::time::Instant::now();
        let mut stats = CollectStats::default();

        // Unmark all live cells.
        for slot in self.slots.iter().flatten() {
            slot.set_marked(false);
        }

        // Mark phase — walk from roots (isolate roots + stack roots).
        let mut worklist: VecDeque<GcKey> = VecDeque::new();
        for root in &self.roots {
            if self.get(*root).is_some() {
                worklist.push_back(*root);
            }
        }
        for root in &self.stack_roots {
            if self.get(*root).is_some() {
                worklist.push_back(*root);
            }
        }

        while let Some(key) = worklist.pop_front() {
            if let Some(cell) = self.get(key) {
                if cell.is_marked() {
                    continue;
                }
                cell.set_marked(true);
                stats.marked += 1;
                // Phase 1: roots from the VM are passed in via `roots`.
                // Child tracing is a Phase 2 concern (requires JsObject
                // to hold GcKey references, not owned JsValue).
            }
        }

        // Sweep phase — free unmarked cells.
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if let Some(cell) = slot {
                if !cell.is_marked() {
                    let freed = cell.size as u32;
                    stats.bytes_freed += freed;
                    self.live_bytes -= freed as usize;
                    *slot = None;
                    self.free_list.push(i as u32);
                    stats.swept += 1;
                }
            }
        }

        stats.duration_us = start.elapsed().as_micros() as u64;
        stats
    }

    /// Check if the arena has exceeded its GC threshold.
    #[must_use]
    pub fn needs_gc(&self) -> bool {
        self.live_bytes >= self.gc_threshold
    }

    /// Number of live cells in this arena.
    #[must_use]
    pub fn live_count(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_string_arena() -> OriginArena {
        OriginArena::new(1)
    }

    #[test]
    fn new_arena_is_empty() {
        let arena = make_string_arena();
        assert_eq!(arena.live_count(), 0);
        assert_eq!(arena.live_bytes, 0);
    }

    #[test]
    fn alloc_returns_distinct_keys() {
        let mut arena = make_string_arena();
        let a = arena.alloc(TaggedCell::string("a".into(), 1));
        let b = arena.alloc(TaggedCell::string("b".into(), 1));
        assert_ne!(a, b);
    }

    #[test]
    fn get_returns_allocated_cell() {
        let mut arena = make_string_arena();
        let key = arena.alloc(TaggedCell::string("hello".into(), 1));
        let cell = arena.get(key).unwrap();
        assert_eq!(cell.cell_type(), super::super::header::CellType::String);
    }

    #[test]
    fn collect_sweeps_unrooted_cells() {
        let mut arena = make_string_arena();
        let a = arena.alloc(TaggedCell::string("a".into(), 1));
        let b = arena.alloc(TaggedCell::string("b".into(), 1));
        arena.add_root(a);

        let stats = arena.collect();
        assert_eq!(stats.swept, 1);
        assert!(arena.get(a).is_some());
        assert!(arena.get(b).is_none());
    }

    #[test]
    fn collect_keeps_rooted_cells() {
        let mut arena = make_string_arena();
        let a = arena.alloc(TaggedCell::string("a".into(), 1));
        arena.add_root(a);
        let stats = arena.collect();
        assert_eq!(stats.swept, 0);
        assert!(arena.get(a).is_some());
    }

    #[test]
    fn stale_key_returns_none() {
        let mut arena = make_string_arena();
        let key = arena.alloc(TaggedCell::string("a".into(), 1));
        arena.collect(); // sweep without roots -> cell is freed
        assert!(arena.get(key).is_none());
    }

    #[test]
    fn slot_reuse_increments_version() {
        let mut arena = make_string_arena();
        let key1 = arena.alloc(TaggedCell::string("a".into(), 1));
        arena.collect(); // frees it
        let key2 = arena.alloc(TaggedCell::string("b".into(), 1));
        // Same slot, different version.
        assert_eq!(key1.slot, key2.slot);
        assert_ne!(key1.version, key2.version);
    }

    #[test]
    fn cross_origin_key_returns_none() {
        let arena = OriginArena::new(1);
        let key = GcKey::new(0, 1, 2); // origin 2, not 1
        assert!(arena.get(key).is_none());
    }

    #[test]
    fn stack_roots_count_as_live() {
        let mut arena = make_string_arena();
        let a = arena.alloc(TaggedCell::string("a".into(), 1));
        arena.push_stack_root(a);
        let stats = arena.collect();
        assert_eq!(stats.swept, 0);
        assert!(arena.get(a).is_some());
    }
}
