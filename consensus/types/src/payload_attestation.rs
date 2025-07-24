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
#[serde(bound = "E: EthSpec", untagged, deny_unknown_fields)]
#[arbitrary(bound = "E: EthSpec")]
#[ssz(enum_behaviour = "transparent")]
#[tree_hash(enum_behaviour = "transparent")]
pub struct PayloadAttestation<E: EthSpec> {
    pub aggregation_bits: BitList<E::PTCSize>,
    #[superstruct(only(Gloas), partial_getter(rename = "data_gloas"))]
    pub data: PayloadAttestationDataGloas,
    #[superstruct(only(NextFork), partial_getter(rename = "data_next_fork"))]
    pub data: PayloadAttestationDataNextFork,
    pub signature: AggregateSignature,
}

impl<E: EthSpec> PayloadAttestation<E> {
    pub fn data(&self) -> PayloadAttestationDataRef {
        match self {
            Self::Gloas(attestation) => PayloadAttestationDataRef::Gloas(&attestation.data),
            Self::NextFork(attestation) => PayloadAttestationDataRef::NextFork(&attestation.data),
        }
    }
}

impl<E: EthSpec> PayloadAttestationRef<'_, E> {
    pub fn data(&self) -> PayloadAttestationDataRef {
        match self {
            Self::Gloas(attestation) => PayloadAttestationDataRef::Gloas(&attestation.data),
            Self::NextFork(attestation) => PayloadAttestationDataRef::NextFork(&attestation.data),
        }
    }
}

#[cfg(test)]
mod payload_attestation_tests {
    use super::*;

    ssz_and_tree_hash_tests!(PayloadAttestationGloas<MinimalEthSpec>);
}
