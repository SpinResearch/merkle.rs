
use ring::digest::Algorithm;

use tree::Tree;
use hashutils::HashUtils;

use proof::{ Proof, Lemma };

/// A Merkle tree is a binary tree, with values of type `T` at the leafs,
/// and where every node holds the hash of the concatenation of the hashes of
/// its children nodes.
#[derive(Clone, Debug)]
pub struct MerkleTree<T> {

    /// The hashing algorithm used by this Merkle tree
    pub algorithm: &'static Algorithm,

    /// The root of the inner binary tree
    root: Tree<T>,

    /// The height of the tree
    height: usize,

    /// The number of leaf nodes in the tree
    count: usize
}

impl <T> MerkleTree<T> where T: Into<Vec<u8>> + Clone {

    /// Constructs a Merkle Tree from a vector of data blocks.
    /// Returns None if `values` is empty.
    pub fn from_vec(algo: &'static Algorithm, values: Vec<T>) -> Option<Self> {
        if values.is_empty() {
            return None
        }

        let count      = values.len();
        let mut height = 0;
        let mut cur    = Vec::with_capacity(count);

        for v in values {
            let leaf = Tree::make_leaf(algo, v);
            cur.push(leaf);
        }

        while cur.len() > 1 {
            let mut next = Vec::new();
            while !cur.is_empty() {
                if cur.len() == 1 {
                    next.push(cur.remove(0));
                }
                else {
                    let left  = cur.remove(0);
                    let right = cur.remove(0);

                    let combined_hash = algo.combine_hashes(
                        left.hash(),
                        right.hash()
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

        let root = cur.remove(0);

        Some(MerkleTree {
            algorithm: algo,
            root: root,
            height: height,
            count: count
        })
    }

    /// Returns the root hash of Merkle tree
    pub fn root_hash(&self) -> &Vec<u8> {
        self.root.hash()
    }

    /// Returns the height of Merkle tree
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the number of leaves in the Merkle tree
    pub fn count(&self) -> usize {
        self.count
    }

    /// Generate an inclusion proof for the given value.
    /// Returns `None` if the given value is not found in the tree.
    pub fn gen_proof(&self, value: T) -> Option<Proof<T>> {
        let root_hash  = self.root_hash().clone();
        let node_hash  = self.algorithm.hash_bytes(&value.clone().into());

        Lemma::new(&self.root, &node_hash).map(|lemma|
            Proof::new(self.algorithm, root_hash, lemma, value)
        )
    }

}
