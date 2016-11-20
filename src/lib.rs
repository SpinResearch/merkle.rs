#![warn(missing_docs)]

//! *merkle* implements a Merkle Tree in Rust.

extern crate crypto;

mod merkletree;
pub use merkletree::MerkleTree;

mod proof;
pub use proof::Proof;

mod merkledigest;

mod tree;

#[cfg(test)]
mod tests;

