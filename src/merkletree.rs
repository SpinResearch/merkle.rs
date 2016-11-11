use crypto::digest::Digest;

use tree::{ Tree };
use merkledigest::{ MerkleDigest };

pub use proof::{
    Proof,
    ProofBlock
};

/// The Merkle tree
pub struct MerkleTree<D, T> {
    pub digest: D,
    pub tree: Tree<T>,
    pub height: usize,
    pub count: usize
}

impl <D, T> MerkleTree<D, T> where D: Digest + Clone, T: Into<Vec<u8>> + Clone {

    /// Constructs a Merkle Tree from a vector of data blocks.
    pub fn from_vec(mut digest: D, values: Vec<T>) -> Self {
        if values.is_empty() {
            panic!("Cannot build a Merkle tree from an empty vector.");
        }

        let count      = values.len();
        let mut height = 0;

        let mut cur = Vec::with_capacity(count);

        for v in values.into_iter() {
            let leaf = Tree::make_leaf(&mut digest, v);
            cur.push(leaf);
        }

        while cur.len() > 1 {
            let mut next = Vec::new();
            while cur.len() > 0 {
                if cur.len() == 1 {
                    next.push(cur.remove(0));
                }
                else {
                    let left  = cur.remove(0);
                    let right = cur.remove(0);

                    let combined_hash = digest.combine_hashes(
                        left.get_hash(),
                        right.get_hash()
                    );

                    let node = Tree::Node {
                       hash: combined_hash,
                       left: Box::new(left),
                       right: Box::new(right)
                    };

                    next.push(node);
                }
            }

            height += 1;

            cur = next;
        }

        assert!(cur.len() == 1);

        let tree = cur.remove(0);

        MerkleTree {
            digest: digest,
            tree: tree,
            height: height,
            count: count
        }
    }

    /// Returns the tree's root hash.
    pub fn root_hash(&self) -> &Vec<u8> {
        self.tree.get_hash()
    }

    /// Generate an inclusion proof for the given value
    pub fn gen_proof(&mut self, value: &T) -> Option<Proof<D, T>> {
        let hash = self.digest.hash_bytes(&value.clone().into());

        ProofBlock::new(&self.tree, &hash).map(|block|
            Proof {
                digest: self.digest.clone(),
                root_hash: self.root_hash().clone(),
                block: block,
                value: value.clone()
            }
        )
    }

    pub fn is_proof_valid(&mut self, proof: &Proof<D, T>) -> bool {
        proof.validate_against(self)
    }

}

