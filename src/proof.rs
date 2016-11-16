
use crypto::digest::Digest;

use merkledigest::{ MerkleDigest };
use tree::{ Tree };

/// An inclusion proof represent the fact that `value` is a member of a `MerkleTree`
/// with root hash `root_hash`, and hash function `digest`.
/// A proof is a linked-list of `ProofBlock`s.
/// TODO: Represent a proof as a vector of ProofBlock instead of a linked-list?
pub struct Proof<D, T> {
    pub digest: D,
    pub root_hash: Vec<u8>,
    pub block: ProofBlock,
    pub value: T
}

impl <D, T> Proof<D, T> where D: Digest + Clone, T: Into<Vec<u8>> + Clone {

    pub fn validate(&self, root_hash: &Vec<u8>) -> bool {
        if self.root_hash != *root_hash || self.block.node_hash != *root_hash {
            return false
        }

        self.validate_block(&self.block, &mut self.digest.clone())
    }

    pub fn validate_block(&self, block: &ProofBlock, digest: &mut D) -> bool {
        match block.sub_proof {

            None =>
                block.sibling_hash == Positioned::Nowhere,

            Some(ref sub) =>
                match block.sibling_hash {
                    Positioned::Nowhere =>
                        false,

                    Positioned::Left(ref hash) => {
                        let hashes_match = digest.combine_hashes(&hash, &sub.node_hash) == block.node_hash;
                        hashes_match && self.validate_block(sub, digest)
                    }

                    Positioned::Right(ref hash) => {
                        let hashes_match = digest.combine_hashes(&sub.node_hash, &hash) == block.node_hash;
                        hashes_match && self.validate_block(sub, digest)
                    }

                }
        }
    }

}


/// A `ProofBlock` is a linked-list holding the hash of the node, the hash of its sibling node,
/// and the rest of the inclusion proof.
pub struct ProofBlock {
    pub node_hash: Vec<u8>,
    pub sibling_hash: Positioned<Vec<u8>>,
    pub sub_proof: Option<Box<ProofBlock>>
}

impl ProofBlock {

    /// Attempt to generate a proof that the hash `needle` is a member of the given `tree`.
    pub fn new<T>(tree: &Tree<T>, needle: &Vec<u8>) -> Option<ProofBlock>
        where T: Into<Vec<u8>> + Clone
    {
        match *tree {
            Tree::Leaf { ref hash, .. } =>
                ProofBlock::new_leaf_proof(hash, needle),

            Tree::Node { ref hash, ref left, ref right } =>
                ProofBlock::new_tree_proof(hash, needle, left, right)
        }
    }

    fn new_leaf_proof(hash: &Vec<u8>, needle: &Vec<u8>) -> Option<ProofBlock> {
        if *hash == *needle {
            Some(ProofBlock {
                node_hash: hash.clone(),
                sibling_hash: Positioned::Nowhere,
                sub_proof: None
            })
        } else {
            None
        }
    }

    fn new_tree_proof<T>(hash: &Vec<u8>, needle: &Vec<u8>, left: &Tree<T>, right: &Tree<T>) -> Option<ProofBlock>
        where T: Into<Vec<u8>> + Clone
    {
        ProofBlock::new(left, needle)
            .map(|block| {
                let right_hash = right.get_hash().clone();
                let sub_proof = Positioned::Right(right_hash);
                (block, sub_proof)
            })
            .or_else(|| {
                let sub_proof = ProofBlock::new(right, needle);
                sub_proof.map(|block| {
                    let left_hash = left.get_hash().clone();
                    let sub_proof = Positioned::Left(left_hash);
                    (block, sub_proof)
                })
            })
            .map(|(sub_proof, sibling_hash)| {
                ProofBlock {
                    node_hash: hash.clone(),
                    sibling_hash: sibling_hash,
                    sub_proof: Some(Box::new(sub_proof))
                }
            })
    }

}

/// Tags a value so that we know from in branch (if any) it was found.
#[derive(PartialEq)]
pub enum Positioned<T> {

    /// No value was found
    Nowhere,

    /// The value was found in the left branch
    Left(T),

    /// The value was found in the right branch
    Right(T)
}

