#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

extern crate merkle;
extern crate rand;
extern crate ring;

use merkle::MerkleTree;
use rand::RngCore;
use ring::digest::{Algorithm, SHA512};

static DIGEST: &Algorithm = &SHA512;

fn bench_small_str_tree(c: &mut Criterion) {
    c.bench_function("MerkleTree::from_bec - small", |b| {
        let values = vec!["one", "two", "three", "four"];
        b.iter(|| MerkleTree::from_vec(DIGEST, black_box(values.clone())))
    });
}

fn bench_small_str_proof_gen(c: &mut Criterion) {
    c.bench_function("MerkleTree::gen_proof - small", |b| {
        let values = vec!["one", "two", "three", "four"];
        let tree = MerkleTree::from_vec(DIGEST, values.clone());

        b.iter(|| {
            for value in &values {
                tree.gen_proof(black_box(value));
            }
        })
    });
}

fn bench_small_str_proof_check(c: &mut Criterion) {
    c.bench_function("MerkleTree::validate_proof - small", |b| {
        let values = vec!["one", "two", "three", "four"];
        let tree = MerkleTree::from_vec(DIGEST, values.clone());
        let proofs = values
            .iter()
            .map(|v| tree.gen_proof(v).unwrap())
            .collect::<Vec<_>>();

        b.iter(|| {
            for proof in &proofs {
                proof.validate(black_box(tree.root_hash()));
            }
        })
    });
}

fn bench_big_rnd_tree(c: &mut Criterion) {
    c.bench_function("MerkleTree::from_vec - big", |b| {
        let mut values = vec![vec![0u8; 256]; 160];
        let mut rng = rand::thread_rng();

        for mut v in &mut values {
            rng.fill_bytes(&mut v);
        }

        b.iter(|| MerkleTree::from_vec(DIGEST, black_box(values.clone())))
    });
}

fn bench_big_rnd_proof_gen(c: &mut Criterion) {
    c.bench_function("MerkleTree::gen_proof - big", |b| {
        let mut values = vec![vec![0u8; 256]; 160];
        let mut rng = rand::thread_rng();

        for mut v in &mut values {
            rng.fill_bytes(&mut v);
        }

        let tree = MerkleTree::from_vec(DIGEST, values.clone());

        b.iter(|| {
            for value in &values {
                tree.gen_proof(black_box(value.clone()));
            }
        })
    });
}

fn bench_big_rnd_proof_check(c: &mut Criterion) {
    c.bench_function("MerkleTree::validate_proof - big", |b| {
        let mut values = vec![vec![0u8; 256]; 160];
        let mut rng = rand::thread_rng();

        for mut v in &mut values {
            rng.fill_bytes(&mut v);
        }

        let tree = MerkleTree::from_vec(DIGEST, values.clone());
        let proofs = values
            .into_iter()
            .map(|v| tree.gen_proof(v).unwrap())
            .collect::<Vec<_>>();

        b.iter(|| {
            for proof in &proofs {
                proof.validate(black_box(tree.root_hash()));
            }
        })
    });
}

fn bench_big_rnd_iter(c: &mut Criterion) {
    c.bench_function("MerkleTree::iter - big", |b| {
        let mut values = vec![vec![0u8; 256]; 160];
        let mut rng = rand::thread_rng();

        for mut v in &mut values {
            rng.fill_bytes(&mut v);
        }

        let tree = MerkleTree::from_vec(DIGEST, values);
        b.iter(|| {
            for value in &tree {
                black_box(value);
            }
        })
    });
}

criterion_group!(
    benches,
    bench_small_str_tree,
    bench_small_str_proof_gen,
    bench_small_str_proof_check,
    bench_big_rnd_tree,
    bench_big_rnd_proof_gen,
    bench_big_rnd_proof_check,
    bench_big_rnd_iter,
);

criterion_main!(benches);
