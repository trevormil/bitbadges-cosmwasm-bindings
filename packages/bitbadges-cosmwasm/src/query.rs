use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::CustomQuery;

use crate::{ApprovalTracker, AddressList, BadgeCollection, UserBalanceStore};

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
    QueryBalance {
      collection_id: String,
      address: String,
    },
    #[serde(rename_all = "camelCase")]
    QueryAddressList {
      list_id: String,
    },
    #[serde(rename_all = "camelCase")]
    QueryApprovalTracker {
      collection_id: String,
      approval_level: String,
      approver_address: String,
      amount_tracker_id: String,
      tracker_type: String,
      approved_address: String,
    },
    #[serde(rename_all = "camelCase")]
    QueryChallengeTracker {
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
pub struct QueryGetAddressListResponse {
    pub list: AddressList,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryGetApprovalTrackerResponse {
    pub tracker: ApprovalTracker,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryChallengeTrackerResponse {
    pub num_used: String, //Uint
}




