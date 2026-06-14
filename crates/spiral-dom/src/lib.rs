//! Spiral Browser — DOM Tree
//!
//! DOM tree representation for the Spiral Browser.

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
    /// Create a new empty DOM.
    pub fn new() -> Self {
        let doc = Document {
            children: Vec::new(),
            quirks_mode: false,
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
    pub fn append_child(&mut self, parent: NodeId, child: NodeId) -> Result<(), String> {
        if parent >= self.nodes.len() || child >= self.nodes.len() {
            return Err("Invalid node ID".to_string());
        }
        match &mut self.nodes[parent] {
            Some(Node::Element(el)) => {
                el.children.push(child);
            }
            Some(Node::Document(doc)) => {
                doc.children.push(child);
            }
            _ => {
                return Err("Parent cannot have children".to_string());
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

    /// Get a node by ID.
    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id).and_then(|n| n.as_ref())
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
}
