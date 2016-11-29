
mod proof;

use ring::digest::Algorithm;

use proof::{ Proof, Lemma, Positioned };
pub use self::proof::{ ProofProto, LemmaProto };

use protobuf::Message;
use protobuf::error::ProtobufResult;
use protobuf::core::parse_from_bytes;

impl <T> Proof<T> {

    /// Constructs a `Proof` struct from its Protobuf representation.
    pub fn from_protobuf(algorithm: &'static Algorithm, proto: ProofProto) -> Option<Self>
        where T: From<Vec<u8>>
    {
        proto.into_proof(algorithm)
    }

    /// Encode this `Proof` to its Protobuf representation.
    pub fn into_protobuf(self) -> ProofProto
        where T: Into<Vec<u8>>
    {
        ProofProto::from_proof(self)
    }

    /// Parse a `Proof` from its Protobuf binary representation.
    pub fn parse_from_bytes(bytes: &[u8], algorithm: &'static Algorithm) -> ProtobufResult<Option<Self>>
        where T: From<Vec<u8>>
    {
        parse_from_bytes::<ProofProto>(bytes).map(|proto| proto.into_proof(algorithm))
    }

    /// Serialize this `Proof` with Protobuf.
    pub fn write_to_bytes(self) -> ProtobufResult<Vec<u8>> where T: Into<Vec<u8>>  {
        self.into_protobuf().write_to_bytes()
    }

}

impl ProofProto {

    pub fn from_proof<T>(proof: Proof<T>) -> Self
        where T: Into<Vec<u8>>
    {
        let mut proto = Self::new();

        match proof {
            Proof { root_hash, lemma, value, .. } => {
                proto.set_root_hash(root_hash);
                proto.set_lemma(LemmaProto::from_lemma(lemma));
                proto.set_value(value.into());
            }
        }

        proto
    }

    pub fn into_proof<T>(mut self, algorithm: &'static Algorithm) -> Option<Proof<T>>
        where T: From<Vec<u8>>
    {
        if !self.has_root_hash() || !self.has_lemma() {
            return None;
        }

        self.take_lemma().into_lemma().map(|lemma| {
            Proof::new(
                algorithm,
                self.take_root_hash(),
                lemma,
                self.take_value().into()
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
                Some(Positioned::Right(self.take_right_sibling_hash()))
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

