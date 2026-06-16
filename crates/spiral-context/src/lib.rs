//! Spiral Browser — Capability-Typed Context API
//!
//! Provides capability-based security for Spiral's shared-everything
//! multi-process renderer. One renderer process, N typed-isolated contexts
//! (one per origin). A context has a set of *capabilities* — unforgeable
//! tokens that grant access to specific resources.
//!
//! # Architecture
//!
//! ```text
//!   Browser runtime (full authority)
//!          │
//!          ├─ grants capabilities ──▶ Context<'a, InProcess>
//!          │                              │
//!          │                         run_script, dom, fetch, hit_test
//!          │
//!          └─ grants capabilities ──▶ Context<'a, Escalated>
//!                                         │
//!                                    all calls proxied over IPC
//! ```
//!
//! # Brand safety
//!
//! The brand `'brand` is an invariant-lifetime tag that prevents
//! cross-origin data access at compile time. Two handles with different
//! brands are different types.

pub mod brand;
pub mod caps;
pub mod context;
pub mod dom;
pub mod origin;

pub use brand::Brand;
pub use caps::{CapabilitySet, ClockCap, DomCap, FsCap, NetCap, RngCap};
pub use context::{Context, ContextOps, InProcess};
pub use origin::Origin;
