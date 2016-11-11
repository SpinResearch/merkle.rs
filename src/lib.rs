
#![allow(dead_code)]

pub extern crate crypto;
use crypto::digest::Digest;

mod tree;
pub use tree::{
    Tree,
    Hashable,
    MerkleDigest,
};

mod merkletree;
pub use merkletree::{
    MerkleTree
};

#[cfg(test)]
mod tests;
