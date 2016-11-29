#![deny(
    missing_docs,
    missing_debug_implementations, missing_copy_implementations,
    trivial_casts, trivial_numeric_casts,
    unsafe_code, unstable_features,
    unused_import_braces, unused_qualifications
)]

//! *merkle* implements a Merkle Tree in Rust.

extern crate ring;

#[cfg(feature = "serialization-protobuf")]
extern crate protobuf;

mod merkletree;
pub use merkletree::MerkleTree;

mod proof;
pub use proof::Proof;

mod hashutils;

mod tree;

#[cfg(feature = "serialization-protobuf")]
mod proto;

#[cfg(test)]
mod tests;

