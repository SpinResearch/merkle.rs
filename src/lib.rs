
pub extern crate crypto;

use std::marker::PhantomData;
use crypto::digest::Digest;

pub trait Hashable {
    fn hash(&self) -> &[u8];
}

impl Hashable for String {
    fn hash(&self) -> &[u8] {
        return self.as_bytes();
    }
}

impl Hashable for str {
    fn hash(&self) -> &[u8] {
        return self.as_bytes();
    }
}

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

pub struct MerkleTree<D, T> where D: Digest + Clone, T: Hashable + Clone {
    digest: D,
    tree: Tree<T>
}

impl <D, T> MerkleTree<D, T> where D: Digest + Clone, T: Hashable + Clone {

    pub fn from_vec(values: Vec<T>, mut digest: D) -> MerkleTree<D, T> {
        return MerkleTree::new(digest, Leaf { hash: Vec::new(), value: values[0].clone });
        // let value = values[0].clone();
        // let bytes = value.hash();
        // let mut hash = vec![0; digest.output_bytes()];
        // digest.input(bytes);
        // digest.result(hash.as_mut_slice());
        // digest.reset();

        // return Tree::Leaf {
        //     hash: hash,
        //     value: values[0].clone(),
        //     _marker: PhantomData
        // };
    }

}

#[cfg(test)]
use crypto::sha3::Sha3;

#[test]
fn test_stuff() {
}

#[cfg(test)]
pub mod tests;

