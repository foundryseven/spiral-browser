//! Style resolution and selector matching for the Gyre layout engine.

use spiral_css::{
    AttributeCase, AttributeMatcher, Combinator, ComplexSelector, ComplexSelectorStep,
    CompoundSelector, Declaration, Rule, Specificity, Stylesheet, TypeSelector, Value,
};
use spiral_dom::{Dom, NodeId};
use std::collections::HashMap;

/// Resolved CSS properties for a layout node.
#[derive(Debug, Clone)]
pub struct ResolvedStyle {
    pub display: String,
    pub width: Option<Value>,
    pub height: Option<Value>,
    pub margin_top: Value,
    pub margin_right: Value,
    pub margin_bottom: Value,
    pub margin_left: Value,
    pub padding_top: Value,
    pub padding_right: Value,
    pub padding_bottom: Value,
    pub padding_left: Value,
    pub border_top_width: Value,
    pub border_right_width: Value,
    pub border_bottom_width: Value,
    pub border_left_width: Value,
}

impl Default for ResolvedStyle {
    fn default() -> Self {
        Self {
            display: "block".to_string(),
            width: None,
            height: None,
            margin_top: Value::Length(0.0),
            margin_right: Value::Length(0.0),
            margin_bottom: Value::Length(0.0),
            margin_left: Value::Length(0.0),
            padding_top: Value::Length(0.0),
            padding_right: Value::Length(0.0),
            padding_bottom: Value::Length(0.0),
            padding_left: Value::Length(0.0),
            border_top_width: Value::Length(0.0),
            border_right_width: Value::Length(0.0),
            border_bottom_width: Value::Length(0.0),
            border_left_width: Value::Length(0.0),
        }
    }
}

/// A matched declaration in the CSS cascade.
struct MatchedDeclaration {
    value: Value,
    specificity: Specificity,
    important: bool,
    source_order: usize,
}

impl ResolvedStyle {
    /// Resolve style for a given DOM node against the stylesheet.
    pub fn resolve(dom: &Dom, node_id: NodeId, stylesheet: &Stylesheet) -> Self {
        let node = match dom.get_node(node_id) {
            Some(spiral_dom::Node::Element(el)) => el,
            _ => return ResolvedStyle::default(),
        };

        // Collect all matched qualified rules in the stylesheet
        let mut raw_decls: Vec<(Declaration, Specificity, usize)> = Vec::new();
        let mut source_order = 0;

        for rule in &stylesheet.rules {
            if let Rule::Qualified(q_rule) = rule {
                let mut matches = false;
                let mut best_specificity = Specificity::default();

                for selector in &q_rule.selector.alternatives {
                    if matches_selector(dom, node_id, selector) {
                        matches = true;
                        if selector.specificity > best_specificity {
                            best_specificity = selector.specificity;
                        }
                    }
                }

                if matches {
                    for decl in &q_rule.declarations {
                        raw_decls.push((decl.clone(), best_specificity, source_order));
                    }
                }
                source_order += 1;
            }
        }

        // Apply cascading rules to resolve final properties
        let mut cascade: HashMap<String, MatchedDeclaration> = HashMap::new();

        for (decl, specificity, order) in raw_decls {
            let name = decl.name.to_lowercase();
            let new_matched = MatchedDeclaration {
                value: decl.value,
                specificity,
                important: decl.important,
                source_order: order,
            };

            if let Some(existing) = cascade.get(&name) {
                if is_higher_priority(&new_matched, existing) {
                    cascade.insert(name, new_matched);
                }
            } else {
                cascade.insert(name, new_matched);
            }
        }

        // Shorthand expansion helpers
        let get_prop = |name: &str, cascade: &HashMap<String, MatchedDeclaration>| {
            cascade.get(name).map(|m| m.value.clone())
        };

        let mut style = ResolvedStyle::default();

        // 1. Resolve Display
        if let Some(Value::Keyword(kw)) = get_prop("display", &cascade) {
            style.display = kw.to_lowercase();
        } else {
            // Default inline vs block depending on tag name
            let tag = node.tag.to_lowercase();
            if matches!(
                tag.as_str(),
                "div"
                    | "p"
                    | "h1"
                    | "h2"
                    | "h3"
                    | "h4"
                    | "h5"
                    | "h6"
                    | "ul"
                    | "ol"
                    | "li"
                    | "section"
                    | "header"
                    | "footer"
                    | "nav"
                    | "article"
                    | "aside"
                    | "hr"
            ) {
                style.display = "block".to_string();
            } else {
                style.display = "inline".to_string();
            }
        }

        // 2. Resolve width and height
        style.width = get_prop("width", &cascade);
        style.height = get_prop("height", &cascade);

        // 3. Resolve Margins
        if let Some(val) = get_prop("margin", &cascade) {
            let (t, r, b, l) = expand_four_sides(&val);
            style.margin_top = t;
            style.margin_right = r;
            style.margin_bottom = b;
            style.margin_left = l;
        }
        if let Some(t) = get_prop("margin-top", &cascade) {
            style.margin_top = t;
        }
        if let Some(r) = get_prop("margin-right", &cascade) {
            style.margin_right = r;
        }
        if let Some(b) = get_prop("margin-bottom", &cascade) {
            style.margin_bottom = b;
        }
        if let Some(l) = get_prop("margin-left", &cascade) {
            style.margin_left = l;
        }

        // 4. Resolve Padding
        if let Some(val) = get_prop("padding", &cascade) {
            let (t, r, b, l) = expand_four_sides(&val);
            style.padding_top = t;
            style.padding_right = r;
            style.padding_bottom = b;
            style.padding_left = l;
        }
        if let Some(t) = get_prop("padding-top", &cascade) {
            style.padding_top = t;
        }
        if let Some(r) = get_prop("padding-right", &cascade) {
            style.padding_right = r;
        }
        if let Some(b) = get_prop("padding-bottom", &cascade) {
            style.padding_bottom = b;
        }
        if let Some(l) = get_prop("padding-left", &cascade) {
            style.padding_left = l;
        }

        // 5. Resolve Border Width
        if let Some(val) = get_prop("border", &cascade) {
            let width = extract_border_width(&val);
            style.border_top_width = width.clone();
            style.border_right_width = width.clone();
            style.border_bottom_width = width.clone();
            style.border_left_width = width;
        }
        if let Some(val) = get_prop("border-width", &cascade) {
            let (t, r, b, l) = expand_four_sides(&val);
            style.border_top_width = t;
            style.border_right_width = r;
            style.border_bottom_width = b;
            style.border_left_width = l;
        }
        if let Some(t) = get_prop("border-top-width", &cascade) {
            style.border_top_width = t;
        }
        if let Some(r) = get_prop("border-right-width", &cascade) {
            style.border_right_width = r;
        }
        if let Some(b) = get_prop("border-bottom-width", &cascade) {
            style.border_bottom_width = b;
        }
        if let Some(l) = get_prop("border-left-width", &cascade) {
            style.border_left_width = l;
        }

        style
    }
}

/// Helper to sort matched declarations in cascade priority order.
fn is_higher_priority(new_decl: &MatchedDeclaration, existing: &MatchedDeclaration) -> bool {
    if new_decl.important != existing.important {
        return new_decl.important; // important overrides non-important
    }
    if new_decl.specificity != existing.specificity {
        return new_decl.specificity > existing.specificity;
    }
    new_decl.source_order >= existing.source_order
}

/// Helper to expand 1, 2, 3, or 4 sides of shorthand values.
fn expand_four_sides(val: &Value) -> (Value, Value, Value, Value) {
    match val {
        Value::List(list) if !list.is_empty() => match list.len() {
            1 => (
                list[0].clone(),
                list[0].clone(),
                list[0].clone(),
                list[0].clone(),
            ),
            2 => (
                list[0].clone(),
                list[1].clone(),
                list[0].clone(),
                list[1].clone(),
            ),
            3 => (
                list[0].clone(),
                list[1].clone(),
                list[2].clone(),
                list[1].clone(),
            ),
            _ => (
                list[0].clone(),
                list[1].clone(),
                list[2].clone(),
                list[3].clone(),
            ),
        },
        single => (
            single.clone(),
            single.clone(),
            single.clone(),
            single.clone(),
        ),
    }
}

/// Helper to extract border-width from `border: 1px solid black` shorthands.
fn extract_border_width(val: &Value) -> Value {
    match val {
        Value::List(list) => {
            for item in list {
                if matches!(item, Value::Length(_) | Value::Number(_)) {
                    return item.clone();
                }
            }
            Value::Length(0.0)
        }
        single @ (Value::Length(_) | Value::Number(_)) => single.clone(),
        _ => Value::Length(0.0),
    }
}

/// Match a `ComplexSelector` against a DOM element node.
pub fn matches_selector(dom: &Dom, node_id: NodeId, selector: &ComplexSelector) -> bool {
    if selector.steps.is_empty() {
        return false;
    }
    matches_selector_steps(dom, node_id, &selector.steps, selector.steps.len() - 1)
}

fn matches_selector_steps(
    dom: &Dom,
    node_id: NodeId,
    steps: &[ComplexSelectorStep],
    idx: usize,
) -> bool {
    if !matches_compound(dom, node_id, &steps[idx].compound) {
        return false;
    }

    if idx == 0 {
        return true;
    }

    let combinator = steps[idx - 1].combinator.unwrap_or(Combinator::Descendant);
    match combinator {
        Combinator::Child => {
            if let Some(parent_id) = dom.get_parent(node_id) {
                matches_selector_steps(dom, parent_id, steps, idx - 1)
            } else {
                false
            }
        }
        Combinator::Descendant => {
            let mut curr = dom.get_parent(node_id);
            while let Some(ancestor_id) = curr {
                if matches_selector_steps(dom, ancestor_id, steps, idx - 1) {
                    return true;
                }
                curr = dom.get_parent(ancestor_id);
            }
            false
        }
        Combinator::NextSibling => {
            if let Some(sibling_id) = get_previous_sibling_element(dom, node_id) {
                matches_selector_steps(dom, sibling_id, steps, idx - 1)
            } else {
                false
            }
        }
        Combinator::SubsequentSibling => {
            let siblings = get_all_previous_sibling_elements(dom, node_id);
            for sibling_id in siblings {
                if matches_selector_steps(dom, sibling_id, steps, idx - 1) {
                    return true;
                }
            }
            false
        }
    }
}

/// Match a `CompoundSelector` against a DOM node.
fn matches_compound(dom: &Dom, node_id: NodeId, compound: &CompoundSelector) -> bool {
    let el = match dom.get_node(node_id) {
        Some(spiral_dom::Node::Element(el)) => el,
        _ => return false,
    };

    // 1. Match type selector (tag name)
    if let Some(type_sel) = &compound.type_selector {
        match type_sel {
            TypeSelector::Universal => {}
            TypeSelector::Element(tag) => {
                if el.tag.to_lowercase() != tag.to_lowercase() {
                    return false;
                }
            }
        }
    }

    // 2. Match ID selectors
    if !compound.ids.is_empty() {
        let id_attr = el
            .attributes
            .iter()
            .find(|(k, _)| k == "id")
            .map(|(_, v)| v);
        for id in &compound.ids {
            if id_attr.map(|s| s.as_str()) != Some(id.as_str()) {
                return false;
            }
        }
    }

    // 3. Match class selectors
    if !compound.classes.is_empty() {
        let class_attr = el
            .attributes
            .iter()
            .find(|(k, _)| k == "class")
            .map(|(_, v)| v);
        let classes: Vec<&str> = match class_attr {
            Some(c) => c.split_whitespace().collect(),
            None => return false,
        };
        for cls in &compound.classes {
            if !classes.contains(&cls.as_str()) {
                return false;
            }
        }
    }

    // 4. Match attribute selectors
    for attr_sel in &compound.attributes {
        let attr_val = el
            .attributes
            .iter()
            .find(|(k, _)| k == &attr_sel.name)
            .map(|(_, v)| v);
        match attr_val {
            Some(val) => {
                if !match_attribute_value(val, &attr_sel.matcher, attr_sel.case) {
                    return false;
                }
            }
            None => return false,
        }
    }

    true
}

fn match_attribute_value(val: &str, matcher: &AttributeMatcher, case: AttributeCase) -> bool {
    let match_val = |v: &str, expected: &str| {
        if case == AttributeCase::Insensitive {
            v.to_lowercase() == expected.to_lowercase()
        } else {
            v == expected
        }
    };

    match matcher {
        AttributeMatcher::Present => true,
        AttributeMatcher::Exact(expected) => match_val(val, expected),
        AttributeMatcher::Includes(expected) => {
            let parts = val.split_whitespace();
            for part in parts {
                if match_val(part, expected) {
                    return true;
                }
            }
            false
        }
        AttributeMatcher::DashMatch(expected) => {
            match_val(val, expected)
                || (val.starts_with(expected) && val.as_bytes().get(expected.len()) == Some(&b'-'))
        }
        AttributeMatcher::Prefix(expected) => {
            if case == AttributeCase::Insensitive {
                val.to_lowercase().starts_with(&expected.to_lowercase())
            } else {
                val.starts_with(expected)
            }
        }
        AttributeMatcher::Suffix(expected) => {
            if case == AttributeCase::Insensitive {
                val.to_lowercase().ends_with(&expected.to_lowercase())
            } else {
                val.ends_with(expected)
            }
        }
        AttributeMatcher::Substring(expected) => {
            if case == AttributeCase::Insensitive {
                val.to_lowercase().contains(&expected.to_lowercase())
            } else {
                val.contains(expected)
            }
        }
    }
}

fn get_previous_sibling_element(dom: &Dom, node_id: NodeId) -> Option<NodeId> {
    let parent_id = dom.get_parent(node_id)?;
    let children = dom.get_children(parent_id)?;
    let pos = children.iter().position(|&id| id == node_id)?;

    for &sibling_id in children[..pos].iter().rev() {
        if let Some(spiral_dom::Node::Element(_)) = dom.get_node(sibling_id) {
            return Some(sibling_id);
        }
    }
    None
}

fn get_all_previous_sibling_elements(dom: &Dom, node_id: NodeId) -> Vec<NodeId> {
    let mut siblings = Vec::new();
    let mut curr = get_previous_sibling_element(dom, node_id);
    while let Some(sibling_id) = curr {
        siblings.push(sibling_id);
        curr = get_previous_sibling_element(dom, sibling_id);
    }
    siblings
}
