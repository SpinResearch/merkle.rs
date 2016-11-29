
#![cfg(test)]

use ring::digest::{ Algorithm, SHA512 };

use merkletree::MerkleTree;
use hashutils::HashUtils;
use proof::Positioned;

#[allow(non_upper_case_globals)]
static digest: &'static Algorithm = &SHA512;

#[test]
fn test_from_str_vec() {
    let values = vec!["one", "two", "three", "four"];

    let hashes = vec![
        digest.hash_bytes(&values[0].as_bytes()),
        digest.hash_bytes(&values[1].as_bytes()),
        digest.hash_bytes(&values[2].as_bytes()),
        digest.hash_bytes(&values[3].as_bytes())
    ];

    let count = values.len();
    let tree  = MerkleTree::from_vec(digest, values).unwrap();

    let h01 = digest.combine_hashes(&hashes[0], &hashes[1]);
    let h23 = digest.combine_hashes(&hashes[2], &hashes[3]);

    let root_hash = digest.combine_hashes(&h01, &h23);

    assert_eq!(tree.count(), count);
    assert_eq!(tree.height(), 2);
    assert_eq!(tree.root_hash(), &root_hash);
}


#[test]
#[should_panic]
fn test_from_vec_empty() {
    let values: Vec<Vec<u8>> = vec![];
    MerkleTree::from_vec(digest, values).unwrap();
}

#[test]
fn test_from_vec1() {
    let values = vec!["hello, world".to_string()];
    let tree   = MerkleTree::from_vec(digest, values).unwrap();

    let root_hash = &digest.hash_bytes(&"hello, world".as_bytes());

    assert_eq!(tree.count(), 1);
    assert_eq!(tree.height(), 0);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_slice());
}


#[test]
fn test_from_vec3() {
    let values = vec![vec![1], vec![2], vec![3]];
    let tree   = MerkleTree::from_vec(digest, values).unwrap();

    let hashes = vec![
        digest.hash_bytes(&vec![1]),
        digest.hash_bytes(&vec![2]),
        digest.hash_bytes(&vec![3])
    ];

    let h01       = &digest.combine_hashes(&hashes[0], &hashes[1]);
    let h2        = &hashes[2];
    let root_hash = &digest.combine_hashes(h01, h2);

    assert_eq!(tree.count(), 3);
    assert_eq!(tree.height(), 2);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_slice());
}

#[test]
fn test_from_vec9() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let tree   = MerkleTree::from_vec(digest, values.clone()).unwrap();

    let hashes = values.iter().map(|v| digest.hash_bytes(v)).collect::<Vec<_>>();

    let h01   = digest.combine_hashes(&hashes[0], &hashes[1]);
    let h23   = digest.combine_hashes(&hashes[2], &hashes[3]);
    let h45   = digest.combine_hashes(&hashes[4], &hashes[5]);
    let h67   = digest.combine_hashes(&hashes[6], &hashes[7]);
    let h8    = &hashes[8];
    let h0123 = digest.combine_hashes(&h01, &h23);
    let h4567 = digest.combine_hashes(&h45, &h67);
    let h1to7 = digest.combine_hashes(&h0123, &h4567);

    let root_hash = &digest.combine_hashes(&h1to7, &h8);

    assert_eq!(tree.count(), 9);
    assert_eq!(tree.height(), 4);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_slice());
}

#[test]
fn test_valid_proof() {
    let values    = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let tree      = MerkleTree::from_vec(digest, values.clone()).unwrap();
    let root_hash = tree.root_hash();

    for value in values {
        let proof    = tree.gen_proof(value);
        let is_valid = proof.map(|p| p.validate(&root_hash)).unwrap_or(false);

        assert!(is_valid);
    }
}

#[test]
fn test_valid_proof_str() {
    let values    = vec!["Hello", "my", "name", "is", "Rusty"];
    let tree      = MerkleTree::from_vec(digest, values.clone()).unwrap();
    let root_hash = tree.root_hash();

    let value = "Rusty";

    let proof    = tree.gen_proof(&value);
    let is_valid = proof.map(|p| p.validate(&root_hash)).unwrap_or(false);

    assert!(is_valid);
}

#[test]
fn test_wrong_proof() {
    let values1   = vec![vec![1], vec![2], vec![3], vec![4]];
    let tree1     = MerkleTree::from_vec(digest, values1.clone()).unwrap();

    let values2   = vec![vec![4], vec![5], vec![6], vec![7]];
    let tree2     = MerkleTree::from_vec(digest, values2.clone()).unwrap();

    let root_hash = tree2.root_hash();

    for value in values1 {
        let proof    = tree1.gen_proof(value);
        let is_valid = proof.map(|p| p.validate(root_hash)).unwrap_or(false);

        assert_eq!(is_valid, false);
    }
}

#[test]
fn test_mutate_proof_first_lemma() {
    let values    = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let tree      = MerkleTree::from_vec(digest, values.clone()).unwrap();
    let root_hash = tree.root_hash();

    let mut i = 0;

    for value in values {
        let mut proof = tree.gen_proof(value).unwrap();

        match i % 3 {
            0 => {
                proof.lemma.node_hash = vec![1, 2, 3];
            },
            1 => {
                proof.lemma.sibling_hash = Some(Positioned::Left(vec![1, 2, 3]));
            },
            _ => {
                proof.lemma.sibling_hash = Some(Positioned::Right(vec![1, 2, 3]));
            }
        }

        let is_valid = proof.validate(root_hash);
        assert_eq!(is_valid, false);

        i += 1;
    }
}
