
#![cfg(feature="serialization-protobuf")]

extern crate ring;
extern crate merkle;
extern crate protobuf;

use ring::digest::{ Algorithm, SHA512 };

use merkle::{ MerkleTree, Proof };

#[allow(non_upper_case_globals)]
static digest: &'static Algorithm = &SHA512;

#[test]
fn test_protobuf_inverse() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();

    let tree = MerkleTree::from_vec(digest, values.clone()).unwrap();

    for value in values {
        let proof = tree.gen_proof(value).unwrap();
        let bytes = proof.clone().write_to_bytes().unwrap();
        let res   = Proof::<Vec<u8>>::parse_from_bytes(&bytes, digest).unwrap().unwrap();

        assert_eq!(proof.root_hash, res.root_hash);
        assert_eq!(proof.value, res.value);
        assert_eq!(proof.lemma, res.lemma);
    }
}
