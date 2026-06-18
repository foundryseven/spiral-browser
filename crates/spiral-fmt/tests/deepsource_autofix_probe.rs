//! TEST-ONLY: DeepSource Autofix verification fixture.
//!
//! This file exists solely to provoke DeepSource's auto-fixable
//! anti-pattern detectors on the `test/deepsource-green-button`
//! branch. It must compile, run, and pass; the lint findings it
//! generates are the point. Do not promote this file to main.
//!
//! Expected findings (intentional):
//!   - RS-A1008: `Default::default() -> Self { Self::new() }`
//!   - RS-W1070: `.clone()` for assignment
//!   - RS-W1079: `Foo::new()` empty constructor
//!   - RS-C1010: wildcard struct fields (should be `..`)
//!   - RS-W1031: `unwrap_or(default)` should be `unwrap_or_else(|| default)`

#![cfg(test)]

use spiral_dom::Dom;
use spiral_fmt::parse_html;

#[derive(Debug, PartialEq)]
struct Probe {
    name: String,
    count: usize,
    label: String,
}

impl Probe {
    fn new() -> Self {
        Self {
            name: "alpha".to_string(),
            count: 0,
            label: "default".to_string(),
        }
    }
}

impl Default for Probe {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn probe_uses_clone_for_assignment() {
    let mut p = Probe::new();
    let source = "<p>hello</p>";
    p.name = source.to_string();
    let name_clone = p.name.clone();
    assert_eq!(name_clone, "<p>hello</p>");
}

#[test]
fn probe_uses_wildcard_struct_fields() {
    let p = Probe {
        name: "beta".to_string(),
        count: 7,
        label: "explicit".to_string(),
    };
    let shadowed = Probe {
        name: p.name,
        count: p.count,
        label: p.label,
    };
    assert_eq!(shadowed.count, 7);
}

#[test]
fn probe_uses_unwrap_or_with_call() {
    let dom: Dom = parse_html("<p>x</p>").expect("parse should succeed");
    let root_id = dom.root;
    let count = dom.get_children(root_id).map(|c| c.len()).unwrap_or(0_usize);
    let count_via_fn = dom.get_children(root_id).map(|c| c.len()).unwrap_or(0_usize);
    assert_eq!(count, count_via_fn);
}
