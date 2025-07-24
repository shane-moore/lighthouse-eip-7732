use crate::test_utils::TestRandom;
use crate::*;
use core::slice::Iter;
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
        derivative(PartialEq, Hash(bound = "E: EthSpec")),
        serde(bound = "E: EthSpec", deny_unknown_fields),
        arbitrary(bound = "E: EthSpec")
    ),
    ref_attributes(
        derive(Debug, PartialEq, TreeHash),
        tree_hash(enum_behaviour = "transparent")
    )
)]
#[derive(
    Debug, Clone, Serialize, Encode, Deserialize, TreeHash, Derivative, arbitrary::Arbitrary,
)]
#[derivative(PartialEq, Hash(bound = "E: EthSpec"))]
#[serde(bound = "E: EthSpec", untagged)]
#[arbitrary(bound = "E: EthSpec")]
#[ssz(enum_behaviour = "transparent")]
#[tree_hash(enum_behaviour = "transparent")]
pub struct IndexedPayloadAttestation<E: EthSpec> {
    pub attesting_indices: VariableList<u64, E::PTCSize>,
    #[superstruct(only(Gloas), partial_getter(rename = "data_gloas"))]
    pub data: PayloadAttestationDataGloas,
    #[superstruct(only(NextFork), partial_getter(rename = "data_next_fork"))]
    pub data: PayloadAttestationDataNextFork,
    pub signature: AggregateSignature,
}

impl<E: EthSpec> IndexedPayloadAttestation<E> {
    pub fn attesting_indices_iter(&self) -> Iter<'_, u64> {
        self.attesting_indices().iter()
    }

    pub fn data(&self) -> PayloadAttestationDataRef {
        match self {
            Self::Gloas(attestation) => PayloadAttestationDataRef::Gloas(&attestation.data),
            Self::NextFork(attestation) => PayloadAttestationDataRef::NextFork(&attestation.data),
        }
    }
}

impl<E: EthSpec> IndexedPayloadAttestationRef<'_, E> {
    pub fn data(&self) -> PayloadAttestationDataRef {
        match self {
            Self::Gloas(attestation) => PayloadAttestationDataRef::Gloas(&attestation.data),
            Self::NextFork(attestation) => PayloadAttestationDataRef::NextFork(&attestation.data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    ssz_and_tree_hash_tests!(IndexedPayloadAttestationGloas<MainnetEthSpec>);
}
