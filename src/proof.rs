use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use ring::digest::Algorithm;

use hashutils::HashUtils;
use tree::Tree;

/// An inclusion proof represent the fact that a `value` is a member
/// of a `MerkleTree` with root hash `root_hash`, and hash function `algorithm`.
#[cfg_attr(feature = "serialization-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Proof<T> {
    /// The hashing algorithm used in the original `MerkleTree`
    #[cfg_attr(feature = "serialization-serde", serde(with = "algorithm_serde"))]
    pub algorithm: &'static Algorithm,

    /// The hash of the root of the original `MerkleTree`
    pub root_hash: Vec<u8>,

    /// The first `Lemma` of the `Proof`
    pub lemma: Lemma,

    /// The value concerned by this `Proof`
    pub value: T,
}

#[cfg(feature = "serialization-serde")]
mod algorithm_serde {
    use ring::digest::{self, Algorithm};
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(
        algorithm: &'static Algorithm,
        se: S,
    ) -> Result<S::Ok, S::Error> {
        // The `Debug` implementation of `Algorithm` prints its ID.
        format!("{:?}", algorithm).serialize(se)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<&'static Algorithm, D::Error> {
        match Deserialize::deserialize(de)? {
            "SHA1" => Ok(&digest::SHA1),
            "SHA256" => Ok(&digest::SHA256),
            "SHA384" => Ok(&digest::SHA384),
            "SHA512" => Ok(&digest::SHA512),
            "SHA512_256" => Ok(&digest::SHA512_256),
            _ => Err(D::Error::custom("unknown hash algorithm")),
        }
    }

    mod test {
        use super::*;
        use ring::digest::{
            SHA1 as sha1, SHA256 as sha256, SHA384 as sha384, SHA512 as sha512,
            SHA512_256 as sha512_256,
        };

        static SHA1: &Algorithm = &sha1;
        static SHA256: &Algorithm = &sha256;
        static SHA384: &Algorithm = &sha384;
        static SHA512: &Algorithm = &sha512;
        static SHA512_256: &Algorithm = &sha512_256;

        #[test]
        fn test_serialize_known_algorithms() {
            extern crate serde_json;

            for alg in [SHA1, SHA256, SHA384, SHA512, SHA512_256].iter() {
                let mut serializer = serde_json::Serializer::with_formatter(
                    vec![],
                    serde_json::ser::PrettyFormatter::new(),
                );

                let _ = serialize(alg, &mut serializer).expect(&format!("{:?}", alg));
                let alg_ = deserialize(&mut serde_json::Deserializer::from_slice(
                    &serializer.into_inner()[..],
                )).expect(&format!("{:?}", alg));

                assert_eq!(*alg, alg_);
            }
        }

        #[test]
        #[should_panic(expected = "unknown hash algorithm")]
        fn test_serialize_unknown_algorithm() {
            extern crate serde_json;
            {
                let alg_str = "\"BLAKE2b\"";
                let mut deserializer = serde_json::Deserializer::from_str(alg_str);
                let _ = deserialize(&mut deserializer).expect(&format!("{:?}", alg_str));
            }
        }
    }
}

impl<T: PartialEq> PartialEq for Proof<T> {
    fn eq(&self, other: &Proof<T>) -> bool {
        self.root_hash == other.root_hash && self.lemma == other.lemma && self.value == other.value
    }
}

impl<T: Eq> Eq for Proof<T> {}

impl<T: Ord> PartialOrd for Proof<T> {
    fn partial_cmp(&self, other: &Proof<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for Proof<T> {
    fn cmp(&self, other: &Proof<T>) -> Ordering {
        self.root_hash
            .cmp(&other.root_hash)
            .then(self.value.cmp(&other.value))
            .then_with(|| self.lemma.cmp(&other.lemma))
    }
}

impl<T: Hash> Hash for Proof<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.root_hash.hash(state);
        self.lemma.hash(state);
        self.value.hash(state);
    }
}

impl<T> Proof<T> {
    /// Constructs a new `Proof`
    pub fn new(algorithm: &'static Algorithm, root_hash: Vec<u8>, lemma: Lemma, value: T) -> Self {
        Proof {
            algorithm,
            root_hash,
            lemma,
            value,
        }
    }

    /// Checks whether this inclusion proof is well-formed,
    /// and whether its root hash matches the given `root_hash`.
    pub fn validate(&self, root_hash: &[u8]) -> bool {
        if self.root_hash != root_hash || self.lemma.node_hash != root_hash {
            return false;
        }

        self.lemma.validate(self.algorithm)
    }

}

/// A `Lemma` holds the hash of a node, the hash of its sibling node,
/// and a sub lemma, whose `node_hash`, when combined with this `sibling_hash`
/// must be equal to this `node_hash`.
#[cfg_attr(feature = "serialization-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lemma {
    pub node_hash: Vec<u8>,
    pub sibling_hash: Option<Positioned<Vec<u8>>>,
    pub sub_lemma: Option<Box<Lemma>>,
}

impl Lemma {
    /// Attempts to generate a proof that the a value with hash `needle` is a member of the given `tree`.
    pub fn new<T>(tree: &Tree<T>, needle: &[u8]) -> Option<Lemma> {
        match *tree {
            Tree::Empty { .. } => None,

            Tree::Leaf { ref hash, .. } => Lemma::new_leaf_proof(hash, needle),

            Tree::Node {
                ref hash,
                ref left,
                ref right,
            } => Lemma::new_tree_proof(hash, needle, left, right),
        }
    }

    fn new_leaf_proof(hash: &[u8], needle: &[u8]) -> Option<Lemma> {
        if *hash == *needle {
            Some(Lemma {
                node_hash: hash.into(),
                sibling_hash: None,
                sub_lemma: None,
            })
        } else {
            None
        }
    }

    fn new_tree_proof<T>(
        hash: &[u8],
        needle: &[u8],
        left: &Tree<T>,
        right: &Tree<T>,
    ) -> Option<Lemma> {
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
            .map(|(sub_lemma, sibling_hash)| Lemma {
                node_hash: hash.into(),
                sibling_hash,
                sub_lemma: Some(Box::new(sub_lemma)),
            })
    }

    fn validate(&self, algorithm: &'static Algorithm) -> bool {
        match self.sub_lemma {
            None => self.sibling_hash.is_none(),

            Some(ref sub) => match self.sibling_hash {
                None => false,

                Some(Positioned::Left(ref hash)) => {
                    let combined = algorithm.hash_nodes(hash, &sub.node_hash);
                    let hashes_match = combined.as_ref() == self.node_hash.as_slice();
                    hashes_match && sub.validate(algorithm)
                }

                Some(Positioned::Right(ref hash)) => {
                    let combined = algorithm.hash_nodes(&sub.node_hash, hash);
                    let hashes_match = combined.as_ref() == self.node_hash.as_slice();
                    hashes_match && sub.validate(algorithm)
                }
            },
        }
    }
}

/// Tags a value so that we know from which branch of a `Tree` (if any) it was found.
#[cfg_attr(feature = "serialization-serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Positioned<T> {
    /// The value was found in the left branch
    Left(T),

    /// The value was found in the right branch
    Right(T),
}
