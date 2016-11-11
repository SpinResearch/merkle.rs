use crypto::digest::Digest;

use hashable::{ Hashable };
use merkledigest::{ MerkleDigest };

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

impl <T> Tree<T> where T: Hashable {
    /// Create a new tree
    pub fn new(hash: Vec<u8>, value: T) -> Self {
        Tree::Leaf {
            hash: hash,
            value: value
        }
    }

    /// Returns a hash from the tree.
    pub fn get_hash(&self) -> &Vec<u8> {
        match *self {
            Tree::Leaf { ref hash, value: _ }          => hash,
            Tree::Node { ref hash, left: _, right: _ } => hash
        }
    }

    /// Create a new leaf
    pub fn make_leaf<D: Digest>(digest: &mut D, value: T) -> Tree<T> {
        let hash = digest.hash_bytes(&value.to_bytes());
        Tree::new(hash, value)
    }
}
