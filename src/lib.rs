
//! *merkle* implements a Merkle Tree in Rust.

extern crate crypto;

mod merkletree;
pub use merkletree::{ MerkleTree };

mod proof;
pub use proof::{ Proof, ProofBlock, Positioned };

mod merkledigest;
pub use merkledigest::{ MerkleDigest };

mod tree;

#[cfg(test)]
mod tests;

