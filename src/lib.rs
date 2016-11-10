
#![allow(dead_code)]

pub extern crate crypto;

use crypto::digest::Digest;

pub trait Hashable {
    fn to_bytes(&self) -> Vec<u8>;
}

impl Hashable for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.clone().into_bytes()
    }
}

impl Hashable for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

pub trait MerkleDigest {
    fn hash_bytes(&mut self, bytes: &Vec<u8>) -> Vec<u8>;
    fn combine_hashes(&mut self, left: &Vec<u8>, right: &Vec<u8>) -> Vec<u8>;
}

impl <D> MerkleDigest for D where D: Digest {

    fn hash_bytes(&mut self, bytes: &Vec<u8>) -> Vec<u8> {
        let mut hash = vec![0; self.output_bytes()];

        self.reset();
        self.input(bytes);
        self.result(&mut hash);
        self.reset();

        hash
    }

    fn combine_hashes(&mut self, left: &Vec<u8>, right: &Vec<u8>) -> Vec<u8> {
        let mut hash = vec![0; self.output_bytes()];

        self.reset();
        self.input(left);
        self.input(right);
        self.result(&mut hash);
        self.reset();

        hash
    }

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
    let hash = digest.hash_bytes(&value.to_bytes());
    Tree::new(hash, value)
}

pub struct MerkleTree<D, T> where D: Digest, T: Hashable {
    digest: D,
    tree: Tree<T>,
    height: usize,
    count: usize
}

impl <D, T> MerkleTree<D, T> where D: Digest, T: Hashable {

    pub fn from_vec(mut digest: D, values: Vec<T>) -> Self {
        if values.is_empty() {
            panic!("Cannot build a Merkle tree from an empty vector.");
        }

        let count  = values.len();
        let height = (count as f32 + 1.0).log2().ceil() as usize;

        let mut cur = Vec::with_capacity(count);

        for v in values.into_iter().rev() {
            let leaf = make_leaf(&mut digest, v);
            cur.push(leaf);
        }

        for i in 0 .. height {
            let mut next = Vec::with_capacity(height / 2 << i);

            while cur.len() > 0 {
                if cur.len() == 1 {
                    next.push(cur.pop().unwrap());
                }
                else {
                    let right = cur.pop().unwrap();
                    let left  = cur.pop().unwrap();

                    let combined_hash = digest.combine_hashes(
                        left.get_hash(),
                        right.get_hash()
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

        assert!(cur.len() == 1);

        let tree = cur.remove(0);

        MerkleTree {
            digest: digest,
            tree: tree,
            height: height,
            count: count
        }
    }

    fn root_hash(&self) -> &Vec<u8> {
        self.tree.get_hash()
    }

}

#[cfg(test)]
mod tests;

