
use ring::digest::{ Algorithm, Context, digest, Digest };

/// The sole purpose of this trait is to extend the standard
/// `ring::algo::Algorithm` type with a couple utility functions.
pub trait HashUtils {

    /// Compute the hash the given byte array
    fn hash_bytes(&'static self, bytes: &AsRef<[u8]>) -> Digest;

    /// Compute the hash of the concatenation of `left` and `right`.
    // XXX: This is overly generic temporarily to make refactoring easier.
    // TODO: Give `left` and `right` type &Digest.
    fn combine_hashes(&'static self, left: &AsRef<[u8]>, right: &AsRef<[u8]>) -> Digest;
}

impl HashUtils for Algorithm {

    fn hash_bytes(&'static self, bytes: &AsRef<[u8]>) -> Digest {
        digest(self, bytes.as_ref())
    }

    fn combine_hashes(&'static self, left: &AsRef<[u8]>, right: &AsRef<[u8]>) -> Digest {
        let mut ctx = Context::new(self);
        ctx.update(left.as_ref());
        ctx.update(right.as_ref());
        ctx.finish()
    }
}

