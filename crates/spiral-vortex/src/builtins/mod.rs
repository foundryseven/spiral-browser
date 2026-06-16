//! Built-in JavaScript objects and functions.
//!
//! This module provides the initial global bindings that every JS realm
//! starts with: `Object`, `Array`, `Function`, `Boolean`, `Number`,
//! `String`, `Math`, `JSON`, `Date`, `RegExp`, `Map`, `Set`,
//! `Promise`, `console`, `setTimeout`/`setInterval`, etc.
//!
//! In Phase 1 only `console.log` is wired up (directly in the
//! interpreter). The remaining builtins are stubbed here as a design
//! target — their implementation is tracked in the roadmap.

pub mod array;
/// The initial global object created for every JS realm.
///
/// Contains: `Object`, `Array`, `Function`, `Number`, `String`,
/// `Boolean`, `Math`, `JSON`, `Date`, `RegExp`, `Map`, `Set`,
/// `WeakMap`, `WeakSet`, `Symbol`, `BigInt`, `Error`, `TypeError`,
/// `RangeError`, `ReferenceError`, `SyntaxError`, `URIError`,
/// `EvalError`, `console`, `setTimeout`, `setInterval`,
/// `clearTimeout`, `clearInterval`, `queueMicrotask`,
/// `fetch` (Phase 3), `requestAnimationFrame` (Phase 4),
/// `document` (DOM bindings), `window` (alias for globalThis),
/// `globalThis`, `undefined`, `NaN`, `Infinity`, `isNaN`,
/// `isFinite`, `parseInt`, `parseFloat`, `encodeURI`,
/// `decodeURI`, `encodeURIComponent`, `decodeURIComponent`,
/// `eval`, `atob`, `btoa`.
/// Stubs for Phase 2+ builtin registration.
///
/// Each builtin module will register itself into the global environment
/// via `Interpreter::register_builtin()`. The design is intentionally
/// modular so each builtin can be developed and tested independently.
pub mod console;
pub mod math;
pub mod object;
