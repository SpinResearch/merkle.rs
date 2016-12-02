
use ring::digest::{ Algorithm, digest };

/// The sole purpose of this trait is to extend the standard
/// `ring::algo::Algorithm` type with a couple utility functions.
pub trait HashUtils {

    /// Compute the hash the given byte array
    fn hash_bytes(&'static self, bytes: &[u8]) -> Vec<u8>;

    /// Compute the hash of the concatenation of `left` and `right`
    fn combine_hashes(&'static self, left: &[u8], right: &[u8]) -> Vec<u8>;
}

impl HashUtils for Algorithm {

    fn hash_bytes(&'static self, bytes: &[u8]) -> Vec<u8> {
        digest(self, bytes).as_ref().into()
    }

    fn combine_hashes(&'static self, left: &[u8], right: &[u8]) -> Vec<u8> {
        let combined = [left, right].concat();
        digest(self, &combined).as_ref().into()
    }
}

