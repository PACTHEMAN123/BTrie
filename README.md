# BTrie

⚠️ This repo is under construction.

> A Bitwise Trie Implementation in Rust

This data structure has been used in Chronix, we use it for indexing pages in page cache.

## Features

- Fast insertion and lookup operations
- Memory efficiency through adaptive node bursting
- Thread-safe implementation (optional)
- Detailed documentation with examples (todo)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
btrie = "0.1"
```


## Usage

```rust
use btrie::BTrie;

fn main() {
    let mut trie = BTrie::new();
    let page_offset1 = 0x12341000;
    let page_offset2 = 0x34890000;
    let v1 = 1;
    let v2 = 4;
    
    trie.insert(page_offset1, v1);
    trie.insert(page_offset2, v2);
    
    assert_eq!(trie.get(page_offset1), Some(&1));
}
```