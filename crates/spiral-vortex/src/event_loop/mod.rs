//! Event loop for the Vortex JavaScript engine.
//!
//! JavaScript is single-threaded with cooperative concurrency via
//! promises and callbacks. The event loop processes:
//!
//! 1. **Microtasks** (promise `.then()` callbacks, `queueMicrotask`)
//!    — drained completely after each task/macrotask.
//! 2. **Macrotasks** (`setTimeout`, `setInterval`, I/O callbacks,
//!    `requestAnimationFrame`) — one per event loop tick.
//!
//! Phase 1: stub — the event loop is not yet wired to the interpreter.
//! Phase 2: the bytecode VM will call `EventLoop::tick()` after each
//! statement or function return to drain microtasks.

use std::collections::VecDeque;

/// A callback scheduled for future execution.
pub type TaskCallback = Box<dyn FnOnce()>;

/// The event loop state.
pub struct EventLoop {
    /// Microtask queue (promise callbacks, `queueMicrotask`).
    microtasks: VecDeque<TaskCallback>,
    /// Macrotask queue (`setTimeout`, `setInterval`, I/O).
    macrotasks: VecDeque<TaskCallback>,
    /// Timer queue (sorted by deadline).
    timers: Vec<TimerEntry>,
}

/// A scheduled timer.
///
/// Fields are stored on construction; the dispatch logic that reads
/// them lands in the M10+ event-loop tick work. Marked
/// `#[allow(dead_code)]` until then so the public surface compiles
/// under `-D warnings`.
#[allow(dead_code)]
struct TimerEntry {
    /// Milliseconds from epoch when this timer should fire.
    deadline_ms: u64,
    /// The callback to run.
    callback: TaskCallback,
    /// If true, re-schedule after firing (setInterval behaviour).
    repeating: bool,
    /// The interval in milliseconds (used only if `repeating`).
    interval_ms: u64,
}

impl EventLoop {
    pub fn new() -> Self {
        Self {
            microtasks: VecDeque::new(),
            macrotasks: VecDeque::new(),
            timers: Vec::new(),
        }
    }

    /// Schedule a microtask (promise callback, `queueMicrotask`).
    pub fn enqueue_microtask(&mut self, task: TaskCallback) {
        self.microtasks.push_back(task);
    }

    /// Schedule a macrotask (`setTimeout` callback, I/O callback).
    pub fn enqueue_macrotask(&mut self, task: TaskCallback) {
        self.macrotasks.push_back(task);
    }

    /// Schedule a one-shot timer (`setTimeout`).
    pub fn set_timeout(&mut self, callback: TaskCallback, delay_ms: u64) {
        let deadline_ms = now_ms() + delay_ms;
        self.timers.push(TimerEntry {
            deadline_ms,
            callback,
            repeating: false,
            interval_ms: delay_ms,
        });
    }

    /// Schedule a repeating timer (`setInterval`).
    pub fn set_interval(&mut self, callback: TaskCallback, interval_ms: u64) {
        let deadline_ms = now_ms() + interval_ms;
        self.timers.push(TimerEntry {
            deadline_ms,
            callback,
            repeating: true,
            interval_ms,
        });
    }

    /// Run one tick of the event loop:
    /// 1. Drain all microtasks.
    /// 2. Run one macrotask (if any).
    /// 3. Drain all microtasks again.
    pub fn tick(&mut self) {
        self.drain_microtasks();
        if let Some(task) = self.macrotasks.pop_front() {
            task();
        }
        self.drain_microtasks();
    }

    /// Drain all pending microtasks.
    fn drain_microtasks(&mut self) {
        while let Some(task) = self.microtasks.pop_front() {
            task();
        }
    }

    /// Whether there is any pending work.
    pub fn has_pending_work(&self) -> bool {
        !self.microtasks.is_empty() || !self.macrotasks.is_empty() || !self.timers.is_empty()
    }
}

impl Default for EventLoop {
    fn default() -> Self {
        Self::new()
    }
}

/// Current time in milliseconds (monotonic clock).
fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn test_microtask_ordering() {
        let mut el = EventLoop::new();
        let order = Rc::new(Cell::new(0));
        let o1 = order.clone();
        let o2 = order.clone();

        el.enqueue_microtask(Box::new(move || {
            o1.set(1);
        }));
        el.enqueue_microtask(Box::new(move || {
            o2.set(2);
        }));

        el.tick();
        assert_eq!(order.get(), 2); // both ran, second set 2
    }

    #[test]
    fn test_has_pending_work() {
        let mut el = EventLoop::new();
        assert!(!el.has_pending_work());
        el.enqueue_microtask(Box::new(|| {}));
        assert!(el.has_pending_work());
    }
}
