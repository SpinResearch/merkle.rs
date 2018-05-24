#![deny(missing_docs, unused_qualifications, missing_debug_implementations,
        missing_copy_implementations, trivial_casts, trivial_numeric_casts, unsafe_code,
        unstable_features, unused_import_braces)]

//! *merkle* implements a Merkle Tree in Rust.

#[macro_use]
extern crate cfg_if;
extern crate ring;

cfg_if! {
    if #[cfg(feature = "serialization-protobuf")] {
        extern crate protobuf;
        #[allow(unused_qualifications)]
        mod proto;
    }
}

cfg_if! {
    if #[cfg(feature = "serialization-serde")] {
        extern crate serde;
        #[macro_use]
        extern crate serde_derive;
    }
}

mod merkletree;
pub use merkletree::MerkleTree;

mod proof;
pub use proof::Proof;

mod hashutils;
pub use hashutils::Hashable;

mod tree;
pub use tree::{LeavesIntoIterator, LeavesIterator};

#[cfg(test)]
mod tests;
