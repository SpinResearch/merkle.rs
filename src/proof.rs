
use crypto::digest::Digest;

use merkledigest::{ MerkleDigest };
use merkletree::{ MerkleTree };
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

    // TODO: Refactor this mess (@romac)
    pub fn validate_against(&self, tree: &mut MerkleTree<D, T>) -> bool {
        if &self.root_hash != tree.root_hash() {
            return false;
        }

        let mut block = &self.block;
        let mut node  = &tree.tree;

        loop {
            if &block.node_hash != node.get_hash() {
                return false;
            }

            match *node {
                Tree::Leaf { .. } =>
                    match block.sibling_hash {
                        Positioned::Nowhere => return true,
                        _                   => return false
                    },

                Tree::Node { ref hash, ref left, ref right } =>
                    match block.sibling_hash {
                        Positioned::Nowhere =>
                            return false,

                        Positioned::Left(ref left_hash) => {
                            if left_hash != left.get_hash() {
                                return false
                            }

                            match block.sub_proof {
                                Some(ref sub_proof) => {
                                    let combined_hash = tree.digest.combine_hashes(left_hash, &sub_proof.node_hash);

                                    if &combined_hash != hash {
                                        return false
                                    }

                                    node  = right;
                                    block = sub_proof;
                                }

                                None =>
                                    return false
                            }
                        },

                        Positioned::Right(ref right_hash) => {
                            if right_hash != right.get_hash() {
                                return false
                            }

                            match block.sub_proof {
                                Some(ref sub_proof) => {
                                    let combined_hash = tree.digest.combine_hashes(&sub_proof.node_hash, right_hash);

                                    if &combined_hash != hash {
                                        return false
                                    }

                                    node  = left;
                                    block = sub_proof;
                                }

                                None =>
                                    return false
                            }
                        }
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
pub enum Positioned<T> {

    /// No value was found
    Nowhere,

    /// The value was found in the left branch
    Left(T),

    /// The value was found in the right branch
    Right(T)
}

