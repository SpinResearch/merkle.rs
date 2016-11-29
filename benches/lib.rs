
#![feature(test)]
#![feature(rand)]

extern crate test;
extern crate rand;

extern crate merkle;
extern crate ring;

use test::Bencher;
use rand::Rng;

use ring::digest::{Algorithm, SHA512};

use merkle::MerkleTree;

#[allow(non_upper_case_globals)]
static digest: &'static Algorithm = &SHA512;

#[bench]
fn bench_small_str_tree(b: &mut Bencher) {
    let values = vec!["one", "two", "three", "four"];

    b.iter(|| {
        MerkleTree::from_vec(digest, values.clone())
    });
}

#[bench]
fn bench_small_str_proof_gen(b: &mut Bencher) {
    let values = vec!["one", "two", "three", "four"];
    let tree   = MerkleTree::from_vec(digest, values.clone()).unwrap();

    b.iter(|| {
        for value in &values {
            let proof = tree.gen_proof(value);
            test::black_box(proof);
        }
    });
}

#[bench]
fn bench_small_str_proof_check(b: &mut Bencher) {
    let values = vec!["one", "two", "three", "four"];
    let tree   = MerkleTree::from_vec(digest, values.clone()).unwrap();
    let proofs = values.iter().map(|v| tree.gen_proof(v).unwrap()).collect::<Vec<_>>();

    b.iter(|| {
        for proof in &proofs {
            test::black_box(proof.validate(tree.root_hash()));
        }
    });
}

#[bench]
fn bench_big_rnd_tree(b: &mut Bencher) {
    let mut values = vec![vec![0u8; 256]; 160];
    let mut rng    = rand::IsaacRng::new_unseeded();

    for mut v in &mut values {
        rng.fill_bytes(&mut v);
    }

    b.iter(|| {
        MerkleTree::from_vec(digest, values.clone()).unwrap()
    });
}

#[bench]
fn bench_big_rnd_proof_gen(b: &mut Bencher) {
    let mut values = vec![vec![0u8; 256]; 160];
    let mut rng    = rand::IsaacRng::new_unseeded();

    for mut v in &mut values {
        rng.fill_bytes(&mut v);
    }

    let tree = MerkleTree::from_vec(digest, values.clone()).unwrap();

    b.iter(|| {
        for value in &values {
            let proof = tree.gen_proof(value.clone());
            test::black_box(proof);
        }
    });
}

#[bench]
fn bench_big_rnd_proof_check(b: &mut Bencher) {
    let mut values = vec![vec![0u8; 256]; 160];
    let mut rng    = rand::IsaacRng::new_unseeded();

    for mut v in &mut values {
        rng.fill_bytes(&mut v);
    }

    let tree   = MerkleTree::from_vec(digest, values.clone()).unwrap();
    let proofs = values.into_iter().map(|v| tree.gen_proof(v).unwrap()).collect::<Vec<_>>();

    b.iter(|| {
        for proof in &proofs {
            test::black_box(proof.validate(tree.root_hash()));
        }
    });
}
