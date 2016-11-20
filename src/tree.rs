use crypto::digest::Digest;

use merkledigest::{ MerkleDigest };

pub use proof::{
    Proof,
    Lemma,
    Positioned
};

/// Binary Tree where leaves hold a stand-alone value.
pub enum Tree<T> {
    Leaf {
        hash: Vec<u8>,
        value: T
    },

    Node {
        hash: Vec<u8>,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>
    }
}

impl <T> Tree<T> where T: Into<Vec<u8>> + Clone {

    /// Create a new tree
    pub fn new(hash: Vec<u8>, value: T) -> Self {
        Tree::Leaf {
            hash: hash,
            value: value
        }
    }

    /// Create a new leaf
    pub fn make_leaf<D: Digest>(digest: &mut D, value: T) -> Tree<T> {
        let hash = digest.hash_bytes(&value.clone().into());
        Tree::new(hash, value)
    }

    /// Returns a hash from the tree.
    pub fn hash(&self) -> &Vec<u8> {
        match *self {
            Tree::Leaf { ref hash, .. } => hash,
            Tree::Node { ref hash, .. } => hash
        }
    }

}

