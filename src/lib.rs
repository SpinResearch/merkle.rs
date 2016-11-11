
//! *merkle* implements a Merkle Tree in Rust.

#[doc(no_inline)]
pub extern crate crypto;

mod tree;
pub use tree::{ Tree };

mod merkletree;
pub use merkletree::{ MerkleTree };

mod merkledigest;
pub use merkledigest::{ MerkleDigest };

mod proof;
pub use proof::{ Proof, ProofBlock };

#[cfg(test)]
mod tests;

