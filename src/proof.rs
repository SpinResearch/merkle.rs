
use crypto::digest::Digest;

use merkledigest::{ MerkleDigest };
use merkletree::{ MerkleTree };
use tree::{ Tree };

pub struct Proof<D, T> {
    pub digest: D,
    pub root_hash: Vec<u8>,
    pub block: ProofBlock,
    pub value: T
}

impl <D, T> Proof<D, T> where D: Digest + Clone, T: Into<Vec<u8>> + Clone {

    /// TODO: Refactor this mess (@romac)
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
                            return node.is_leaf(),

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

pub struct ProofBlock {
    pub node_hash: Vec<u8>,
    pub sibling_hash: Positioned<Vec<u8>>,
    pub sub_proof: Option<Box<ProofBlock>>
}

impl ProofBlock {

    pub fn new<T>(tree: &Tree<T>, needle: &Vec<u8>) -> Option<ProofBlock> where T: Into<Vec<u8>> + Clone {
        match *tree {
            Tree::Leaf { ref hash, value: _ } =>
                if hash == needle {
                    Some(ProofBlock {
                        node_hash: hash.clone(),
                        sibling_hash: Positioned::Nowhere,
                        sub_proof: None
                    })
                }
                else {
                    None
                },

            Tree::Node { ref hash, ref left, ref right } =>
                if hash == needle {
                    Some(ProofBlock {
                        node_hash: hash.clone(),
                        sibling_hash: Positioned::Nowhere,
                        sub_proof: None
                    })
                }
                else {
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
    }

}

pub enum Positioned<T> {
    Nowhere,
    Left(T),
    Right(T)
}

