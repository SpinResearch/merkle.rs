
#![cfg(test)]

use crypto::sha3::Sha3;

use merkletree::{ MerkleTree };
use merkledigest::{ MerkleDigest };
use proof::{ Positioned };

#[test]
fn test_from_str_vec() {
    let mut digest = Sha3::sha3_256();

    let values = vec![
                        "one",
                        "two",
                        "three",
                        "four"
                    ];

    let hashes = vec![
                        digest.hash_bytes(&values[0].into()),
                        digest.hash_bytes(&values[1].into()),
                        digest.hash_bytes(&values[2].into()),
                        digest.hash_bytes(&values[3].into())
                    ];

    let count  = values.len();

    let tree   = MerkleTree::from_vec(digest, values);


    let mut d = Sha3::sha3_256();

    let h01 = &d.combine_hashes(&hashes[0], &hashes[1]);
    let h23 = &d.combine_hashes(&hashes[2], &hashes[3]);

    let root_hash = d.combine_hashes(h01, h23);

    assert_eq!(tree.count, count);
    assert_eq!(tree.height, 2);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_slice());
}


#[test]
#[should_panic]
fn test_from_vec_empty() {
    let digest          = Sha3::sha3_256();
    let values: Vec<Vec<u8>> = vec![];

    MerkleTree::from_vec(digest, values);
}

#[test]
fn test_from_vec1() {
    let digest = Sha3::sha3_256();

    let values = vec!["hello, world".to_string()];
    let tree = MerkleTree::from_vec(digest, values);

    let mut d = Sha3::sha3_256();

    let root_hash = &d.hash_bytes(&"hello, world".to_string().into());

    assert_eq!(tree.count, 1);
    assert_eq!(tree.height, 0);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_slice());
}


#[test]
fn test_from_vec3() {
    let digest = Sha3::sha3_256();

    let values = vec![vec![1], vec![2], vec![3]];
    let tree = MerkleTree::from_vec(digest, values);

    let mut d = Sha3::sha3_256();

    let hashes = vec![
        d.hash_bytes(&vec![1].into()),
        d.hash_bytes(&vec![2].into()),
        d.hash_bytes(&vec![3].into())
    ];

    let h01       = &d.combine_hashes(&hashes[0], &hashes[1]);
    let h2        = &hashes[2];
    let root_hash = &d.combine_hashes(h01, h2);

    assert_eq!(tree.count, 3);
    assert_eq!(tree.height, 2);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_slice());
}

#[test]
fn test_from_vec9() {
    let digest = Sha3::sha3_256();

    let values = vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![6], vec![7], vec![8], vec![9]];
    let count  = values.len();

    let tree = MerkleTree::from_vec(digest, values.clone());

    let mut d = Sha3::sha3_256();

    let hashes = values.iter().map(|v| d.hash_bytes(v.into())).collect::<Vec<_>>();

    let h01   = &d.combine_hashes(&hashes[0], &hashes[1]);
    let h23   = &d.combine_hashes(&hashes[2], &hashes[3]);
    let h45   = &d.combine_hashes(&hashes[4], &hashes[5]);
    let h67   = &d.combine_hashes(&hashes[6], &hashes[7]);
    let h8    = &hashes[8];
    let h0123 = &d.combine_hashes(h01, h23);
    let h4567 = &d.combine_hashes(h45, h67);
    let h1to7 = &d.combine_hashes(h0123, h4567);

    let root_hash = &d.combine_hashes(h1to7, h8);

    assert_eq!(tree.count, count);
    assert_eq!(tree.height, 4);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_slice());
}

#[test]
fn test_valid_proof() {
    let digest   = Sha3::sha3_256();
    let values: Vec<Vec<u8>> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9].iter().map(|x| vec![*x]).collect();
    let mut tree = MerkleTree::from_vec(digest, values.clone());

    for value in values.iter() {
        let proof    = tree.gen_proof(value);
        let is_valid = proof.map(|p| tree.is_proof_valid(&p)).unwrap_or(false);

        assert!(is_valid);
    }
}

#[test]
fn test_valid_proof_str() {
    let digest   = Sha3::sha3_256();
    let values   = vec!["Hello", "my", "name", "is", "Rusty"];
    let mut tree = MerkleTree::from_vec(digest, values.clone());
    let value = "Rusty";

    let proof    = tree.gen_proof(&value);
    let is_valid = proof.map(|p| tree.is_proof_valid(&p)).unwrap_or(false);

    assert!(is_valid);
}

#[test]
fn test_wrong_proof() {
    let digest1   = Sha3::sha3_256();
    let values1   = vec![vec![1], vec![2], vec![3], vec![4]];
    let mut tree1 = MerkleTree::from_vec(digest1, values1.clone());

    let digest2   = Sha3::sha3_256();
    let values2   = vec![vec![4], vec![5], vec![6], vec![7]];
    let mut tree2 = MerkleTree::from_vec(digest2, values2.clone());

    for value in values1.iter() {
        let proof    = tree1.gen_proof(value);
        let is_valid = proof.map(|p| tree2.is_proof_valid(&p)).unwrap_or(false);

        assert_eq!(is_valid, false);
    }
}

#[test]
fn test_mutate_proof_first_block() {
    let digest   = Sha3::sha3_256();
    let values   = vec![1, 2, 3, 4].iter().map(|x| vec![*x]).collect::<Vec<Vec<u8>>>();
    let mut tree = MerkleTree::from_vec(digest, values.clone());
    let mut i    = 0;

    for value in values.iter() {
        let mut proof = tree.gen_proof(value).unwrap();

        if i % 2 == 0 {
            proof.block.node_hash = vec![1,2,3];
        } else {
            proof.block.sibling_hash = Positioned::Left(vec![1,2,3]);
        }

        let is_valid = tree.is_proof_valid(&proof);
        assert_eq!(is_valid, false);

        i += 1;
    }
}

