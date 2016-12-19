
use ring::digest::{ Algorithm, Context, Digest, digest };

/// The sole purpose of this trait is to extend the standard
/// `ring::algo::Algorithm` type with a couple utility functions.
pub trait HashUtils {

    /// Compute the hash of the empty string
    fn hash_empty(&'static self) -> Digest;

    /// Compute the hash of the given leaf
    fn hash_leaf(&'static self, bytes: &AsRef<[u8]>) -> Digest;

    /// Compute the hash of the concatenation of `left` and `right`.
    // XXX: This is overly generic temporarily to make refactoring easier.
    // TODO: Give `left` and `right` type &Digest.
    fn hash_nodes(&'static self, left: &AsRef<[u8]>, right: &AsRef<[u8]>) -> Digest;
}

impl HashUtils for Algorithm {

    fn hash_empty(&'static self) -> Digest {
        digest(self, &[])
    }

    fn hash_leaf(&'static self, leaf: &AsRef<[u8]>) -> Digest {
        let mut ctx = Context::new(self);
        ctx.update(&[0x00]);
        ctx.update(leaf.as_ref());
        ctx.finish()
    }

    fn hash_nodes(&'static self, left: &AsRef<[u8]>, right: &AsRef<[u8]>) -> Digest {
        let mut ctx = Context::new(self);
        ctx.update(&[0x01]);
        ctx.update(left.as_ref());
        ctx.update(right.as_ref());
        ctx.finish()
    }
}

