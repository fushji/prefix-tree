//! A Trie (prefix tree) implementation in Rust.
//!
//! This data structure supports efficient storage and retrieval of strings
//! based on their prefixes. It provides operations to insert words, check if
//! a word exists, and check if any words share a given prefix.
//!
//! # Examples
//!
//! ```
//! use prefix_tree::Trie;
//!
//! let mut trie = Trie::new();
//! trie.insert("hello");
//! trie.insert("world");
//!
//! assert!(trie.search("hello"));
//! assert!(!trie.search("hell"));
//! assert!(trie.starts_with("he"));
//! ```

use std::collections::HashMap;

/// A single node in the Trie.
///
/// Each node represents a character and may have child nodes or mark
/// the end of a word.
#[derive(Debug, Default)]
pub struct TrieNode {
    /// Indicates if this node marks the end of a word.
    pub is_end_of_word: bool,
    /// Children nodes mapped by characters.
    pub children: HashMap<char, TrieNode>,
}

/// A Trie data structure for managing strings.
///
/// The Trie supports efficient insertion, search, and prefix checking.
#[derive(Debug, Default)]
pub struct Trie {
    /// The root node of the Trie.
    pub root: TrieNode,
}

impl Trie {
    /// Creates a new, empty Trie.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::Trie;
    ///
    /// let trie = Trie::new();
    /// ```
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    /// Inserts a word into the Trie.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::Trie;
    ///
    /// let mut trie = Trie::new();
    /// trie.insert("hello");
    /// trie.insert("world");
    /// ```
    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for ch in word.chars() {
            current = current.children.entry(ch).or_default();
        }
        current.is_end_of_word = true;
    }

    /// Checks if a word exists in the Trie.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::Trie;
    ///
    /// let mut trie = Trie::new();
    /// trie.insert("hello");
    /// assert!(trie.search("hello"));
    /// assert!(!trie.search("hell"));
    /// ```
    pub fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        for ch in word.chars() {
            match current.children.get(&ch) {
                Some(node) => current = node,
                None => return false,
            }
        }
        current.is_end_of_word
    }

    /// Checks if there is any word in the Trie that starts with the given prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::Trie;
    ///
    /// let mut trie = Trie::new();
    /// trie.insert("hello");
    /// assert!(trie.starts_with("he"));
    /// assert!(!trie.starts_with("hero"));
    /// ```
    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut current = &self.root;
        for ch in prefix.chars() {
            match current.children.get(&ch) {
                Some(node) => current = node,
                None => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("world");

        assert!(trie.search("hello"));
        assert!(trie.search("world"));
        assert!(!trie.search("hell"));
        assert!(!trie.search("worlds"));
    }

    #[test]
    fn test_starts_with() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("helium");

        assert!(trie.starts_with("he"));
        assert!(trie.starts_with("hel"));
        assert!(trie.starts_with("hello"));
        assert!(!trie.starts_with("hero"));
    }
}
