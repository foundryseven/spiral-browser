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

pub mod compile;
pub mod error;
pub mod lists;
pub mod policy;
pub mod rule;
pub mod runtime;
pub mod syntax;

pub use error::FilterError;
pub use rule::{Action, Matcher, Rule, RuleKind, Severity};
