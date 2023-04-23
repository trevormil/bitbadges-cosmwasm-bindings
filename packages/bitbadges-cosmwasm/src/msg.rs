use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::CosmosMsg;
use cosmwasm_std::CustomMsg;

// implement custom msg
impl CustomMsg for BitBadgesMsg {}

// this is a helper to be able to return these as CosmosMsg easier
impl From<BitBadgesMsg> for CosmosMsg<BitBadgesMsg> {
    fn from(original: BitBadgesMsg) -> Self {
        CosmosMsg::Custom(original)
    }
}

//define BadgeSupply
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BadgeSupply {
    pub supply: u64,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BadgeUri {
    pub uri: String,
    pub badge_ids: Vec<IdRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdRange {
    pub start: u64,
    pub end: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Addresses {
    pub account_ids: Vec<IdRange>,
    pub options: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TransferMapping {
    pub from: Addresses,
    pub to: Addresses,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Transfers {
    pub to_addresses: Vec<u64>,
    pub balances: Vec<Balance>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub balance: u64,
    pub badge_ids: Vec<IdRange>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserBalance {
    pub balances: Vec<Balance>,
    pub approvals: Vec<Approval>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Approval {
    pub address: u64,
    pub balances: Vec<Balance>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub balances: Vec<Balance>,
    pub code_root: String,
    pub expected_merkle_proof_length: u64,
    pub whitelist_root: String,
    pub increment_ids_by: u64,
    pub amount: u64,
    pub badge_ids: Vec<IdRange>,
    pub restrict_options: u64,
    pub uri: String,
    pub time_range: IdRange,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BadgeCollection {
    collection_id: u64,
    collection_uri: String,
    badge_uris: Vec<BadgeUri>,
    bytes: String,
    manager: u64,
    permissions: u64,
    disallowed_transfers: Vec<TransferMapping>,
    manager_approved_transfers: Vec<TransferMapping>,
    next_badge_id: u64,
    unminted_supplys: Vec<Balance>,
    max_supplys: Vec<Balance>,
    claims: Vec<Claims>,
    standard: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClaimProof {
    pub leaf: String,
    pub aunts: Vec<ClaimProofItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClaimProofItem {
    pub aunt: String,
    pub on_right: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum BitBadgesMsg {
    #[serde(rename_all = "camelCase")]
    RegisterAddressesMsg {
        addresses_to_register: Vec<String>,
    },
    #[serde(rename_all = "camelCase")]
    NewCollectionMsg {
        standard: u64,
        badge_supplys: Vec<BadgeSupply>,
        collection_uri: String,
        badge_uris: Vec<BadgeUri>,
        permissions: u64,
        disallowed_transfers: Vec<TransferMapping>,
        manager_approved_transfers: Vec<TransferMapping>,
        bytes: String,
        transfers: Vec<Transfers>,
        claims: Vec<Claims>,
    },
    #[serde(rename_all = "camelCase")]
    MintBadgeMsg {
        collection_id: u64,
        badge_supplys: Vec<BadgeSupply>,
        transfers: Vec<Transfers>,
        claims: Vec<Claims>,
        collection_uri: String,
        badge_uris: Vec<BadgeUri>,
    },
    #[serde(rename_all = "camelCase")]
    ClaimBadgeMsg {
        claim_id: u64,
        collection_id: u64,
        whitelist_proof: ClaimProof,
        code_proof: ClaimProof,
    },
    #[serde(rename_all = "camelCase")]
    DeleteCollectionMsg {
        collection_id: u64,
    },
    #[serde(rename_all = "camelCase")]
    RequestTransferManagerMsg {
        collection_id: u64,
        address: u64,
    },
    #[serde(rename_all = "camelCase")]
    SetApprovalMsg {
        collection_id: u64,
        address: u64,
        balances: Vec<Balance>,
    },
    #[serde(rename_all = "camelCase")]
    TransferBadgeMsg {
        collection_id: u64,
        from: u64,
        transfers: Vec<Transfers>,
    },
    #[serde(rename_all = "camelCase")]
    TransferManagerMsg {
        collection_id: u64,
        address: u64,
    },
    #[serde(rename_all = "camelCase")]
    UpdateBytesMsg {
        collection_id: u64,
        new_bytes: String,
    },
    #[serde(rename_all = "camelCase")]
    UpdateDisallowedTransfersMsg {
        collection_id: u64,
        new_disallowed_transfers: Vec<TransferMapping>,
    },
    #[serde(rename_all = "camelCase")]
    UpdatePermissionsMsg {
        collection_id: u64,
        permissions: u64,
    },
    #[serde(rename_all = "camelCase")]
    UpdateUrisMsg {
        collection_id: u64,
        new_collection_uri: String,
        new_badge_uris: Vec<BadgeUri>,
    },
}

pub fn create_register_addresses_msg(
    addresses_to_register: Vec<String>,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::RegisterAddressesMsg {
        addresses_to_register,
    }.into()
}

pub fn create_new_collection_msg(
    standard: u64,
    badge_supplys: Vec<BadgeSupply>,
    collection_uri: String,
    badge_uris: Vec<BadgeUri>,
    permissions: u64,
    disallowed_transfers: Vec<TransferMapping>,
    manager_approved_transfers: Vec<TransferMapping>,
    bytes: String,
    transfers: Vec<Transfers>,
    claims: Vec<Claims>,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::NewCollectionMsg {
        standard,
        badge_supplys,
        collection_uri,
        badge_uris,
        permissions,
        disallowed_transfers,
        manager_approved_transfers,
        bytes,
        transfers,
        claims,
    }.into()
}

pub fn create_mint_badge_msg(
    collection_id: u64,
    badge_supplys: Vec<BadgeSupply>,
    transfers: Vec<Transfers>,
    claims: Vec<Claims>,
    collection_uri: String,
    badge_uris: Vec<BadgeUri>,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::MintBadgeMsg {
        collection_id,
        badge_supplys,
        transfers,
        claims,
        collection_uri,
        badge_uris,
    }.into()
}

pub fn create_claim_badge_msg(
    claim_id: u64,
    collection_id: u64,
    whitelist_proof: ClaimProof,
    code_proof: ClaimProof,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::ClaimBadgeMsg {
        claim_id,
        collection_id,
        whitelist_proof,
        code_proof,
    }.into()
}

pub fn create_delete_collection_msg(
    collection_id: u64,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::DeleteCollectionMsg {
        collection_id,
    }.into()
}

pub fn create_request_transfer_manager_msg(
    collection_id: u64,
    address: u64,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::RequestTransferManagerMsg {
        collection_id,
        address,
    }.into()
}

pub fn create_set_approval_msg(
    collection_id: u64,
    address: u64,
    balances: Vec<Balance>,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::SetApprovalMsg {
        collection_id,
        address,
        balances,
    }.into()
}

pub fn create_transfer_badge_msg(
    collection_id: u64,
    from: u64,
    transfers: Vec<Transfers>,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::TransferBadgeMsg {
        collection_id,
        from,
        transfers,
    }.into()
}

pub fn create_transfer_manager_msg(
    collection_id: u64,
    address: u64,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::TransferManagerMsg {
        collection_id,
        address,
    }.into()
}

pub fn create_update_bytes_msg(
    collection_id: u64,
    new_bytes: String,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::UpdateBytesMsg {
        collection_id,
        new_bytes,
    }.into()
}

pub fn create_update_disallowed_transfers_msg(
    collection_id: u64,
    new_disallowed_transfers: Vec<TransferMapping>,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::UpdateDisallowedTransfersMsg {
        collection_id,
        new_disallowed_transfers,
    }.into()
}

pub fn create_update_permissions_msg(
    collection_id: u64,
    permissions: u64,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::UpdatePermissionsMsg {
        collection_id,
        permissions,
    }.into()
}

pub fn create_update_uris_msg(
    collection_id: u64,
    new_collection_uri: String,
    new_badge_uris: Vec<BadgeUri>,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::UpdateUrisMsg {
        collection_id,
        new_collection_uri,
        new_badge_uris,
    }.into()
}
