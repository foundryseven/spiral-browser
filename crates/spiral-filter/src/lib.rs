//! Spiral Browser — Ad Filter & Policy Engine
//!
//! `spiral-filter` is Spiral's compile-time HTML/CSS policy engine.
//! It sits between the network layer and the HTML parser. It receives
//! raw HTTP body bytes, parses HTML+CSS, and produces a *transformed*
//! document with worst-offender ads removed or constrained.
//!
//! # Default policy
//!
//! "Worst offenders only" — block layout-breaking banners, popups,
//! autoplay video/audio, interstitials. Allow reasonable ads. No
//! telemetry, no third-party tracking.
//!
//! # Rule model
//!
//! Rules have three parts: a **matcher** (URL pattern, CSS selector,
//! or element attribute pattern), an **action** (block/hide/constrain/allow),
//! and **metadata** (source, severity, stewardship score).
//!
//! # Network hook
//!
//! The [`runtime::Filter`] is the default implementer of
//! [`spiral_core::FilterHook`]. The trait itself lives in
//! `spiral-core` (per ADR 0005, 2026-06-16) so the network layer and
//! the policy engine can both consume it without an upward
//! dependency. Re-exported here for convenience.

pub mod compile;
pub mod error;
pub mod lists;
pub mod policy;
pub mod rule;
pub mod runtime;
pub mod syntax;

pub use error::FilterError;
pub use rule::{Action, Matcher, Rule, RuleKind, Severity};
// `Party` and `FilterHook` + `Decision` are owned by `spiral-core`
// per ADR 0005. Re-export them from the canonical home so consumers
// can `use spiral_filter::Party` if they prefer.
pub use spiral_core::{Decision, FilterHook, Party};
pub use runtime::{Filter, default_network_rules};
