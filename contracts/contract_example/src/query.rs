use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum QueryMsg {
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