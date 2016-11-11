extern crate crypto;
use crypto::digest::Digest;

pub trait Hashable {
    fn to_bytes(&self) -> Vec<u8>;
}

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

    pub fn new(hash: Vec<u8>, value: T) -> Self {
        Tree::Leaf {
            hash: hash,
            value: value
        }
    }

    pub fn get_hash(&self) -> &Vec<u8> {
        match *self {
            Tree::Leaf { ref hash, value: _ }          => hash,
            Tree::Node { ref hash, left: _, right: _ } => hash
        }
    }

    pub fn make_leaf<D: Digest>(digest: &mut D, value: T) -> Tree<T> {
        let hash = digest.hash_bytes(&value.to_bytes());
        Tree::new(hash, value)
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
