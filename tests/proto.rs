
#![cfg(feature="serialization-protobuf")]

extern crate crypto;
extern crate merkle;
extern crate protobuf;

use crypto::sha3::Sha3;
use merkle::{ MerkleTree, Proof };

#[test]
fn test_protobuf_inverse() {
    let digest = Sha3::sha3_256();
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();

    let tree = MerkleTree::from_vec_unsafe(digest.clone(), values.clone());

    for value in values {
        let proof = tree.gen_proof(value).unwrap();
        let bytes = proof.clone().write_to_bytes().unwrap();
        let res   = Proof::<Sha3, Vec<u8>>::parse_from_bytes(&bytes, digest).unwrap().unwrap();

        assert_eq!(proof.root_hash, res.root_hash);
        assert_eq!(proof.value, res.value);
        assert_eq!(proof.lemma, res.lemma);
    }
}

