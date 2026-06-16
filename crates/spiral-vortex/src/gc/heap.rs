//! Vortex GC heap — the process-wide, origin-tagged garbage collector.
//!
//! `VortexHeap` owns one `OriginArena` per origin. Allocations
//! dispatch to the appropriate arena via `alloc_in`. GC is per-arena;
//! collecting origin X does not pause origin Y.

use std::collections::HashMap;

use super::arena::{CollectStats, OriginArena};
use super::key::{GcKey, TaggedCell};

/// The process-wide Vortex GC heap.
pub struct VortexHeap {
    /// One arena per origin.
    arenas: HashMap<u16, OriginArena>,
    /// Process-wide interned strings. Not origin-tagged.
    interned: OriginArena,
    /// Next available origin id.
    next_origin: u16,
}

impl std::fmt::Debug for VortexHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VortexHeap")
            .field("origin_count", &self.arenas.len())
            .finish_non_exhaustive()
    }
}

impl VortexHeap {
    /// Create a new Vortex heap.
    #[must_use]
    pub fn new() -> Self {
        Self {
            arenas: HashMap::new(),
            interned: OriginArena::new(0), // origin 0 = interned
            next_origin: 1,                // 0 is reserved for interned
        }
    }

    /// Allocate a new origin id. Returns the new id.
    pub fn new_origin(&mut self) -> u16 {
        let id = self.next_origin;
        self.next_origin += 1;
        id
    }

    /// Get or create the arena for an origin.
    pub fn arena(&mut self, origin_id: u16) -> &mut OriginArena {
        self.arenas
            .entry(origin_id)
            .or_insert_with(|| OriginArena::new(origin_id))
    }

    /// Allocate a cell in the given origin's arena.
    pub fn alloc_in(&mut self, origin_id: u16, cell: TaggedCell) -> GcKey {
        self.arena(origin_id).alloc(cell)
    }

    /// Look up a cell by key. Returns `None` if the key is stale
    /// or from a different origin.
    #[must_use]
    pub fn get(&self, key: GcKey) -> Option<&TaggedCell> {
        if key.origin_id == 0 {
            return self.interned.get(key);
        }
        self.arenas.get(&key.origin_id)?.get(key)
    }

    /// Run GC on a single origin's arena.
    pub fn collect_origin(&mut self, origin_id: u16) -> Option<CollectStats> {
        Some(self.arenas.get_mut(&origin_id)?.collect())
    }

    /// Run GC on all arenas. Returns total stats.
    pub fn collect_all(&mut self) -> CollectStats {
        let mut total = CollectStats::default();
        for arena in self.arenas.values_mut() {
            let stats = arena.collect();
            total.marked += stats.marked;
            total.swept += stats.swept;
            total.bytes_freed += stats.bytes_freed;
            total.duration_us += stats.duration_us;
        }
        let interned_stats = self.interned.collect();
        total.marked += interned_stats.marked;
        total.swept += interned_stats.swept;
        total.bytes_freed += interned_stats.bytes_freed;
        total.duration_us += interned_stats.duration_us;
        total
    }

    /// Number of live origins.
    #[must_use]
    pub fn origin_count(&self) -> usize {
        self.arenas.len()
    }

    /// Total live cells across all arenas (including the shared
    /// interned arena).
    #[must_use]
    pub fn total_live_count(&self) -> usize {
        self.arenas.values().map(|a| a.live_count()).sum::<usize>() + self.interned.live_count()
    }

    /// Total live bytes across all arenas.
    #[must_use]
    pub fn total_live_bytes(&self) -> usize {
        self.arenas.values().map(|a| a.live_bytes).sum::<usize>() + self.interned.live_bytes
    }
}

impl Default for VortexHeap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_heap_has_no_arenas() {
        let heap = VortexHeap::new();
        assert_eq!(heap.origin_count(), 0);
    }

    #[test]
    fn new_origin_returns_distinct_ids() {
        let mut heap = VortexHeap::new();
        let a = heap.new_origin();
        let b = heap.new_origin();
        assert_ne!(a, b);
        assert_eq!(heap.origin_count(), 0); // not created until first alloc
    }

    #[test]
    fn alloc_in_creates_arena() {
        let mut heap = VortexHeap::new();
        let id = heap.new_origin();
        let key = heap.alloc_in(id, TaggedCell::string("hi".into(), id));
        assert_eq!(heap.origin_count(), 1);
        assert!(heap.get(key).is_some());
    }

    #[test]
    fn cross_origin_key_returns_none() {
        let mut heap = VortexHeap::new();
        let a = heap.new_origin();
        let b = heap.new_origin();
        let key_a = heap.alloc_in(a, TaggedCell::string("a".into(), a));
        // Looking up key from origin A via origin B's arena fails.
        let arena_b = heap.arena(b);
        assert!(arena_b.get(key_a).is_none());
    }

    #[test]
    fn collect_origin_keeps_rooted_cells() {
        let mut heap = VortexHeap::new();
        let id = heap.new_origin();
        let key = heap.alloc_in(id, TaggedCell::string("rooted".into(), id));
        heap.arena(id).add_root(key);

        let stats = heap.collect_origin(id).unwrap();
        assert_eq!(stats.swept, 0);
        assert!(heap.get(key).is_some());
    }

    #[test]
    fn collect_all_sweeps_all_arenas() {
        let mut heap = VortexHeap::new();
        let a = heap.new_origin();
        let b = heap.new_origin();
        let _ = heap.alloc_in(a, TaggedCell::string("a".into(), a));
        let _ = heap.alloc_in(b, TaggedCell::string("b".into(), b));

        let stats = heap.collect_all();
        assert_eq!(stats.swept, 2);
    }
}
