#![allow(dead_code)]
#![warn(missing_docs)]

//! *merkle* implements a Merkle Tree in Rust.

extern crate crypto;

#[cfg(feature = "serialization-protobuf")]
extern crate protobuf;

mod merkletree;
pub use merkletree::MerkleTree;

mod proof;
pub use proof::Proof;

mod merkledigest;

mod tree;

#[cfg(feature = "serialization-protobuf")]
mod proto;

#[cfg(test)]
mod tests;

