
pub extern crate crypto;

use crypto::digest::Digest;

pub trait Hashable {
    fn to_bytes(&self) -> &[u8];
}

impl Hashable for String {
    fn to_bytes(&self) -> &[u8] {
        return self.as_bytes();
    }
}

impl Hashable for str {
    fn to_bytes(&self) -> &[u8] {
        return self.as_bytes();
    }
}

#[derive(Clone)]
enum Tree<T> where T: Hashable + Clone {
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

impl <T> Tree<T> where T: Hashable + Clone {
    fn get_hash<'a>(&'a self) -> &'a Vec<u8> {
        match *self {
            Tree::Leaf { ref hash, value: _ }          => hash,
            Tree::Node { ref hash, left: _, right: _ } => hash
        }
    }
}

pub struct MerkleTree<D, T> where D: Digest + Clone, T: Hashable + Clone {
    digest: D,
    tree: Tree<T>
}

impl <T> Tree<T> where T: Hashable + Clone {

}

fn make_hash<D, T>(mut digest: D, bytes: &[u8]) -> Vec<u8> where D: Digest {
    let mut hash = Vec::with_capacity(digest.output_bytes());
    digest.reset();
    digest.input(bytes);
    digest.result(hash.as_mut_slice());
    digest.reset();
    return hash;
}

fn combine_hashes<D>(mut digest: D, left: &[u8], right: &[u8]) -> Vec<u8> where D: Digest {
    let mut hash = Vec::with_capacity(digest.output_bytes());
    digest.reset();
    digest.input(left);
    digest.input(right);
    digest.result(hash.as_mut_slice());
    digest.reset();
    return hash;
}

fn make_leaf<D, T>(digest: D, value: T) -> Tree<T> where D: Digest, T: Hashable + Clone {
    return Tree::Leaf {
        hash: make_hash::<D, T>(digest, value.to_bytes()),
        value: value
    };
}

impl <D, T> MerkleTree<D, T> where D: Digest + Clone, T: Hashable + Clone {

    // TODO: Don't clone all the things
    pub fn from_vec(digest: D, values: Vec<T>) -> MerkleTree<D, T> {
        if values.is_empty() {
            panic!("Cannot build a Merkle tree from an empty vector.");
        }

        let mut cur: Vec<_> =
            values.iter()
                  .map(|v| make_leaf::<D, T>(digest.clone(), v.clone()))
                  .collect();

        while cur.len() > 1 {
            cur = cur.chunks(2).map(|chunk|
                if chunk.len() == 1 {
                    chunk[0].clone()
                } else {
                    let left:  Tree<T> = chunk[0].clone();
                    let right: Tree<T> = chunk[1].clone();

                    let combined_hash = combine_hashes::<D>(
                        digest.clone(),
                        left.get_hash().as_slice(),
                        right.get_hash().as_slice()
                    );

                    Tree::Node {
                       hash: combined_hash,
                       left: Box::new(left),
                       right: Box::new(right)
                    }
                }
            ).collect();
        }

        return MerkleTree {
            digest: digest.clone(),
            tree: cur[0].clone()
        };

    }

}

#[cfg(test)]
use crypto::sha3::Sha3;

#[test]
fn test_stuff() {
}

#[cfg(test)]
pub mod tests;
