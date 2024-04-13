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


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum BitBadgesMsg {
  // #[serde(rename_all = "camelCase")]
  // SetCollectionForProtocolMsg {
  //   creator: String,
  //   name: String,
  //   collection_id: String,
  // },

  // #[serde(rename_all = "camelCase")]
  // UnsetCollectionForProtocolMsg {
  //   creator: String,
  //   name: String,
  // },

  // #[serde(rename_all = "camelCase")]
  // CreateProtocolMsg {
  //   creator: String,
  //   name: String,
  //   uri: String,
  //   custom_data: String,
  //   is_frozen: bool,
  // },

  // #[serde(rename_all = "camelCase")]
  // UpdateProtocolMsg {
  //   creator: String,
  //   name: String,
  //   uri: String,
  //   custom_data: String,
  //   is_frozen: bool,
  // },

  // #[serde(rename_all = "camelCase")]
  // DeleteProtocolMsg {
  //   creator: String,
  //   name: String,
  // },

  #[serde(rename_all = "camelCase")]
  AddCustomData {
    data: String,
  },

  #[serde(rename_all = "camelCase")]
  DeleteCollectionMsg {
      //As a string
      collection_id: String, 
  },

  #[serde(rename_all = "camelCase")]
  CreateAddressListsMsg {
    address_lists: Vec<AddressList>,
  },

  #[serde(rename_all = "camelCase")]
  TransferBadgesMsg {
    collection_id: String,
    transfers: Vec<Transfer>,
  },

  #[serde(rename_all = "camelCase")]
  CreateCollectionMsg {
    balances_type: String,
    default_balances: UserBalanceStore,
    badges_to_create: Vec<Balance>,
    collection_permissions: CollectionPermissions,
    manager_timeline: Vec<ManagerTimeline>,
    collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
    badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
    off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
    custom_data_timeline: Vec<CustomDataTimeline>,
    collection_approvals: Vec<CollectionApproval>,
    standards_timeline: Vec<StandardsTimeline>,
    is_archived_timeline: Vec<IsArchivedTimeline>
  },


  #[serde(rename_all = "camelCase")]
  UpdateCollectionMsg {
    collection_id: String,
    badges_to_create: Vec<Balance>,
    update_collection_permissions: bool,
    collection_permissions: CollectionPermissions,
    update_manager_timeline: bool,
    manager_timeline: Vec<ManagerTimeline>,
    update_collection_metadata_timeline: bool,
    collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
    update_badge_metadata_timeline: bool,
    badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
    update_off_chain_balances_metadata_timeline: bool,
    off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
    update_custom_data_timeline: bool,
    custom_data_timeline: Vec<CustomDataTimeline>,
    update_collection_approvals: bool,
    collection_approvals: Vec<CollectionApproval>,
    update_standards_timeline: bool,
    standards_timeline: Vec<StandardsTimeline>,
    update_is_archived_timeline: bool,
    is_archived_timeline: Vec<IsArchivedTimeline>,
  },

  #[serde(rename_all = "camelCase")]
  UniversalUpdateCollectionMsg {
    collection_id: String,
    balances_type: String,
    default_balances: UserBalanceStore,
    badges_to_create: Vec<Balance>,
    update_collection_permissions: bool,
    collection_permissions: CollectionPermissions,
    update_manager_timeline: bool,
    manager_timeline: Vec<ManagerTimeline>,
    update_collection_metadata_timeline: bool,
    collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
    update_badge_metadata_timeline: bool,
    badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
    update_off_chain_balances_metadata_timeline: bool,
    off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
    update_custom_data_timeline: bool,
    custom_data_timeline: Vec<CustomDataTimeline>,
    update_collection_approvals: bool,
    collection_approvals: Vec<CollectionApproval>,
    update_standards_timeline: bool,
    standards_timeline: Vec<StandardsTimeline>,
    update_is_archived_timeline: bool,
    is_archived_timeline: Vec<IsArchivedTimeline>
  },
}

// pub fn set_collection_for_protocol_msg(
//   creator: String,
//   name: String,
//   collection_id: String,
// ) -> CosmosMsg<BitBadgesMsg> {
//   BitBadgesMsg::SetCollectionForProtocolMsg {
//     creator,
//     name,
//     collection_id,
//   }.into()
// }

// pub fn unset_collection_for_protocol_msg(
//   creator: String,
//   name: String,
// ) -> CosmosMsg<BitBadgesMsg> {
//   BitBadgesMsg::UnsetCollectionForProtocolMsg {
//     creator,
//     name,
//   }.into()
// }

// pub fn create_protocol_msg(
//   creator: String,
//   name: String,
//   uri: String,
//   custom_data: String,
//   is_frozen: bool,
// ) -> CosmosMsg<BitBadgesMsg> {
//   BitBadgesMsg::CreateProtocolMsg {
//     creator,
//     name,
//     uri,
//     custom_data,
//     is_frozen,
//   }.into()
// }

// pub fn update_protocol_msg(
//   creator: String,
//   name: String,
//   uri: String,
//   custom_data: String,
//   is_frozen: bool,
// ) -> CosmosMsg<BitBadgesMsg> {
//   BitBadgesMsg::UpdateProtocolMsg {
//     creator,
//     name,
//     uri,
//     custom_data,
//     is_frozen,
//   }.into()
// }

// pub fn delete_protocol_msg(
//   creator: String,
//   name: String,
// ) -> CosmosMsg<BitBadgesMsg> {
//   BitBadgesMsg::DeleteProtocolMsg {
//     creator,
//     name,
//   }.into()
// }

pub fn add_custom_data_msg(
  data: String,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::AddCustomData {
    data,
  }.into()
}

pub fn delete_collection_msg(
    collection_id: String,
) -> CosmosMsg<BitBadgesMsg> {
    BitBadgesMsg::DeleteCollectionMsg {
        collection_id,
    }.into()
}

pub fn address_lists_msg(
  address_lists: Vec<AddressList>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::CreateAddressListsMsg {
    address_lists,
  }.into()
}

pub fn transfer_badges_msg(
  collection_id: String,
  transfers: Vec<Transfer>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::TransferBadgesMsg {
    collection_id,
    transfers,
  }.into()
}

pub fn create_collection_msg(
  balances_type: String,
  default_balances: UserBalanceStore,
  badges_to_create: Vec<Balance>,
  collection_permissions: CollectionPermissions,
  manager_timeline: Vec<ManagerTimeline>,
  collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
  badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
  off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
  custom_data_timeline: Vec<CustomDataTimeline>,
  collection_approvals: Vec<CollectionApproval>,
  standards_timeline: Vec<StandardsTimeline>,
  is_archived_timeline: Vec<IsArchivedTimeline>
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::CreateCollectionMsg { 
    balances_type,
    default_balances: default_balances,
    badges_to_create,
    collection_permissions,
    manager_timeline,
    collection_metadata_timeline,
    badge_metadata_timeline,
    off_chain_balances_metadata_timeline,
    custom_data_timeline,
    collection_approvals,
    standards_timeline,
    is_archived_timeline,
  }.into()
}
  
pub fn update_collection_msg(
  collection_id: String,
  badges_to_create: Vec<Balance>,
  update_collection_permissions: bool,
  collection_permissions: CollectionPermissions,
  update_manager_timeline: bool,
  manager_timeline: Vec<ManagerTimeline>,
  update_collection_metadata_timeline: bool,
  collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
  update_badge_metadata_timeline: bool,
  badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
  update_off_chain_balances_metadata_timeline: bool,
  off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
  update_custom_data_timeline: bool,
  custom_data_timeline: Vec<CustomDataTimeline>,
  update_collection_approvals: bool,
  collection_approvals: Vec<CollectionApproval>,
  update_standards_timeline: bool,
  standards_timeline: Vec<StandardsTimeline>,
  update_is_archived_timeline: bool,
  is_archived_timeline: Vec<IsArchivedTimeline>,
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::UpdateCollectionMsg {
      collection_id,
      badges_to_create,
      update_collection_permissions,
      collection_permissions,
      update_manager_timeline,
      manager_timeline,
      update_collection_metadata_timeline,
      collection_metadata_timeline,
      update_badge_metadata_timeline,
      badge_metadata_timeline,
      update_off_chain_balances_metadata_timeline,
      off_chain_balances_metadata_timeline,
      update_custom_data_timeline,
      custom_data_timeline,
      update_collection_approvals,
      collection_approvals,
      update_standards_timeline,
      standards_timeline,
      update_is_archived_timeline,
      is_archived_timeline,
  }
  .into()
}

pub fn universal_update_collection_msg(
  collection_id: String,
  balances_type: String,
  default_balances: UserBalanceStore,
  badges_to_create: Vec<Balance>,
  update_collection_permissions: bool,
  collection_permissions: CollectionPermissions,
  update_manager_timeline: bool,
  manager_timeline: Vec<ManagerTimeline>,
  update_collection_metadata_timeline: bool,
  collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
  update_badge_metadata_timeline: bool,
  badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
  update_off_chain_balances_metadata_timeline: bool,
  off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
  update_custom_data_timeline: bool,
  custom_data_timeline: Vec<CustomDataTimeline>,
  update_collection_approvals: bool,
  collection_approvals: Vec<CollectionApproval>,
  update_standards_timeline: bool,
  standards_timeline: Vec<StandardsTimeline>,
  update_is_archived_timeline: bool,
  is_archived_timeline: Vec<IsArchivedTimeline>
) -> CosmosMsg<BitBadgesMsg> {
  BitBadgesMsg::UniversalUpdateCollectionMsg {
      collection_id,
      balances_type,
      default_balances: default_balances,
      badges_to_create,
      update_collection_permissions,
      collection_permissions,
      update_manager_timeline,
      manager_timeline,
      update_collection_metadata_timeline,
      collection_metadata_timeline,
      update_badge_metadata_timeline,
      badge_metadata_timeline,
      update_off_chain_balances_metadata_timeline,
      off_chain_balances_metadata_timeline,
      update_custom_data_timeline,
      custom_data_timeline,
      update_collection_approvals,
      collection_approvals,
      update_standards_timeline,
      standards_timeline,
      update_is_archived_timeline,
      is_archived_timeline
  }
  .into()
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Protocol {
  pub name: String,
  pub uri: String,
  pub custom_data: String,
  pub created_by: String,
  pub is_frozen: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddressList {
    pub list_id: String,
    pub addresses: Vec<String>,
    pub whitelist: bool,
    pub uri: String,
    pub custom_data: String,
    pub created_by: Option<String>,
    pub alias_address: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub from: String,
    pub to_addresses: Vec<String>,
    pub balances: Vec<Balance>,
    pub precalculate_balances_from_approval: Option<ApprovalIdentifierDetails>,
    pub merkle_proofs: Vec<MerkleProof>,
    pub memo: String,
    pub prioritized_approvals: Vec<ApprovalIdentifierDetails>,
    pub only_check_prioritized_approvals: bool,
    pub zk_proof_solutions: Vec<ZkProofSolution>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ZkProofSolution {
  pub public_inputs: String,
  pub proof: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ZkProof {
  // The verification key for the proof.
  pub verification_key: String,
  // The URI associated with this proof, optionally providing metadata about the proof.
  pub uri: String,
  // Arbitrary custom data associated with this proof.
  pub custom_data: String,
  // The ID of this proof
  pub zkp_tracker_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub amount: String,
    pub ownership_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UintRange {
    pub start: String,
    pub end: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalIdentifierDetails {
    pub approval_id: String,
    pub approval_level: String,
    pub approver_address: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MerkleProof {
    pub leaf: String,
    pub aunts: Vec<MerklePathItem>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MerklePathItem {
    pub aunt: String,
    pub on_right: bool,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MustOwnBadges {
    pub collection_id: String,
    pub amount_range: UintRange,
    pub ownership_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub override_with_current_time: bool,
    pub must_satify_for_all_assets: bool,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BadgeMetadata {
    pub uri: String,
    pub custom_data: String,
    pub badge_ids: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CollectionMetadata {
    pub uri: String,
    pub custom_data: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OffChainBalancesMetadata {
    pub uri: String,
    pub custom_data: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CollectionMetadataTimeline {
    pub collection_metadata: CollectionMetadata,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BadgeMetadataTimeline {
    pub badge_metadata: Vec<BadgeMetadata>,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OffChainBalancesMetadataTimeline {
    pub off_chain_balances_metadata: OffChainBalancesMetadata,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CustomDataTimeline {
    pub custom_data: String,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ManagerTimeline {
    pub manager: String,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IsArchivedTimeline {
    pub is_archived: bool,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContractAddressTimeline {
    pub contract_address: String,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StandardsTimeline {
    pub standards: Vec<String>,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ManualBalances {
    pub balances: Vec<Balance>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IncrementedBalances {
    pub start_balances: Vec<Balance>,
    pub increment_badge_ids_by: String,
    pub increment_ownership_times_by: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PredeterminedOrderCalculationMethod {
    pub use_overall_num_transfers: bool,
    pub use_per_to_address_num_transfers: bool,
    pub use_per_from_address_num_transfers: bool,
    pub use_per_initiated_by_address_num_transfers: bool,
    pub use_merkle_challenge_leaf_index: bool,
    pub challenge_tracker_id: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PredeterminedBalances {
    pub manual_balances: Vec<ManualBalances>,
    pub incremented_balances: IncrementedBalances,
    pub order_calculation_method: PredeterminedOrderCalculationMethod,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalAmounts {
    pub overall_approval_amount: String,
    pub per_to_address_approval_amount: String,
    pub per_from_address_approval_amount: String,
    pub per_initiated_by_address_approval_amount: String,
    pub amount_tracker_id: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MaxNumTransfers {
  pub overall_max_num_transfers: String,
  pub per_to_address_max_num_transfers: String,
  pub per_from_address_max_num_transfers: String,
  pub per_initiated_by_address_max_num_transfers: String,
  pub amount_tracker_id: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalTracker {
    pub num_transfers: String,
    pub amounts: Vec<Balance>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CosmosCoin {
    pub amount: String,
    pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CoinTransfer {
  pub to: String,
  pub coins: Vec<CosmosCoin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalCriteria {
  must_own_badges: Vec<MustOwnBadges>,
  merkle_challenges: Vec<MerkleChallenge>,
  zk_proofs: Vec<ZkProof>,
  coin_transfers: Vec<CoinTransfer>,
  predetermined_balances: PredeterminedBalances,
  approval_amounts: ApprovalAmounts,
  max_num_transfers: MaxNumTransfers,
  require_to_equals_initiated_by: bool,
  require_from_equals_initiated_by: bool,
  require_to_does_not_equal_initiated_by: bool,
  require_from_does_not_equal_initiated_by: bool,
  overrides_from_outgoing_approvals: bool,
  overrides_to_incoming_approvals: bool,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OutgoingApprovalCriteria {
  must_own_badges: Vec<MustOwnBadges>,
  merkle_challenges: Vec<MerkleChallenge>,
  zk_proofs: Vec<ZkProof>,
  coin_transfers: Vec<CoinTransfer>,
  predetermined_balances: PredeterminedBalances,
  approval_amounts: ApprovalAmounts,
  max_num_transfers: MaxNumTransfers,
  require_to_equals_initiated_by: bool,
  require_to_does_not_equal_initiated_by: bool,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IncomingApprovalCriteria {
  must_own_badges: Vec<MustOwnBadges>,
  merkle_challenges: Vec<MerkleChallenge>,
  zk_proofs: Vec<ZkProof>,
  coin_transfers: Vec<CoinTransfer>,
  predetermined_balances: PredeterminedBalances,
  approval_amounts: ApprovalAmounts,
  max_num_transfers: MaxNumTransfers,
  require_from_equals_initiated_by: bool,
  require_from_does_not_equal_initiated_by: bool,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MerkleChallenge {
    pub root: String,
    pub expected_proof_length: String,
    pub use_creator_address_as_leaf: bool,
    pub max_uses_per_leaf: String,
    pub uri: String,
    pub custom_data: String,
    pub challenge_tracker_id: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserOutgoingApproval {
    pub to_list_id: String,
    pub initiated_by_list_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub uri: String,
    pub custom_data: String,
    pub approval_id: String,
    pub approval_criteria: OutgoingApprovalCriteria,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserIncomingApproval {
    pub from_list_id: String,
    pub initiated_by_list_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub uri: String,
    pub custom_data: String,
    pub approval_id: String,
    pub approval_criteria: IncomingApprovalCriteria,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CollectionApproval {
    pub from_list_id: String,
    pub to_list_id: String,
    pub initiated_by_list_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub uri: String,
    pub custom_data: String,
    pub approval_id: String,
    pub approval_criteria: ApprovalCriteria,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPermissions {
    pub can_delete_collection: Vec<ActionPermission>,
    pub can_archive_collection: Vec<TimedUpdatePermission>,
    pub can_update_off_chain_balances_metadata: Vec<TimedUpdatePermission>,
    pub can_update_standards: Vec<TimedUpdatePermission>,
    pub can_update_custom_data: Vec<TimedUpdatePermission>,
    pub can_update_manager: Vec<TimedUpdatePermission>,
    pub can_update_collection_metadata: Vec<TimedUpdatePermission>,
    pub can_create_more_badges: Vec<BalancesActionPermission>,
    pub can_update_badge_metadata: Vec<TimedUpdateWithBadgeIdsPermission>,
    pub can_update_collection_approvals: Vec<CollectionApprovalPermission>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserPermissions {
    pub can_update_outgoing_approvals: Vec<UserOutgoingApprovalPermission>,
    pub can_update_incoming_approvals: Vec<UserIncomingApprovalPermission>,
    pub can_update_auto_approve_self_initiated_outgoing_transfers: Vec<ActionPermission>,
    pub can_update_auto_approve_self_initiated_incoming_transfers: Vec<ActionPermission>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CollectionApprovalPermission {
    pub from_list_id: String,
    pub to_list_id: String,
    pub initiated_by_list_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub permanently_permitted_times: Vec<UintRange>,
    pub permanenty_forbidden_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserOutgoingApprovalPermission {
    pub to_list_id: String,
    pub initiated_by_list_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub permanently_permitted_times: Vec<UintRange>,
    pub permanenty_forbidden_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserIncomingApprovalPermission {
    pub from_list_id: String,
    pub initiated_by_list_id: String,
    pub transfer_times: Vec<UintRange>,
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub permanently_permitted_times: Vec<UintRange>,
    pub permanenty_forbidden_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BalancesActionPermission {
    pub badge_ids: Vec<UintRange>,
    pub ownership_times: Vec<UintRange>,
    pub permanently_permitted_times: Vec<UintRange>,
    pub permanenty_forbidden_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ActionPermission {
    pub permanently_permitted_times: Vec<UintRange>,
    pub permanenty_forbidden_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TimedUpdatePermission {
    pub permanently_permitted_times: Vec<UintRange>,
    pub permanenty_forbidden_times: Vec<UintRange>,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TimedUpdateWithBadgeIdsPermission {
    pub badge_ids: Vec<UintRange>,
    pub permanently_permitted_times: Vec<UintRange>,
    pub permanenty_forbidden_times: Vec<UintRange>,
    pub timeline_times: Vec<UintRange>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserBalanceStore {
    pub balances: Vec<Balance>,
    pub outgoing_approvals: Vec<UserOutgoingApproval>,
    pub incoming_approvals: Vec<UserIncomingApproval>,
    pub auto_approve_self_initiated_outgoing_transfers: bool,
    pub auto_approve_self_initiated_incoming_transfers: bool,
    pub user_permissions: UserPermissions,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BadgeCollection {
  collection_id: String,
  collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
  badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
  balances_type: String,
  off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
  custom_data_timeline: Vec<CustomDataTimeline>,
  manager_timeline: Vec<ManagerTimeline>,
  collection_permissions: CollectionPermissions,
  collection_approvals: Vec<CollectionApproval>,
  standards_timeline: Vec<StandardsTimeline>,
  is_archived_timeline: Vec<IsArchivedTimeline>,
  default_balances: UserBalanceStore,
  created_by: String,
  alias_address: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnchorData {
  creator: String,
  data: String,
  timestamp: String,
}