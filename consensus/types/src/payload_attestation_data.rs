use crate::test_utils::TestRandom;
use crate::*;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use superstruct::superstruct;
use test_random_derive::TestRandom;
use tree_hash_derive::TreeHash;

/// The data upon which a ptc attestation is based.
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
pub struct PayloadAttestationData {
    pub beacon_block_root: Hash256,
    pub slot: Slot,
    #[superstruct(only(Gloas), partial_getter(rename = "payload_status_gloas"))]
    pub payload_status: bool,
    #[superstruct(only(NextFork), partial_getter(rename = "payload_status_next_fork"))]
    pub payload_status: bool,
}

impl SignedRoot for PayloadAttestationData {}

impl PayloadAttestationData {
    pub fn payload_status(&self) -> bool {
        match self {
            Self::Gloas(data) => data.payload_status,
            Self::NextFork(data) => data.payload_status,
        }
    }
}

impl<'a> PayloadAttestationDataRef<'a> {
    pub fn payload_status(&self) -> bool {
        match self {
            Self::Gloas(data) => data.payload_status,
            Self::NextFork(data) => data.payload_status,
        }
    }
}

#[cfg(test)]
mod payload_attestation_data_tests {
    use super::*;

    ssz_and_tree_hash_tests!(PayloadAttestationDataGloas);
}
