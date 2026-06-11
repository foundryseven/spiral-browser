//! Spiral Browser — HTML5 Parser
//!
//! HTML5 parser wrapping html5ever for the Spiral Browser.

use spiral_core::{Error, Result};
use spiral_dom::{Dom, NodeId};

/// HTML parser.
pub struct HtmlParser {
    dom: Dom,
}

impl HtmlParser {
    /// Create a new HTML parser.
    pub fn new() -> Self {
        Self {
            dom: Dom::new(),
        }
    }

    /// Parse HTML string into a DOM tree.
    pub fn parse(&mut self, html: &str) -> Result<NodeId> {
        // For Phase 1, implement basic HTML parsing
        // Phase 2 will integrate html5ever properly
        self.parse_simple(html)
    }

    /// Get the DOM tree.
    pub fn dom(&self) -> &Dom {
        &self.dom
    }

    /// Get mutable reference to the DOM tree.
    pub fn dom_mut(&mut self) -> &mut Dom {
        &mut self.dom
    }

    /// Simple HTML parser for basic tags.
    fn parse_simple(&mut self, html: &str) -> Result<NodeId> {
        let root = self.dom.create_element("html");
        self.dom.append_child(self.dom.root, root).ok();

        let mut current_parent = root;
        let mut i = 0;
        let bytes = html.as_bytes();

        while i < bytes.len() {
            if bytes[i] == b'<' {
                // Find closing bracket
                if let Some(end) = html[i..].find('>') {
                    let tag_content = &html[i + 1..i + end];

                    if tag_content.starts_with('/') {
                        // Closing tag - move up to parent
                        if let Some(parent) = self.dom.get_parent(current_parent) {
                            current_parent = parent;
                        }
                    } else if !tag_content.starts_with('!') {
                        // Opening tag
                        let tag_name = tag_content.split_whitespace().next().unwrap_or("");
                        if !tag_name.is_empty() {
                            let element = self.dom.create_element(tag_name);
                            self.dom.append_child(current_parent, element).ok();
                            current_parent = element;
                        }
                    }

                    i += end + 1;
                } else {
                    i += 1;
                }
            } else {
                // Text content
                let text_start = i;
                while i < bytes.len() && bytes[i] != b'<' {
                    i += 1;
                }
                let text = &html[text_start..i];
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    let text_node = self.dom.create_text(trimmed);
                    self.dom.append_child(current_parent, text_node).ok();
                }
            }
        }

        Ok(root)
    }
}

impl Default for HtmlParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_html() {
        let mut parser = HtmlParser::new();
        let root = parser.parse("<html><body><div>Hello</div></body></html>").unwrap();
        assert!(parser.dom().get_tag(root).is_some());
    }

    #[test]
    fn test_parse_empty_html() {
        let mut parser = HtmlParser::new();
        let root = parser.parse("").unwrap();
        assert!(parser.dom().get_tag(root).is_some());
    }

    #[test]
    fn test_parse_text_content() {
        let mut parser = HtmlParser::new();
        parser.parse("<div>Hello World</div>").unwrap();
        // The DOM should have a div with text content
        assert!(parser.dom().get_node(1).is_some());
    }
}
