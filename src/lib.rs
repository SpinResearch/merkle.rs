#![deny(
    missing_docs,
    unused_qualifications,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces
)]

//! *merkle* implements a Merkle Tree in Rust.

extern crate ring;

#[cfg(feature = "serialization-protobuf")]
extern crate protobuf;

#[cfg(feature = "serialization-serde")]
extern crate serde;
#[cfg(feature = "serialization-serde")]
#[macro_use]
extern crate serde_derive;

mod merkletree;
pub use crate::merkletree::MerkleTree;

mod proof;
pub use crate::proof::Proof;

mod hashutils;
pub use crate::hashutils::Hashable;

mod tree;
pub use crate::tree::{LeavesIntoIterator, LeavesIterator};

#[cfg(feature = "serialization-protobuf")]
#[allow(unused_qualifications)]
mod proto;

#[cfg(test)]
mod tests;
