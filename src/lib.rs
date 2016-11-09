
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
            Tree::Leaf { hash: ref hash, value: _ }          => hash,
            Tree::Node { hash: ref hash, left: _, right: _ } => hash
        }
    }
}

pub struct MerkleTree<D, T> where D: Digest + Clone, T: Hashable + Clone {
    digest: D,
    tree: Tree<T>
}

impl <T> Tree<T> where T: Hashable + Clone {

    // fn from_chunks<D>(digest: D, iter: Chunks<Tree<T>>) -> Tree<T> where D: Digest {

    // }

}

fn make_hash<D, T>(mut digest: D, bytes: &[u8]) -> Vec<u8> where D: Digest, T: Hashable + Clone {
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

fn make_leaf<D, T>(digest: D, value: T) -> Tree<T> where D: Digest + Clone, T: Hashable + Clone {
    return Tree::Leaf {
        hash: make_hash::<D, T>(digest, value.to_bytes()),
        value: value
    };
}

impl <D, T> MerkleTree<D, T> where D: Digest + Clone, T: Hashable + Clone {

    // TODO: Don't clone all the things
    pub fn from_vec(digest: D, values: Vec<T>) -> MerkleTree<D, T> {
        let leafs: Vec<Tree<T>> = values.iter().map(|v| make_leaf::<D, T>(digest.clone(), v.clone())).collect();

        for chunk in leafs.chunks(2) {
            if chunk.len() == 1 {

            } else {
                let left: Tree<T>  = chunk[0].clone();
                let right: Tree<T> = chunk[1].clone();

                let combined_hash = combine_hashes::<D>(digest.clone(), left.get_hash().as_slice(), right.get_hash().as_slice());
                let node = Tree::Node {
                   hash: combined_hash,
                   left: Box::new(left),
                   right: Box::new(right)
                };
            }
        }

        return MerkleTree {
            digest: digest.clone(),
            tree: make_leaf::<D, T>(digest, values[0].clone())
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
