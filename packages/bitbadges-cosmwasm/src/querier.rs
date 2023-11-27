use cosmwasm_std::{QuerierWrapper, StdResult};
use crate::query::BitBadgesQuery;
// msg::{BadgeCollection, Balance}};

pub struct BitBadgesQuerier<'a> {
    querier: &'a QuerierWrapper<'a, BitBadgesQuery>,
}

impl<'a> BitBadgesQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, BitBadgesQuery>) -> Self {
        BitBadgesQuerier { querier }
    }

    pub fn query_address_by_id<T: Into<String>>(&self, address: String) -> StdResult<String> {
        let request = BitBadgesQuery::QueryAddressById { 
            address
        }
        .into();

        self.querier.query(&request)
    }
    pub fn query_collection<T: Into<String>>(&self, collection_id: String) -> StdResult<String> {
        let request = BitBadgesQuery::QueryCollection { 
            collection_id
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_balance<T: Into<String>>(&self, collection_id: String, address: String) -> StdResult<String> {
        let request = BitBadgesQuery::QueryBalance { 
            collection_id,
            address
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_address_mapping<T: Into<String>>(&self, mapping_id: String) -> StdResult<String> {
        let request = BitBadgesQuery::QueryAddressMapping { 
            mapping_id
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_approvals_tracker<T: Into<String>>(&self, collection_id: String, approval_level: String, approver_address: String, amount_tracker_id: String, tracker_type: String, approved_address: String) -> StdResult<String> {
        let request = BitBadgesQuery::QueryApprovalsTracker { 
            collection_id,
            approval_level,
            approver_address,
            amount_tracker_id,
            tracker_type,
            approved_address
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_num_used_for_merkle_challenge<T: Into<String>>(&self, collection_id: String, approval_level: String, approver_address: String, challenge_tracker_id: String, leaf_index: String) -> StdResult<String> {
        let request = BitBadgesQuery::QueryNumUsedForMerkleChallenge { 
            collection_id,
            approval_level,
            approver_address,
            challenge_tracker_id,
            leaf_index
        }
        .into();
        self.querier.query(&request)
    }
}