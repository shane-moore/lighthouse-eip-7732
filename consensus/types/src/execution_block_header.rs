// Copyright (c) 2022 Reth Contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use crate::{Address, EthSpec, ExecutionPayloadRef, Hash256, Hash64, Uint256};
use alloy_rlp::RlpEncodable;
use metastruct::metastruct;

/// Execution block header as used for RLP encoding and Keccak hashing.
///
/// For EIP-7886 delayed execution (Gloas fork), the block header maintains
/// RLP compatibility by setting legacy fields to default values:
/// - state_root: Set to zero (actual pre-execution state is in EIP-7886 fields)
/// - receipts_root: Set to zero (actual parent receipts are in EIP-7886 fields)
/// - logs_bloom: Set to zero bloom (actual parent bloom is in EIP-7886 fields)
///
/// The EIP-7886 delayed execution fields would be appended to the RLP structure.
///
/// Credit to Reth for the type definition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[metastruct(mappings(map_execution_block_header_fields_base(exclude(
    withdrawals_root,
    blob_gas_used,
    excess_blob_gas,
    parent_beacon_block_root
)),))]
pub struct ExecutionBlockHeader {
    pub parent_hash: Hash256,
    pub ommers_hash: Hash256,
    pub beneficiary: Address,
    pub state_root: Hash256,
    pub transactions_root: Hash256,
    pub receipts_root: Hash256,
    pub logs_bloom: Vec<u8>,
    pub difficulty: Uint256,
    pub number: Uint256,
    pub gas_limit: Uint256,
    pub gas_used: Uint256,
    pub timestamp: u64,
    pub extra_data: Vec<u8>,
    pub mix_hash: Hash256,
    pub nonce: Hash64,
    pub base_fee_per_gas: Uint256,
    pub withdrawals_root: Option<Hash256>,
    pub blob_gas_used: Option<u64>,
    pub excess_blob_gas: Option<u64>,
    pub parent_beacon_block_root: Option<Hash256>,
    pub requests_root: Option<Hash256>,
    // EIP-7886 delayed execution fields (Gloas fork and later)
    pub pre_state_root: Option<Hash256>,
    pub parent_transactions_root: Option<Hash256>,
    pub parent_receipts_root: Option<Hash256>,
    pub parent_bloom: Option<Vec<u8>>,
    pub parent_requests_hash: Option<Hash256>,
    pub parent_execution_reverted: Option<bool>,
    // EIP-7886 delayed execution fields (Gloas fork and later)
    pub pre_state_root: Option<Hash256>,
    pub parent_transactions_root: Option<Hash256>,
    pub parent_receipts_root: Option<Hash256>,
    pub parent_bloom: Option<Vec<u8>>,
    pub parent_requests_hash: Option<Hash256>,
    pub parent_execution_reverted: Option<bool>,
}

impl ExecutionBlockHeader {
    #[allow(clippy::too_many_arguments)]
    pub fn from_payload<E: EthSpec>(
        payload: ExecutionPayloadRef<E>,
        rlp_empty_list_root: Hash256,
        rlp_transactions_root: Hash256,
        rlp_withdrawals_root: Option<Hash256>,
        rlp_blob_gas_used: Option<u64>,
        rlp_excess_blob_gas: Option<u64>,
        rlp_parent_beacon_block_root: Option<Hash256>,
        rlp_requests_root: Option<Hash256>,
    ) -> Self {
        // Most of these field mappings are defined in EIP-3675 except for `mixHash`, which is
        // defined in EIP-4399.
        // For EIP-7886 (Gloas fork), the execution block header uses different fields
        // to support delayed execution.

        // For EIP-7886 (Gloas fork), the execution block header uses different fields
        // to support delayed execution.

        ExecutionBlockHeader {
            parent_hash: payload.parent_hash().into_root(),
            ommers_hash: rlp_empty_list_root,
            beneficiary: payload.fee_recipient(),
            state_root: payload.state_root(),
            transactions_root: rlp_transactions_root,
            receipts_root: payload.receipts_root(),
            logs_bloom: payload.logs_bloom().clone().into(),
            difficulty: Uint256::ZERO,
            number: Uint256::saturating_from(payload.block_number()),
            gas_limit: Uint256::saturating_from(payload.gas_limit()),
            gas_used: Uint256::saturating_from(payload.gas_used()),
            timestamp: payload.timestamp(),
            extra_data: payload.extra_data().clone().into(),
            mix_hash: payload.prev_randao(),
            nonce: Hash64::ZERO,
            base_fee_per_gas: payload.base_fee_per_gas(),
            withdrawals_root: rlp_withdrawals_root,
            blob_gas_used: rlp_blob_gas_used,
            excess_blob_gas: rlp_excess_blob_gas,
            parent_beacon_block_root: rlp_parent_beacon_block_root,
            requests_root: rlp_requests_root,
            // EIP-7886 delayed execution fields (automatically None for pre-Gloas forks)
            pre_state_root: payload.pre_state_root(),
            parent_transactions_root: payload.parent_transactions_root(),
            parent_receipts_root: payload.parent_receipts_root(),
            parent_bloom: payload.parent_bloom().clone().into(),
            parent_requests_hash: payload.parent_requests_hash(),
            parent_execution_reverted: payload.parent_execution_reverted(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, RlpEncodable)]
#[rlp(trailing)]
pub struct EncodableExecutionBlockHeader<'a> {
    pub parent_hash: &'a [u8],
    pub ommers_hash: &'a [u8],
    pub beneficiary: &'a [u8],
    pub state_root: &'a [u8],
    pub transactions_root: &'a [u8],
    pub receipts_root: &'a [u8],
    pub logs_bloom: &'a [u8],
    pub difficulty: Uint256,
    pub number: Uint256,
    pub gas_limit: Uint256,
    pub gas_used: Uint256,
    pub timestamp: u64,
    pub extra_data: &'a [u8],
    pub mix_hash: &'a [u8],
    pub nonce: &'a [u8],
    pub base_fee_per_gas: Uint256,
    pub withdrawals_root: Option<&'a [u8]>,
    pub blob_gas_used: Option<u64>,
    pub excess_blob_gas: Option<u64>,
    pub parent_beacon_block_root: Option<&'a [u8]>,
    pub requests_root: Option<&'a [u8]>,
    // EIP-7886 delayed execution fields (Gloas fork and later)
    pub pre_state_root: Option<&'a [u8]>,
    pub parent_transactions_root: Option<&'a [u8]>,
    pub parent_receipts_root: Option<&'a [u8]>,
    pub parent_bloom: Option<&'a [u8]>,
    pub parent_requests_hash: Option<&'a [u8]>,
    pub parent_execution_reverted: Option<bool>,
    // EIP-7886 delayed execution fields (Gloas fork and later)
}

impl<'a> From<&'a ExecutionBlockHeader> for EncodableExecutionBlockHeader<'a> {
    fn from(header: &'a ExecutionBlockHeader) -> Self {
        let mut encodable = Self {
            parent_hash: header.parent_hash.as_slice(),
            ommers_hash: header.ommers_hash.as_slice(),
            beneficiary: header.beneficiary.as_slice(),
            state_root: header.state_root.as_slice(),
            transactions_root: header.transactions_root.as_slice(),
            receipts_root: header.receipts_root.as_slice(),
            logs_bloom: header.logs_bloom.as_slice(),
            difficulty: header.difficulty,
            number: header.number,
            gas_limit: header.gas_limit,
            gas_used: header.gas_used,
            timestamp: header.timestamp,
            extra_data: header.extra_data.as_slice(),
            mix_hash: header.mix_hash.as_slice(),
            nonce: header.nonce.as_slice(),
            base_fee_per_gas: header.base_fee_per_gas,
            withdrawals_root: None,
            blob_gas_used: header.blob_gas_used,
            excess_blob_gas: header.excess_blob_gas,
            parent_beacon_block_root: None,
            requests_root: None,
            // EIP-7886 delayed execution fields
            pre_state_root: None,
            parent_transactions_root: None,
            parent_receipts_root: None,
            parent_bloom: None,
            parent_requests_hash: None,
            parent_execution_reverted: None,
        };

        // Set optional fields if present
        if let Some(withdrawals_root) = &header.withdrawals_root {
            encodable.withdrawals_root = Some(withdrawals_root.as_slice());
        }
        if let Some(parent_beacon_block_root) = &header.parent_beacon_block_root {
            encodable.parent_beacon_block_root = Some(parent_beacon_block_root.as_slice())
        }
        if let Some(requests_root) = &header.requests_root {
            encodable.requests_root = Some(requests_root.as_slice())
        }

        // EIP-7886 delayed execution fields
        if let Some(pre_state_root) = &header.pre_state_root {
            encodable.pre_state_root = Some(pre_state_root.as_slice());
        }
        if let Some(parent_transactions_root) = &header.parent_transactions_root {
            encodable.parent_transactions_root = Some(parent_transactions_root.as_slice());
        }
        if let Some(parent_receipts_root) = &header.parent_receipts_root {
            encodable.parent_receipts_root = Some(parent_receipts_root.as_slice());
        }
        if let Some(parent_bloom) = &header.parent_bloom {
            encodable.parent_bloom = Some(parent_bloom.as_slice());
        }
        if let Some(parent_requests_hash) = &header.parent_requests_hash {
            encodable.parent_requests_hash = Some(parent_requests_hash.as_slice());
        }
        if let Some(parent_execution_reverted) = header.parent_execution_reverted {
            encodable.parent_execution_reverted = Some(parent_execution_reverted);
        }

        // EIP-7886 delayed execution fields
        if let Some(pre_state_root) = &header.pre_state_root {
            encodable.pre_state_root = Some(pre_state_root.as_slice());
        }
        if let Some(parent_transactions_root) = &header.parent_transactions_root {
            encodable.parent_transactions_root = Some(parent_transactions_root.as_slice());
        }
        if let Some(parent_receipts_root) = &header.parent_receipts_root {
            encodable.parent_receipts_root = Some(parent_receipts_root.as_slice());
        }
        if let Some(parent_bloom) = &header.parent_bloom {
            encodable.parent_bloom = Some(parent_bloom.as_slice());
        }
        if let Some(parent_requests_hash) = &header.parent_requests_hash {
            encodable.parent_requests_hash = Some(parent_requests_hash.as_slice());
        }
        if let Some(parent_execution_reverted) = header.parent_execution_reverted {
            encodable.parent_execution_reverted = Some(parent_execution_reverted);
        }

        encodable
    }
}
