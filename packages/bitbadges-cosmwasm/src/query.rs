use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::CustomQuery;

use crate::{ApprovalsTracker, AddressMapping, BadgeCollection, UserBalanceStore};

// use crate::msg::{UserBalance, BadgeCollection};

// implement custom query
impl CustomQuery for BitBadgesQuery {}

/// BitBadgesQuery is defines available query datas
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum BitBadgesQuery {
    #[serde(rename_all = "camelCase")]
    QueryCollection {
      collection_id: String,
    },
    #[serde(rename_all = "camelCase")]
    QueryAddressById {
      address: String,
    },
    #[serde(rename_all = "camelCase")]
    QueryBalance {
      collection_id: String,
      address: String,
    },
    #[serde(rename_all = "camelCase")]
    QueryAddressMapping {
      mapping_id: String,
    },
    #[serde(rename_all = "camelCase")]
    QueryApprovalsTracker {
      collection_id: String,
      approval_level: String,
      approver_address: String,
      amount_tracker_id: String,
      tracker_type: String,
      approved_address: String,
    },
    #[serde(rename_all = "camelCase")]
    QueryNumUsedForMerkleChallenge {
      collection_id: String,
      approval_level: String,
      approver_address: String,
      challenge_tracker_id: String,
      leaf_index: String,
    },

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryAddressByIdResponse {
    pub address: String, //cosmos address
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryGetCollectionResponse {
    pub collection: BadgeCollection,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryGetBalanceResponse {
    pub balance: UserBalanceStore,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryGetAddressMappingResponse {
    pub mapping: AddressMapping,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryGetApprovalsTrackerResponse {
    pub tracker: ApprovalsTracker,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryGetNumUsedForMerkleChallengeResponse {
    pub num_used: String, //Uint
}




