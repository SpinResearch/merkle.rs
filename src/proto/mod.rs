
mod proof;

use proof::{ Proof, Lemma, Positioned };
pub use self::proof::{ ProofProto, LemmaProto };

impl ProofProto {

    pub fn from<D, T>(proof: Proof<D, T>) -> Self  {
        let mut proto = Self::new();

        match proof {
            Proof { root_hash, lemma, .. } => {
                proto.set_root_hash(root_hash);
                proto.set_lemma(LemmaProto::from(lemma));
            }
        }

        proto
    }

}

impl LemmaProto {

    pub fn from(lemma: Lemma) -> Self {
        let mut proto = Self::new();

        match lemma {
            Lemma { node_hash, sibling_hash, sub_lemma } => {
                proto.set_node_hash(node_hash);

                if let Some(sub_proto) = sub_lemma.map(|l| Self::from(*l)) {
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

}

