use crypto::digest::Digest;

/// The sole purpose of this trait is to extend the standard
/// `crypto::digest::Digest` with a couple utility functions.
pub trait MerkleDigest {

    /// Compute the hash the given byte array
    fn hash_bytes(&mut self, bytes: &Vec<u8>) -> Vec<u8>;

    /// Compute the hash of the concatenation of `left` and `right`
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
