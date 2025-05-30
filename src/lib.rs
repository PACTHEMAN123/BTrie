//! Bitwise trie tree implementation for sparse array.
//! Adapt from Lunaix-os
//! Implement from C to Rust
#![allow(missing_docs)]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::option::Option;
use core::option::Option::None;
use core::option::Option::Some;
use core::cmp::Ord;
use core::result::Result::Ok;
use alloc::boxed::Box;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
