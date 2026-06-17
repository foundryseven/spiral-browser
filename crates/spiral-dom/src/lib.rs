//! Spiral Browser — DOM Tree
//!
//! DOM tree representation for the Spiral Browser.

use spiral_core::{Error, Result};

/// Unique identifier for a DOM node.
pub type NodeId = usize;

/// DOM node types.
#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Text(Text),
    Comment(Comment),
    Document(Document),
}

/// Element node.
#[derive(Debug, Clone)]
pub struct Element {
    /// Tag name (e.g., "div", "span").
    pub tag: String,
    /// Attributes as key-value pairs.
    pub attributes: Vec<(String, String)>,
    /// Child node IDs.
    pub children: Vec<NodeId>,
    /// Parent node ID.
    pub parent: Option<NodeId>,
}

/// Text node.
#[derive(Debug, Clone)]
pub struct Text {
    /// Text content.
    pub content: String,
    /// Parent node ID.
    pub parent: Option<NodeId>,
}

/// Comment node.
#[derive(Debug, Clone)]
pub struct Comment {
    /// Comment text.
    pub content: String,
    /// Parent node ID.
    pub parent: Option<NodeId>,
}

/// Document root node.
#[derive(Debug, Clone)]
pub struct Document {
    /// Child node IDs.
    pub children: Vec<NodeId>,
    /// Quirks mode.
    pub quirks_mode: bool,
}

/// DOM tree.
#[derive(Debug, Clone)]
pub struct Dom {
    /// All nodes in the tree.
    nodes: Vec<Option<Node>>,
    /// Root document node.
    pub root: NodeId,
}

impl Dom {
    /// Create a new empty DOM in quirks mode.
    ///
    /// The default is quirks mode because the HTML parser
    /// (§12.1) treats the absence of a DOCTYPE — by far the
    /// most common case on the modern web — as the quirks
    /// mode trigger. The parser flips this off when it sees a
    /// no-quirks DOCTYPE (e.g. `<!DOCTYPE html>`).
    pub fn new() -> Self {
        let doc = Document {
            children: Vec::new(),
            quirks_mode: true,
        };
        let nodes = vec![Some(Node::Document(doc))];
        Self { nodes, root: 0 }
    }

    /// Create a new element node.
    pub fn create_element(&mut self, tag: &str) -> NodeId {
        let element = Element {
            tag: tag.to_string(),
            attributes: Vec::new(),
            children: Vec::new(),
            parent: None,
        };
        self.insert_node(Node::Element(element))
    }

    /// Create a new text node.
    pub fn create_text(&mut self, content: &str) -> NodeId {
        let text = Text {
            content: content.to_string(),
            parent: None,
        };
        self.insert_node(Node::Text(text))
    }

    /// Create a new comment node.
    pub fn create_comment(&mut self, content: &str) -> NodeId {
        let comment = Comment {
            content: content.to_string(),
            parent: None,
        };
        self.insert_node(Node::Comment(comment))
    }

    /// Append a child node to a parent.
    pub fn append_child(&mut self, parent: NodeId, child: NodeId) -> Result<()> {
        if parent >= self.nodes.len() || child >= self.nodes.len() {
            return Err(Error::Dom("Invalid node ID".to_string()));
        }
        match &mut self.nodes[parent] {
            Some(Node::Element(el)) => {
                el.children.push(child);
            }
            Some(Node::Document(doc)) => {
                doc.children.push(child);
            }
            _ => {
                return Err(Error::Dom("Parent cannot have children".to_string()));
            }
        }
        if let Some(node) = &mut self.nodes[child] {
            match node {
                Node::Element(el) => el.parent = Some(parent),
                Node::Text(t) => t.parent = Some(parent),
                Node::Comment(c) => c.parent = Some(parent),
                Node::Document(_) => {}
            }
        }
        Ok(())
    }

    /// Insert a child node at a specific position in the parent's
    /// child list. Used by the HTML parser's foster-parenting
    /// algorithm (WHATWG §12.2.6.1) to splice an orphan BEFORE
    /// an existing table sibling. `pos` is clamped to the parent's
    /// current child count.
    pub fn insert_child(&mut self, parent: NodeId, pos: usize, child: NodeId) -> Result<()> {
        if parent >= self.nodes.len() || child >= self.nodes.len() {
            return Err(Error::Dom("Invalid node ID".to_string()));
        }
        let pos = match &self.nodes[parent] {
            Some(Node::Element(el)) => pos.min(el.children.len()),
            Some(Node::Document(doc)) => pos.min(doc.children.len()),
            _ => return Err(Error::Dom("Parent cannot have children".to_string())),
        };
        match &mut self.nodes[parent] {
            Some(Node::Element(el)) => {
                el.children.insert(pos, child);
            }
            Some(Node::Document(doc)) => {
                doc.children.insert(pos, child);
            }
            _ => unreachable!(),
        }
        if let Some(node) = &mut self.nodes[child] {
            match node {
                Node::Element(el) => el.parent = Some(parent),
                Node::Text(t) => t.parent = Some(parent),
                Node::Comment(c) => c.parent = Some(parent),
                Node::Document(_) => {}
            }
        }
        Ok(())
    }

    /// Get a node by ID.
    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id).and_then(|n| n.as_ref())
    }

    /// Get a mutable reference to a node by ID.
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id).and_then(|n| n.as_mut())
    }

    /// Set the document's quirks mode.
    ///
    /// Quirks mode affects CSS rendering, not the DOM shape.
    /// Set by the HTML parser from the DOCTYPE token.
    pub fn set_quirks_mode(&mut self, quirks: bool) {
        if let Some(Node::Document(doc)) = self.get_node_mut(self.root) {
            doc.quirks_mode = quirks;
        }
    }

    /// Get the document's quirks mode.
    ///
    /// Returns `true` when the document is in quirks mode (the
    /// default before the parser has seen a non-quirks DOCTYPE).
    /// WHATWG HTML §12.1 defines quirks mode; the parser sets
    /// this to `false` when it sees a no-quirks or limited-quirks
    /// DOCTYPE (e.g. `<!DOCTYPE html>`).
    pub fn quirks_mode(&self) -> bool {
        match self.get_node(self.root) {
            Some(Node::Document(doc)) => doc.quirks_mode,
            _ => true,
        }
    }

    /// Get element tag name.
    pub fn get_tag(&self, id: NodeId) -> Option<&str> {
        match self.get_node(id)? {
            Node::Element(el) => Some(&el.tag),
            _ => None,
        }
    }

    /// Get element attributes.
    pub fn get_attributes(&self, id: NodeId) -> Option<&[(String, String)]> {
        match self.get_node(id)? {
            Node::Element(el) => Some(&el.attributes),
            _ => None,
        }
    }

    /// Get a reference to a text node, if the node is a text node.
    pub fn get_text(&self, id: NodeId) -> Option<&Text> {
        match self.get_node(id)? {
            Node::Text(t) => Some(t),
            _ => None,
        }
    }

    /// Get a mutable reference to a text node, if the node is a text node.
    pub fn get_text_mut(&mut self, id: NodeId) -> Option<&mut Text> {
        match self.nodes.get_mut(id).and_then(|n| n.as_mut()) {
            Some(Node::Text(t)) => Some(t),
            _ => None,
        }
    }

    /// Get children of a node.
    pub fn get_children(&self, id: NodeId) -> Option<Vec<NodeId>> {
        match self.get_node(id)? {
            Node::Element(el) => Some(el.children.clone()),
            Node::Document(doc) => Some(doc.children.clone()),
            _ => None,
        }
    }

    /// Get parent of a node.
    pub fn get_parent(&self, id: NodeId) -> Option<NodeId> {
        match self.get_node(id)? {
            Node::Element(el) => el.parent,
            Node::Text(t) => t.parent,
            Node::Comment(c) => c.parent,
            _ => None,
        }
    }

    /// Insert a node into the tree.
    fn insert_node(&mut self, node: Node) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(Some(node));
        id
    }

    /// Set or overwrite an attribute on an element node.
    pub fn set_attribute(&mut self, id: NodeId, name: &str, value: &str) -> Result<()> {
        match self.nodes.get_mut(id).and_then(|n| n.as_mut()) {
            Some(Node::Element(el)) => {
                if let Some(attr) = el.attributes.iter_mut().find(|(k, _)| k == name) {
                    attr.1 = value.to_string();
                } else {
                    el.attributes.push((name.to_string(), value.to_string()));
                }
                Ok(())
            }
            _ => Err(Error::Dom("Node is not an element".to_string())),
        }
    }

    /// Remove a child from its parent and return it.
    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) -> Result<()> {
        if parent >= self.nodes.len() || child >= self.nodes.len() {
            return Err(Error::Dom("Invalid node ID".to_string()));
        }
        let removed = match &mut self.nodes[parent] {
            Some(Node::Element(el)) => {
                let pos = el
                    .children
                    .iter()
                    .position(|&c| c == child)
                    .ok_or_else(|| Error::Dom("Child not found in parent".to_string()))?;
                el.children.remove(pos)
            }
            Some(Node::Document(doc)) => {
                let pos = doc
                    .children
                    .iter()
                    .position(|&c| c == child)
                    .ok_or_else(|| Error::Dom("Child not found in parent".to_string()))?;
                doc.children.remove(pos)
            }
            _ => return Err(Error::Dom("Parent cannot have children".to_string())),
        };
        debug_assert_eq!(removed, child);
        if let Some(node) = &mut self.nodes[child] {
            match node {
                Node::Element(el) => el.parent = None,
                Node::Text(t) => t.parent = None,
                Node::Comment(c) => c.parent = None,
                Node::Document(_) => {}
            }
        }
        Ok(())
    }

    /// Returns an iterator over all descendants of `id` in document order.
    #[must_use]
    pub fn descendants(&self, id: NodeId) -> Descendants<'_> {
        Descendants {
            dom: self,
            stack: vec![(id, 0)],
        }
    }

    /// Returns an iterator over the ancestors of `id`, starting with `id`
    /// itself.
    #[must_use]
    pub fn ancestors(&self, id: NodeId) -> Ancestors<'_> {
        Ancestors {
            dom: self,
            current: Some(id),
        }
    }

    /// Returns the total number of nodes (including `None` tombstones).
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

/// A `(NodeId, depth)` pair used by the tree-walker iterators.
pub type NodeDepth = (NodeId, usize);

/// Depth-first pre-order iterator over all descendants of a node.
pub struct Descendants<'a> {
    dom: &'a Dom,
    stack: Vec<NodeDepth>,
}

impl<'a> Iterator for Descendants<'a> {
    type Item = NodeDepth;
    fn next(&mut self) -> Option<Self::Item> {
        let (id, depth) = self.stack.pop()?;
        if let Some(children) = self.dom.get_children(id) {
            for child in children.into_iter().rev() {
                self.stack.push((child, depth + 1));
            }
        }
        Some((id, depth))
    }
}

/// Iterator over the ancestors of a node (starting with the node itself).
pub struct Ancestors<'a> {
    dom: &'a Dom,
    current: Option<NodeId>,
}

impl<'a> Iterator for Ancestors<'a> {
    type Item = NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        let id = self.current?;
        self.current = self.dom.get_parent(id);
        Some(id)
    }
}

impl Default for Dom {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_dom() {
        let dom = Dom::new();
        assert_eq!(dom.root, 0);
        assert!(dom.get_node(0).is_some());
    }

    #[test]
    fn test_create_element() {
        let mut dom = Dom::new();
        let div = dom.create_element("div");
        assert_eq!(dom.get_tag(div), Some("div"));
    }

    #[test]
    fn test_append_child() {
        let mut dom = Dom::new();
        let div = dom.create_element("div");
        let text = dom.create_text("Hello");
        dom.append_child(div, text).unwrap();
        let children = dom.get_children(div).unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0], text);
    }

    #[test]
    fn test_parent_relationship() {
        let mut dom = Dom::new();
        let div = dom.create_element("div");
        let span = dom.create_element("span");
        dom.append_child(div, span).unwrap();
        assert_eq!(dom.get_parent(span), Some(div));
    }

    #[test]
    fn test_set_attribute() {
        let mut dom = Dom::new();
        let div = dom.create_element("div");
        dom.set_attribute(div, "id", "main").unwrap();
        dom.set_attribute(div, "class", "container").unwrap();
        let attrs = dom.get_attributes(div).unwrap();
        assert_eq!(attrs.len(), 2);
        assert_eq!(attrs[0], ("id".to_string(), "main".to_string()));
        assert_eq!(attrs[1], ("class".to_string(), "container".to_string()));
    }

    #[test]
    fn test_set_attribute_overwrite() {
        let mut dom = Dom::new();
        let div = dom.create_element("div");
        dom.set_attribute(div, "id", "old").unwrap();
        dom.set_attribute(div, "id", "new").unwrap();
        let attrs = dom.get_attributes(div).unwrap();
        assert_eq!(attrs.len(), 1);
        assert_eq!(attrs[0].1, "new");
    }

    #[test]
    fn test_set_attribute_non_element_errors() {
        let mut dom = Dom::new();
        let text = dom.create_text("hello");
        assert!(dom.set_attribute(text, "id", "bad").is_err());
    }

    #[test]
    fn test_remove_child() {
        let mut dom = Dom::new();
        let div = dom.create_element("div");
        let span = dom.create_element("span");
        dom.append_child(div, span).unwrap();
        assert_eq!(dom.get_children(div).unwrap().len(), 1);
        dom.remove_child(div, span).unwrap();
        assert_eq!(dom.get_children(div).unwrap().len(), 0);
        assert_eq!(dom.get_parent(span), None);
    }

    #[test]
    fn test_remove_child_not_found_errors() {
        let mut dom = Dom::new();
        let div = dom.create_element("div");
        let span = dom.create_element("span");
        assert!(dom.remove_child(div, span).is_err());
    }

    #[test]
    fn test_descendants_iterator() {
        let mut dom = Dom::new();
        let root = dom.root;
        let div = dom.create_element("div");
        let span = dom.create_element("span");
        let text = dom.create_text("hello");
        dom.append_child(root, div).unwrap();
        dom.append_child(div, span).unwrap();
        dom.append_child(span, text).unwrap();
        let nodes: Vec<NodeId> = dom.descendants(root).map(|(id, _)| id).collect();
        assert_eq!(nodes, vec![root, div, span, text]);
    }

    #[test]
    fn test_descendants_depths() {
        let mut dom = Dom::new();
        let root = dom.root;
        let div = dom.create_element("div");
        let span = dom.create_element("span");
        dom.append_child(root, div).unwrap();
        dom.append_child(div, span).unwrap();
        let depths: Vec<usize> = dom.descendants(root).map(|(_, d)| d).collect();
        assert_eq!(depths, vec![0, 1, 2]);
    }

    #[test]
    fn test_ancestors_iterator() {
        let mut dom = Dom::new();
        let root = dom.root;
        let div = dom.create_element("div");
        let span = dom.create_element("span");
        dom.append_child(root, div).unwrap();
        dom.append_child(div, span).unwrap();
        let ancestors: Vec<NodeId> = dom.ancestors(span).collect();
        assert_eq!(ancestors, vec![span, div, root]);
    }

    #[test]
    fn test_node_count() {
        let mut dom = Dom::new();
        assert_eq!(dom.node_count(), 1); // document root
        let div = dom.create_element("div");
        dom.append_child(dom.root, div).unwrap();
        assert_eq!(dom.node_count(), 2);
    }
}
