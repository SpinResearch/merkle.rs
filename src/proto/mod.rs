
mod proof;

use proof::{ Proof, Lemma, Positioned };
pub use self::proof::{ ProofProto, LemmaProto };

use protobuf::Message;
use protobuf::error::ProtobufResult;
use protobuf::core::parse_from_bytes;

impl <D, T> Proof<D, T> {

    /// Constructs a `Proof` struct from its Protobuf representation.
    pub fn from_protobuf(digest: D, proto: ProofProto) -> Option<Self> {
        proto.into_proof(digest)
    }

    /// Encode this `Proof` to its Protobuf representation.
    pub fn into_protobuf(self) -> ProofProto {
        ProofProto::from_proof(self)
    }

    /// Parse a `Proof` from its Protobuf binary representation.
    pub fn parse_from_bytes(bytes: &[u8], digest: D) -> ProtobufResult<Option<Proof<D, T>>> {
        parse_from_bytes::<ProofProto>(bytes).map(|proto| proto.into_proof(digest))
    }

    /// Serialize this `Proof` with Protobuf.
    pub fn write_to_bytes(self) -> ProtobufResult<Vec<u8>> {
        self.into_protobuf().write_to_bytes()
    }

}

impl ProofProto {

    pub fn from_proof<D, T>(proof: Proof<D, T>) -> Self  {
        let mut proto = Self::new();

        match proof {
            Proof { root_hash, lemma, .. } => {
                proto.set_root_hash(root_hash);
                proto.set_lemma(LemmaProto::from_lemma(lemma));
            }
        }

        proto
    }

    pub fn into_proof<D, T>(mut self, digest: D) -> Option<Proof<D, T>> {
        if !self.has_root_hash() || !self.has_lemma() {
            return None;
        }

        self.take_lemma().into_lemma().map(|lemma| {
            Proof::new(
                digest,
                self.take_root_hash(),
                lemma
            )
        })
    }

}

impl LemmaProto {

    pub fn from_lemma(lemma: Lemma) -> Self {
        let mut proto = Self::new();

        match lemma {
            Lemma { node_hash, sibling_hash, sub_lemma } => {
                proto.set_node_hash(node_hash);

                if let Some(sub_proto) = sub_lemma.map(|l| Self::from_lemma(*l)) {
                    proto.set_sub_lemma(sub_proto);
                }

                match sibling_hash {
                    Some(Positioned::Left(hash)) =>
                        proto.set_left_sibling_hash(hash),

                    Some(Positioned::Right(hash)) =>
                        proto.set_right_sibling_hash(hash),

                    None => {}
                }
            }
        }

        proto
    }

    pub fn into_lemma(mut self) -> Option<Lemma> {
        if !self.has_node_hash() {
            return None;
        }

        let node_hash = self.take_node_hash();

        let sibling_hash =
            if self.has_left_sibling_hash() {
                Some(Positioned::Left(self.take_left_sibling_hash()))
            }
            else if self.has_right_sibling_hash() {
                Some(Positioned::Left(self.take_right_sibling_hash()))
            }
            else {
                None
            };

        if self.has_sub_lemma() {
            // If a `sub_lemma` is present is the Protobuf,
            // then we expect it to unserialize to a valid `Lemma`,
            // otherwise we return `None`
            self.take_sub_lemma().into_lemma().map(|sub_lemma| {
                Lemma {
                    node_hash: node_hash,
                    sibling_hash: sibling_hash,
                    sub_lemma: Some(Box::new(sub_lemma))
                }
            })
        }
        else {
            // We might very well not have a sub_lemma,
            // in which case we just set it to `None`,
            // but still return a potentially valid `Lemma`.
            Some(Lemma {
                node_hash: node_hash,
                sibling_hash: sibling_hash,
                sub_lemma: None
            })
        }
    }

}

