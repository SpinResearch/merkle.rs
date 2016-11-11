use crypto::digest::Digest;

/// Extends the standard `crypto::digest::Digest` to play nicely with our Merkle Tree
pub trait MerkleDigest {
    fn hash_bytes(&mut self, bytes: &Vec<u8>) -> Vec<u8>;
    fn combine_hashes(&mut self, left: &Vec<u8>, right: &Vec<u8>) -> Vec<u8>;
}

impl <D> MerkleDigest for D where D: Digest {

    fn hash_bytes(&mut self, bytes: &Vec<u8>) -> Vec<u8> {
        let mut hash = vec![0; self.output_bytes()];

        self.reset();
        self.input(bytes);
        self.result(&mut hash);
        self.reset();

        hash
    }

    fn combine_hashes(&mut self, left: &Vec<u8>, right: &Vec<u8>) -> Vec<u8> {
        let mut hash = vec![0; self.output_bytes()];

        self.reset();
        self.input(left);
        self.input(right);
        self.result(&mut hash);
        self.reset();

        hash
    }
}
