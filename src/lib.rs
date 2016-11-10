
#![allow(dead_code)]

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

fn make_hash<D, T>(digest: &mut D, bytes: &[u8]) -> Vec<u8> where D: Digest {
    let mut hash = vec![0; digest.output_bytes()];

    digest.reset();
    digest.input(bytes);
    digest.result(&mut hash);
    digest.reset();

    hash
}

fn combine_hashes<D>(digest: &mut D, left: &[u8], right: &[u8]) -> Vec<u8> where D: Digest {
    let mut hash = vec![0; digest.output_bytes()];

    digest.reset();
    digest.input(left);
    digest.input(right);
    digest.result(&mut hash);
    digest.reset();

    hash
}

enum Tree<T> {
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

    fn new(hash: Vec<u8>, value: T) -> Self {
        Tree::Leaf {
            hash: hash,
            value: value
        }
    }

    fn get_hash(&self) -> &Vec<u8> {
        match *self {
            Tree::Leaf { ref hash, value: _ }          => hash,
            Tree::Node { ref hash, left: _, right: _ } => hash
        }
    }
}

fn make_leaf<D, T>(digest: &mut D, value: T) -> Tree<T> where D: Digest, T: Hashable {
    let hash = make_hash::<D, T>(digest, value.to_bytes());
    Tree::new(hash, value)
}

pub struct MerkleTree<D, T> where D: Digest, T: Hashable {
    digest: D,
    tree: Tree<T>
}

impl <D, T> MerkleTree<D, T> where D: Digest, T: Hashable {

    pub fn from_vec(mut digest: D, values: Vec<T>) -> Self {
        if values.is_empty() {
            panic!("Cannot build a Merkle tree from an empty vector.");
        }

        let len = values.len();
        let mut cur = Vec::with_capacity(len);

        for v in values {
            let leaf = make_leaf(&mut digest, v);
            cur.push(leaf);
        }

        cur.reverse();

        while cur.len() > 1 {
            let mut next = Vec::with_capacity(len);

            while cur.len() > 0 {
                if cur.len() == 1 {
                    next.push(cur.pop().unwrap());
                }
                else {
                    let right = cur.pop().unwrap();
                    let left  = cur.pop().unwrap();

                    let combined_hash = combine_hashes::<D>(
                        &mut digest,
                        left.get_hash().as_slice(),
                        right.get_hash().as_slice()
                    );

                    let node = Tree::Node {
                       hash: combined_hash,
                       left: Box::new(left),
                       right: Box::new(right)
                    };

                    next.push(node);
                }
            }

            cur = next;
        }

        let tree = cur.remove(0);

        MerkleTree {
            digest: digest,
            tree: tree
        }
    }

}

#[cfg(test)]
use crypto::sha3::Sha3;

#[test]
fn test_stuff() {
}

#[cfg(test)]
pub mod tests;
