use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// use cosmwasm_std::{Coin, VoteOption};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ExecuteMsg {
    #[serde(rename_all = "camelCase")]
    DeleteCollectionMsg {
        collection_id: String,
    },
    // Add other messages here as needed
    // Same format as above
}
