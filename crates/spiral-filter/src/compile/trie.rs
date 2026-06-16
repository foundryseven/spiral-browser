//! Hostname trie for fast hostname-anchored rule lookup.
//!
//! A trie where each node is a hostname label (split on `.`).
//! Lookup is O(labels) which is typically O(5–6).

/// A node in the hostname trie.
#[derive(Debug, Default)]
pub struct TrieNode {
    /// Children keyed by label.
    children: std::collections::HashMap<String, TrieNode>,
    /// Whether this node represents a complete hostname.
    is_match: bool,
}

/// Hostname trie for fast lookup.
#[derive(Debug, Default)]
pub struct HostnameTrie {
    root: TrieNode,
}

impl HostnameTrie {
    /// Create a new empty trie.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a hostname into the trie.
    pub fn insert(&mut self, hostname: &str) {
        let mut node = &mut self.root;
        for label in hostname.rsplit('.') {
            node = node.children.entry(label.to_ascii_lowercase()).or_default();
        }
        node.is_match = true;
    }

    /// Check if a hostname matches.
    #[must_use]
    pub fn contains(&self, hostname: &str) -> bool {
        let mut node = &self.root;
        for label in hostname.rsplit('.') {
            match node.children.get(&label.to_ascii_lowercase()) {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_match
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_contains() {
        let mut trie = HostnameTrie::new();
        trie.insert("example.com");
        assert!(trie.contains("example.com"));
        assert!(!trie.contains("other.com"));
    }

    #[test]
    fn contains_subdomain() {
        let mut trie = HostnameTrie::new();
        trie.insert("example.com");
        // Exact match only — subdomains don't match.
        assert!(!trie.contains("sub.example.com"));
    }

    #[test]
    fn empty_trie() {
        let trie = HostnameTrie::new();
        assert!(!trie.contains("example.com"));
    }

    #[test]
    fn case_insensitive() {
        let mut trie = HostnameTrie::new();
        trie.insert("Example.COM");
        assert!(trie.contains("example.com"));
        assert!(trie.contains("EXAMPLE.COM"));
    }
}
