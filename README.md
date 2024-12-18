# Prefix Tree (Trie) Implementation in Rust

A lightweight and efficient implementation of a Trie (prefix tree) data structure in Rust. This library provides a way to store and retrieve strings based on their prefixes, making it ideal for applications like autocomplete, spell checkers, and IP routing tables.

## Features

- Fast prefix-based string operations
- Memory-efficient storage of strings with common prefixes
- Simple and intuitive API
- Fully documented with examples
- Comprehensive test coverage

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
prefix_tree_rs = "0.1.0"
```

## Usage

### Basic Example

```rust
use prefix_tree_rs::Trie;

fn main() {
    // Create a new Trie
    let mut trie = Trie::new();

    // Insert some words
    trie.insert("hello");
    trie.insert("world");
    trie.insert("helm");

    // Search for words
    assert!(trie.search("hello"));     // true
    assert!(!trie.search("hell"));     // false

    // Check prefixes
    assert!(trie.starts_with("he"));   // true
    assert!(!trie.starts_with("wo")); // true
}
```

### API Reference

#### `Trie::new()`
Creates a new, empty Trie.

#### `Trie::insert(&mut self, word: &str)`
Inserts a word into the Trie.

#### `Trie::search(&self, word: &str) -> bool`
Checks if a word exists in the Trie.

#### `Trie::starts_with(&self, prefix: &str) -> bool`
Checks if there is any word in the Trie that starts with the given prefix.

## Implementation Details

The Trie is implemented using two main structures:

- `Trie`: The main structure containing the root node
- `TrieNode`: Individual nodes containing:
  - A boolean flag indicating if it's the end of a word
  - A HashMap of child nodes mapped by characters

## Performance

- Time Complexity:
  - Insertion: O(m) where m is the length of the word
  - Search: O(m) where m is the length of the word
  - Prefix checking: O(p) where p is the length of the prefix

- Space Complexity:
  - O(ALPHABET_SIZE * N * W) where N is the number of words and W is the average word length

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.