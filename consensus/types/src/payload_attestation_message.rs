use crate::test_utils::TestRandom;
use crate::*;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use superstruct::superstruct;
use test_random_derive::TestRandom;
use tree_hash_derive::TreeHash;

#[superstruct(
    variants(Gloas, NextFork),
    variant_attributes(
        derive(
            Debug,
            Clone,
            Serialize,
            Deserialize,
            Encode,
            Decode,
            TreeHash,
            TestRandom,
            Derivative,
            arbitrary::Arbitrary
        ),
        derivative(PartialEq, Hash),
        serde(deny_unknown_fields)
    ),
    ref_attributes(
        derive(Debug, PartialEq, TreeHash),
        tree_hash(enum_behaviour = "transparent")
    )
)]
#[derive(
    Debug, Clone, Serialize, Encode, Deserialize, TreeHash, Derivative, arbitrary::Arbitrary,
)]
#[derivative(PartialEq, Hash)]
#[serde(untagged, deny_unknown_fields)]
#[ssz(enum_behaviour = "transparent")]
#[tree_hash(enum_behaviour = "transparent")]
pub struct PayloadAttestationMessage {
    #[serde(with = "serde_utils::quoted_u64")]
    pub validator_index: u64,
    #[superstruct(only(Gloas), partial_getter(rename = "data_gloas"))]
    pub data: PayloadAttestationDataGloas,
    #[superstruct(only(NextFork), partial_getter(rename = "data_next_fork"))]
    pub data: PayloadAttestationDataNextFork,
    pub signature: Signature, // todo(eip-7732): verify if should be Signature or AggregateSignature. thinking Signature since this message is just signed by a single validator
}

impl PayloadAttestationMessage {
    pub fn data(&self) -> PayloadAttestationDataRef {
        match self {
            Self::Gloas(message) => PayloadAttestationDataRef::Gloas(&message.data),
            Self::NextFork(message) => PayloadAttestationDataRef::NextFork(&message.data),
        }
    }
}

impl<'a> PayloadAttestationMessageRef<'a> {
    pub fn data(&self) -> PayloadAttestationDataRef {
        match self {
            Self::Gloas(message) => PayloadAttestationDataRef::Gloas(&message.data),
            Self::NextFork(message) => PayloadAttestationDataRef::NextFork(&message.data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    ssz_and_tree_hash_tests!(PayloadAttestationMessageGloas);
}
