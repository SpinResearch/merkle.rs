#![cfg(test)]

#[cfg(feature = "serialization-serde")]
extern crate serde_json;

use ring::digest::{Algorithm, Context, SHA512};

use crate::hashutils::{HashUtils, Hashable};
use crate::merkletree::MerkleTree;
use crate::proof::Positioned;

static DIGEST: &Algorithm = &SHA512;

#[test]
fn test_from_str_vec() {
    let values = vec!["one", "two", "three", "four"];

    let hashes = vec![
        DIGEST.hash_leaf(&values[0].as_bytes()),
        DIGEST.hash_leaf(&values[1].as_bytes()),
        DIGEST.hash_leaf(&values[2].as_bytes()),
        DIGEST.hash_leaf(&values[3].as_bytes()),
    ];

    let count = values.len();
    let tree = MerkleTree::from_vec(DIGEST, values);

    let h01 = DIGEST.hash_nodes(&hashes[0], &hashes[1]);
    let h23 = DIGEST.hash_nodes(&hashes[2], &hashes[3]);

    let root_hash = DIGEST.hash_nodes(&h01, &h23);

    assert_eq!(tree.count(), count);
    assert_eq!(tree.height(), 2);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_ref());
}

#[test]
fn test_from_vec_empty() {
    let values: Vec<Vec<u8>> = vec![];
    let tree = MerkleTree::from_vec(DIGEST, values);
    let empty_hash: Vec<u8> = DIGEST.hash_empty().as_ref().into();
    let root_hash = tree.root_hash().clone();

    assert_eq!(root_hash, empty_hash);
}

#[test]
fn test_from_vec1() {
    let values = vec!["hello, world".to_string()];
    let tree = MerkleTree::from_vec(DIGEST, values);

    let root_hash = &DIGEST.hash_leaf(b"hello, world");

    assert_eq!(tree.count(), 1);
    assert_eq!(tree.height(), 0);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_ref());
}

#[test]
fn test_from_vec3() {
    let values = vec![vec![1], vec![2], vec![3]];
    let tree = MerkleTree::from_vec(DIGEST, values);

    let hashes = vec![
        DIGEST.hash_leaf(&vec![1]),
        DIGEST.hash_leaf(&vec![2]),
        DIGEST.hash_leaf(&vec![3]),
    ];

    let h01 = DIGEST.hash_nodes(&hashes[0], &hashes[1]);
    let h2 = &hashes[2];
    let root_hash = &DIGEST.hash_nodes(&h01, h2);

    assert_eq!(tree.count(), 3);
    assert_eq!(tree.height(), 2);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_ref());
}

#[test]
fn test_from_vec9() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let tree = MerkleTree::from_vec(DIGEST, values.clone());

    let hashes = values
        .iter()
        .map(|v| DIGEST.hash_leaf(v))
        .collect::<Vec<_>>();

    let h01 = DIGEST.hash_nodes(&hashes[0], &hashes[1]);
    let h23 = DIGEST.hash_nodes(&hashes[2], &hashes[3]);
    let h45 = DIGEST.hash_nodes(&hashes[4], &hashes[5]);
    let h67 = DIGEST.hash_nodes(&hashes[6], &hashes[7]);
    let h8 = &hashes[8];
    let h0123 = DIGEST.hash_nodes(&h01, &h23);
    let h4567 = DIGEST.hash_nodes(&h45, &h67);
    let h1to7 = DIGEST.hash_nodes(&h0123, &h4567);

    let root_hash = &DIGEST.hash_nodes(&h1to7, h8);

    assert_eq!(tree.count(), 9);
    assert_eq!(tree.height(), 4);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_ref());
}

#[test]
fn test_valid_proof() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let tree = MerkleTree::from_vec(DIGEST, values.clone());
    let root_hash = tree.root_hash();

    for value in values {
        let proof = tree.gen_proof(value);
        let is_valid = proof.map(|p| p.validate(&root_hash)).unwrap_or(false);

        assert!(is_valid);
    }
}

#[test]
fn test_valid_proof_str() {
    let values = vec!["Hello", "my", "name", "is", "Rusty"];
    let tree = MerkleTree::from_vec(DIGEST, values);
    let root_hash = tree.root_hash();

    let value = "Rusty";

    let proof = tree.gen_proof(&value);
    let is_valid = proof.map(|p| p.validate(&root_hash)).unwrap_or(false);

    assert!(is_valid);
}

#[test]
fn test_wrong_proof() {
    let values1 = vec![vec![1], vec![2], vec![3], vec![4]];
    let tree1 = MerkleTree::from_vec(DIGEST, values1.clone());

    let values2 = vec![vec![4], vec![5], vec![6], vec![7]];
    let tree2 = MerkleTree::from_vec(DIGEST, values2);

    let root_hash = tree2.root_hash();

    for value in values1 {
        let proof = tree1.gen_proof(value);
        let is_valid = proof.map(|p| p.validate(root_hash)).unwrap_or(false);

        assert_eq!(is_valid, false);
    }
}

#[test]
fn test_nth_proof() {
    // Calculation depends on the total count. Try a few numbers: odd, even, powers of two...
    for &count in &[1, 2, 3, 10, 15, 16, 17, 22] {
        let values = (1..=count).map(|x| vec![x as u8]).collect::<Vec<_>>();
        let tree = MerkleTree::from_vec(DIGEST, values.clone());
        let root_hash = tree.root_hash();

        for i in 0..count {
            let proof = tree.gen_nth_proof(i).expect("gen proof by index");
            assert_eq!(vec![i as u8 + 1], proof.value);
            assert!(proof.validate(&root_hash));
            assert_eq!(i, proof.index(tree.count()));
        }

        assert!(tree.gen_nth_proof(count).is_none());
        assert!(tree.gen_nth_proof(count + 1000).is_none());
    }
}

#[test]
fn test_mutate_proof_first_lemma() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let tree = MerkleTree::from_vec(DIGEST, values.clone());
    let root_hash = tree.root_hash();

    for (i, value) in values.into_iter().enumerate() {
        let mut proof = tree.gen_proof(value).unwrap();

        match i % 3 {
            0 => {
                proof.lemma.node_hash = vec![1, 2, 3];
            }
            1 => {
                proof.lemma.sibling_hash = Some(Positioned::Left(vec![1, 2, 3]));
            }
            _ => {
                proof.lemma.sibling_hash = Some(Positioned::Right(vec![1, 2, 3]));
            }
        }

        let is_valid = proof.validate(root_hash);
        assert_eq!(is_valid, false);
    }
}

#[test]
fn test_tree_iter() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let tree = MerkleTree::from_vec(DIGEST, values.clone());
    let iter = tree.iter().cloned().collect::<Vec<_>>();

    assert_eq!(values, iter);
}

#[test]
fn test_tree_into_iter() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let tree = MerkleTree::from_vec(DIGEST, values.clone());
    let iter = tree.into_iter().collect::<Vec<_>>();

    assert_eq!(values, iter);
}

#[test]
fn test_tree_into_iter_loop() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let tree = MerkleTree::from_vec(DIGEST, values.clone());

    let mut collected = Vec::new();

    for value in tree {
        collected.push(value);
    }

    assert_eq!(values, collected);
}

#[test]
fn test_tree_into_iter_loop_borrow() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let tree = MerkleTree::from_vec(DIGEST, values.clone());

    let mut collected = Vec::new();

    for value in &tree {
        collected.push(value);
    }

    let refs = values.iter().collect::<Vec<_>>();

    assert_eq!(refs, collected);
}

pub struct PublicKey {
    zero_values: Vec<Vec<u8>>,
    one_values: Vec<Vec<u8>>,
}

impl PublicKey {
    pub fn new(zero_values: Vec<Vec<u8>>, one_values: Vec<Vec<u8>>) -> Self {
        PublicKey {
            zero_values,
            one_values,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.zero_values
            .iter()
            .chain(self.one_values.iter())
            .fold(Vec::new(), |mut acc, i| {
                acc.append(&mut i.clone());
                acc
            })
    }
}

impl Hashable for PublicKey {
    fn update_context(&self, context: &mut Context) {
        context.update(&self.to_bytes());
    }
}

#[test]
fn test_custom_hashable_impl() {
    let keys = (0..10)
        .map(|i| {
            let zero_values = vec![vec![i], vec![i + 1], vec![i + 2]];
            let one_values = vec![vec![i + 3], vec![i + 4], vec![i + 5]];

            PublicKey::new(zero_values, one_values)
        })
        .collect::<Vec<_>>();

    let tree = MerkleTree::from_vec(DIGEST, keys);

    assert_eq!(tree.count(), 10);
    assert_eq!(tree.height(), 4);
}

#[cfg(feature = "serialization-serde")]
#[test]
fn test_serialize_proof_with_serde() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let tree = MerkleTree::from_vec(DIGEST, values);
    let proof = tree.gen_proof(vec![5]);

    let serialized = serde_json::to_string(&proof).expect("serialize proof");

    assert_eq!(
        proof,
        serde_json::from_str(&serialized).expect("deserialize proof")
    );
}
