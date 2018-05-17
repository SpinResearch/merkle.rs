#![cfg(feature = "serialization-protobuf")]

extern crate merkle;
extern crate protobuf;
extern crate ring;

use ring::digest::{Algorithm, Context, SHA512};

use merkle::{Hashable, MerkleTree, Proof};

#[allow(non_upper_case_globals)]
static digest: &'static Algorithm = &SHA512;

#[test]
fn test_protobuf_inverse() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();

    let tree = MerkleTree::from_vec(digest, values.clone());

    for value in values {
        let proof = tree.gen_proof(value).unwrap();
        let bytes = proof.clone().write_to_bytes().unwrap();
        let res = Proof::<Vec<u8>>::parse_from_bytes(&bytes, digest)
            .unwrap()
            .unwrap();

        assert_eq!(proof.root_hash, res.root_hash);
        assert_eq!(proof.value, res.value);
        assert_eq!(proof.lemma, res.lemma);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PublicKey {
    zero_values: Vec<u8>,
    one_values: Vec<u8>,
}

impl PublicKey {
    pub fn new(zero_values: Vec<u8>, one_values: Vec<u8>) -> Self {
        PublicKey {
            zero_values,
            one_values,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.zero_values
            .iter()
            .chain(self.one_values.iter())
            .cloned()
            .collect()
    }
}

impl From<Vec<u8>> for PublicKey {
    fn from(mut bytes: Vec<u8>) -> Self {
        let len = bytes.len();
        let ones = bytes.split_off(len / 2);
        let zeros = bytes;

        PublicKey::new(zeros, ones)
    }
}

impl Into<Vec<u8>> for PublicKey {
    fn into(self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl Hashable for PublicKey {
    fn update_context(&self, context: &mut Context) {
        context.update(&self.to_bytes());
    }
}

#[test]
fn test_protobuf_custom_hashable_impl() {
    let keys = (0..10)
        .map(|i| {
            let zero_values = (i..i + 16).collect::<Vec<_>>();
            let one_values = (i * 10..i * 10 + 16).collect::<Vec<_>>();

            PublicKey::new(zero_values, one_values)
        })
        .collect::<Vec<_>>();

    let tree = MerkleTree::from_vec(digest, keys.clone());

    for key in keys {
        let proof = tree.gen_proof(key).unwrap();
        let bytes = proof.clone().write_to_bytes().unwrap();
        let res = Proof::parse_from_bytes(&bytes, digest).unwrap().unwrap();

        assert_eq!(proof.root_hash, res.root_hash);
        assert_eq!(proof.value, res.value);
        assert_eq!(proof.lemma, res.lemma);
    }
}
