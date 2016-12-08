
use ring::digest::{ Algorithm, Context, digest, Digest };

/// The sole purpose of this trait is to extend the standard
/// `ring::algo::Algorithm` type with a couple utility functions.
pub trait HashUtils {

    /// Compute the hash the given byte array
    fn hash_bytes(&'static self, bytes: &[u8]) -> Digest;

    /// Compute the hash of the concatenation of `left` and `right`.
    //
    // XXX: We don't verify that `self` and `left.algorithm()` and
    // `right.algorithm()` are the same algorithm.
    // XXX: We really don't need `self` here because we can get the algorithm
    // from either `left` or `right`.
    fn combine_hashes(&'static self, left: &Digest, right: &Digest) -> Digest;
}

impl HashUtils for Algorithm {

    fn hash_bytes(&'static self, bytes: &[u8]) -> Digest {
        digest(self, bytes)
    }

    fn combine_hashes(&'static self, left: &Digest, right: &Digest) -> Digest {
        let mut ctx = Context::new(self);
        ctx.update(left.as_ref());
        ctx.update(right.as_ref());
        ctx.finish()
    }
}

