
use std::marker::PhantomData;

use crypto::digest::Digest;

use tree::Tree;
use merkledigest::MerkleDigest;

/// An inclusion proof represent the fact that a `value` is a member
/// of a `MerkleTree` with root hash `root_hash`, and hash function `digest`.
pub struct Proof<D, T> {

    /// The hash function used in the original `MerkleTree`
    pub digest: D,

    /// The hash of the root of the original `MerkleTree`
    pub root_hash: Vec<u8>,

    /// The first `Lemma` of the `Proof`
    pub lemma: Lemma,

    _value_marker: PhantomData<T>
}

impl <D, T> Proof<D, T> {

    /// Constructs a new `Proof`
    pub fn new(digest: D, root_hash: Vec<u8>, lemma: Lemma) -> Self {
        Proof {
            digest: digest,
            root_hash: root_hash,
            lemma: lemma,
            _value_marker: PhantomData
        }
    }

    /// Checks whether this inclusion proof is well-formed,
    /// and whether its root hash matches the given `root_hash`.
    pub fn validate(&self, root_hash: &Vec<u8>) -> bool where D: Digest + Clone {
        if self.root_hash != *root_hash || self.lemma.node_hash != *root_hash {
            return false
        }

        self.validate_lemma(&self.lemma, &mut self.digest.clone())
    }

    fn validate_lemma(&self, lemma: &Lemma, digest: &mut D) -> bool where D: Digest {
        match lemma.sub_lemma {

            None =>
                lemma.sibling_hash.is_none(),

            Some(ref sub) =>
                match lemma.sibling_hash {
                    None =>
                        false,

                    Some(Positioned::Left(ref hash)) => {
                        let hashes_match = digest.combine_hashes(&hash, &sub.node_hash) == lemma.node_hash;
                        hashes_match && self.validate_lemma(sub, digest)
                    }

                    Some(Positioned::Right(ref hash)) => {
                        let hashes_match = digest.combine_hashes(&sub.node_hash, &hash) == lemma.node_hash;
                        hashes_match && self.validate_lemma(sub, digest)
                    }

                }
        }
    }

}


/// A `Lemma` holds the hash of a node, the hash of its sibling node,
/// and a sub lemma, whose `node_hash`, when combined with this `sibling_hash`
/// must be equal to this `node_hash`.
pub struct Lemma {
    pub node_hash: Vec<u8>,
    pub sibling_hash: Option<Positioned<Vec<u8>>>,
    pub sub_lemma: Option<Box<Lemma>>
}

impl Lemma {

    /// Attempts to generate a proof that the a value with hash `needle` is a member of the given `tree`.
    pub fn new<T>(tree: &Tree<T>, needle: &Vec<u8>) -> Option<Lemma> where T: Into<Vec<u8>> + Clone {
        match *tree {
            Tree::Leaf { ref hash, .. } =>
                Lemma::new_leaf_proof(hash, needle),

            Tree::Node { ref hash, ref left, ref right } =>
                Lemma::new_tree_proof(hash, needle, left, right)
        }
    }

    fn new_leaf_proof(hash: &Vec<u8>, needle: &Vec<u8>) -> Option<Lemma> {
        if *hash == *needle {
            Some(Lemma {
                node_hash: hash.clone(),
                sibling_hash: None,
                sub_lemma: None
            })
        } else {
            None
        }
    }

    fn new_tree_proof<T>(hash: &Vec<u8>, needle: &Vec<u8>, left: &Tree<T>, right: &Tree<T>) -> Option<Lemma> where T: Into<Vec<u8>> + Clone {
        Lemma::new(left, needle)
            .map(|lemma| {
                let right_hash = right.hash().clone();
                let sub_lemma = Some(Positioned::Right(right_hash));
                (lemma, sub_lemma)
            })
            .or_else(|| {
                let sub_lemma = Lemma::new(right, needle);
                sub_lemma.map(|lemma| {
                    let left_hash = left.hash().clone();
                    let sub_lemma = Some(Positioned::Left(left_hash));
                    (lemma, sub_lemma)
                })
            })
            .map(|(sub_lemma, sibling_hash)| {
                Lemma {
                    node_hash: hash.clone(),
                    sibling_hash: sibling_hash,
                    sub_lemma: Some(Box::new(sub_lemma))
                }
            })
    }

}

/// Tags a value so that we know from which branch of a `Tree` (if any) it was found.
#[derive(PartialEq)]
pub enum Positioned<T> {

    /// The value was found in the left branch
    Left(T),

    /// The value was found in the right branch
    Right(T)
}

