use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum QueryMsg {
    #[serde(rename_all = "camelCase")]
    QueryAddressById {
        id: u64,
    },
    #[serde(rename_all = "camelCase")]
    QueryCollection {
        id: u64,
    },
    #[serde(rename_all = "camelCase")]
    QueryBalance {
        badge_id: u64,
        address: String,
    },
}